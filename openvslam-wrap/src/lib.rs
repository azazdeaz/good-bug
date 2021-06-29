use prost::Message;
use std::{
    io::Cursor,
    path::Path,
    process::{Command, Stdio},
    sync::{mpsc, Arc, RwLock},
};
pub mod openvslam_api {
    tonic::include_proto!("openvslam_api"); // The string specified here must match the proto package name
}
use tokio::sync::{oneshot, watch, Mutex};
use tokio_stream::{wrappers::WatchStream, StreamExt};

use nalgebra as na;
type Iso3 = na::Isometry3<f64>;
use common::types::Landmark;

// fn mat44_to_iso3(m: openvslam_api::stream::Mat44) -> Iso3 {
//     let translation = na::Translation3::new(m.m14, m.m24, m.m34);
//     let rotation = na::Matrix3::new(m.m11, m.m12, m.m13, m.m21, m.m22, m.m23, m.m31, m.m32, m.m33);
//     let rotation = na::Rotation3::from_matrix(&rotation);
//     let rotation = na::UnitQuaternion::from_rotation_matrix(&rotation);
//     na::Isometry3::from_parts(translation, rotation)
// }

fn mat44_to_iso3(m: openvslam_api::stream::Mat44) -> Iso3 {
    let d = m.pose.to_vec();
    let translation = na::Translation3::new(d[3], d[7], d[11]);
    let rotation = na::Matrix3::new(d[0], d[1], d[2], d[4], d[5], d[6], d[8], d[9], d[10]);
    let rotation = na::Rotation3::from_matrix(&rotation);
    let rotation = na::UnitQuaternion::from_rotation_matrix(&rotation);
    // let rotation = UnitQuaternion::from_basis_unchecked(&[
    //     na::Vector3::new(d[0], d[1], d[2]),
    //     na::Vector3::new(d[4], d[5], d[6]),
    //     na::Vector3::new(d[8], d[9], d[10]),
    // ]);
    na::Isometry3::from_parts(translation, rotation)
}

#[derive(Debug)]
struct ApiRequest {
    request: openvslam_api::request::Msg,
    callback: oneshot::Sender<openvslam_api::Response>,
}

#[derive(Debug)]
pub struct OpenVSlamWrapper {
    request_sender: Arc<Mutex<mpsc::Sender<ApiRequest>>>,
    // request_receiver: mpsc::Receiver<ApiRequest>,
    thread: Arc<Mutex<std::thread::JoinHandle<()>>>,

    camera_position_receiver: watch::Receiver<Option<Iso3>>,
    landmarks_receiver: watch::Receiver<Vec<Landmark>>,
}

fn get_path(path: &str) -> String {
    let path = Path::new("./openvslam-wrap").join(path);
    path.canonicalize()
        .expect(&format!("can't find {:?} from {:?}", path, Path::new(".").canonicalize()))
        .to_str()
        .unwrap()
        .into()
}

impl OpenVSlamWrapper {
    pub fn new() -> Self {
        let (request_sender, mut request_receiver) = mpsc::channel::<ApiRequest>();
        let mut openvslam_process = {
            let bin = get_path("openvslam/build/run_api");

            let mut cmd = Command::new(bin);
            cmd.stdin(Stdio::null());

            let config = get_path("config/dataset/aist_living_lab_1/config.yaml");
            // let config = get_path("config/cfg.yaml");
            cmd.arg("-c").arg(config);

            let vocab = get_path("config/orb_vocab.fbow");
            cmd.arg("-v").arg(vocab);

            let video = get_path("config/dataset/aist_living_lab_1/video.mp4");
            cmd.arg("-m").arg(video);

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
                    println!("send 0");
                    if let Ok(api_request) = api_request {
                        let mut msg = openvslam_api::Request::default();
                        msg.msg = Some(api_request.request);
                        let mut buf = Vec::new();
                        buf.reserve(msg.encoded_len());
                        msg.encode(&mut buf).expect("failed to encode message");
                        println!("send 1 {:?}", buf);
                        req.send(&buf, 0).expect("failed to send request");
                        println!("send 2");
                        let response = req.recv_bytes(0).expect("failed to receive response");
                        println!("send 3");
                        let response = openvslam_api::Response::decode(&mut Cursor::new(response))
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
        let (landmarks_sender, landmarks_receiver) =
            watch::channel::<Vec<Landmark>>(Vec::new());

        let stream_handle = std::thread::spawn(move || {
            let context = zmq::Context::new();
            let mut stream = context.socket(zmq::PULL).unwrap();
            stream
                .connect("ipc:///tmp/openvslam_wrapper_ipc_stream")
                .expect("failed to connect OpenVSlam response socket");

            loop {
                let message = stream
                    .recv_bytes(0)
                    .expect("failed to receive stream message");
                let message = openvslam_api::Stream::decode(&mut Cursor::new(message))
                    .expect("failed to decode stream message");

                if let Some(msg) = message.msg {
                    match msg {
                        openvslam_api::stream::Msg::CameraPosition(transform) => {
                            camera_position_sender
                                .send(Some(mat44_to_iso3(transform)))
                                .unwrap();
                        }
                        openvslam_api::stream::Msg::Landmarks(landmarks) => {
                            let landmarks: Vec<Landmark> = landmarks.landmarks.iter().map(|lm| {
                                Landmark {
                                    id: lm.id,
                                    x: lm.x,
                                    y: lm.y,
                                    z: lm.z,
                                    num_observations: lm.num_observations,
                                }
                            }).collect();
                            landmarks_sender.send(landmarks).unwrap();
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
        OpenVSlamWrapper {
            request_sender,
            thread,
            camera_position_receiver,
            landmarks_receiver,
        }
    }

    pub async fn terminate(&self) {
        let request =
            openvslam_api::request::Msg::Terminate(openvslam_api::request::Terminate::default());
        let (callback, rx) = oneshot::channel();
        let sender = self.request_sender.lock().await;
        println!("{:?}", sender.send(ApiRequest { request, callback }));

        // tokio::spawn(async move {

        //     let sender = asender.lock().await;

        // }).await;

        println!("{:?}", rx.await);
    }

    pub fn stream_position(&self) -> WatchStream<Option<Iso3>> {
        WatchStream::new(self.camera_position_receiver.clone())
    }

    pub fn stream_landmarks(&self) -> WatchStream<Vec<Landmark>> {
        WatchStream::new(self.landmarks_receiver.clone())
    }
}
