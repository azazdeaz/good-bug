
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::{thread, time::Duration};

fn main() {
    enable();
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    pwm.enable().unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).unwrap();

    pwm.set_channel_on_off(Channel::C0, 0, 4095).unwrap();
    pwm.set_channel_on_off(Channel::C2, 0, 4095).unwrap();
    thread::sleep(Duration::from_secs(7));
    pwm.set_channel_on_off(Channel::C0, 0, 0).unwrap();
    pwm.set_channel_on_off(Channel::C2, 0, 0).unwrap();

    // loop {
    //     for off in (0..4096).step_by(120) {
    //         pwm.set_channel_on_off(Channel::C0, 0, off).unwrap();
    //         thread::sleep(Duration::from_millis(20));
    //         println!("{}", off);
    //     }
    // }
}


// use std::thread::sleep;

// fn main() {
//     thread::sleep(Duration::from_secs(1));
//     let mut chip = Chip::new("/dev/gpiochip0").unwrap();
//     let handle = chip
//         .get_line(50).unwrap()
//         .request(LineRequestFlags::OUTPUT, 1, "blinky").unwrap();
    
    
//     handle.set_value(1).unwrap();
//     thread::sleep(Duration::from_secs(2));
//     handle.set_value(0).unwrap();
// }


extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};

fn enable() {
    let my_led = Pin::new(50); // number depends on chip, etc.
    my_led.with_exported(|| {
        loop {
            thread::sleep(Duration::from_millis(10));
            match my_led.set_direction(Direction::Out) {
                Ok(()) => break,
                Err(error) => {
                    // TODO check for specific error
                    println!("Can't access GPIO. Try again... {:?}", error);
                }
            }
        }
        
        my_led.set_value(1).unwrap();
        Ok(())
    }).unwrap();
}