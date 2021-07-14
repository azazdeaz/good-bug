use common::{msg::Msg, types};
use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use crate::components::traits::Updatable;
use crate::signal_map::SignalData;

pub struct GroundPlane {
    static_body_path: String,
}

impl GroundPlane {
    pub fn new(owner: TRef<Node>, _path: String, context: &mut Context) -> Self {
        let static_body_path: String = "/Spatial/Ground/StaticBody".into();

        {
            let mut recv_pressed = context.signal_map.connect_fff(owner, "/Spatial/Ground/StaticBody", "select_nav_goal");
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
        }
    }
}

impl Updatable for GroundPlane {
    fn update(&self, _owner: &Node) {
        // let tracking_state = &*self.tracking_state.borrow();
        // let track_label = get_node::<Label>(owner, self.track_label_path.clone());
        // track_label.set_text(format!("{:?}", tracking_state));
    }
}
