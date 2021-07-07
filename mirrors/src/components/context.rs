use crate::signal_map::SignalMap;
use crate::grpc_client::GrpcClient;
use std::sync::Arc;
use tokio::{runtime::Runtime, sync::{RwLock, RwLockReadGuard, broadcast}};
use common::types;

pub struct Context {
    pub signal_map: SignalMap,
    pub client: Arc<RwLock<GrpcClient>>,
    rt: Runtime,
    input_sender: broadcast::Sender<Option<types::GDInput>>,
    input_receiver: broadcast::Receiver<Option<types::GDInput>>, 
}

impl Context {
    pub fn new() -> Self {
        let rt = Runtime::new().unwrap();
        let signal_map = SignalMap::new();
        let client = GrpcClient::new(rt.handle().clone()).unwrap();
        let (input_sender, input_receiver) = broadcast::channel(12);
        Context { signal_map, client: Arc::new(RwLock::new(client)), rt, input_sender, input_receiver }
    }
    pub fn runtime(&self) -> tokio::runtime::Handle {
        self.rt.handle().clone()
    }
    // pub fn read_client(&self) -> RwLockReadGuard<GrpcClient>{
    //     let client = Arc::clone(&self.client);
    //     self.rt.block_on(async { client.read().await })
    // }
    pub fn use_client<T>(&self, exec: fn(RwLockReadGuard<GrpcClient>) -> T ) -> T {
        let client = Arc::clone(&self.client);
        self.rt.block_on(async { 
            let client = client.read().await;
            exec(client)
        })
    }

    pub fn send_input(&self, event: types::GDInput) {
        self.rt.block_on(async { 
            self.input_sender.send(Some(event)).unwrap();
        })
    }

    pub fn subscribe_input(&self) -> broadcast::Receiver<Option<types::GDInput>> {
        self.input_sender.subscribe()
    }
}