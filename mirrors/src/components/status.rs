use common::msg::Msg;
use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use crate::signal_map::SignalData;
use tokio;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::find_node;
use crate::watch_msg;
use common::types::TrackingState;

pub struct Status {
    tracking_state: Receiver<Option<TrackingState>>,
    enable_auto_nav: Receiver<Option<bool>>,
    // track_label_path: String,
    // enable_auto_switch_path: String,
}

impl Status {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let tracking_state = watch_msg!(context, Msg::TrackingState);
        let enable_auto_nav = watch_msg!(context, Msg::EnableAutoNav);

        // let panel = HBoxContainer::new();
        // let panel_name = "status";
        // let panel_path = format!("{}/{}", path, panel_name);
        // panel.set_name(panel_name);

        // let track_label = Label::new();
        // let track_label_name = "status";
        // let track_label_path = format!("{}/{}", panel_path, track_label_name);
        // track_label.set_name(track_label_name);
        // panel.add_child(track_label, false);

        // let enable_auto_switch = CheckButton::new();
        // let enable_auto_switch_name = "enableauto";
        // let enable_auto_switch_path = format!("{}/{}", panel_path, enable_auto_switch_name);
        // enable_auto_switch.set_text("enable auto nav");
        // enable_auto_switch.set_name(enable_auto_switch_name);
        // panel.add_child(enable_auto_switch, false);

        // get_node::<Node>(&*owner, path).add_child(panel, false);

        {
            let btn = find_node::<Node>(&*owner, "EnableAutoNavToggle".into());
            let mut recv_toggled = context.signal_map.connect_b(owner, &btn.get_path().to_string(), "toggled");
            let publisher = context.broadcaster.publisher();
            context.runtime().spawn(async move {
                while let Some(sig) = recv_toggled.recv().await {
                    if let SignalData::B(enable) = sig {
                        publisher.send(Msg::EnableAutoNav(enable)).ok();
                    }
                }
            });
        }

        

        Status {
            tracking_state,
            enable_auto_nav,
            // track_label_path,
            // enable_auto_switch_path,
        }
    }
}

impl Updatable for Status {
    fn update(&self, owner: &Node) {
        let tracking_state = &*self.tracking_state.borrow();
        if let Some(tracking_state) = tracking_state {
            let track_label = find_node::<Label>(&*owner, "TrackingStateLabel".into());
            track_label.set_text(format!("{:?}", tracking_state));
        }

        let enable_auto_nav = &*self.enable_auto_nav.borrow();
        if let Some(enable_auto_nav) = enable_auto_nav {
            let enable_auto_switch = find_node::<CheckButton>(&*owner, "EnableAutoNavToggle".into());
            enable_auto_switch.set_pressed(*enable_auto_nav);
        }
    }
}
