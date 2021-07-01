use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use tokio;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::get_node;

pub struct Frame {
    frame: Receiver<Option<Vec<u8>>>,
    panel_path: String,
}

impl Frame {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let frame = context.use_client(|c| c.watch_frame());

        let panel = Sprite::new();
        let panel_name = "panel";
        let panel_path = format!("{}/{}", path, panel_name);
        panel.set_name(panel_name);
        
        get_node::<Node>(&*owner, path).add_child(panel, false);


        Frame {
            frame,
            panel_path,
        }
    }
}

impl Updatable for Frame {
    fn update(&self, owner: &Node) {
        if let Some(frame) = &*self.frame.borrow() {
            let panel = get_node::<Sprite>(owner, self.panel_path.clone());
            let im = Image::new();
            im.load_jpg_from_buffer(TypedArray::from_vec(frame.clone())).expect("failed to load jpeg with godot");
            // im.create_from_data(1280, 960, true, Image::FORMAT_RGB8, TypedArray::from_vec(pixels));
            let imt = ImageTexture::new();
            imt.create_from_image(im, 7);
            panel.set_texture(imt);
        }
    }
}
