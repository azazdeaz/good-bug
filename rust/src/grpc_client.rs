use hello_world::greeter_client::GreeterClient;
use hello_world::{Speed, Empty};
use tokio::{runtime::Runtime, sync::{watch,Mutex}, time::{sleep, Duration}};
use std::sync::{Arc};
use common::types::{Keyframe, Landmark, TrackingState};

type Iso3 = nalgebra::Isometry3<f64>;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct GrpcClient {
    rt: Runtime,
    client: Arc<Mutex<GreeterClient<tonic::transport::Channel>>>,
}



impl GrpcClient {
    pub fn new() -> Self {
        println!("Creating GRPC client!!!!");
        let rt = Runtime::new().unwrap();
        let client = rt.block_on(async {
            // TODO load this from conf
            let dst = "http://127.0.0.1:50051";
            // let dst = "http://192.168.50.19:50051";
            let conn = tonic::transport::Endpoint::new(dst).unwrap().connect_lazy().unwrap();
            let client = GreeterClient::new(conn);
            Arc::new(Mutex::new(client))
        });
        println!("Created GRPC client!!!!");
        
        GrpcClient { rt, client }
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

    pub fn set_speed(&self, left: f64, right: f64) {
        let client = Arc::clone(&self.client);
        self.rt.block_on(async {
            println!("REQUESTING {} {}", left, right);
            let request = tonic::Request::new(Speed { left: left as f32, right: right as f32 });

            let response = client.lock().await.set_speed(request).await.unwrap();

            println!("RESPONSE={:?}", response);
        });
    }

    pub fn watch_camera_pose(&self) -> watch::Receiver<Option<Iso3>> {
        let client = Arc::clone(&self.client);
        let (sx, rx) = watch::channel(None);
        self.rt.spawn(async move {
            println!("REQUESTING stream");
            let mut response = loop {
                let request = tonic::Request::new(Empty {});
                if let Ok(response) = client.lock().await.stream_camera_position(request).await {
                    break response.into_inner();
                }
                else {
                    sleep(Duration::from_secs(1)).await;
                }
            };

            println!("RESPONSE={:?}", response);

            while let Some(iso3) = response.message().await.unwrap() {
                let iso3: Iso3 = serde_json::from_str(&iso3.json).expect(&format!("Coulnd't parse as Iso3 Serde:{}", iso3.json));
                sx.send(Some(iso3)).unwrap();
            }
        }); 
        rx
    }

    pub fn watch_landmarks(&self) -> watch::Receiver<Vec<Landmark>> {
        let client = Arc::clone(&self.client);
        let (sx, rx) = watch::channel(Default::default());
        self.rt.spawn(async move {
            println!("REQUESTING stream");
            let mut response = loop {
                let request = tonic::Request::new(Empty {});
                if let Ok(response) = client.lock().await.landmarks(request).await {
                    break response.into_inner();
                }
                else {
                    sleep(Duration::from_secs(1)).await;
                }
            };

            println!("RESPONSE={:?}", response);

            while let Some(message) = response.message().await.unwrap() {
                let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
                sx.send(message).unwrap();
            }
        }); 
        rx
    }

    pub fn watch_keyframes(&self) -> watch::Receiver<Vec<Keyframe>> {
        let client = Arc::clone(&self.client);
        let (sx, rx) = watch::channel(Default::default());
        self.rt.spawn(async move {
            println!("REQUESTING stream");
            let mut response = loop {
                let request = tonic::Request::new(Empty {});
                if let Ok(response) = client.lock().await.keyframes(request).await {
                    break response.into_inner();
                }
                else {
                    sleep(Duration::from_secs(1)).await;
                }
            };

            println!("RESPONSE={:?}", response);

            while let Some(message) = response.message().await.unwrap() {
                let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
                sx.send(message).unwrap();
            }
        }); 
        rx
    }

    pub fn watch_tracking_state(&self) -> watch::Receiver<TrackingState> {
        let client = Arc::clone(&self.client);
        let (sx, rx) = watch::channel(TrackingState::NotInitialized);
        self.rt.spawn(async move {
            println!("REQUESTING stream ts");
            let mut response = loop {
                let request = tonic::Request::new(Empty {});
                let response = client.lock().await.tracking_state(request).await;
                if let Ok(response) = response {
                    break response.into_inner();
                }
                else {
                    println!("{:?}", response);
                    sleep(Duration::from_secs(1)).await;
                }
            };

            println!("RESPONSE={:?} ts", response);

            while let Some(message) = response.message().await.unwrap() {
                let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
                sx.send(message).unwrap();
            }
        }); 
        rx
    }

    pub fn watch_frame(&self) -> watch::Receiver<Option<Vec<u8>>> {
        let client = Arc::clone(&self.client);
        let (sx, rx) = watch::channel(None);
        self.rt.spawn(async move {
            println!("REQUESTING stream ts");
            let mut response = loop {
                let request = tonic::Request::new(Empty {});
                let response = client.lock().await.frame(request).await;
                if let Ok(response) = response {
                    break response.into_inner();
                }
                else {
                    println!("{:?}", response);
                    sleep(Duration::from_secs(1)).await;
                }
            };

            println!("RESPONSE={:?} ts", response);

            while let Some(message) = response.message().await.unwrap() {
                let message = serde_json::from_str(&message.json).expect(&format!("Coulnd't parse Serde:{}", message.json));
                sx.send(message).unwrap();
            }
        }); 
        rx
    }
}