use std::sync::{Arc, RwLock};

use common::types::Feature;
use common::utils::LastValue;

use common::types::SlamFrame;
use gdnative::api::*;
// use gdnative::api::texture_rect::StretchMode;
use gdnative::prelude::*;

use tokio;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::components::Context;
use crate::utils::find_node;
use crate::utils::get_node;
use crate::watch_msg_once;
use common::types::BoxDetection;

pub struct Frame {
    frame: Arc<RwLock<LastValue<SlamFrame>>>,
    detections: Arc<RwLock<LastValue<Vec<BoxDetection>>>>,
    viz_scale: Receiver<f64>,
    geometry_path: String,
    // panel_path: String,
    // rect_paths: Vec<String>,
}

impl Frame {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let frame = watch_msg_once!(context, Msg::Frame);
        let detections = watch_msg_once!(context, Msg::Detections);
        let viz_scale = context.ui_state.watch(|s| s.map_to_viz_scale());

        let geometry = ImmediateGeometry::new();
        let geometry_name = "detection_landmarks_component";
        let geometry_path = format!("{}/{}", path, geometry_name);
        geometry.set_name(geometry_name);

        let material = SpatialMaterial::new();
        material.set_point_size(5.0);
        material.set_flag(SpatialMaterial::FLAG_USE_POINT_SIZE, true);
        material.set_flag(SpatialMaterial::FLAG_ALBEDO_FROM_VERTEX_COLOR, true);
        geometry.set_material_override(material);

        get_node::<Node>(&*owner, path).add_child(geometry, false);

        Frame {
            frame,
            detections,
            viz_scale,
            geometry_path,
        }
    }
}

impl Updatable for Frame {
    fn update(&self, owner: &Node) {
        if let Some(frame) = self.frame.write().unwrap().pop() {
            let panel = find_node::<TextureRect>(owner, "CameraImage".into());
            let im = Image::new();
            im.load_jpg_from_buffer(TypedArray::from_vec(frame.jpeg.clone()))
                .expect("failed to load jpeg with godot");
            // im.create_from_data(1280, 960, true, Image::FORMAT_RGB8, TypedArray::from_vec(pixels));
            let imt = ImageTexture::new();
            imt.create_from_image(im, 7);
            panel.set_texture(imt);
        }

        if let Some(detections) = self.detections.write().unwrap().pop() {
            let landmark_mesh = get_node::<ImmediateGeometry>(owner, self.geometry_path.clone());
            landmark_mesh.clear();
            let gd_detections = detections.to_variant();
            let panel = find_node::<TextureRect>(owner, "CameraImage".into());
            unsafe {
                panel.call("update_detections", &[gd_detections]);
            }

            let viz_scale = *self.viz_scale.borrow();
            landmark_mesh.begin(Mesh::PRIMITIVE_POINTS, Null::null());
            for detection in detections {
                for Feature { landmark, .. } in detection.features {
                    let color = Color::rgb(1.0, 0.0, 0.0);

                    let point = Vector3::new(
                        (landmark.point.x * viz_scale) as f32,
                        (landmark.point.y * viz_scale) as f32,
                        (landmark.point.z * viz_scale) as f32,
                    );
                    landmark_mesh.set_color(color);
                    landmark_mesh.add_vertex(point);
                }
            }

            landmark_mesh.end();
        }
    }
}
