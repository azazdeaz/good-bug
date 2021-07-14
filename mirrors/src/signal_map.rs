use gdnative::api::*;
use gdnative::prelude::*;

use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::utils::get_node;
pub struct SignalMap {
    id_counter: u32,
    connections: HashMap<u32, mpsc::Sender<()>>,
}

impl SignalMap {
    pub fn new() -> Self {
        SignalMap {
            id_counter: 0,
            connections: HashMap::new(),
        }
    }
    
    pub fn next_id(&mut self) -> u32 {
        self.id_counter += 1;
        self.id_counter
    }

    pub fn connect(&mut self, owner: TRef<Node>, emitter_path: &str, signal: &str) -> mpsc::Receiver<()> {
        let emitter = get_node::<Node>(&owner, emitter_path.into());
        let (sender, receiver) = mpsc::channel(12);
        let id  = self.next_id();
        let binds = VariantArray::new();
        binds.push(id);
        self.connections.insert(id, sender);
        emitter.connect(
            signal,
            owner,
            "signal_map_callback",
            binds.into_shared(),
            0,
        )
        .unwrap();

        receiver
    }

    pub fn callback(&mut self, id:u32) {
        if let Some(sender) = self.connections.get(&id) {
            if let Err(e) = sender.blocking_send(()) {
                println!("[signal map] remove connection {}:{:?}", id, e);
                self.connections.remove(&id).unwrap();
            }
        }
    }
}