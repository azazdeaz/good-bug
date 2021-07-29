use common::{msg::Msg, types, robot_body::RobotBody};
use gdnative::api::*;
use gdnative::prelude::*;
use tokio::sync::watch::Receiver;

use crate::components::Context;
use crate::components::traits::Updatable;
use crate::signal_map::SignalData;

use crate::utils::get_node;
use crate::watch_msg;

pub struct GroundPlane {
    map_scale: Receiver<Option<f64>>,
    viz_scale: Receiver<f64>,
    static_body_path: String,
}

impl GroundPlane {
    pub fn new(owner: TRef<Node>, _path: String, context: &mut Context) -> Self {
        let static_body_path: String = "GUI/ViewportContainer/Viewport/Spatial/Ground/StaticBody".into();
        let map_scale = watch_msg!(context, Msg::MapScale);
        let viz_scale = context.ui_state.watch(|s| s.viz_scale);

        {
            let mut recv_pressed = context.signal_map.connect_fff(owner, "GUI/ViewportContainer/Viewport/Spatial/Ground/StaticBody", "select_nav_goal");
            let publisher = context.broadcaster.publisher();
            context.runtime().spawn(async move {
                while let Some(SignalData::FFF(x, y, z)) = recv_pressed.recv().await {
                    println!("selecting pose {} {} {}", x, y, z);
                    let _ = publisher.send(Msg::NavTarget(types::Point3::new(x, y, z)));
                }
            });
        }

        GroundPlane {
            static_body_path,
            map_scale,
            viz_scale,
        }
    }
}

impl Updatable for GroundPlane {
    fn update(&self, owner: &Node) {
        let viz_scale = *self.viz_scale.borrow();   
        let map_scale = self.map_scale.borrow().unwrap_or(1.0) * viz_scale;
        let mesh = get_node::<CSGMesh>(owner, "GUI/ViewportContainer/Viewport/Spatial/Ground".into());
        mesh.transform().origin.y = (-RobotBody::get_cam_height() * map_scale) as f32;
    }
}
