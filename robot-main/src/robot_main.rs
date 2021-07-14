use common::msg::Broadcaster;
use openvslam_wrap::OpenVSlamWrapper;
use grpc_server;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broadcaster = Broadcaster::new();
    let _slam = OpenVSlamWrapper::new(&broadcaster, tokio::runtime::Handle::current())?;
    grpc_server::start_server(broadcaster).await?;
    Ok(())
}