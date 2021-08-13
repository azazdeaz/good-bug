use crate::{grpc_client::GrpcClient};
use crate::signal_map::SignalMap;
use crate::ui_state::UiState;
use common::msg::Msg;
use common::{msg::Broadcaster, types};
use std::sync::Arc;
use tokio::{
    runtime::Runtime,
    sync::{broadcast, RwLock, RwLockReadGuard},
};

pub struct Context {
    pub signal_map: SignalMap,
    pub broadcaster: Broadcaster,
    pub ui_state: UiState,
    pub client: Arc<RwLock<GrpcClient>>,
    pub rt: Runtime,
    input_sender: broadcast::Sender<Option<types::GDInput>>,
    input_receiver: broadcast::Receiver<Option<types::GDInput>>,
}

impl Context {
    pub fn new() -> Self {
        let rt = Runtime::new().unwrap();
        let signal_map = SignalMap::new();
        let broadcaster = Broadcaster::new();
        let client = GrpcClient::new(rt.handle().clone(), &broadcaster).unwrap();
        let (input_sender, input_receiver) = broadcast::channel(12);
        let mut ui_state = UiState::new(rt.handle().clone());

        // Add some messages to the UI state (very bad! TODO redesign)
        {
            let mut subscriber = broadcaster.subscribe();
            let state = Arc::clone(&ui_state.state);
            let udpate = ui_state.updater();
            rt.spawn(async move {
                loop {
                    if let Ok(msg) = subscriber.recv().await {
                        if let Msg::MapScale(map_scale) = msg {
                            state.write().unwrap().map_scale = map_scale;
                            udpate.send(()).ok();
                        }
                    }
                }
            });
        }

        Context {
            signal_map,
            broadcaster,
            client: Arc::new(RwLock::new(client)),
            rt,
            input_sender,
            input_receiver,
            ui_state,
        }
    }
    pub fn runtime(&self) -> tokio::runtime::Handle {
        self.rt.handle().clone()
    }
    // pub fn read_client(&self) -> RwLockReadGuard<GrpcClient>{
    //     let client = Arc::clone(&self.client);
    //     self.rt.block_on(async { client.read().await })
    // }
    pub fn use_client<T>(&self, exec: fn(RwLockReadGuard<GrpcClient>) -> T) -> T {
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
