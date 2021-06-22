use hello_world::greeter_client::GreeterClient;
use hello_world::{Speed};
use tokio::runtime::Runtime;
use std::sync::{Arc,RwLock};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct GrpcClient {
    rt: Runtime,
    client: Arc<RwLock<GreeterClient<tonic::transport::Channel>>>
}

impl GrpcClient {
    pub fn new() -> Self {
        println!("Creating GRPC client!!!!");
        let rt = Runtime::new().unwrap();
        let client = rt.block_on(async {
            let dst = "http://[::1]:50051";
            let conn = tonic::transport::Endpoint::new(dst).unwrap().connect_lazy().unwrap();
            Arc::new(RwLock::new(GreeterClient::new(conn)))
        });
        println!("Created GRPC client!!!!");
        
        GrpcClient { rt, client }
    }

    pub fn set_speed(&self, left: f64, right: f64) {
        let client = Arc::clone(&self.client);
        self.rt.block_on(async {
            println!("REQUESTING {} {}", left, right);
            let request = tonic::Request::new(Speed { left: left as f32, right: right as f32 });

            let response = client.write().unwrap().set_speed(request).await.unwrap();

            println!("RESPONSE={:?}", response);
        });
    }
}
