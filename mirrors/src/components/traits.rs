use gdnative::api::*;

pub trait Updatable {
    fn update(&self, owner: &Node);
}