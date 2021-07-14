use common::{
    msg::{Broadcaster, Msg},
    settings::Settings,
};
use hello_world::greeter_client::GreeterClient;
use hello_world::{Empty, Serde};
use std::sync::Arc;
use tokio::{
    runtime::Handle,
    sync::{watch, Mutex},
    time::{sleep, Duration},
};
use tokio_stream::StreamExt;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct GrpcClient {
    pub rt: Handle,
    client: Arc<Mutex<GreeterClient<tonic::transport::Channel>>>,
}

unsafe impl Send for GrpcClient {}

impl GrpcClient {
    pub fn new(rt: Handle, broadcaster: &Broadcaster) -> anyhow::Result<Self> {
        let settings = Settings::new()?;
        let client = rt.block_on(async {
            // TODO load this from conf
            let dst = format!("http://{}:{}", settings.rover_address, settings.grpc_port);
            let conn = tonic::transport::Endpoint::new(dst)
                .unwrap()
                .connect_lazy()
                .unwrap();
            let client = GreeterClient::new(conn);
            Arc::new(Mutex::new(client))
        });

        {
            let client = Arc::clone(&client);
            let mut input = broadcaster
                .stream()
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .filter(Msg::is_mirrors_command);

            rt.spawn(async move {
                while let Some(input) = input.next().await {
                    println!("grpc client send req: {:?}", input);
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
                let mut response = loop {
                    let request = tonic::Request::new(Empty {});
                    if let Ok(response) = client.lock().await.updates(request).await {
                        break response.into_inner();
                    } else {
                        sleep(Duration::from_secs(1)).await;
                    }
                };

                while let Some(serde) = response.message().await.unwrap() {
                    let msg: Msg = serde_json::from_str(&serde.json)
                        .expect(&format!("Coulnd't parse as Msg Serde:{}", serde.json));

                    publisher.send(msg).unwrap();
                }
            });
        }

        Ok(GrpcClient { rt, client })
    }

    // fn create_watch<T: Default + DeserializeOwned + Debug + Send + Sync>(&self, send_request: fn(client: MutexGuard<GreeterClient<Channel>>) -> Result<Response<Streaming<Serde>>, Status>) -> watch::Receiver<T> {
    //     let client = Arc::clone(&self.client);
    //     let (sx, rx) = watch::channel(Default::default());
    //     self.rt.spawn(async move {
    //         println!("REQUESTING stream");
    //         let mut response = loop {
    //             let request = tonic::Request::new(Empty {});
    //             let client = client.lock().await;
    //             let req = client.landmarks(request).await;
    //             // let req = send_request(client);
    //             if let Ok(response) = req {
    //                 break response.into_inner();
    //             }
    //             else {
    //                 sleep(Duration::from_secs(1)).await;
    //             }
    //         };

    //         // println!("RESPONSE={:?}", response);

    //         while let Some(message) = response.message().await.unwrap() {
    //             let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
    //             sx.send(message).unwrap();
    //         }
    //     });
    //     rx
    // }

    // pub async fn input(&self, msg: Msg) {
    //     let client = Arc::clone(&self.client);
    //     println!("REQUESTING {}", msg);
    //     let request = tonic::Request::new(Serde { json: serde_json::to_string(&msg).unwrap() });

    //     let response = client.lock().await.input(request).await.unwrap();

    //     println!("RESPONSE={:?}", response);
    // }

    // pub async fn set_speed(&self, left: f64, right: f64) {
    //     let client = Arc::clone(&self.client);
    //     println!("REQUESTING {} {}", left, right);
    //     let request = tonic::Request::new(Speed { left: left as f32, right: right as f32 });

    //     let mut client = client.lock().await;
    //     if let Err(e) = client.set_speed(request).await {
    //         println!("Failed to send speed message to grpc server {:?}", e);
    //     }
    // }

    // pub fn updates(&self) -> watch::Receiver<Option<Iso3>> {

    //     let client = Arc::clone(&self.client);
    //     let (sx, rx) = watch::channel(None);
    //     self.rt.spawn(async move {
    //         println!("REQUESTING stream");
    //         let mut response = loop {
    //             let request = tonic::Request::new(Empty {});
    //             if let Ok(response) = client.lock().await.stream_camera_position(request).await {
    //                 break response.into_inner();
    //             }
    //             else {
    //                 sleep(Duration::from_secs(1)).await;
    //             }
    //         };

    //         println!("RESPONSE={:?}", response);

    //         while let Some(iso3) = response.message().await.unwrap() {
    //             let iso3: Iso3 = serde_json::from_str(&iso3.json).expect(&format!("Coulnd't parse as Iso3 Serde:{}", iso3.json));
    //             sx.send(Some(iso3)).unwrap();
    //         }
    //     });
    //     rx
    // }

    // pub fn watch_landmarks(&self) -> watch::Receiver<Vec<Landmark>> {
    //     let client = Arc::clone(&self.client);
    //     let (sx, rx) = watch::channel(Default::default());
    //     self.rt.spawn(async move {
    //         println!("REQUESTING stream");
    //         let mut response = loop {
    //             let request = tonic::Request::new(Empty {});
    //             if let Ok(response) = client.lock().await.landmarks(request).await {
    //                 break response.into_inner();
    //             }
    //             else {
    //                 sleep(Duration::from_secs(1)).await;
    //             }
    //         };

    //         println!("RESPONSE={:?}", response);

    //         while let Some(message) = response.message().await.unwrap() {
    //             let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
    //             sx.send(message).unwrap();
    //         }
    //     });
    //     rx
    // }

    // pub fn watch_keyframes(&self) -> watch::Receiver<Vec<Keyframe>> {
    //     let client = Arc::clone(&self.client);
    //     let (sx, rx) = watch::channel(Default::default());
    //     self.rt.spawn(async move {
    //         println!("REQUESTING stream");
    //         let mut response = loop {
    //             let request = tonic::Request::new(Empty {});
    //             if let Ok(response) = client.lock().await.keyframes(request).await {
    //                 break response.into_inner();
    //             }
    //             else {
    //                 sleep(Duration::from_secs(1)).await;
    //             }
    //         };

    //         println!("RESPONSE={:?}", response);

    //         while let Some(message) = response.message().await.unwrap() {
    //             let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
    //             sx.send(message).unwrap();
    //         }
    //     });
    //     rx
    // }

    // pub fn watch_edges(&self) -> watch::Receiver<Vec<Edge>> {
    //     let client = Arc::clone(&self.client);
    //     let (sx, rx) = watch::channel(Default::default());
    //     self.rt.spawn(async move {
    //         println!("REQUESTING stream");
    //         let mut response = loop {
    //             let request = tonic::Request::new(Empty {});
    //             if let Ok(response) = client.lock().await.edges(request).await {
    //                 break response.into_inner();
    //             }
    //             else {
    //                 sleep(Duration::from_secs(1)).await;
    //             }
    //         };

    //         println!("RESPONSE={:?}", response);

    //         while let Some(message) = response.message().await.unwrap() {
    //             let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
    //             sx.send(message).unwrap();
    //         }
    //     });
    //     rx
    // }

    // pub fn watch_tracking_state(&self) -> watch::Receiver<TrackingState> {
    //     let client = Arc::clone(&self.client);
    //     let (sx, rx) = watch::channel(TrackingState::NotInitialized);
    //     self.rt.spawn(async move {
    //         println!("REQUESTING stream ts");
    //         let mut response = loop {
    //             let request = tonic::Request::new(Empty {});
    //             let response = client.lock().await.tracking_state(request).await;
    //             if let Ok(response) = response {
    //                 break response.into_inner();
    //             }
    //             else {
    //                 println!("{:?}", response);
    //                 sleep(Duration::from_secs(1)).await;
    //             }
    //         };

    //         println!("RESPONSE={:?} ts", response);

    //         while let Some(message) = response.message().await.unwrap() {
    //             let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
    //             sx.send(message).unwrap();
    //         }
    //     });
    //     rx
    // }

    // pub fn watch_frame(&self) -> watch::Receiver<Option<Vec<u8>>> {
    //     let client = Arc::clone(&self.client);
    //     let (sx, rx) = watch::channel(None);
    //     self.rt.spawn(async move {
    //         println!("REQUESTING stream ts");
    //         let mut response = loop {
    //             let request = tonic::Request::new(Empty {});
    //             let response = client.lock().await.frame(request).await;
    //             if let Ok(response) = response {
    //                 break response.into_inner();
    //             }
    //             else {
    //                 println!("{:?}", response);
    //                 sleep(Duration::from_secs(1)).await;
    //             }
    //         };

    //         println!("RESPONSE={:?} ts", response);

    //         while let Some(message) = response.message().await.unwrap() {
    //             let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
    //             sx.send(message).unwrap();
    //         }
    //     });
    //     rx
    // }
}
