use common::msg::{Broadcaster, Msg};
use hello_world::greeter_client::GreeterClient;
use hello_world::{Empty, Serde};
use tokio::sync::broadcast::Sender;
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
    publisher: Sender<Msg>,
}

unsafe impl Send for GrpcClient {}

impl GrpcClient {
    pub fn new(
        rt: Handle,
        broadcaster: &Broadcaster,
        robot_address: String,
    ) -> anyhow::Result<Self> {
        let client = rt.block_on(async {
            Self::create_client(robot_address)
        });
        let client = Arc::new(Mutex::new(client));

        // let client = Arc::new(Mutex::new(Self::create_client(robot_address)));

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

        let publisher = broadcaster.publisher();
        publisher.send(Msg::RequestRobotParams).ok();

        Ok(Self { client, publisher })
    }

    fn create_client(robot_address: String) -> GreeterClient<Channel> {
        let conn = tonic::transport::Endpoint::new(robot_address)
            .unwrap()
            .connect_lazy()
            .unwrap();
        GreeterClient::new(conn)
    }

    pub async fn reconnect(&mut self, robot_address: String) -> anyhow::Result<()> {
        let mut client = self.client.lock().await;
        *client = Self::create_client(robot_address);
        self.publisher.send(Msg::RequestRobotParams).ok();
        Ok(())
    }
}
