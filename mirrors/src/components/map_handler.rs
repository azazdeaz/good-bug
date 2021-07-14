use common::msg::Msg;
use gdnative::api::*;
use gdnative::prelude::*;

use tokio;

use crate::components::context::Context;
use crate::components::traits::Updatable;
use crate::utils::get_node;

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
            let publisher = context.broadcaster.publisher();
            context.runtime().spawn(async move {
                while let Some(_) = recv_pressed.recv().await {
                    println!("Saving Map...");
                    let _ = publisher.send(Msg::SaveMapDB("map.db".into()));
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
    fn update(&self, owner: &Node) {}
}
