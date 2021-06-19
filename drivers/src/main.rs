
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{thread, time::Duration};

const RIGHT_BACKWARD: Channel = Channel::C12;
const RIGHT_FORWARD: Channel = Channel::C13;
const LEFT_BACKWARD: Channel = Channel::C14;
const LEFT_FORWARD: Channel = Channel::C15;

fn main() {
    // enable();
    let dev = I2cdev::new("/dev/i2c-0").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    pwm.enable().unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).unwrap();

    pwm.set_channel_on_off(LEFT_BACKWARD, 0, 4095).unwrap();
    pwm.set_channel_on_off(RIGHT_BACKWARD, 0, 4095).unwrap();
    thread::sleep(Duration::from_millis(1150));
    pwm.set_channel_on_off(LEFT_BACKWARD, 0, 0).unwrap();
    pwm.set_channel_on_off(RIGHT_BACKWARD, 0, 0).unwrap();
}
