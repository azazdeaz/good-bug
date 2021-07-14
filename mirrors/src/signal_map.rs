use gdnative::api::*;
use gdnative::prelude::*;

use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::utils::get_node;

#[derive(Debug, Clone, Copy)]
pub enum SignalData {
    Empty,
    FFF(f64, f64, f64),
    B(bool),
}

pub struct SignalMap {
    id_counter: u32,
    connections: HashMap<u32, mpsc::Sender<SignalData>>,
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

    fn create_connector(&mut self, owner: TRef<Node>, emitter_path: &str, signal: &str, callback: &str) -> mpsc::Receiver<SignalData> {
        let emitter = get_node::<Node>(&owner, emitter_path.into());
        let (sender, receiver) = mpsc::channel(12);
        let id  = self.next_id();
        let binds = VariantArray::new();
        binds.push(id);
        self.connections.insert(id, sender);
        emitter.connect(
            signal,
            owner,
            callback,
            binds.into_shared(),
            0,
        )
        .unwrap();

        receiver
    }

    pub fn connect(&mut self, owner: TRef<Node>, emitter_path: &str, signal: &str) -> mpsc::Receiver<SignalData> {
        self.create_connector(owner, emitter_path, signal, "signal_map_callback")
    }

    pub fn connect_fff(&mut self, owner: TRef<Node>, emitter_path: &str, signal: &str) -> mpsc::Receiver<SignalData> {
        self.create_connector(owner, emitter_path, signal, "signal_map_callback_fff")
    }

    pub fn connect_b(&mut self, owner: TRef<Node>, emitter_path: &str, signal: &str) -> mpsc::Receiver<SignalData> {
        self.create_connector(owner, emitter_path, signal, "signal_map_callback_b")
    }

    pub fn callback(&mut self, id:u32, signal: SignalData) {
        if let Some(sender) = self.connections.get(&id) {
            if let Err(e) = sender.blocking_send(signal) {
                println!("[signal map] remove connection {}:{:?}", id, e);
                self.connections.remove(&id).unwrap();
            }
        }
    }
}