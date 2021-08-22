
use std::error::Error;
use common::msg::{Broadcaster, Msg};
use rppal::gpio::{Gpio, OutputPin};


use std::{boxed::Box};
use tokio::{self, sync::mpsc};
use tokio_stream::StreamExt;
trait SetSpeed {
    fn set(&mut self, speed: f64) -> ();
}




struct SetSpeedSoftPwm {
    in1: OutputPin,
    freq: f64,
}

impl SetSpeedSoftPwm {
    fn new() -> Result<Self, Box<dyn Error>> {
        // TODO move these to settings
        let in1 = 4;

        Ok(Self {
            in1: Gpio::new()?.get(in1)?.into_output(),
            freq: 30.0
        })
    }
}

impl SetSpeed for SetSpeedSoftPwm {
    fn set(&mut self, speed: f64) {
        self.in1.set_pwm_frequency(self.freq.clone(), speed).ok();
    }
}

struct SetSpeedNoop {}

impl SetSpeed for SetSpeedNoop {
    fn set(&mut self, _speed: f64) {
        println!("[weeder noop] speed: {}", _speed);
    }
}

#[derive(Debug)]
pub struct Weeder {
    pub speed_sender: mpsc::Sender<f64>,
}

impl Weeder {
    pub fn run(broadcaster: &Broadcaster) {
        let mut set_speed: Box<dyn SetSpeed + Send> = if let Ok(set_speed) = SetSpeedSoftPwm::new() {
            Box::new(set_speed)
        } else {
            Box::new(SetSpeedNoop {})
        };

        let mut updates = broadcaster.stream();

        tokio::spawn(async move {
            loop {
                while let Some(msg) = updates.next().await {
                    if let Ok(msg) = msg {
                        match msg {
                            Msg::SetWeederSpeed(speed) => {
                                set_speed.set(speed);
                            }
                            _ => (),
                        }
                    }
                }
            }
        });

        // let (speed_sender, mut speed_receiver) = mpsc::channel(1);

        // tokio::spawn(async move {
        //     while let Some(speed) = speed_receiver.recv().await {
        //         set_speed.set(speed);
        //     }
        // });

        // Self { speed_sender }
    }
}
