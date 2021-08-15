use serde::{Serialize, Deserialize};

pub type Iso3 = nalgebra::Isometry3<f64>;
pub type Point3 = nalgebra::Point3<f64>;

use gdnative::prelude::*;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToVariant)]
pub struct BoxDetection {
    pub ymin: f32,
    pub xmin: f32,
    pub ymax: f32,
    pub xmax: f32,
    pub score: f32,
    pub class: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToVariant, FromVariant)]
pub enum NavigationMode {
    Teleop,
    Goal,
    Waypoints
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToVariant, FromVariant, Default)]
pub struct NavGoal {
    pub x: f64,
    pub z: f64,
}
impl NavGoal {
    pub fn new(x: f64, z: f64) -> Self {
        Self {x, z}
    }

    pub fn scale_to_map(&mut self, map_to_viz_scale: f64) {
        self.x /= map_to_viz_scale;
        self.z /= map_to_viz_scale;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToVariant)]
pub struct Map {
    pub name: String,
    pub db_path: String,
    pub waypoints: Vec<NavGoal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToVariant, Default)]
pub struct RobotParams {
    pub maps: Vec<Map>,
    pub current_map_name: Option<String>,
}



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