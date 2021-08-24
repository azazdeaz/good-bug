use std::sync::{Arc, RwLock};

use common::msg::Msg;
use common::types::NavigatorState;
use common::types::RobotParams;
use common::types::SystemStatus;
use common::utils::LastValue;
use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use crate::ui_state::MirrorsState;
use tokio;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::find_node;
use crate::{watch_msg, watch_msg_once};
use common::types::TrackingState;

pub struct Status {
    tracking_state: Receiver<Option<TrackingState>>,
    robot_params: Arc<RwLock<LastValue<RobotParams>>>,
    navigator_state: Arc<RwLock<LastValue<NavigatorState>>>,
    system_status: Arc<RwLock<LastValue<SystemStatus>>>,
    ui_state: tokio::sync::watch::Receiver<MirrorsState>,
    got_first_robot_params_update: Arc<RwLock<bool>>,
    viz_scale: Receiver<f64>,
}

impl Status {
    pub fn new(owner: TRef<Node>, _path: String, context: &mut Context) -> Self {
        let tracking_state = watch_msg!(context, Msg::TrackingState);
        let robot_params = watch_msg_once!(context, Msg::RobotParams);
        let navigator_state = watch_msg_once!(context, Msg::NavigatorState);
        let system_status = watch_msg_once!(context, Msg::SystemStatus);
        let viz_scale = context.ui_state.watch(|s| s.map_to_viz_scale());

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

        // HACK poll the robot until Grpc connection is established
        let got_first_robot_params_update = Arc::new(RwLock::new(false));
        {
            let publisher = context.broadcaster.publisher();
            let got_first_robot_params_update = Arc::clone(&got_first_robot_params_update);
            context.runtime().spawn(async move {
                while !*got_first_robot_params_update.read().unwrap() {
                    publisher.send(Msg::RequestRobotParams).ok();
                    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                }
            });
        }
        Status {
            tracking_state,
            robot_params,
            navigator_state,
            system_status,
            ui_state: context.ui_state.watch_all(),
            viz_scale,
            got_first_robot_params_update,
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

        let viz_scale = *self.viz_scale.borrow();

        if let Some(ref mut robot_params) = self.robot_params.write().unwrap().pop() {
            *self.got_first_robot_params_update.write().unwrap() = true;
            for map in &mut robot_params.maps {
                for waypoint in &mut map.waypoints {
                    waypoint.mul(viz_scale);
                }
            }
            owner.emit_signal("robot_params", &[robot_params.to_variant()]);
        }

        if let Some(mut navigator_state) = self.navigator_state.write().unwrap().pop() {
            if let Some(ref mut goal) = navigator_state.goal {
                goal.mul(viz_scale);
            }
            owner.emit_signal("navigator_state", &[navigator_state.to_variant()]);
        }

        if let Some(mut system_status) = self.system_status.write().unwrap().pop() {
            owner.emit_signal("system_status", &[system_status.to_variant()]);
        }

        // TODO only emit if changed
        let ui_state = self.ui_state.borrow();
        owner.emit_signal("ui_state", &[ui_state.to_variant()]);
    }
}
