use crate::types::Iso3;
use nalgebra as na;
pub struct RobotBody {}

impl RobotBody {
    pub fn get_cam_height() -> f64 {
        0.105
    }
    pub fn base_pose(cam_pose: Iso3, slam_scale: f64) -> Iso3 {
        let cam_height = RobotBody::get_cam_height();
        let cam_ahead = 0.128;
        let cam2base = na::Translation3::new(0.0, cam_height * slam_scale, -cam_ahead * slam_scale);
        cam_pose * cam2base
    }
    pub fn real_distance(slam_distance: f64, slam_scale: f64) -> f64 {
        slam_distance / slam_scale
    }
}