
use common::msg::{Msg, Broadcaster};

const TEST_IMG: &str = "/home/azazdeaz/Downloads/dogs.jpeg";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let img = std::fs::read(TEST_IMG).unwrap();
    let broadcaster = Broadcaster::new();
    // test_mnist(&FlatBufferModel::build_from_buffer(buf).unwrap());
    detector::Detector::run(&broadcaster);
    for _ in 0..10 {
        broadcaster.publisher().send(Msg::Frame(img.clone())).ok();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    Ok(())
}

