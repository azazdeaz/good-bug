
use common::{msg::{Msg, Broadcaster}, types::SlamFrame};
use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::AllowLeadingHyphen)]
struct Opts {
    #[clap(about("Test image path"))]
    image: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    let img = std::fs::read(opts.image).unwrap();
    let broadcaster = Broadcaster::new();
    // test_mnist(&FlatBufferModel::build_from_buffer(buf).unwrap());
    detector::Detector::run(&broadcaster);
    for _ in 0..10 {
        broadcaster.publisher().send(Msg::Frame(SlamFrame {
            jpeg: img.clone(),
            features: Vec::new(),
        })).ok();
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
    
    Ok(())
}

