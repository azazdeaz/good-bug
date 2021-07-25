use std::{thread, time::Duration};
use clap::{AppSettings, Clap};

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(about("left side speed (between -1.0 an 1.0)"), allow_hyphen_values(true))]
    left: f64,
    #[clap(about("right side speed (between -1.0 an 1.0)"), allow_hyphen_values(true))]
    right: f64,
    #[clap(about("duration in ms"))]
    duration: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    let wheels = drivers::Wheels::new();
    wheels.speed_sender.send((opts.left, opts.right)).await?;
    thread::sleep(Duration::from_millis(opts.duration));
    wheels.speed_sender.send((0.0, 0.0)).await?;
    // HACK wait until the board gets the message
    thread::sleep(Duration::from_millis(100));
    Ok(())
}