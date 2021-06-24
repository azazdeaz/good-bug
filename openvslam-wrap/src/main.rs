use prost::Message;
use std::{io::Cursor, path::Path};
use zmq;
pub mod openvslam_api {
    tonic::include_proto!("openvslam_api"); // The string specified here must match the proto package name
}

fn main() {
    // Create ZMQ connection
    let context = zmq::Context::new();
    let req = context.socket(zmq::REQ).unwrap();
    req.connect("ipc:///tmp/openvslam_wrapper_ipc")
        .expect("failed connecting requester");

    let mut msg = openvslam_api::StartSystem::default();
    // msg.vocab_file_path = Path::new("./openvslam-wrap/config/orb_vocab.fbow").canonicalize().unwrap().to_str().unwrap().into();
    let mut wmsg = openvslam_api::Call::default();
    wmsg.msg = Some(openvslam_api::call::Msg::StartSystem(msg));
    let mut buf = Vec::new();
    buf.reserve(wmsg.encoded_len());
    wmsg.encode(&mut buf).unwrap();
    println!("tell {:?}", wmsg);
    req.send(&buf, 0).expect("failed to send cmd");
    let response = req.recv_msg(0).unwrap();

    let response =
        openvslam_api::Call::decode(&mut Cursor::new(response.as_str().unwrap().as_bytes()));
    println!("Received reply {:?}", response);
}
