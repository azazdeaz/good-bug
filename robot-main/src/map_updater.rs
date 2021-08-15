

use common::{
    msg::{Broadcaster, Msg},
    types::Landmark,
    utils::LastValue,
    robot_body::RobotBody,
    settings::Settings,
};
use std::{
    sync::{Arc, Mutex},
};
use tokio_stream::StreamExt;

// Updates the map data (waypoints, current_map) in the settings file
pub struct MapUpdater {}

impl MapUpdater {
    pub fn run(broadcaster: &Broadcaster) {
        // TODO use a util for this
        let mut stream = broadcaster.stream();
        {
            tokio::spawn(async move {
                while let Some(msg) = stream.next().await {
                    if let Ok(msg) = msg {
                        match msg {
                            Msg::Waypoints(waypoints) => {
                                println!("robot got waypoints {:?}", waypoints);
                                Settings::new().unwrap().set_waypoints(waypoints);
                            }
                            Msg::SelectMap(name) => {
                                Settings::new().unwrap().set_current_map_name(name);
                            }
                            _ => {}
                        }
                    }
                }
            });
        }
    }
}

fn estimate_scale(landmarks: &Vec<Landmark>) -> Option<f64> {
    let z = landmarks.into_iter().filter_map(|v| {
        let height: f64 = v.point.y;
        if height.is_sign_negative() {
            Some(height)
        } else {
            None
        }
    });

    let z = ndarray::Array::from_iter(z);
    let ground_level = z.mean();
    
    if let Some(ground_level) = ground_level {
        Some(RobotBody::get_cam_height() / ground_level.abs() as f64)
    } else {
        None
    }
}
