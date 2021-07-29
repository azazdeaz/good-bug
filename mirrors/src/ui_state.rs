use tokio::{sync::{watch, broadcast}, runtime::Handle};
use std::sync::{Arc, RwLock};

#[derive(Default, Clone, Copy)]
pub struct Annotator {
    topleft: (f64, f64),
    bottomright: (f64, f64),
}

#[derive(Clone, Copy)]
pub struct MirrorsState {
    pub viz_scale: f64,
    pub map_scale: f64,
    pub annotator: Annotator,
}

impl Default for MirrorsState {
    fn default() -> Self {
        Self {
            viz_scale: 2.0,
            map_scale: 1.0,
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
        let state = Arc::new(RwLock::new(MirrorsState::default()));
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
    pub fn watch<T: PartialEq + Copy + Send + Sync + 'static>(&self, mapper: fn(MirrorsState) -> T) -> watch::Receiver<T> {
        let (tx, rx) = watch::channel(mapper(self.state.read().unwrap().clone()));

        let mut rec_state = self.receive.clone();
        self.rt.spawn(async move {
            let mut prev: Option<T> = None;
            while rec_state.changed().await.is_ok() {
               let state = mapper(*rec_state.borrow());
               if prev.is_none() || prev.unwrap() != state {
                    tx.send(state.clone()).ok();
                    prev = Some(state);
               }
            }
        });
        rx
    }
}