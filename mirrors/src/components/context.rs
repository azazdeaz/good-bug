use crate::signal_map::SignalMap;
use crate::grpc_client::GrpcClient;
use std::sync::Arc;
use tokio::{runtime::Runtime, sync::{RwLock, RwLockReadGuard}};

pub struct Context {
    pub signal_map: SignalMap,
    pub client: Arc<RwLock<GrpcClient>>,
    rt: Runtime,
}

impl Context {
    pub fn new() -> Self {
        let rt = Runtime::new().unwrap();
        let signal_map = SignalMap::new();
        let client = GrpcClient::new(rt.handle().clone());
        Context { signal_map, client: Arc::new(RwLock::new(client)), rt }
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
}