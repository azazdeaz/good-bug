use std::sync::{Arc, RwLock};

use common::msg::Msg;
use common::types::RobotParams;
use common::utils::LastValue;
use gdnative::api::*;
use gdnative::prelude::*;
use tokio::runtime::Handle;

use crate::components::Context;
use tokio;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::find_node;
use crate::{watch_msg, watch_msg_once};
use common::types::TrackingState;

pub struct Status {
    tracking_state: Receiver<Option<TrackingState>>,
    robot_params: Arc<RwLock<LastValue<RobotParams>>>,
    rt: Handle,
    // enable_auto_nav: Receiver<Option<bool>>,
    // track_label_path: String,
    // enable_auto_switch_path: String,
}

impl Status {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let tracking_state = watch_msg!(context, Msg::TrackingState);
        let robot_params = watch_msg_once!(context, Msg::RobotParams);
        // let enable_auto_nav = watch_msg!(context, Msg::EnableAutoNav);

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

        // {
        //     let btn = find_node::<Node>(&*owner, "EnableAutoNavToggle".into());
        //     let mut recv_toggled =
        //         context
        //             .signal_map
        //             .connect_b(owner, &btn.get_path().to_string(), "toggled");
        //     let publisher = context.broadcaster.publisher();
        //     context.runtime().spawn(async move {
        //         while let Some(sig) = recv_toggled.recv().await {
        //             if let SignalData::B(enable) = sig {
        //                 publisher.send(Msg::EnableAutoNav(enable)).ok();
        //             }
        //         }
        //     });
        // }

        // set initial value for connection address
        find_node::<LineEdit>(&*owner, "ConnectionAddress".into()).set_text(
            context
                .ui_state
                .state
                .read()
                .unwrap()
                .robot_addresses
                .first()
                .unwrap(),
        );

        // set initial values to connection history
        let connection_history = find_node::<MenuButton>(&*owner, "ConnectionHistory".into());
        for (idx, robot_address) in context
            .ui_state
            .state
            .read()
            .unwrap()
            .robot_addresses
            .iter()
            .enumerate()
        {
            unsafe {
                connection_history
                    .get_popup()
                    .unwrap()
                    .assume_safe()
                    .add_item(robot_address, idx as i64, 0);
            }
        }

        Status {
            tracking_state,
            robot_params,
            rt: context.runtime(),
            // enable_auto_nav,
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


        // let changed = {
        //     let robot_params = Arc::clone(&self.robot_params);
        //     self.rt.block_on(async {
        //         robot_params.lock().unwrap().changed().await
        //     }).is_ok()
        // };
        // if changed {
        //     let robot_params = self.robot_params.lock().unwrap();
        //     let robot_params = &*robot_params.borrow();
        //     if let Some(robot_params) = robot_params {
        //         println!("emit robot params {:?}", robot_params);
        //         owner.emit_signal("robot_params", &[robot_params.to_variant()]);
        //     }
        // }

        if let Some(robot_params) = self.robot_params.write().unwrap().pop() {
            println!("emit robot params {:?}", robot_params);
            owner.emit_signal("robot_params", &[robot_params.to_variant()]);
        }

        // let enable_auto_nav = &*self.enable_auto_nav.borrow();
        // if let Some(enable_auto_nav) = enable_auto_nav {
        //     let enable_auto_switch =
        //         find_node::<CheckButton>(&*owner, "EnableAutoNavToggle".into());
        //     enable_auto_switch.set_pressed(*enable_auto_nav);
        // }
    }
}
