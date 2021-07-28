use tokio::{sync::watch, runtime::Handle};
use std::sync::{Arc, RwLock};

#[derive(Default, Clone, Copy)]
pub struct Annotator {
    topleft: (f64, f64),
    bottomright: (f64, f64),
}

#[derive(Clone, Copy)]
pub struct MirrorsState {
    pub viz_scale: f64,
    pub annotator: Annotator,
}

impl Default for MirrorsState {
    fn default() -> Self {
        Self {
            viz_scale: 6.0,
            annotator: Annotator::default(),
        }
    }
}

pub struct UiState {
    state: Arc<RwLock<MirrorsState>>,
    publish: watch::Sender<MirrorsState>,
    receive: watch::Receiver<MirrorsState>,
    rt: Handle,
}
impl UiState {
    pub fn new(rt: Handle) -> Self {
        let state = Arc::new(RwLock::new(MirrorsState::default()));
        let (publish, receive) = watch::channel(state.read().unwrap().clone());
        Self {
            state,
            publish,
            receive,
            rt,
        }
    }
    pub fn update(&self) {
        self.publish.send(self.state.read().unwrap().clone()).ok();
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