use common::robot_body::RobotBody;
use gdnative::api::*;
use gdnative::prelude::*;

use crate::components::Context;
use crate::utils::iso3_to_gd;
use tokio;
type Iso3 = nalgebra::Isometry3<f64>;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;
use crate::utils::{find_node, get_node};
use crate::watch_msg;

pub struct CameraPose {
    camera_pose: Receiver<Option<Iso3>>,
    // nav_target: Receiver<Option<Point3>>,
    viz_scale: Receiver<f64>,
    mesh_path: String,
}

impl CameraPose {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let camera_pose = watch_msg!(context, Msg::CameraPose);
        let viz_scale = context.ui_state.watch(|s| s.map_to_viz_scale());
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
            // nav_target,
            // mesh_path,
            mesh_path: "GUI/ViewportContainer/Viewport/Spatial/RobotBody".into(),
            viz_scale,
        };

        camera_pose
    }
}

impl Updatable for CameraPose {
    fn update(&self, owner: &Node) {
        if let Some(camera_pose) = *self.camera_pose.borrow() {
            let viz_scale = *self.viz_scale.borrow();
            let mut camera_pose = camera_pose.clone();
            camera_pose.translation.vector *= viz_scale;

            // set the position and scale of the robot model
            let mesh = get_node::<Spatial>(owner, self.mesh_path.clone());
            mesh.set_transform(iso3_to_gd(&camera_pose));
            mesh.set_visible(true);
            let robot_scale = (RobotBody::get_cam_height() * viz_scale) as f32;
            mesh.set_scale(Vector3::new(robot_scale, robot_scale, robot_scale));

            // update the vertical position of the ground plane
            let ground_mesh = find_node::<Spatial>(owner, "Ground".into());
            ground_mesh.set_translation(Vector3::new(
                0.0,
                (-RobotBody::get_cam_height() * viz_scale) as f32,
                0.0,
            ));

            let camera_target_path = "GUI/ViewportContainer/Viewport/Spatial/CamTarget";
            let camera_target = get_node::<Spatial>(owner, camera_target_path.into());
            let translation = camera_pose.translation.vector;
            camera_target.set_translation(Vector3::new(
                translation[0] as f32,
                translation[1] as f32,
                translation[2] as f32,
            ));
        }

        // if let Some(nav_target) = *self.nav_target.borrow() {
        //     let viz_scale = *self.viz_scale.borrow();
        //     let marker = find_node::<Spatial>(owner, "NextTarget".into());
        //     marker.set_translation(Vector3::new(
        //         (nav_target.x * viz_scale) as f32,
        //         (nav_target.y * viz_scale * 0.0) as f32,
        //         (nav_target.z * viz_scale) as f32,
        //     ));
        //     marker.set_visible(true);
        // }
    }
}
