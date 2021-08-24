use common::msg::{Broadcaster, Msg};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let broadcaster = Broadcaster::new();
    drivers::SystemInfo::run(&broadcaster);
    
    let mut sub = broadcaster.subscribe();
    let handle = tokio::spawn(async move {
        while let Ok(msg) = sub.recv().await {
            if let Msg::SystemStatus(status) = msg {
                println!("{:?}", status);
            }
        }
    });
    handle.await.ok();
    Ok(())
}