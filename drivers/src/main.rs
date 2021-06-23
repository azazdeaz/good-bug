use std::{thread, time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wheels = drivers::Wheels::new().await?;
    wheels.speed_sender.send((1.0, 1.0)).await?;
    thread::sleep(Duration::from_secs(1));
    wheels.speed_sender.send((0.0, 0.0)).await?;
    Ok(())
}
