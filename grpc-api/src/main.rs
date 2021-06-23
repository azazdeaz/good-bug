use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest, Speed, Empty};

use drivers::Wheels;
use std::sync::Arc;

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct MyGreeter {
    wheels: Arc<Wheels>,
}

impl MyGreeter {
    async fn new() -> tokio::io::Result<Self> {
        Ok(MyGreeter { 
            wheels: Arc::new(Wheels::new().await?),
        })
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn set_speed(
        &self,
        request: Request<Speed>,
    ) -> Result<Response<Empty>, Status> {
        let speed = request.into_inner();
        println!("{} {} {:?}", speed.left, speed.right, std::time::Instant::now());
        self.wheels.speed_sender.send((speed.left as f64, speed.right as f64)).await.unwrap();
        Ok(Response::new(Empty::default()))
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
    Ok(())
}