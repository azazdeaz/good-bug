use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use crate::utils::iso3_to_gd;
use tokio;
type Iso3 = nalgebra::Isometry3<f64>;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::get_node;

pub struct CameraPose {
    camera_pose: Receiver<Option<Iso3>>,
    mesh_path: String,
}

impl CameraPose {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let camera_pose = context.use_client(|c| c.watch_camera_pose());
        let mesh_name = "camera_pose_box";
        let mesh_path = format!("{}/{}", path, mesh_name);
        let mesh = CSGBox::new();
        mesh.set_name(mesh_name);
        mesh.set_scale(Vector3::new(0.02, 0.02, 0.02));
        let material = SpatialMaterial::new();
        material.set_albedo(Color::rgb(1.0, 0.313726, 0.313726));
        mesh.set_material_override(material);
        
        get_node::<Node>(&*owner, path).add_child(mesh, false);

        let camera_pose = CameraPose {
            camera_pose,
            mesh_path,
        };

        camera_pose
    }
}

impl Updatable for CameraPose {
    fn update(&self, owner: &Node) {
        if let Some(camera_pose) = *self.camera_pose.borrow() {
            let mesh = get_node::<CSGBox>(owner, self.mesh_path.clone());
            mesh.set_transform(iso3_to_gd(&camera_pose));
            mesh.set_scale(Vector3::new(0.2, 0.2, 0.2));
        }
    }
}
