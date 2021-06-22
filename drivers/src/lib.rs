
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{thread, boxed::Box};
use crossbeam_channel::{bounded, Sender};

const RIGHT_BACKWARD: Channel = Channel::C12;
const RIGHT_FORWARD: Channel = Channel::C13;
const LEFT_BACKWARD: Channel = Channel::C14;
const LEFT_FORWARD: Channel = Channel::C15;

trait SetSpeed {
    fn set(&mut self, left: f64, right: f64) -> ();
}

struct SetSpeedPwm {
    pwm: Pca9685<linux_embedded_hal::I2cdev>,
}

impl SetSpeed for SetSpeedPwm {
    fn set(&mut self, left: f64, right: f64) {
        let left_channel = if left > 0.0 { LEFT_FORWARD } else { LEFT_BACKWARD };
        let right_channel = if right > 0.0 { RIGHT_FORWARD } else { RIGHT_BACKWARD };
        self.pwm.set_channel_on_off(left_channel, 0, (4095.0 * left.abs()) as u16).unwrap();
        self.pwm.set_channel_on_off(right_channel, 0, (4095.0 * right.abs()) as u16).unwrap();
    }
}

struct SetSpeedNoop {}

impl SetSpeed for SetSpeedNoop {
    fn set(&mut self, left: f64, right: f64) {
        println!("[speed noop] left: {} right: {}", left, right);
    }
}

#[derive(Debug)]
pub struct Wheels {
    pub speed_sender: Sender<(f64, f64)>,
}

impl Wheels {
    pub fn new() -> Self {
        let mut set_speed: Box<dyn SetSpeed + Send> = if let Ok(dev) = I2cdev::new("/dev/i2c-0") {
            let address = Address::default();
            let mut pwm = Pca9685::new(dev, address).unwrap();
    
            pwm.enable().unwrap();
    
            pwm.set_prescale(100).unwrap();
    
            Box::new(SetSpeedPwm { pwm })
        } else {
            Box::new(SetSpeedNoop {})
        };

        let (speed_sender, speed_receiver) = bounded(0);

        thread::spawn(move || {
            for (left, right) in speed_receiver.iter() {
                set_speed.set(left, right);
            }
        });

        Wheels { speed_sender }
    }
}
