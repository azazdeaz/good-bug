use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{Empty, Serde};
use tokio::sync::{mpsc};
use tokio_stream::{
    wrappers::{ReceiverStream},
    StreamExt,
};
use tonic::{transport::Server, Request, Response, Status};

use common::{settings::Settings, msg::Broadcaster};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct GrpcService {
    broadcaster: Broadcaster,
}

impl GrpcService {
    async fn new(broadcaster: Broadcaster) -> anyhow::Result<Self> {
        // let (publisher, _) = broadcaster::channel(12);
        Ok(GrpcService {
            broadcaster
        })
    }
}

#[tonic::async_trait]
impl Greeter for GrpcService {
    async fn input(&self, request: Request<Serde>) -> Result<Response<Empty>, Status> {
        let serde = request.into_inner();
        self.broadcaster.publish_serialized(serde.json);
        Ok(Response::new(Empty{}))
    }

    type UpdatesStream = ReceiverStream<Result<Serde, Status>>;
    async fn updates(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::UpdatesStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let mut stream = self.broadcaster.stream()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            // HACK filter messages coming from mirrors to avoid infinite loops
            .filter(|m| !m.is_mirrors_command())
            .map(|f| serde_json::to_string(&f));
            
        
        tokio::spawn(async move {
            while let Some(json) = stream.next().await {
                if let Ok(json) = json {
                    tx.send(Ok(Serde { json })).await.ok();
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

pub async fn start_server(broadcaster: Broadcaster) -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new()?;
    let addr = format!("0.0.0.0:{}", settings.grpc_port).parse()?;
    let greeter = GrpcService::new(broadcaster).await?;
    println!("grpc server is running at {:?}", addr);
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?; 
    println!("server is done");
    Ok(())
}
