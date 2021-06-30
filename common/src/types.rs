use serde::{Serialize, Deserialize};

pub type Iso3 = nalgebra::Isometry3<f64>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Landmark {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub num_observations: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Keyframe {
    pub id: u32,
    pub pose: Iso3,
}