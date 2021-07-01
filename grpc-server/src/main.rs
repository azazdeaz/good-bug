use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{Empty, Serde, Speed};
use tokio::sync::mpsc;
use tokio_stream::{
    wrappers::{ReceiverStream},
    StreamExt,
};
use tonic::{transport::Server, Request, Response, Status};

use drivers::Wheels;
use std::sync::Arc;

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct MyGreeter {
    wheels: Arc<Wheels>,
    slam: openvslam_wrap::OpenVSlamWrapper,
}

impl MyGreeter {
    async fn new() -> tokio::io::Result<Self> {
        Ok(MyGreeter {
            wheels: Arc::new(Wheels::new().await?),
            slam: openvslam_wrap::OpenVSlamWrapper::new(),
        })
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn set_speed(&self, request: Request<Speed>) -> Result<Response<Empty>, Status> {
        let speed = request.into_inner();
        println!(
            "{} {} {:?}",
            speed.left,
            speed.right,
            std::time::Instant::now()
        );
        self.wheels
            .speed_sender
            .send((speed.left as f64, speed.right as f64))
            .await
            .unwrap();
        Ok(Response::new(Empty::default()))
    }

    async fn save_map_db(
        &self,
        request: Request<Serde>,
    ) -> Result<Response<Empty>, Status> {
        let path: String = serde_json::from_str(&request.into_inner().json).unwrap();
        self.slam.save_map_db(path).await;
        Ok(Response::new(Empty::default()))
    }

    type StreamCameraPositionStream = ReceiverStream<Result<Serde, Status>>;
    async fn stream_camera_position(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::StreamCameraPositionStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let mut stream = self.slam.stream_position();
        tokio::spawn(async move {
            while let Some(iso3) = stream.next().await {
                let json = serde_json::to_string(&iso3).unwrap();
                let msg = Serde { json };
                tx.send(Ok(msg)).await.unwrap();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type LandmarksStream = ReceiverStream<Result<Serde, Status>>;
    async fn landmarks(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::LandmarksStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let mut stream = self.slam.stream_landmarks();
        tokio::spawn(async move {
            while let Some(landmarks) = stream.next().await {
                let json = serde_json::to_string(&landmarks).unwrap();
                let msg = Serde { json };
                tx.send(Ok(msg)).await.unwrap();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type KeyframesStream = ReceiverStream<Result<Serde, Status>>;
    async fn keyframes(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::KeyframesStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let mut stream = self.slam.stream_keyframes();
        tokio::spawn(async move {
            while let Some(keyframes) = stream.next().await {
                let json = serde_json::to_string(&keyframes).unwrap();
                let msg = Serde { json };
                tx.send(Ok(msg)).await.unwrap();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type TrackingStateStream = ReceiverStream<Result<Serde, Status>>;
    async fn tracking_state(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::TrackingStateStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let mut stream = self.slam.stream_tracking_state();
        tokio::spawn(async move {
            while let Some(tracking_state) = stream.next().await {
                let json = serde_json::to_string(&tracking_state).unwrap();
                let msg = Serde { json };
                tx.send(Ok(msg)).await.unwrap();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type FrameStream = ReceiverStream<Result<Serde, Status>>;
    async fn frame(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::FrameStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let mut stream = self.slam.stream_frame();
        tokio::spawn(async move {
            while let Some(tracking_state) = stream.next().await {
                let json = serde_json::to_string(&tracking_state).unwrap();
                let msg = Serde { json };
                tx.send(Ok(msg)).await.unwrap();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::new().await?;
    println!("greeter is running");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    println!("server is done");
    // let mat = nalgebra::Isometry3::<f64>::identity();
    // let json = serde_json::to_string(&mat);
    // println!("{:?}", json);
    // let mat: nalgebra::Isometry3<f64> = serde_json::from_str(&json.unwrap()).unwrap();
    // println!("{:?}", mat);
    Ok(())
}
