use common::{
    msg::{Broadcaster, Msg},
    settings::Settings,
};
use hello_world::greeter_client::GreeterClient;
use hello_world::{Empty, Serde};
use std::sync::Arc;
use tokio::{
    runtime::Handle,
    sync::Mutex,
    time::{sleep, Duration},
};
use tokio_stream::StreamExt;
use tonic::transport::Channel;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct GrpcClient {
    client: Arc<Mutex<GreeterClient<Channel>>>,
}

unsafe impl Send for GrpcClient {}

impl GrpcClient {
    pub fn new(rt: Handle, broadcaster: &Broadcaster) -> anyhow::Result<Self> {
        let client = rt.block_on(async {
            Self::create_client().await
        })?;
        let client = Arc::new(Mutex::new(client));

        {
            let client = Arc::clone(&client);
            let mut input = broadcaster
                .stream()
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .filter(Msg::is_mirrors_command);

            rt.spawn(async move {
                while let Some(input) = input.next().await {
                    let json = serde_json::to_string(&input).unwrap();
                    let request = tonic::Request::new(Serde { json });
                    // TODO handle send failure
                    client.lock().await.input(request).await.ok();
                }
            });
        }

        {
            let client = Arc::clone(&client);
            let publisher = broadcaster.publisher();

            rt.spawn(async move {
                loop {
                    let mut response = loop {
                        let request = tonic::Request::new(Empty {});
                        if let Ok(response) = client.lock().await.updates(request).await {
                            break response.into_inner();
                        } else {
                            sleep(Duration::from_secs(1)).await;
                        }
                    };
    
                    while let Ok(Some(serde)) = response.message().await {
                        let msg: Msg = serde_json::from_str(&serde.json)
                            .expect(&format!("Coulnd't parse as Msg Serde:{}", serde.json));
    
                        publisher.send(msg).unwrap();
                    }
                    println!("Restarting update stream query....");
                }
            });
        }

        Ok(Self { client })
    }

    async fn create_client() -> anyhow::Result<GreeterClient<Channel>> {
        let settings = Settings::new()?;
        
        let dst = format!("http://{}:{}", settings.rover_address, settings.grpc_port);
        let conn = tonic::transport::Endpoint::new(dst)
            .unwrap()
            .connect_lazy()
            .unwrap();
        Ok(GreeterClient::new(conn))
    }

    pub async fn reconnect(&mut self) -> anyhow::Result<()> {
        let mut client = self.client.lock().await;
        *client = Self::create_client().await.unwrap();
        Ok(())
    }
}