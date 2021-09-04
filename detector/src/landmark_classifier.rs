use std::collections::HashMap;
use tokio_stream::StreamExt;

use common::msg::{Broadcaster, Msg};

pub struct LandmarkClassifier {}

impl LandmarkClassifier {
    pub fn run(broadcaster: &Broadcaster) {
        let mut stream = broadcaster.stream().filter_map(|m| {
            if let Ok(Msg::Detections(detections)) = m {
                Some(detections)
            } else {
                None
            }
        });

        let mut landmark_map = HashMap::new();
        let publisher = broadcaster.publisher();

        tokio::spawn(async move {
            while let Some(detections) = stream.next().await {
                for detection in detections {
                    for feature in detection.features {
                        let mut classes = landmark_map
                            .entry(feature.landmark.id)
                            .or_insert(HashMap::new());
                        *classes.entry(detection.class).or_insert(0u32) += 1;
                    }
                }
                publisher
                    .send(Msg::LandmarkClassification(landmark_map.clone()))
                    .ok();
            }
        });
    }
}
