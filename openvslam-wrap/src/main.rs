

use common::msg::{Broadcaster, Msg};
use openvslam_wrap::OpenVSlamWrapper;
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broadcaster = Broadcaster::new();
    let api = Arc::new(Mutex::new(OpenVSlamWrapper::new(&broadcaster, tokio::runtime::Handle::current())?));
    println!("api started");

    {
        let publisher = broadcaster.publisher();
        let _api = api.clone();
        let _ = tokio::spawn(async move {
            println!("terminate sleep...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            println!("terminate awake");
            publisher.send(Msg::TerminateSlam).unwrap();
            println!("terminate came back");
        });
    }
    
    let _ = tokio::spawn(async move {
        let mut stream = broadcaster.stream();
        while let Some(x) = stream.next().await {
            if let Ok(Msg::CameraPose(pose)) = x {
                println!("\n\nstreamed position {:?}", pose);
            }     
        }
    }).await;
    Ok(())
}
