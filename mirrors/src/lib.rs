pub mod game;
pub mod types;
pub mod grpc_client;
pub mod components;
pub mod ui_state;
pub mod utils;
pub mod signal_map;

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
