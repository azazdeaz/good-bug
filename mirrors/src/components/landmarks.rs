use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use common::{robot_body::RobotBody, utils::LastValue};
use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use crate::utils::find_node;
use scarlet::color::RGBColor;
use scarlet::colormap::ListedColorMap;
use scarlet::colorpoint::ColorPoint;
use tokio;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::get_node;
use crate::watch_msg_once;
use common::types::Landmark;

pub struct Landmarks {
    landmarks: Arc<RwLock<LastValue<Vec<Landmark>>>>,
    landmark_classes: Arc<RwLock<LastValue<HashMap<u32, HashMap<u32, u32>>>>>,
    viz_scale: Receiver<f64>,
    geometry_path: String,
    class_colors: [Color; 5],
    color_map: ListedColorMap,
}

impl Landmarks {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let landmarks = watch_msg_once!(context, Msg::Landmarks);
        let landmark_classes = watch_msg_once!(context, Msg::LandmarkClassification);
        let viz_scale = context.ui_state.watch(|s| s.map_to_viz_scale());

        let geometry = ImmediateGeometry::new();
        let geometry_name = "landmarks_component";
        let geometry_path = format!("{}/{}", path, geometry_name);
        geometry.set_name(geometry_name);

        let material = SpatialMaterial::new();
        material.set_point_size(3.0);
        material.set_flag(SpatialMaterial::FLAG_USE_POINT_SIZE, true);
        material.set_flag(SpatialMaterial::FLAG_ALBEDO_FROM_VERTEX_COLOR, true);
        geometry.set_material_override(material);

        get_node::<Node>(&*owner, path).add_child(geometry, false);

        #[rustfmt::skip]
        let class_colors = [
            Color::rgb(0xE0 as f32 / 255.0,0x13 as f32 / 255.0,0x00 as f32 / 255.0,), //#E01300
            Color::rgb(0xF0 as f32 / 255.0,0x00 as f32 / 255.0,0xE2 as f32 / 255.0,), //#F000E2
            Color::rgb(0xED as f32 / 255.0,0x6F as f32 / 255.0,0x0C as f32 / 255.0,), //#ED6F0C
            Color::rgb(0xF7 as f32 / 255.0,0x0C as f32 / 255.0,0x58 as f32 / 255.0,), //#F70C58
            Color::rgb(0xF7 as f32 / 255.0,0x4A as f32 / 255.0,0x0C as f32 / 255.0,), //#F74A0C
        ];

        let landmarks = Landmarks {
            landmarks,
            landmark_classes,
            viz_scale,
            geometry_path,
            class_colors,
            color_map: ListedColorMap::viridis(),
        };

        landmarks
    }
}

impl Updatable for Landmarks {
    fn update(&self, owner: &Node) {
        if let Some(landmarks) = self.landmarks.write().unwrap().pop() {
            let viz_scale = *self.viz_scale.borrow();
            let landmark_mesh = get_node::<ImmediateGeometry>(owner, self.geometry_path.clone());

            landmark_mesh.clear();
            landmark_mesh.begin(Mesh::PRIMITIVE_POINTS, Null::null());

            let start = RGBColor::from_hex_code("#FFE801").unwrap();
            let end = RGBColor::from_hex_code("#00FFC3").unwrap();
            let landmark_colors = start.gradient(&end);

            let classes = self
                .landmark_classes
                .write()
                .unwrap()
                .pop()
                .unwrap_or(HashMap::new());

            for landmark in landmarks {
                let color = if classes.contains_key(&landmark.id) {
                    let top = classes[&landmark.id]
                        .iter()
                        .max_by_key(|(_, val)| val.clone());

                    if let Some((class, _)) = top {
                        let idx = class.clone() as usize % self.class_colors.len();
                        self.class_colors[idx]
                    } else {
                        Color::rgb(0.0, 0.0, 0.0)
                    }
                } else {
                    let val = f64::min(1.0, landmark.num_observations as f64 / 24.0);
                    let color = landmark_colors(val);
                    Color::rgb(color.r as f32, color.g as f32, color.b as f32)
                };

                let point = Vector3::new(
                    (landmark.point.x * viz_scale) as f32,
                    (landmark.point.y * viz_scale) as f32,
                    (landmark.point.z * viz_scale) as f32,
                );
                landmark_mesh.set_color(color);
                landmark_mesh.add_vertex(point);
            }

            landmark_mesh.end();

            // update the vertical position of the ground plane
            let mesh = find_node::<Spatial>(owner, "Ground".into());
            mesh.set_translation(Vector3::new(
                0.0,
                (-RobotBody::get_cam_height() * viz_scale) as f32 * 1.2,
                0.0,
            ));
        }
    }
}
