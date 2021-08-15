use common::{
    msg::{Broadcaster, Msg},
    robot_body::RobotBody,
    settings::{Navigation, Settings},
    types::{NavigationMode, Point3, TrackingState},
};
use drivers::Wheels;
use nalgebra as na;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio_stream::StreamExt;

type Iso3 = na::Isometry3<f64>;

fn angle_difference(bearing_from: f64, bearing_to: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let diff = bearing_to - bearing_from;

    if diff > pi {
        diff - pi * 2.0
    } else if diff < -pi {
        diff + pi * 2.0
    } else {
        diff
    }
}
// fn test() {
//     let pi = std::f64::consts::PI;
//     println!("{} {}", angle_difference(pi-0.1, -pi+0.1), 0.2);
//     println!("{} {}", angle_difference(-0.1, 0.1), 0.2);
//     println!("{} {}", angle_difference(-pi, pi), 0.0);
//     println!("{} {}", angle_difference(pi, -pi), 0.0);
//     println!("{} {}", angle_difference(-pi, pi-0.1), -0.1);
// }

#[derive(Debug)]
struct NavState {
    speed: (f64, f64),
    teleop_speed: ((f64, f64), Instant),
    cam_pose: (Iso3, Instant),
    target_pose: Option<Point3>,
    navigation_mode: NavigationMode,
    tracker_state: TrackingState,
    slam_scale: f64,
    settings: Navigation,
}

impl NavState {
    fn new() -> Self {
        let settings = Settings::new().unwrap().navigation;
        NavState {
            speed: (0.0, 0.0),
            teleop_speed: ((0.0, 0.0), Instant::now()),
            cam_pose: (Iso3::identity(), Instant::now()),
            target_pose: None,
            navigation_mode: NavigationMode::Teleop,
            tracker_state: TrackingState::NotInitialized,
            slam_scale: 1.0,
            settings,
        }
    }

    fn set_teleop_speed(&mut self, speed: (f64, f64)) {
        self.teleop_speed = (speed, Instant::now());
    }

    fn set_cam_pose(&mut self, cam_pose: Iso3) {
        self.cam_pose = (cam_pose, Instant::now());
    }

    fn set_target_pose(&mut self, target_pose: Point3) {
        self.target_pose = Some(target_pose);
    }

    fn set_navigation_mode(&mut self, mode: NavigationMode) {
        self.navigation_mode = mode;
    }

    fn set_tracker_state(&mut self, tracker_state: TrackingState) {
        self.tracker_state = tracker_state;
    }

    fn set_slam_scale(&mut self, slam_scale: f64) {
        self.slam_scale = slam_scale;
    }

    fn is_expired(time: Instant) -> bool {
        time.checked_add(Duration::from_millis(600)).unwrap() < Instant::now()
    }

    fn compute_speed(&self) -> (f64, f64) {
        match self.navigation_mode {
            NavigationMode::Teleop => {
                if NavState::is_expired(self.teleop_speed.1) {
                    (0.0, 0.0)
                } else {
                    self.teleop_speed.0
                }
            }
            NavigationMode::Goal => {
                if NavState::is_expired(self.cam_pose.1)
                    || !matches!(self.tracker_state, TrackingState::Tracking)
                {
                    (0.0, 0.0)
                } else if let Some(target_pose) = self.target_pose {
                    let pose = RobotBody::base_pose(self.cam_pose.0, self.slam_scale);

                    let p = na::Point3::new(0.0, 0.0, 1.0);
                    let p = pose.rotation * p;
                    let yaw_bot = p.x.atan2(p.z);

                    let dx = target_pose.x - pose.translation.vector.x;
                    let dz = target_pose.z - pose.translation.vector.z;
                    let yaw_target = dx.atan2(dz);
                    let yawd = angle_difference(yaw_bot, yaw_target);
                    let distance = dx.hypot(dz);
                    let distance = RobotBody::real_distance(distance, self.slam_scale);

                    println!(
                        "\nfrom {:?} to {:?} is |{},{}|={}; yaw_target={} yaw_bot={} yawd={}",
                        pose.translation.vector,
                        target_pose,
                        dx,
                        dz,
                        distance,
                        yaw_target,
                        yaw_bot,
                        yawd
                    );

                    if distance < self.settings.xy_goal_tolerance {
                        (0., 0.)
                    } else if yawd.abs() < 0.3 {
                        (self.settings.travel_thrust, self.settings.travel_thrust)
                    } else if yawd > 0. {
                        // turning left
                        (
                            self.settings.turn_right_thrust.1,
                            self.settings.turn_right_thrust.0,
                        )
                    } else {
                        (
                            self.settings.turn_right_thrust.0,
                            self.settings.turn_right_thrust.1,
                        )
                    }
                } else {
                    (0.0, 0.0)
                }
            }
            NavigationMode::Waypoints => (0.0, 0.0),
        }
    }
}

pub struct Navigator {}

impl Navigator {
    pub fn new(broadcaster: &Broadcaster) -> Self {
        let mut state = Arc::new(tokio::sync::RwLock::new(NavState::new()));
        let mut wheels = Wheels::new();

        {
            let mut updates = broadcaster.stream();
            let state = Arc::clone(&state);
            tokio::spawn(async move {
                loop {
                    while let Some(msg) = updates.next().await {
                        if let Ok(msg) = msg {
                            let mut state = state.write().await;
                            match msg {
                                Msg::CameraPose(iso3) => state.set_cam_pose(iso3),
                                Msg::NavTarget(point3) => state.set_target_pose(point3),
                                Msg::Teleop(speed) => state.set_teleop_speed(speed),
                                Msg::SetNavigationMode(mode) => state.set_navigation_mode(mode),
                                Msg::TrackingState(tracking_state) => {
                                    state.set_tracker_state(tracking_state)
                                }
                                // recv(recv_slam_scale) -> msg => if let Ok(msg) = msg { state.set_slam_scale(msg) },
                                _ => (),
                            }
                        }
                    }
                }
            });
        }

        {
            let state = Arc::clone(&state);
            let freq = 50.0;
            let tick_time = tokio::time::Duration::from_secs_f64(1.0 / freq);
            tokio::spawn(async move {
                loop {
                    let speed = state.read().await.compute_speed();
                    wheels
                        .speed_sender
                        .send(speed)
                        .await
                        .expect("Failed to set speed on wheel driver");
                    tokio::time::sleep(tick_time).await;
                }
            });
        }

        Navigator {}
    }
}
