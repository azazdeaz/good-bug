use gdnative::api::*;
use gdnative::prelude::*;

use crate::grpc_client::GrpcClient;
use tokio;
type Iso3 = nalgebra::Isometry3<f64>;
use tokio::sync::watch::Receiver;
use common::types::Landmark;
use crate::components::traits::Updatable;

pub struct Landmarks {
    landmarks: Receiver<Vec<Landmark>>,
    btn_path: String,
}

impl Landmarks {
    pub fn new(owner: TRef<Node>, path: String, client: &GrpcClient) -> Self {
        let landmarks = client.watch_landmarks();
        let btn = Button::new();
        let btn_name = "new_new_buttonian_lm";
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

        let landmarks = Landmarks {
            landmarks,
            btn_path,
        };

        landmarks
    }
}

impl Updatable for Landmarks {
    fn update(&self, owner: &Node) {
        let landmarks = &*self.landmarks.borrow();
        let btn = unsafe {
            owner
                .get_node(&self.btn_path)
                .unwrap()
                .assume_safe()
                .cast::<Button>()
                .unwrap()
        };
        btn.set_text(format!("IMA LM! {:?}", landmarks));
    }
}
