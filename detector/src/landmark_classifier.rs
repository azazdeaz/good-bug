use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tokio_stream::StreamExt;

use common::{
    msg::{Broadcaster, Msg},
    settings::Settings,
    types::LocalizedDetection,
};

use linfa::dataset::{DatasetBase, Labels};
use linfa::traits::Transformer;
use linfa_clustering::Dbscan;
use ndarray::Array2;

pub struct LandmarkClassifier {}

impl LandmarkClassifier {
    pub fn run(broadcaster: &Broadcaster) {
        let detector = Settings::new().unwrap().detector;

        let detector = if let Some(detector) = detector {
            if detector.enabled {
                detector
            } else {
                return;
            }
        } else {
            return;
        };

        let landmark_map = Arc::new(RwLock::new(HashMap::new()));

        {
            let mut detections_stream = broadcaster.stream().filter_map(|m| {
                if let Ok(Msg::Detections(detections)) = m {
                    Some(detections)
                } else {
                    None
                }
            });
            let publisher = broadcaster.publisher();
            let landmark_map = Arc::clone(&landmark_map);
            tokio::spawn(async move {
                while let Some(detections) = detections_stream.next().await {
                    let mut landmark_map = landmark_map.write().await;
                    for detection in detections {
                        for feature in detection.features {
                            let classes = landmark_map
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

        {
            let publisher = broadcaster.publisher();
            let landmark_map = Arc::clone(&landmark_map);
            let mut stream = broadcaster.stream();
            tokio::spawn(async move {
                let mut map_scale = 1.0;
                while let Some(Ok(msg)) = stream.next().await {
                    match msg {
                        Msg::MapScale(scale) => map_scale = scale,
                        Msg::Landmarks(landmarks) => {
                            let mut classifieds = HashMap::new();

                            {
                                let landmark_map = landmark_map.read().await;
                                for landmark in landmarks {
                                    if landmark_map.contains_key(&landmark.id) {
                                        let top = landmark_map[&landmark.id]
                                            .iter()
                                            .max_by_key(|(_, val)| val.clone());

                                        if let Some((class, _)) = top {
                                            classifieds
                                                .entry(class.clone())
                                                .or_insert(Vec::new())
                                                .push(landmark);
                                        }
                                    }
                                }
                            }

                            let mut result = Vec::new();

                            for (class, landmarks) in classifieds {
                                let dataset = landmarks
                                    .iter()
                                    .flat_map(|lm| lm.point.coords.as_slice().to_owned())
                                    .collect::<Vec<_>>();
                                let dataset =
                                    Array2::from_shape_vec((dataset.len() / 3, 3), dataset)
                                        .unwrap();
                                let dataset: DatasetBase<_, _> = dataset.into();

                                let cluster_memberships =
                                    Dbscan::params(detector.clustering_min_landmarks as usize)
                                        .tolerance(detector.clustering_max_distance * map_scale)
                                        .transform(dataset);

                                // sigle target dataset
                                let label_count = cluster_memberships.label_count().remove(0);

                                println!();
                                println!("Result: ");
                                for (label, count) in label_count {
                                    match label {
                                        None => println!(" - {} noise points", count),
                                        Some(i) => println!(" - {} points in cluster {}", count, i),
                                    }
                                }

                                let targets = cluster_memberships.targets();
                                for label in cluster_memberships.labels() {
                                    if let Some(_) = label {
                                        let landmarks = landmarks
                                            .iter()
                                            .enumerate()
                                            .filter_map(|(i, lm)| {
                                                if label == targets[i] {
                                                    Some(lm.clone())
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect::<Vec<_>>();

                                        result.push(LocalizedDetection { landmarks, class });
                                    }
                                }
                            }
                            // println!("LocalizedLandmarks {:?}", result);
                            publisher.send(Msg::LocalizedDetections(result)).ok();
                        }
                        _ => (),
                    }
                }
            });
        }
    }
}
