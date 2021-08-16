use std::ops::Mul;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use nalgebra as na;
use tokio;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::get_node;
use crate::watch_msg;
use common::types::Keyframe;

pub struct Keyframes {
    keyframes: Receiver<Option<Vec<Keyframe>>>,
    viz_scale: Receiver<f64>,
    geometry_path: String,
    wireframe: [na::Point3<f64>; 12],
}

impl Keyframes {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let keyframes = watch_msg!(context, Msg::Keyframes);
        let viz_scale = context.ui_state.watch(|s| s.map_to_viz_scale());

        let geometry = ImmediateGeometry::new();
        let geometry_name = "keyframes_component";
        let geometry_path = format!("{}/{}", path, geometry_name);
        geometry.set_name(geometry_name);

        let material = SpatialMaterial::new();
        material.set_point_size(2.0);
        material.set_flag(SpatialMaterial::FLAG_USE_POINT_SIZE, true);
        // material.set_flag(SpatialMaterial::FLAG_ALBEDO_FROM_VERTEX_COLOR, true);
        geometry.set_material_override(material);

        get_node::<Node>(&*owner, path).add_child(geometry, false);

        let wireframe = {
            let scale = 0.1;
            let f = 1.0 * scale;
            let cx = 2.0 * scale;
            let cy = 1.0 * scale;
            let c = na::Point3::new(0.0, 0.0, 0.0);
            let tl = na::Point3::new(-cx, cy, f);
            let tr = na::Point3::new(cx, cy, f);
            let br = na::Point3::new(cx, -cy, f);
            let bl = na::Point3::new(-cx, -cy, f);
            [c, tl, tr, c, tr, br, c, br, bl, c, bl, tl]
        };

        let keyframes = Keyframes {
            keyframes,
            viz_scale,
            geometry_path,
            wireframe,
        };

        keyframes
    }
}

impl Updatable for Keyframes {
    fn update(&self, owner: &Node) {
        let keyframes = &*self.keyframes.borrow();
        let viz_scale = *self.viz_scale.borrow();
        if let Some(keyframes) = keyframes {
            let keyframe_mesh = get_node::<ImmediateGeometry>(owner, self.geometry_path.clone());

            keyframe_mesh.clear();

            for keyframe in keyframes {
                keyframe_mesh.begin(Mesh::PRIMITIVE_LINE_STRIP, Null::null());
                let mut pose = keyframe.pose.clone();
                pose.translation.vector *= viz_scale;
                for p in self.wireframe {
                    let p = pose * p;
                    let point =
                        Vector3::new(p.coords[0] as f32, p.coords[1] as f32, p.coords[2] as f32);
                    // keyframe_mesh.set_color(color);
                    keyframe_mesh.add_vertex(point);
                }
                keyframe_mesh.end();
            }
        }
    }
}
