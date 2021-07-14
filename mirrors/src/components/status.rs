use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use tokio;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::get_node;
use crate::watch_msg;
use common::types::TrackingState;

pub struct Status {
    tracking_state: Receiver<Option<TrackingState>>,
    track_label_path: String,
}

impl Status {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let tracking_state = watch_msg!(context, Msg::TrackingState);

        let panel = PanelContainer::new();
        let panel_name = "status";
        let panel_path = format!("{}/{}", path, panel_name);
        panel.set_name(panel_name);

        let track_label = Label::new();
        let track_label_name = "status";
        let track_label_path = format!("{}/{}", panel_path, track_label_name);
        track_label.set_name(track_label_name);
        panel.add_child(track_label, false);

        get_node::<Node>(&*owner, path).add_child(panel, false);

        Status {
            tracking_state,
            track_label_path,
        }
    }
}

impl Updatable for Status {
    fn update(&self, owner: &Node) {
        let tracking_state = &*self.tracking_state.borrow();
        if let Some(tracking_state) = tracking_state {
            let track_label = get_node::<Label>(owner, self.track_label_path.clone());
            track_label.set_text(format!("{:?}", tracking_state));
        }
    }
}
