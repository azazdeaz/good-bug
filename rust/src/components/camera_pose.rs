use gdnative::api::*;
use gdnative::prelude::*;

use crate::grpc_client::GrpcClient;
use crate::utils::iso3_to_gd;
use tokio;
type Iso3 = nalgebra::Isometry3<f64>;
use tokio::sync::watch::Receiver;

use crate::components::traits::Updatable;

pub struct CameraPose {
    camera_pose: Receiver<Option<Iso3>>,
    btn_path: String,
    mesh_path: String,
}

impl CameraPose {
    pub fn new(owner: TRef<Node>, path: String, client: &GrpcClient) -> Self {
        let camera_pose = client.watch_camera_pose();
        let btn = Button::new();
        let btn_name = "new_new_buttonian";
        let btn_path = format!("{}/{}", path, btn_name);
        btn.set_name(btn_name);
        unsafe {
            owner
                .get_node(path)
                .unwrap()
                .assume_safe()
                .cast::<Node>()
                .unwrap()
                .add_child(btn, false);
        }

        let mesh = CSGBox::new();
        mesh.set_name("camera_pose");

        let material = ShaderMaterial::new();

        let x = unsafe {
            owner
                .get_node("Spatial")
                .unwrap()
                .assume_safe()
                .cast::<Node>()
                .unwrap()
                .add_child(mesh, false);
        };

        let camera_pose = CameraPose {
            camera_pose,
            btn_path,
            mesh_path: "Spatial/camera_pose".into(),
        };

        camera_pose
    }
}

impl Updatable for CameraPose {
    fn update(&self, owner: &Node) {
        if let Some(camera_pose) = *self.camera_pose.borrow() {
            let btn = unsafe {
                owner
                    .get_node(&self.btn_path)
                    .unwrap()
                    .assume_safe()
                    .cast::<Button>()
                    .unwrap()
            };
            btn.set_text(format!("IMA BUTTON! {:?}", camera_pose));

            let mesh = unsafe {
                owner
                    .get_node(&self.mesh_path)
                    .unwrap()
                    .assume_safe()
                    .cast::<CSGBox>()
                    .unwrap()
            };
            mesh.set_transform(iso3_to_gd(&camera_pose));
        }
    }
}
