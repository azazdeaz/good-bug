
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{boxed::Box};
use tokio::{self, sync::mpsc};

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
        let left = if left.abs() < 0.01 { 0.0 } else { left };
        let right = if right.abs() < 0.01 { 0.0 } else { right };
        let (left_channel, left_stop) = if left > 0.0 { (LEFT_FORWARD, LEFT_BACKWARD) } else { (LEFT_BACKWARD, LEFT_FORWARD) };
        let (right_channel, right_stop) = if right > 0.0 { (RIGHT_FORWARD, RIGHT_BACKWARD) } else { (RIGHT_BACKWARD, RIGHT_FORWARD) };
        self.pwm.set_channel_full_off(left_stop).unwrap();
        self.pwm.set_channel_full_off(right_stop).unwrap();
        if left  == 0.0 {
            // println!("left off");
            self.pwm.set_channel_full_off(left_channel).unwrap();
        }
        else {
            self.pwm.set_channel_on_off(left_channel, 0, (4095.0 * left.abs()) as u16).unwrap();
        }
        if right  == 0.0 {
            // println!("right off");
            self.pwm.set_channel_full_off(right_channel).unwrap();
        }
        else {
            self.pwm.set_channel_on_off(right_channel, 0, (4095.0 * right.abs()) as u16).unwrap();
        }
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
    pub speed_sender: mpsc::Sender<(f64, f64)>,
}

impl Wheels {
    pub async fn new() -> tokio::io::Result<Self> {
        let mut set_speed: Box<dyn SetSpeed + Send> = if let Ok(dev) = I2cdev::new("/dev/i2c-0") {
            let address = Address::default();
            let mut pwm = Pca9685::new(dev, address).unwrap();
    
            pwm.enable().unwrap();
    
            pwm.set_prescale(100).unwrap();
    
            Box::new(SetSpeedPwm { pwm })
        } else {
            Box::new(SetSpeedNoop {})
        };

        let (speed_sender, mut speed_receiver) = mpsc::channel(1);

        tokio::spawn(async move {
            while let Some((left, right)) = speed_receiver.recv().await {
                set_speed.set(left, right);
            }
        });

        Ok(Wheels { speed_sender })
    }
}
