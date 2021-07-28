use tokio::sync::watch;
use tokio_stream::{self as stream, wrappers::WatchStream, Stream, StreamExt};

#[derive(Debug, Default, Clone, Copy)]
pub struct Annotator {
    topleft: (f64, f64),
    bottomright: (f64, f64),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MirrorsState {
    pub viz_scale: f64,
    pub annotator: Annotator,
}

pub struct StateManager {
    state: MirrorsState,
    publish: watch::Sender<MirrorsState>,
    receive: watch::Receiver<MirrorsState>,
}
impl StateManager {
    pub fn new() -> Self {
        let state = MirrorsState::default();
        let (publish, receive) = watch::channel(state);
        Self {
            state,
            publish,
            receive,
        }
    }
    pub fn update(&self) {
        self.publish.send(self.state).ok();
    }
    pub fn watch<T: PartialEq + Copy + Send + Sync + 'static>(&self, mapper: fn(MirrorsState) -> T) -> watch::Receiver<T> {
        // let mut last_value = None;
        let (tx, rx) = watch::channel(mapper(self.state));
        // let stream = WatchStream::new(self.receive.clone())
        //     .map(mapper)
        //     .filter(move |v| {
        //         if last_value.is_some() && last_value.unwrap() == *v {
        //             false
        //         } else {
        //             last_value = Some(*v);
        //             true
        //         }
        //     });
        let mut rec_state = self.receive.clone();
        tokio::spawn(async move {
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

#[tokio::main]
async fn main() {
    let stream = stream::iter(vec![0i32, 1, 1, 2, 1]);
    let mut last_value = None;
    let mut stream = stream.filter(|v| {
        if last_value.is_some() && last_value.unwrap() == *v {
            false
        } else {
            last_value = Some(*v);
            true
        }
    });
    // let mut stream = stream.scan(None, |prev, val| {
    //     *prev = Some(val);
    //     if prev.is_some() && prev.unwrap() == val {
    //         future::ready(Some(Some(val)))
    //     }
    //     else {

    //         future::ready(Some(None))
    //     }
    // })
    // .filter(Option::is_some)
    // .map(Option::unwrap);

    let mut state_manager = StateManager::new();
    let mut stream = state_manager.watch(|s| (s.viz_scale, s.annotator.bottomright));

    tokio::spawn(async move {
        for i in 1..4 {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            state_manager.state.viz_scale = i as f64;
            state_manager.update();
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            state_manager.state.viz_scale = i as f64;
            state_manager.update();
        }
    });

    // while let Some(value) = stream.next().await {
    //     println!("Got {:?}", value);
    // }
    while stream.changed().await.is_ok() {
        let value = *stream.borrow();
        println!("Got {:?}", value);
    }
}
