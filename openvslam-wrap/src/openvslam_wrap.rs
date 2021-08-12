use prost::Message;
use std::{io::Cursor, path::Path, process::{Child, Command, Stdio, exit}, sync::{Arc}};
pub mod pb {
    tonic::include_proto!("openvslam_api"); // The string specified here must match the proto package name
}
use tokio::sync::{oneshot, Mutex};

use common::{
    msg::{Broadcaster, Msg},
    settings::Settings,
    types::{Edge, Iso3, Keyframe, Landmark, Point3, TrackingState},
};
use nalgebra as na;

fn mat44_to_iso3(m: &pb::stream::Mat44) -> Iso3 {
    let d = m.pose.to_vec();
    let translation = na::Translation3::new(d[3], d[7], d[11]);
    let rotation = na::Matrix3::new(d[0], d[1], d[2], d[4], d[5], d[6], d[8], d[9], d[10]);
    let rotation = na::Rotation3::from_matrix(&rotation);
    let rotation = na::UnitQuaternion::from_rotation_matrix(&rotation);
    na::Isometry3::from_parts(translation, rotation)
}

#[derive(Debug)]
struct ApiRequest {
    request: pb::request::Msg,
    callback: oneshot::Sender<pb::Response>,
}

#[derive(Debug)]
pub struct OpenVSlamWrapper {
    thread: Arc<Mutex<std::thread::JoinHandle<()>>>,
}

fn get_path(path: &str) -> String {
    let path = Path::new("./openvslam-wrap").join(path);
    path.canonicalize()
        .expect(&format!(
            "can't find {:?} from {:?}",
            path,
            Path::new(".").canonicalize()
        ))
        .to_str()
        .unwrap()
        .into()
}

