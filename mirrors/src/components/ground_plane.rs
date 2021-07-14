use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use crate::components::traits::Updatable;

pub struct GroundPlane {
    static_body_path: String,
}

impl GroundPlane {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let static_body_path: String = "/Spatial/Ground/StaticBody".into();

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
