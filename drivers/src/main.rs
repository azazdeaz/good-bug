use std::{thread, time::Duration};

fn main() {
    let wheels = drivers::Wheels::new();
    wheels.speed_sender.send((1.0, 1.0)).unwrap();
    thread::sleep(Duration::from_secs(1));
    wheels.speed_sender.send((0.0, 0.0)).unwrap();
}
