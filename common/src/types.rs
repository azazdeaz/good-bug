use serde::{Serialize, Deserialize};

pub type Iso3 = nalgebra::Isometry3<f64>;
pub type Point3 = nalgebra::Point3<f64>;
pub type Point2 = nalgebra::Point2<f64>;

use gdnative::prelude::*;
use nalgebra as na;

fn point3_to_variant(p: &Point3) -> Vector3 {
    let x = p.coords[0] as f32;
    let y = p.coords[1] as f32;
    let z = p.coords[2] as f32;
    Vector3::new(x, y, z)
}

fn point2_to_variant(p: &Point2) -> Vector2 {
    let x = p.coords[0] as f32;
    let y = p.coords[1] as f32;
    Vector2::new(x, y)
}
#[derive(Debug, Clone, Serialize, Deserialize, ToVariant)]
pub struct BoxDetection {
    pub ymin: f32,
    pub xmin: f32,
    pub ymax: f32,
    pub xmax: f32,
    pub score: f32,
    pub class: u32,
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToVariant)]
pub struct LocalizedDetection {
    pub class: u32,
    pub landmarks: Vec<Landmark>,
    #[variant(to_variant_with = "point3_to_variant")]
    pub center: Point3,
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

    pub fn div(&mut self, scale: f64) {
        self.x /= scale;
        self.z /= scale;
    }

    pub fn mul(&mut self, scale: f64) {
        self.x *= scale;
        self.z *= scale;
    }

    pub fn as_vector2(&self) -> na::Vector2<f64> {
        na::Vector2::new(self.x, self.z)
    }
}

impl std::ops::MulAssign<f64> for NavGoal {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.z *= rhs;
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


#[derive(Debug, Clone, Serialize, Deserialize, ToVariant, Default)]
pub struct NavigatorState {
    pub goal: Option<NavGoal>,
    pub speed: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToVariant, Default)]
pub struct SystemStatus {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub cpu_temperature: f32,
    pub cpu_usage: f32,
    pub battery: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToVariant)]
pub struct Landmark {
    pub id: u32,
    #[variant(to_variant_with = "point3_to_variant")]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToVariant)]
pub struct Feature {
    #[variant(to_variant_with = "point2_to_variant")]
    pub keypoint: Point2,
    pub landmark: Landmark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlamFrame {
    pub jpeg: Vec<u8>,
    pub features: Vec<Feature>,
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