use crate::signal_map::SignalMap;
use crate::grpc_client::GrpcClient;
use std::sync::{Arc, RwLock};

pub struct Context {
    pub signal_map: SignalMap,
    pub client: Arc<RwLock<GrpcClient>>,
}

impl Context {
    pub fn new(signal_map: SignalMap, client: GrpcClient) -> Self {
        Context { signal_map, client: Arc::new(RwLock::new(client)) }
    }
    pub fn runtime(&self) -> tokio::runtime::Handle {
        self.client.read().unwrap().rt.handle().clone()
    }
}