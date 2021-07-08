use serde::{Serialize, Deserialize};

pub type Iso3 = nalgebra::Isometry3<f64>;
pub type Point3 = nalgebra::Point3<f64>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Landmark {
    pub id: u32,
    pub point: Point3,
    pub num_observations: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Keyframe {
    pub id: u32,
    pub pose: Iso3,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Edge {
    pub id0: u32,
    pub id1: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TrackingState {
    NotInitialized,
    Initializing,
    Tracking,
    Lost,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InJoyMotion {
    pub axis: i64,
    pub axis_value: f64,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InJoyButton {
    pub button_index: i64,
    pub pressed: bool,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GDInput {
    JoyMotion(InJoyMotion),
    JoyButton(InJoyButton),
}