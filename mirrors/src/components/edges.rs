use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::{colors, Context};
use tokio;
use tokio::sync::watch::Receiver;

use common::types::{Edge, Keyframe};
use crate::components::traits::Updatable;
use crate::utils::get_node;
use crate::watch_msg;
use std::collections::HashMap;

pub struct Edges {
    edges: Receiver<Option<Vec<Edge>>>,
    keyframes: Receiver<Option<Vec<Keyframe>>>,
    map_scale: Receiver<Option<f64>>,
    viz_scale: Receiver<f64>,
    geometry_path: String,
}

impl Edges {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let edges = watch_msg!(context, Msg::Edges);
        let keyframes = watch_msg!(context, Msg::Keyframes);
        let map_scale = watch_msg!(context, Msg::MapScale);
        let viz_scale = context.ui_state.watch(|s| s.viz_scale);

        let geometry = ImmediateGeometry::new();
        let geometry_name = "edges";
        let geometry_path = format!("{}/{}", path, geometry_name);
        geometry.set_name(geometry_name);

        let material = SpatialMaterial::new();
        // material.set_point_size(2.0);
        // material.set_flag(SpatialMaterial::FLAG_USE_POINT_SIZE, true);
        material.set_flag(SpatialMaterial::FLAG_ALBEDO_FROM_VERTEX_COLOR, true);
        geometry.set_material_override(material);
        
        get_node::<Node>(&*owner, path).add_child(geometry, false);


        let edges = Edges {
            edges,
            keyframes,
            map_scale,
            viz_scale,
            geometry_path,
        };

        edges
    }
}

impl Updatable for Edges {
    fn update(&self, owner: &Node) {
        let edges = &*self.edges.borrow();
        let keyframes = &*self.keyframes.borrow();
        let viz_scale = *self.viz_scale.borrow();   
        let map_scale = self.map_scale.borrow().unwrap_or(1.0) * viz_scale;
        if let (Some(edges), Some(keyframes)) = (edges, keyframes) {
            let edges_mesh = get_node::<ImmediateGeometry>(owner, self.geometry_path.clone());

            let keyframes: HashMap<u32, &Keyframe> = keyframes.iter().map(|kf| (kf.id, kf)).collect();

            edges_mesh.clear();
            edges_mesh.begin(Mesh::PRIMITIVE_LINES, Null::null());
            edges_mesh.set_color(colors::EDGE.as_godot());

                            //             edges_mesh.clear();
                    //             
                    //             
                    //             for e in edges_.iter() {
                    //                 let k0 = self.values.keyframes.get(&e.id0);
                    //                 let k1 = self.values.keyframes.get(&e.id1);
                    //                 if let (Some(k0), Some(k1)) = (k0, k1) {
                    //                     edges_mesh.add_vertex(k0[0]);
                    //                     edges_mesh.add_vertex(k1[0]);
                    //                 }
                    //             }
                    //             edges_mesh.end();
            
            for edge in edges {
                let keyframe0 = keyframes.get(&edge.id0);
                let keyframe1 = keyframes.get(&edge.id1);
                if let (Some(keyframe0), Some(keyframe1)) = (keyframe0, keyframe1) {
                    let p0 = Vector3::new(
                        (keyframe0.pose.translation.vector[0] * map_scale) as f32,
                        (keyframe0.pose.translation.vector[1] * map_scale) as f32,
                        (keyframe0.pose.translation.vector[2] * map_scale) as f32,
                    );
                    let p1 = Vector3::new(
                        (keyframe1.pose.translation.vector[0] * map_scale) as f32,
                        (keyframe1.pose.translation.vector[1] * map_scale) as f32,
                        (keyframe1.pose.translation.vector[2] * map_scale) as f32,
                    );
                    edges_mesh.add_vertex(p0);
                    edges_mesh.add_vertex(p1);
                }
                
            }
            edges_mesh.end();
        }
    }
}
