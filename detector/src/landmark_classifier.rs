use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tokio_stream::StreamExt;

use common::{
    msg::{Broadcaster, Msg},
    settings::Settings,
    types::{Landmark, LocalizedDetection, Point3},
};

use linfa::dataset::{DatasetBase, Labels};
use linfa::traits::Transformer;
use linfa_clustering::Dbscan;
use nalgebra::distance;
use ndarray::Array2;

pub struct LandmarkClassifier {}

impl LandmarkClassifier {
    pub fn run(broadcaster: &Broadcaster) {
        let detector = Settings::new().unwrap().detector;

        // bail if detector is not enabled
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

        // connect detection messages to landmarks
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
            let mut next_id: u32 = 0;
            let mut prev_result = Vec::new();
            let mut stream = broadcaster.stream();
            tokio::spawn(async move {
                let mut map_scale = 1.0;
                while let Some(Ok(msg)) = stream.next().await {
                    match msg {
                        Msg::MapScale(scale) => map_scale = scale,
                        Msg::Landmarks(landmarks) => {
                            // store {class_id:[(landmark, score)]}
                            let mut classifieds = HashMap::new();

                            {
                                let landmark_map = landmark_map.read().await;
                                for landmark in landmarks {
                                    if landmark_map.contains_key(&landmark.id) {
                                        let top = landmark_map[&landmark.id]
                                            .iter()
                                            .max_by_key(|(_, score)| score.clone());

                                        if let Some((class, score)) = top {
                                            if score.clone() as f64 >= detector.min_landmark_score {
                                                classifieds
                                                    .entry(class.clone())
                                                    .or_insert(Vec::new())
                                                    .push((landmark, score.clone()));
                                            }
                                        }
                                    }
                                }
                            }

                            let mut result = Vec::new();

                            for (class, landmarks_and_weights) in classifieds {
                                let (landmarks, weights): (Vec<Landmark>, Vec<u32>) =
                                    landmarks_and_weights.into_iter().unzip();

                                let dataset = landmarks
                                    .iter()
                                    .flat_map(|lm| lm.point.coords.as_slice().to_owned())
                                    .collect::<Vec<_>>();
                                let dataset =
                                    Array2::from_shape_vec((dataset.len() / 3, 3), dataset)
                                        .unwrap();
                                let dataset: DatasetBase<_, _> = dataset.into();

                                let weights =
                                    weights.into_iter().map(|w| w as f32).collect::<Vec<_>>();
                                let dataset = dataset.with_weights(weights.into());

                                let cluster_memberships =
                                    Dbscan::params(detector.clustering_min_landmarks as usize)
                                        .tolerance(detector.clustering_max_distance * map_scale)
                                        .transform(dataset);

                                let label_count = cluster_memberships.label_count().remove(0);
                                println!();
                                println!("Results in class {}: ", class);
                                for (label, count) in label_count {
                                    match label {
                                        None => println!(" - {} noise points", count),
                                        Some(i) => println!(" - {} points in cluster {}", count, i),
                                    }
                                }

                                let targets = cluster_memberships.targets();
                                for label in cluster_memberships.labels() {
                                    if let Some(_) = label {
                                        let mut center_sum = Point3::new(0.0, 0.0, 0.0);
                                        let mut center_count = 0.0;
                                        let landmarks = landmarks
                                            .iter()
                                            .enumerate()
                                            .filter_map(|(i, lm)| {
                                                if label == targets[i] {
                                                    let weight =
                                                        cluster_memberships.weight_for(i) as f64;
                                                    center_sum += lm.point.coords * weight;
                                                    center_count += weight;
                                                    Some(lm.clone())
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect::<Vec<_>>();

                                        let center = center_sum / center_count;

                                        let scaled_center = center * map_scale;

                                        // try to find the same detection in the previous results
                                        let prev_closest = prev_result
                                            .iter()
                                            .filter_map(|d: &LocalizedDetection| {
                                                let dist = distance(
                                                    &scaled_center,
                                                    &(d.center * map_scale),
                                                );
                                                Some((d, dist))
                                            })
                                            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                                            .and_then(|(d, dist)| {
                                                println!("CLoses dist {} <> {}", dist
                                                , detector.clustering_max_distance * map_scale);
                                                if dist < detector.clustering_max_distance * map_scale
                                                {
                                                    Some(d)
                                                } else {
                                                    None
                                                }
                                            });
                                        
                                        let id = if let Some(prev_closest) = prev_closest {
                                            prev_closest.id
                                        }
                                        else {
                                            next_id += 1;
                                            next_id
                                        };

                                        
                                        result.push(LocalizedDetection {
                                            id,
                                            landmarks,
                                            class,
                                            center,
                                        });
                                    }
                                }
                            }
                            prev_result = result.clone();
                            publisher.send(Msg::LocalizedDetections(result)).ok();
                        }
                        _ => (),
                    }
                }
            });
        }
    }
}
