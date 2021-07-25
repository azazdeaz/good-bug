use gdnative::api::*;
// use gdnative::api::texture_rect::StretchMode;
use gdnative::prelude::*;


use tokio;
use tokio::sync::watch::Receiver;

use common::{types::BoxDetection};
use crate::components::Context;
use crate::components::traits::Updatable;
use crate::utils::find_node;
use crate::watch_msg;

pub struct Frame {
    frame: Receiver<Option<Vec<u8>>>,
    detections: Receiver<Option<Vec<BoxDetection>>>,
    // panel_path: String,
    // rect_paths: Vec<String>,
}

impl Frame {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let frame = watch_msg!(context, Msg::Frame);
        let detections = watch_msg!(context, Msg::Detections);

        // let panel = TextureRect::new();
        // let panel_name = "panel";
        // let panel_path = format!("{}/{}", path, panel_name);
        // panel.set_name(panel_name);
        // panel.set_expand(true);
        // panel.set_stretch_mode(StretchMode::KEEP_ASPECT.into());
        // panel.set_h_size_flags(Control::SIZE_EXPAND_FILL);
        // panel.set_v_size_flags(Control::SIZE_EXPAND_FILL);
        // panel.set_size(Vector2::new(30.0, 30.0), true);
        // panel.set_centered(false);

        // get_node::<Node>(&*owner, path).add_child(panel, false);

        // let rect_paths = (0..12).map(|i| {
        //     let rect = ColorRect::new();
        //     let rect_name = format!("rect_{}", i);
        //     let rect_path = format!("{}/{}", panel_path, rect_name);
        //     rect.set_name(rect_name);
        //     rect.set_frame_color(Color::rgba(1.0, 1.0, 1.0, 1.0));
        //     rect.set_position(Vector2::new((i as f32)*20.0, (i as f32)*20.0), true);
        //     rect.set_size(Vector2::new(30.0, 30.0), true);
        //     get_node::<Node>(&*owner, panel_path.clone()).add_child(rect, false);
        //     rect_path
        // }).collect();
        
        

        Frame {
            frame,
            detections,
            // panel_path,
            // rect_paths,
        }
    }
}

impl Updatable for Frame {
    fn update(&self, owner: &Node) {
        if let Some(frame) = &*self.frame.borrow() {
            let panel = find_node::<TextureRect>(owner, "CameraImage".into());
            let im = Image::new();
            im.load_jpg_from_buffer(TypedArray::from_vec(frame.clone())).expect("failed to load jpeg with godot");
            // im.create_from_data(1280, 960, true, Image::FORMAT_RGB8, TypedArray::from_vec(pixels));
            let imt = ImageTexture::new();
            imt.create_from_image(im, 7);
            panel.set_texture(imt);
            // panel.draw_rect(
            //     Rect2::new(Point2::new(12.0, 12.0), Size2::new(45.0, 45.0)),
            //     Color::rgb(0.0, 1.0, 0.0),
            //     false,
            //     2.0,
            //     true,
            // );
        }

        if let Some(detections) = &*self.detections.borrow() {
            let gd_detections = detections.to_variant();
            let panel = find_node::<TextureRect>(owner, "CameraImage".into());
            unsafe {
                panel.call("update_detections", &[gd_detections]);
            }
        }
    }
}
