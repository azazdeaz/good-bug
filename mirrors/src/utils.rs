use gdnative::api::*;
use gdnative::prelude::*;
use nalgebra as na;

pub fn iso3_to_gd(iso: &na::Isometry3<f64>) -> Transform {
    let origin = Vector3::new(
        iso.translation.x as f32,
        iso.translation.y as f32,
        iso.translation.z as f32,
    );
    let r = iso.rotation.to_rotation_matrix();
    let basis = Basis::from_elements([
        Vector3::new(r[(0, 0)] as f32, r[(0, 1)] as f32, r[(0, 2)] as f32),
        Vector3::new(r[(1, 0)] as f32, r[(1, 1)] as f32, r[(1, 2)] as f32),
        Vector3::new(r[(2, 0)] as f32, r[(2, 1)] as f32, r[(2, 2)] as f32),
    ]);
    Transform { origin, basis }
}

// TODO make path: &str
pub fn get_node< T: SubClass<Node>>(owner: &Node, path: String) -> TRef<T> {
    unsafe {
        let node = owner.get_node(&path);
        if let Some(node) = node {
            node
                .assume_safe()
                .cast::<T>()
                .expect(&format!("failed to cast {}", path))
        }
        else {
            owner.print_tree_pretty();
            panic!("can't find node on path '{}'", path);
        }
    }
}