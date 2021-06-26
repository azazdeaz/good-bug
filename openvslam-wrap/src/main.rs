use prost::Message;
use std::{io::Cursor, path::Path, process::Stdio};
pub mod openvslam_api {
    tonic::include_proto!("openvslam_api"); // The string specified here must match the proto package name
}
use tokio::{
    process::{Command},
    sync::{mpsc, oneshot},
};

#[derive(Debug)]
struct ApiRequest {
    request: openvslam_api::request::Msg,
    callback: oneshot::Sender<openvslam_api::Response>,
}

pub struct OpenVSlamWrapper {
    request_sender: mpsc::Sender<ApiRequest>,
    // request_receiver: mpsc::Receiver<ApiRequest>,
    task: tokio::task::LocalSet,
}

impl OpenVSlamWrapper {
    pub async fn new() -> anyhow::Result<Self> {
        
        let (request_sender, mut request_receiver) = mpsc::channel::<ApiRequest>(100);

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
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .arg("-c")
                .arg(config.to_str().unwrap())
                .arg("-v")
                .arg(vocab.to_str().unwrap())
                .spawn()
                .expect("failed to start OpenVSlam")
        };
        // Create ZMQ connection
        let context = async_zmq::Context::new();
        // let stream = async_zmq::pull("ipc:///tmp/openvslam_wrapper_ipc_stream")?
        //     .with_context(&context)
        //     .connect()?;

        let mut req = async_zmq::request("ipc:///tmp/openvslam_wrapper_ipc_request")
            .expect("failed to create the request socket")
            .with_context(&context)
            .connect()
            .expect("failed to connect OpenVSlam response socket");

        let local = tokio::task::LocalSet::new();

        local.spawn_local(async move {
            loop {
                tokio::select! {
                    exit_status = openvslam_process.wait() => {
                        let exit_status = exit_status.expect("Failed to read OpenVSlam exit status");
                        println!("OpenVSlam closed with status: {:?}", exit_status);
                        break;
                    }
                    api_request = request_receiver.recv() => {
                        println!("send 0");
                        if let Some(api_request) = api_request {
                            let mut msg = openvslam_api::Request::default();
                            msg.msg = Some(api_request.request);
                            let mut buf = Vec::new();
                            buf.reserve(msg.encoded_len());
                            msg.encode(&mut buf).expect("failed to encode message");
                            println!("send 1");
                            req.send(&buf).await.expect("failed to send request");
                            println!("send 2");
                            let response = req.recv().await.expect("failed to receive response");
                            println!("send 3");
                            for response in response {
                                println!("send 4");
                                let response = openvslam_api::Response::decode(&mut Cursor::new(response.as_str().unwrap().as_bytes()))
                                    .expect("failed to decode response");
                                let _ = api_request.callback.send(response);
                                break;
                            }
                            
                        }
                        else {
                            println!("api request channel closed");
                            break;
                        }
                    }
                }
            }
        });

        Ok(OpenVSlamWrapper { request_sender, task: local })

        // let req = context.socket(zmq::REQ).unwrap();
        // req.connect("ipc:///tmp/openvslam_wrapper_ipc")
        //     .expect("failed connecting requester");
        // let mut msg = openvslam_api::StartSystem::default();
        // // msg.vocab_file_path = Path::new("./openvslam-wrap/config/orb_vocab.fbow").canonicalize().unwrap().to_str().unwrap().into();
        // let mut wmsg = openvslam_api::Call::default();
        // wmsg.msg = Some(openvslam_api::call::Msg::StartSystem(msg));
        // let mut buf = Vec::new();
        // buf.reserve(wmsg.encoded_len());
        // wmsg.encode(&mut buf).unwrap();
        // println!("tell {:?}", wmsg);
        // req.send(&buf, 0).expect("failed to send cmd");
        // let response = req.recv_msg(0).unwrap();
        // let response =
        //     openvslam_api::Call::decode(&mut Cursor::new(response.as_str().unwrap().as_bytes()));
        // println!("Received reply {:?}", response);
    }

    async fn shutdown(&self) {
        let request = openvslam_api::request::Msg::Shutdown(openvslam_api::request::Shutdown::default());
        let (callback, rx) = oneshot::channel();

        println!("shutdown 1");
        println!("{:?}", self.request_sender.send(ApiRequest { request, callback }).await);
        println!("shutdown 2");
        println!("{:?}", rx.await);
        println!("shutdown 3");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = OpenVSlamWrapper::new().await?;
    println!("api started");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    api.shutdown().await;
    println!("shutdown came back");
    api.task.await;
    Ok(())
}


// use async_zmq::Result;
// use std::ops::Deref;

// #[tokio::main]
// async fn main() -> Result<()> {
//     tokio::spawn(async move {
//         let zmq = async_zmq::request("tcp://127.0.0.1:5555")?.connect()?;
//         zmq.send(vec!["broadcast message"]).await?;
//         let msg = zmq.recv().await?;
//         Ok(())
//     });
    
//     Ok(())
// }

// use async_zmq::Result;
// use std::ops::Deref;

// #[tokio::main]
// async fn main() -> Result<()> {
//     std::thread::spawn(|| {

//         let zmq = async_zmq::request("tcp://127.0.0.1:5555")?.connect()?;
//         zmq.send(vec!["broadcast message"]).await?;
//         let msg = zmq.recv().await?;
//         Ok(())
//     });
    
//     Ok(())
// }

// use tokio::time::{sleep, Duration};

// struct S {
//     x: f64,
// }
// impl S {
//     async fn something(&self) -> anyhow::Result<()> {
//         sleep(Duration::from_millis(100)).await;
//         Ok(())
//     }
// }



// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let s = S { x: 6.0 };
//     s.something().await.unwrap();
//     tokio::spawn(async move {
//         s.something().await.unwrap();
//         s.something().await.unwrap();
//         s.something().await.unwrap();
//     });
    

//     Ok(())
// }
