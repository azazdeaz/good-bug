

use openvslam_wrap::OpenVSlamWrapper;
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = Arc::new(Mutex::new(OpenVSlamWrapper::new()));
    println!("api started");

    {
        let api = api.clone();
        let _ = tokio::spawn(async move {
            println!("terminate sleep...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            println!("terminate awake");
            api.lock().await.terminate().await;
            println!("terminate came back");
        });
    }
    
    // tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
    let mut position = api.lock().await.stream_position();
    while let Some(pos) = position.next().await {
        println!("streamed position");
    }
    Ok(())
}