impl OpenVSlamWrapper {
    pub fn run_with_auto_restart(broadcaster: &Broadcaster, async_handle: tokio::runtime::Handle) -> anyhow::Result<Self> {
        let context = zmq::Context::new();

        let req_handle = {
            let mut receiver = broadcaster.subscribe();
            std::thread::spawn(move || {
                let req = context.socket(zmq::REQ).unwrap();
                req.connect("ipc:///tmp/openvslam_wrapper_ipc_request")
                    .expect("failed to connect OpenVSlam response socket");

                loop {
                    let msg = async_handle.block_on(async { receiver.recv().await });
                    
                    
                    if let Ok(msg) = msg {
                        let pb_request = match msg {
                            Msg::TerminateSlam => Some(pb::request::Msg::Terminate(
                                pb::request::Terminate::default(),
                            )),
                            Msg::SaveMapDB(path) => {
                                Some(pb::request::Msg::SaveMapDb(pb::request::SaveMapDb { path }))
                            }
                            Msg::UseRawPreview(use_raw) => {
                                let preview_type = if use_raw {
                                    pb::request::PreviewType::Raw
                                }
                                else {
                                    pb::request::PreviewType::SlamInfo
                                };
                                Some(pb::request::Msg::SetPreviewType(preview_type.into()))
                            }
                            _ => None,
                        };
                        if let Some(pb_request) = pb_request {
                            println!("openvslam wrap got command! {:?}", pb_request);
                            let mut msg = pb::Request::default();
                            msg.msg = Some(pb_request);
                            let mut buf = Vec::new();
                            buf.reserve(msg.encoded_len());
                            msg.encode(&mut buf).expect("failed to encode message");
                            req.send(&buf, 0).expect("failed to send request");
                            let response = req.recv_bytes(0).expect("failed to receive response");
                            let _response = pb::Response::decode(&mut Cursor::new(response))
                                .expect("failed to decode response");
                        }
                    }
                }
            })
        };

        let _stream_handle = {
            let publisher = broadcaster.publisher();
            std::thread::spawn(move || {
                let context = zmq::Context::new();
                let stream = context.socket(zmq::PULL).unwrap();
                stream
                    .connect("ipc:///tmp/openvslam_wrapper_ipc_stream")
                    .expect("failed to connect OpenVSlam response socket");

                // Convert OpenVSlam coordinate frame (right-hand y-down z-ahead) to right-hand y-up z-back
                let rot_z_up = na::Rotation3::new(na::Vector3::x() * std::f64::consts::PI);
                let iso_z_up = Iso3::from_parts(na::Translation3::identity(), rot_z_up.into());
                let z_up_point = |p| -> Point3 { rot_z_up * p };
                let z_up_iso = |iso: Iso3| -> Iso3 { iso_z_up * iso };

                loop {
                    let message = stream
                        .recv_bytes(0)
                        .expect("failed to receive stream message");
                    let message = pb::Stream::decode(&mut Cursor::new(message))
                        .expect("failed to decode stream message");

                    if let Some(pb_msg) = message.msg {
                        let msg = match pb_msg {
                            pb::stream::Msg::CameraPosition(transform) => {
                                Msg::CameraPose(z_up_iso(mat44_to_iso3(&transform)))
                            }
                            pb::stream::Msg::Landmarks(landmarks) => {
                                let landmarks: Vec<Landmark> = landmarks
                                    .landmarks
                                    .iter()
                                    .map(|lm| Landmark {
                                        id: lm.id,
                                        point: z_up_point(Point3::new(lm.x, lm.y, lm.z)),
                                        num_observations: lm.num_observations,
                                    })
                                    .collect();
                                Msg::Landmarks(landmarks)
                            }
                            pb::stream::Msg::Keyframes(keyframes) => {
                                let keyframes: Vec<Keyframe> = keyframes
                                    .keyframes
                                    .iter()
                                    .map(|pb_keyframe| {
                                        let pose_mat = pb_keyframe
                                            .pose
                                            .as_ref()
                                            .expect("pose in keyframe can't be empty");
                                        Keyframe {
                                            id: pb_keyframe.id,
                                            pose: z_up_iso(mat44_to_iso3(pose_mat)),
                                        }
                                    })
                                    .collect();
                                Msg::Keyframes(keyframes)
                            }
                            pb::stream::Msg::Edges(edges) => {
                                let edges: Vec<Edge> = edges
                                    .edges
                                    .iter()
                                    .map(|pb_edge| Edge {
                                        id0: pb_edge.id0,
                                        id1: pb_edge.id1,
                                    })
                                    .collect();
                                Msg::Edges(edges)
                            }
                            pb::stream::Msg::TrackingState(pb_tracking_state) => {
                                let pb_tracking_state =
                                    pb::stream::TrackingState::from_i32(pb_tracking_state)
                                        .expect("unknown tracking state");
                                let tracking_state = match pb_tracking_state {
                                    pb::stream::TrackingState::NotInitialized => {
                                        TrackingState::NotInitialized
                                    }
                                    pb::stream::TrackingState::Initializing => {
                                        TrackingState::Initializing
                                    }
                                    pb::stream::TrackingState::Tracking => TrackingState::Tracking,
                                    pb::stream::TrackingState::Lost => TrackingState::Lost,
                                };
                                Msg::TrackingState(tracking_state)
                            }
                            pb::stream::Msg::Frame(pb_frame) => Msg::Frame(pb_frame.jpeg)
                        };
                        publisher.send(msg).ok();
                    }
                }
            })
        };

        let thread = std::thread::spawn(move || {
            loop {
                println!("Starting OpenVSlam...");
                let exit_status = Self::start_openvslam_thread().wait();
                println!("OpenVSlam closed with status: {:?}", exit_status);
                if exit_status.is_ok() && exit_status.unwrap().success() {
                    println!("OpenVSlam terminated without error. Restarting...");
                }
                else {
                    break;
                }
            }
        });

        let thread = Arc::new(Mutex::new(thread));
        Ok(OpenVSlamWrapper {
            thread,
        })
    }

    fn start_openvslam_thread() -> Child {
        let settings = Settings::new().expect("Failed to read settings");

        let bin = get_path("openvslam/build/run_api");

        let mut cmd = Command::new(bin);
        cmd.stdin(Stdio::null());

        let config = settings.slam.openvslam_config;
        // let config = "config/cfg.yaml";
        cmd.arg("-c").arg(config);

        let vocab = settings.slam.vocab;
        cmd.arg("-v").arg(vocab);

        if let Some(video) = settings.slam.video {
            let video = video;
            cmd.arg("-m").arg(video);
        }

        if let Some(mask) = settings.slam.mask {
            let mask = mask;
            cmd.arg("--mask").arg(mask);
        }

        if let Some(map) = settings.slam.map {
            let map = map;
            cmd.arg("--map").arg(map);
        }

        if let Some(gstreamer_pipeline) = settings.slam.gstreamer_pipeline {
            let gstreamer_pipeline = gstreamer_pipeline;
            cmd.arg("--gstreamer_pipeline").arg(gstreamer_pipeline);
        }

        cmd.spawn().expect("failed to start OpenVSlam")
    }
}
