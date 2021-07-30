use common::msg::Broadcaster;
use openvslam_wrap::OpenVSlamWrapper;
use grpc_server;
use tokio;

mod navigator;
mod scale_estimator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broadcaster = Broadcaster::new();
    OpenVSlamWrapper::new(&broadcaster, tokio::runtime::Handle::current())?;
    navigator::Navigator::new(&broadcaster);
    scale_estimator::ScaleEstimator::new(&broadcaster);
    detector::Detector::run(&broadcaster);
    grpc_server::start_server(broadcaster).await?;
    Ok(())
}