use common::msg::Broadcaster;
use grpc_server;
use openvslam_wrap::OpenVSlamWrapper;
use tokio;

mod navigator;
mod scale_estimator;
mod robot_params_echo;
mod map_updater;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broadcaster = Broadcaster::new();
    OpenVSlamWrapper::run_with_auto_restart(&broadcaster, tokio::runtime::Handle::current())?;
    navigator::Navigator::new(&broadcaster);
    drivers::Weeder::run(&broadcaster);
    drivers::SystemInfo::run(&broadcaster);
    scale_estimator::ScaleEstimator::new(&broadcaster);
    detector::Detector::run(&broadcaster);
    robot_params_echo::RobotParamsEcho::run(&broadcaster);
    map_updater::MapUpdater::run(&broadcaster);
    grpc_server::start_server(broadcaster).await?;
    Ok(())
}
