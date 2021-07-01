use gdnative::api::*;
use gdnative::prelude::*;

use crate::grpc_client::GrpcClient;
use tokio;
use tokio::sync::watch::Receiver;
use std::sync::Arc;

use crate::components::traits::Updatable;
use crate::utils::get_node;
use crate::components::context::Context;

pub struct MapHandler {
    // frame: Receiver<Option<Vec<u8>>>,
    save_btn_path: String,
}

impl MapHandler {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        // let frame = client.watch_frame();

        let save_btn = Button::new();
        save_btn.set_text("Save Map");
        let save_btn_name = "save_btn";
        let save_btn_path = format!("{}/{}", path, save_btn_name);
        save_btn.set_name(save_btn_name);
        
        get_node::<Node>(&*owner, path).add_child(save_btn, false);

        {
            let mut recv_pressed = context.signal_map.connect(owner, &save_btn_path, "pressed");
            let client = Arc::clone(&context.client);
            context.runtime().spawn(async move {
                while let Some(_) = recv_pressed.recv().await {
                    println!("Saving Map...");
                    let client = client.read().await;
                    client.save_map_db("map.db".into()).await;
                }
            });
        }
        

        MapHandler {
            // frame,
            save_btn_path,
        }
    }
}

impl Updatable for MapHandler {
    fn update(&self, owner: &Node) {
        
    }
}
