use prost::Message;
use std::{io::Cursor, path::Path, process::{Command, Stdio}, sync::{mpsc, Arc}};
pub mod pb {
    tonic::include_proto!("openvslam_api"); // The string specified here must match the proto package name
}
use tokio::sync::{oneshot, watch, Mutex};
use tokio_stream::{wrappers::WatchStream};

use nalgebra as na;
use common::{types::{Keyframe, Landmark, TrackingState, Iso3, Point3}, settings::Settings};

// fn mat44_to_iso3(m: pb::stream::Mat44) -> Iso3 {
//     let translation = na::Translation3::new(m.m14, m.m24, m.m34);
//     let rotation = na::Matrix3::new(m.m11, m.m12, m.m13, m.m21, m.m22, m.m23, m.m31, m.m32, m.m33);
//     let rotation = na::Rotation3::from_matrix(&rotation);
//     let rotation = na::UnitQuaternion::from_rotation_matrix(&rotation);
//     na::Isometry3::from_parts(translation, rotation)
// }

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
    request_sender: Arc<Mutex<mpsc::Sender<ApiRequest>>>,
    // request_receiver: mpsc::Receiver<ApiRequest>,
    thread: Arc<Mutex<std::thread::JoinHandle<()>>>,

    camera_position_receiver: watch::Receiver<Option<Iso3>>,
    landmarks_receiver: watch::Receiver<Vec<Landmark>>,
    keyframes_receiver: watch::Receiver<Vec<Keyframe>>,
    tracking_state_receiver: watch::Receiver<TrackingState>,
    frame_receiver: watch::Receiver<Option<Vec<u8>>>,
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
    pub fn new() -> anyhow::Result<Self> {
        let settings = Settings::new()?;
        let (request_sender, mut request_receiver) = mpsc::channel::<ApiRequest>();
        let mut openvslam_process = {
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
            

            cmd.spawn().expect("failed to start OpenVSlam")
        };

        let context = zmq::Context::new();

        let req_handle = std::thread::spawn(move || {
            let mut req = context.socket(zmq::REQ).unwrap();
            req.connect("ipc:///tmp/openvslam_wrapper_ipc_request")
                .expect("failed to connect OpenVSlam response socket");

            loop {
                let api_request = request_receiver.recv();
                {
                    if let Ok(api_request) = api_request {
                        let mut msg = pb::Request::default();
                        msg.msg = Some(api_request.request);
                        let mut buf = Vec::new();
                        buf.reserve(msg.encoded_len());
                        msg.encode(&mut buf).expect("failed to encode message");
                        req.send(&buf, 0).expect("failed to send request");
                        let response = req.recv_bytes(0).expect("failed to receive response");
                        let response = pb::Response::decode(&mut Cursor::new(response))
                            .expect("failed to decode response");
                        let _ = api_request.callback.send(response);
                    } else {
                        println!("api request channel closed");
                        break;
                    }
                }
            }
        });

        let (camera_position_sender, camera_position_receiver) =
            watch::channel::<Option<Iso3>>(None);
        let (landmarks_sender, landmarks_receiver) = watch::channel::<Vec<Landmark>>(Vec::new());
        let (keyframes_sender, keyframes_receiver) = watch::channel::<Vec<Keyframe>>(Vec::new());
        let (tracking_state_sender, tracking_state_receiver) = watch::channel::<TrackingState>(TrackingState::NotInitialized);
        let (frame_sender, frame_receiver) = watch::channel::<Option<Vec<u8>>>(None);

        let stream_handle = std::thread::spawn(move || {
            let context = zmq::Context::new();
            let mut stream = context.socket(zmq::PULL).unwrap();
            stream
                .connect("ipc:///tmp/openvslam_wrapper_ipc_stream")
                .expect("failed to connect OpenVSlam response socket");

            // Convert OpenVSlam coordinate frame (right-hand y-down z-ahead) to right-hand y-up z-back
            let rot_z_up = na::Rotation3::new(na::Vector3::x() * std::f64::consts::PI);
            let iso_z_up = Iso3::from_parts(na::Translation3::identity(), rot_z_up.into());
            let z_up_point = |p|-> Point3 {
                rot_z_up * p
            };
            let z_up_iso = |iso: Iso3| -> Iso3 {
                iso_z_up * iso
            };

            loop {
                let message = stream
                    .recv_bytes(0)
                    .expect("failed to receive stream message");
                let message = pb::Stream::decode(&mut Cursor::new(message))
                    .expect("failed to decode stream message");

                if let Some(msg) = message.msg {
                    match msg {
                        pb::stream::Msg::CameraPosition(transform) => {
                            camera_position_sender
                                .send(Some(z_up_iso(mat44_to_iso3(&transform))))
                                .unwrap();
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
                            landmarks_sender.send(landmarks).unwrap();
                        }
                        pb::stream::Msg::Keyframes(keyframes) => {
                            let keyframes: Vec<Keyframe> = keyframes.keyframes.iter().map(|pb_keyframe| {
                                let pose_mat = pb_keyframe.pose.as_ref().expect("pose in keyframe can't be empty");
                                Keyframe {
                                    id: pb_keyframe.id,
                                    pose: z_up_iso(mat44_to_iso3(pose_mat)),
                                }
                            }).collect();
                            keyframes_sender.send(keyframes).unwrap();
                        }
                        pb::stream::Msg::TrackingState(pb_tracking_state) => {
                            let pb_tracking_state = pb::stream::TrackingState::from_i32(pb_tracking_state).expect("unknown tracking state");
                           let tracking_state = match pb_tracking_state {
                                pb::stream::TrackingState::NotInitialized => TrackingState::NotInitialized,
                                pb::stream::TrackingState::Initializing => TrackingState::Initializing,
                                pb::stream::TrackingState::Tracking => TrackingState::Tracking,
                                pb::stream::TrackingState::Lost => TrackingState::Lost,
                            };
                            tracking_state_sender.send(tracking_state).unwrap();
                        }
                        pb::stream::Msg::Frame(pb_frame) => {
                            frame_sender.send(Some(pb_frame.jpeg)).unwrap();
                        }
                    }
                }
            }
        });

        let thread = std::thread::spawn(move || {
            let exit_status = openvslam_process.wait();
            println!("OpenVSlam closed with status: {:?}", exit_status);
            req_handle.join();
        });

        let request_sender = Arc::new(Mutex::new(request_sender));
        let thread = Arc::new(Mutex::new(thread));
        Ok(OpenVSlamWrapper {
            request_sender,
            thread,
            camera_position_receiver,
            landmarks_receiver,
            keyframes_receiver,
            tracking_state_receiver,
            frame_receiver,
        })
    }

    pub async fn terminate(&self) {
        let request =
            pb::request::Msg::Terminate(pb::request::Terminate::default());
        let (callback, rx) = oneshot::channel();
        let sender = self.request_sender.lock().await;
        println!("{:?}", sender.send(ApiRequest { request, callback }));

        println!("{:?}", rx.await);
    }

    pub async fn save_map_db(&self, path: String) {
        let request =
            pb::request::Msg::SaveMapDb(pb::request::SaveMapDb { path });
        let (callback, rx) = oneshot::channel();
        let sender = self.request_sender.lock().await;
        println!("{:?}", sender.send(ApiRequest { request, callback }));

        println!("{:?}", rx.await);
    }

    pub fn stream_position(&self) -> WatchStream<Option<Iso3>> {
        WatchStream::new(self.camera_position_receiver.clone())
    }

    pub fn stream_landmarks(&self) -> WatchStream<Vec<Landmark>> {
        WatchStream::new(self.landmarks_receiver.clone())
    }

    pub fn stream_keyframes(&self) -> WatchStream<Vec<Keyframe>> {
        WatchStream::new(self.keyframes_receiver.clone())
    }

    pub fn stream_tracking_state(&self) -> WatchStream<TrackingState> {
        WatchStream::new(self.tracking_state_receiver.clone())
    }

    pub fn stream_frame(&self) -> WatchStream<Option<Vec<u8>>> {
        WatchStream::new(self.frame_receiver.clone())
    }
}
