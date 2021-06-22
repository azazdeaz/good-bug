mod game;
mod navigator;
mod types;
mod grpc_client;

use gdnative::prelude::{godot_init, InitHandle};

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/map_segment.rs"));
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
