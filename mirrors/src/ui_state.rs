use std::sync::{Arc, RwLock};
use gdnative::api::File;
use tokio::{
    runtime::Handle,
    sync::{broadcast, watch},
};
use serde::{Deserialize, Serialize};
use itertools::Itertools;

const STATE_FILE_PATH: &str = "user://mirrors_state.json";

#[derive(Default, Clone, Copy, Deserialize, Serialize)]
pub struct Annotator {
    topleft: (f64, f64),
    bottomright: (f64, f64),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MirrorsState {
    pub viz_scale: f64,
    pub map_scale: f64,
    pub robot_addresses: Vec<String>,
    pub annotator: Annotator,
}

impl Default for MirrorsState {
    fn default() -> Self {
        Self {
            viz_scale: 2.0,
            map_scale: 1.0,
            robot_addresses: vec!["http://127.0.0.1:50051".into()],
            annotator: Annotator::default(),
        }
    }
}
impl MirrorsState {
    // scaler to scale slam-map coordinates for visualization
    pub fn map_to_viz_scale(&self) -> f64 {
        self.map_scale * self.viz_scale
    }
}

pub struct UiState {
    pub state: Arc<RwLock<MirrorsState>>,
    publish: broadcast::Sender<()>,
    receive: watch::Receiver<MirrorsState>,
    rt: Handle,
}

impl UiState {
    pub fn new(rt: Handle) -> Self {
        let file = File::new();
        let mut state = if file.open(STATE_FILE_PATH, File::READ).is_ok() {
            let state = file.get_as_text().to_string();
            serde_json::from_str(&state).unwrap_or_default()
        }
        else {
            MirrorsState::default()
        };

        if state.robot_addresses.is_empty() {
            state.robot_addresses.push("http://127.0.0.1:50051".into());
        }

        let state = Arc::new(RwLock::new(state));
        

        let (publisher_sx, publisher_rx) = watch::channel(state.read().unwrap().clone());
        let (trigger_publish_sx, mut trigger_publish_rx) = broadcast::channel(1);

        {
            let state = Arc::clone(&state);
            rt.spawn(async move {
                loop {
                    if let Ok(_) = trigger_publish_rx.recv().await {
                        publisher_sx.send(state.read().unwrap().clone()).ok();
                    }
                }
            });
        }

        // save state to file
        {
            let mut publisher_rx = publisher_rx.clone();
            rt.spawn(async move {
                while publisher_rx.changed().await.is_ok() {
                    let state = publisher_rx.borrow().clone();
                    let json = serde_json::to_string_pretty(&state).unwrap();
                    let file = File::new();
                    file.open(STATE_FILE_PATH, File::WRITE).expect("failed to open mirrors state file");
                    file.store_string(json);
                }
            });
        }

        Self {
            state,
            publish: trigger_publish_sx,
            receive: publisher_rx,
            rt,
        }
    }
    pub fn update(&self) {
        self.publish.send(()).ok();
    }
    pub fn updater(&self) -> broadcast::Sender<()> {
        self.publish.clone()
    }
    pub fn watch<T: PartialEq + Copy + Send + Sync + 'static>(
        &self,
        mapper: fn(MirrorsState) -> T,
    ) -> watch::Receiver<T> {
        let (tx, rx) = watch::channel(mapper(self.state.read().unwrap().clone()));

        let mut rec_state = self.receive.clone();
        self.rt.spawn(async move {
            let mut prev: Option<T> = None;
            while rec_state.changed().await.is_ok() {
                let state = rec_state.borrow().clone();
                let state = mapper(state);
                if prev.is_none() || prev.unwrap() != state {
                    tx.send(state.clone()).ok();
                    prev = Some(state);
                }
            }
        });
        rx
    }

    pub fn add_robot_address(&mut self, robot_address: String) {
        {
            let mut state = self.state.write().unwrap();
            state.robot_addresses.insert(0, robot_address);
            state.robot_addresses = state.robot_addresses.clone().into_iter().unique().collect();
        }
        
        self.publish.send(()).ok();
    }
}
