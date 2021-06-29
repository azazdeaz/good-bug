use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Landmark {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub num_observations: u32,
}