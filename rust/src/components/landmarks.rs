use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use tokio;
use tokio::sync::watch::Receiver;
use scarlet::colormap::ListedColorMap;

use common::types::Landmark;
use crate::components::traits::Updatable;
use crate::utils::get_node;

pub struct Landmarks {
    landmarks: Receiver<Vec<Landmark>>,
    geometry_path: String,
}

impl Landmarks {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let landmarks = context.client.read().unwrap().watch_landmarks();
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

        let landmarks = Landmarks {
            landmarks,
            geometry_path,
        };

        landmarks
    }
}

impl Updatable for Landmarks {
    fn update(&self, owner: &Node) {
        let landmarks = &*self.landmarks.borrow();
        let colormap: ListedColorMap = ListedColorMap::plasma();
        let landmark_mesh = get_node::<ImmediateGeometry>(owner, self.geometry_path.clone());
        // let landmark_mesh = unsafe {
        //     owner
        //         .get_node(&self.geometry_path)
        //         .unwrap()
        //         .assume_safe()
        //         .cast::<ImmediateGeometry>()
        //         .unwrap()
        // };
        


        landmark_mesh.clear();
        landmark_mesh.begin(Mesh::PRIMITIVE_POINTS, Null::null());

        for landmark in landmarks {
            let val =
                0.5 + f64::min(0.5, landmark.num_observations as f64 / 24.0); //self.values.max_lm_obs as f64;
            let color = colormap.vals
                [(val * (colormap.vals.len() - 1) as f64) as usize];
            let color =
                Color::rgb(color[0] as f32, color[1] as f32, color[2] as f32);
            
            let point = Vector3::new(
                landmark.x as f32,
                landmark.y as f32,
                landmark.z as f32,
            );
            landmark_mesh.set_color(color);
            landmark_mesh.add_vertex(point);
        }

        landmark_mesh.end();
    }
}
