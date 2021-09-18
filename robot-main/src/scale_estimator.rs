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

pub struct ScaleEstimator {}

impl ScaleEstimator {
    pub fn new(broadcaster: &Broadcaster) -> Self {
        let settings = Settings::new().unwrap();
        if !settings.slam.enable_auto_slace_estimation {
            return ScaleEstimator {};
        }
        // TODO use a util for this
        let mut stream = broadcaster.stream().filter_map(|m| {
            if let Ok(Msg::Landmarks(landmarks)) = m {
                Some(landmarks)
            } else {
                None
            }
        });
        let last_value = Arc::new(Mutex::new(LastValue::new()));
        {
            let last_value = Arc::clone(&last_value);
            tokio::spawn(async move {
                while let Some(landmarks) = stream.next().await {
                    last_value.lock().unwrap().set(landmarks)
                }
            });
        }

        {
            let publisher = broadcaster.publisher();
            let last_value = Arc::clone(&last_value);
            tokio::spawn(async move {
                loop {
                    if let Some(landmarks) = last_value.lock().unwrap().pop() {
                        if let Some(scale) = estimate_scale(&landmarks) {
                            publisher.send(Msg::MapScale(scale)).ok();
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            });
        }

        ScaleEstimator {}
    }
}

fn estimate_scale(landmarks: &Vec<Landmark>) -> Option<f64> {
    // get the height of all landmarks below the camera
    let z = landmarks.into_iter().filter_map(|v| {
        let height: f64 = v.point.y;
        if height.is_sign_negative() {
            Some(height)
        } else {
            None
        }
    });

    let mut z = z.collect::<Vec<f64>>();
    z.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    // remove the upper 80% of the points
    z.drain((z.len() as f32 * 0.2) as usize..);
    // get the median of the remaining heights
    let ground_level = z.get(z.len() / 2);

    if let Some(ground_level) = ground_level {
        Some(RobotBody::get_cam_height() / ground_level.abs() as f64)
    } else {
        None
    }
}
