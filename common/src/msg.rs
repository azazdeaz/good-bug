use std::collections::HashMap;

use crate::types::{BoxDetection, Edge, Iso3, Keyframe, Landmark, LocalizedDetection, NavGoal, NavigationMode, NavigatorState, Point3, RobotParams, SlamFrame, SystemStatus, TrackingState};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Msg {
    //OpenVSlam output
    TrackingState(TrackingState),
    Edges(Vec<Edge>),
    Keyframes(Vec<Keyframe>),
    Landmarks(Vec<Landmark>),
    CameraPose(Iso3),
    Frame(SlamFrame),

    Detections(Vec<BoxDetection>),
    LocalizedDetections(Vec<LocalizedDetection>),
    LandmarkClassification(HashMap<u32, HashMap<u32, u32>>),
    MapScale(f64),
    NavigatorState(NavigatorState),
    SystemStatus(SystemStatus),

    //OpenVSlam input
    TerminateSlam,
    SaveMapDB(String),
    SelectMap(Option<String>),
    UseRawPreview(bool),

    // Mirrors
    Teleop((f64, f64)),
    SetWeederSpeed(f64),
    SetNavigationMode(NavigationMode),
    NavTarget(NavGoal),
    Waypoints(Vec<NavGoal>),
    EnableAutoNav(bool),
    RequestRobotParams,
    RobotParams(RobotParams),

    RequestToRobot,
}

pub enum Req {
    SetWaypoints(Vec<Point3>),
    GetSettings(),
}

impl Msg {
    pub fn is_mirrors_command(&self) -> bool {
        matches!(
            self,
            Msg::Teleop(_)
                | Msg::SetWeederSpeed(_)
                | Msg::SetNavigationMode(_)
                | Msg::SaveMapDB(_)
                | Msg::NavTarget(_)
                | Msg::SelectMap(_)
                | Msg::Waypoints(_)
                | Msg::EnableAutoNav(_)
                | Msg::TerminateSlam
                | Msg::UseRawPreview(_)
                | Msg::RequestRobotParams
        )
    }
}

#[derive(Debug)]
pub struct Broadcaster {
    sender: broadcast::Sender<Msg>,
}

impl Broadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(12);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Msg> {
        self.sender.subscribe()
    }

    pub fn stream(&self) -> BroadcastStream<Msg> {
        BroadcastStream::new(self.subscribe())
    }

    pub fn publisher(&self) -> broadcast::Sender<Msg> {
        self.sender.clone()
    }

    pub fn publish_serialized(&self, json: String) {
        let data = serde_json::from_str(&json).unwrap();
        self.sender.send(data).ok();
    }
}
