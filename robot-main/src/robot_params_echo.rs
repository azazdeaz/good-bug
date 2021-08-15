use common::{
    msg::{Broadcaster, Msg},
    settings::Settings,
    types::RobotParams,
};
use tokio_stream::StreamExt;

pub struct RobotParamsEcho {}

fn read_robot_params() -> RobotParams {
    let mut robot_params = RobotParams::default();
    let settings = Settings::new().unwrap();
    robot_params.current_map_name = settings.slam.current_map_name;
    if let Some(maps) = settings.slam.maps {
        for map in maps {
            robot_params.maps.push(map.clone());
        }
    }
    robot_params
}

impl RobotParamsEcho {
    pub fn run(broadcaster: &Broadcaster) {
        let mut updates = broadcaster.stream();
        let publisher = broadcaster.publisher();

        tokio::spawn(async move {
            loop {
                while let Some(msg) = updates.next().await {
                    if let Ok(msg) = msg {
                        match msg {
                            Msg::RequestRobotParams => {
                                let robot_params = read_robot_params();
                                publisher.send(Msg::RobotParams(robot_params)).ok();
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }
}
