
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};


use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{boxed::Box};
use tokio::{self, sync::mpsc};

const LEFT_BACKWARD: Channel = Channel::C12;
const LEFT_FORWARD: Channel = Channel::C13;
const RIGHT_BACKWARD: Channel = Channel::C14;
const RIGHT_FORWARD: Channel = Channel::C15;

trait SetSpeed {
    fn set(&mut self, left: f64, right: f64) -> ();
}

struct SetSpeedPwm {
    pwm: Pca9685<linux_embedded_hal::I2cdev>,
}

impl SetSpeedPwm {
    pub fn new() -> Self {
        let dev = I2cdev::new("/dev/i2c-0").expect("Couldn't access i2c device. Is this running on an IoT device (RPi/Jetson)?");
        let address = Address::default();
        let mut pwm = Pca9685::new(dev, address).unwrap();

        pwm.enable().unwrap();

        pwm.set_prescale(100).unwrap();

        Self { pwm }
    }
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

struct SetSpeedSoftPwm {
    in1: OutputPin,
    in2: OutputPin,
    in3: OutputPin,
    in4: OutputPin,
    freq: f64,
}

impl SetSpeedSoftPwm {
    fn new() -> Result<Self, Box<dyn Error>> {
        // TODO move these to settings
        let in1 = 19;
        let in2 = 26;
        let in3 = 12;
        let in4 = 13;

        Ok(Self {
            in1: Gpio::new()?.get(in1)?.into_output(),
            in2: Gpio::new()?.get(in2)?.into_output(),
            in3: Gpio::new()?.get(in3)?.into_output(),
            in4: Gpio::new()?.get(in4)?.into_output(),
            freq: 30.0
        })
    }
}

impl SetSpeed for SetSpeedSoftPwm {
    fn set(&mut self, left: f64, right: f64) {
        let left = if left.abs() < 0.01 { 0.0 } else { left };
        let right = if right.abs() < 0.01 { 0.0 } else { right };
        let mut in1;
        let mut in2;
        let mut in3;
        let mut in4;
        if left > 0.0 { in1 = left; in2 = 0.0 } else { in2 = left.abs(); in1 = 0.0 };
        if right > 0.0 { in3 = right; in4 = 0.0 } else { in4 = right.abs(); in3 = 0.0 };
        if in1 == 0.0 { self.in1.clear_pwm().ok(); } else { self.in1.set_pwm_frequency(self.freq.clone(), in1).ok();}
        if in2 == 0.0 { self.in2.clear_pwm().ok(); } else { self.in2.set_pwm_frequency(self.freq.clone(), in2).ok();}
        if in3 == 0.0 { self.in3.clear_pwm().ok(); } else { self.in3.set_pwm_frequency(self.freq.clone(), in3).ok();}
        if in4 == 0.0 { self.in4.clear_pwm().ok(); } else { self.in4.set_pwm_frequency(self.freq.clone(), in4).ok();}
    }
}

struct SetSpeedNoop {}

impl SetSpeed for SetSpeedNoop {
    fn set(&mut self, _left: f64, _right: f64) {
        // println!("[speed noop] left: {} right: {}", _left, _right);
    }
}

#[derive(Debug)]
pub struct Wheels {
    pub speed_sender: mpsc::Sender<(f64, f64)>,
}

impl Wheels {
    pub fn new() -> Self {
        let mut set_speed: Box<dyn SetSpeed + Send> = if let Ok(set_speed) = SetSpeedSoftPwm::new() {
            Box::new(set_speed)
        } else {
            Box::new(SetSpeedNoop {})
        };

        let (speed_sender, mut speed_receiver) = mpsc::channel(1);

        tokio::spawn(async move {
            while let Some((left, right)) = speed_receiver.recv().await {
                set_speed.set(left, right);
            }
        });

        Wheels { speed_sender }
    }
}
