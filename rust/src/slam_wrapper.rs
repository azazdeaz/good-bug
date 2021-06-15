use zmq;
use std::thread;
use crossbeam_channel::unbounded;
use nalgebra as na;

use prost::Message;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/map_segment.rs"));
}
enum Incoming {
    OpenVSlamPB(items::VSlamMap),
    GroundTruthPose(na::Isometry3<f64>),
}
struct SlamListener {
    requester: zmq::Socket,
}

impl SlamListener {
    fn new(&self) -> Self {
        let context = zmq::Context::new();
        let requester = context.socket(zmq::REQ).unwrap();
        requester
            .connect("tcp://localhost:5561")
            .expect("failed connecting requester");

        let subscriber = context.socket(zmq::SUB).unwrap();
        subscriber
            .connect("tcp://127.0.0.1:5566")
            .expect("failed connecting subscriber");
        subscriber.set_subscribe(b"").expect("failed subscribing");

        let (s, r) = unbounded();

        thread::spawn(move || {
            loop {
                let envelope = subscriber
                    .recv_string(0)
                    .expect("failed receiving envelope")
                    .unwrap();
                let message = subscriber.recv_bytes(0).expect("failed receiving message");
                // println!("{:?}", message);
                let message = ::base64::decode(message).unwrap();
                let msg = items::VSlamMap::decode(&mut std::io::Cursor::new(message)).unwrap();
                s.send(Incoming::OpenVSlamPB(msg)).unwrap();
            }
        });

        SlamListener {
            requester,
        }
    }
}