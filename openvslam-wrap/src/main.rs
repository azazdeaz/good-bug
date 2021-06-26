use prost::Message;
use std::{io::Cursor, path::Path, process::{Command, Stdio}, sync::mpsc};
pub mod openvslam_api {
    tonic::include_proto!("openvslam_api"); // The string specified here must match the proto package name
}
use tokio::{
    sync::{oneshot},
};

#[derive(Debug)]
struct ApiRequest {
    request: openvslam_api::request::Msg,
    callback: oneshot::Sender<openvslam_api::Response>,
}

pub struct OpenVSlamWrapper {
    request_sender: mpsc::Sender<ApiRequest>,
    // request_receiver: mpsc::Receiver<ApiRequest>,
    thread:std::thread::JoinHandle<()>,
}

impl OpenVSlamWrapper {
    pub fn new() -> Self {
        
        let (request_sender, mut request_receiver) = mpsc::channel::<ApiRequest>();

        let mut openvslam_process = {
            let bin = Path::new("./openvslam-wrap/openvslam/build/run_api")
                .canonicalize()
                .expect("can't find OpenVSlam run_api binary");
            
            let config = Path::new("./openvslam-wrap/config/cfg.yaml")
                .canonicalize()
                .expect("can't find OpenVSlam config file");

            let vocab = Path::new("./openvslam-wrap/config/orb_vocab.fbow")
                .canonicalize()
                .expect("can't find vocabulary file");

            Command::new(bin.to_str().unwrap())
                .stdin(Stdio::null())
                .arg("-c")
                .arg(config.to_str().unwrap())
                .arg("-v")
                .arg(vocab.to_str().unwrap())
                .spawn()
                .expect("failed to start OpenVSlam")
        };

        let context = zmq::Context::new();

        let req_handle = std::thread::spawn(move || {
            let mut req = context.socket(zmq::REQ).unwrap();
            req.connect("ipc:///tmp/openvslam_wrapper_ipc_request")
                .expect("failed to connect OpenVSlam response socket");

            loop {
                let api_request = request_receiver.recv() ; {
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
                        let response = req.recv_msg(0).expect("failed to receive response");
                        println!("send 3");
                        let response = openvslam_api::Response::decode(&mut Cursor::new(response.as_str().unwrap().as_bytes()))
                            .expect("failed to decode response");
                        let _ = api_request.callback.send(response);
                        
                    }
                    else {
                        println!("api request channel closed");
                        break;
                    }
                }
            }
        });

        let thread = std::thread::spawn(move || {
            let exit_status = openvslam_process.wait();
            println!("OpenVSlam closed with status: {:?}", exit_status);
            req_handle.join();
        });

        OpenVSlamWrapper { request_sender, thread }
    }

    async fn shutdown(&self) {
        let request = openvslam_api::request::Msg::Shutdown(openvslam_api::request::Shutdown::default());
        let (callback, rx) = oneshot::channel();

        println!("shutdown 1");
        println!("{:?}", self.request_sender.send(ApiRequest { request, callback }));
        println!("shutdown 2");
        println!("{:?}", rx.await);
        println!("shutdown 3");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = OpenVSlamWrapper::new();
    println!("api started");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    api.shutdown().await;
    println!("shutdown came back");
    api.thread.join();
    Ok(())
}
