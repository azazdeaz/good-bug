use common::msg::Broadcaster;
use openvslam_wrap::OpenVSlamWrapper;
use grpc_server;
use tokio;

mod navigator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broadcaster = Broadcaster::new();
    let _slam = OpenVSlamWrapper::new(&broadcaster, tokio::runtime::Handle::current())?;
    let _nav = navigator::Navigator::new(&broadcaster);
    grpc_server::start_server(broadcaster).await?;
    Ok(())
}