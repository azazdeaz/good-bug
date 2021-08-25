
use common::{msg::{Broadcaster, Msg}, types::SystemStatus};
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};
use tokio::{self,  time};
use linux_embedded_hal::I2cdev;
use ina219::{INA219};


#[derive(Debug)]
pub struct SystemInfo {
}

fn create_ina() -> anyhow::Result<INA219<I2cdev>> {
    let device = I2cdev::new("/dev/i2c-1")?;
    let mut ina = INA219::new(device, 0x42);
    ina.calibrate(0x0100)?;
    Ok(ina)
}

impl SystemInfo {
    pub fn run(broadcaster: &Broadcaster) {
        let publisher = broadcaster.publisher();
        
        tokio::spawn(async move {
            // TODO load only the necessary stuff
            let mut sys = System::new_all();
            let mut ina = create_ina();
            loop {
                sys.refresh_system();

                let mut cpu_temperature = 0.0;
                for component in sys.components() {
                    if component.label() == "CPU" {
                        cpu_temperature = component.temperature();
                        break;
                    }
                }

                
                let battery = if ina.is_ok() {
                    // TODO clean this up and extract the current usage
                    let voltage = ina.as_mut().unwrap().voltage().unwrap();
                    ((voltage as f32/1000.0) -6.0)/2.4
                }
                else {
                    0.0
                };

                publisher.send(Msg::SystemStatus(SystemStatus {
                    total_memory: sys.total_memory(),
                    used_memory: sys.used_memory(),
                    total_swap: sys.total_swap(),
                    used_swap: sys.used_swap(),
                    cpu_temperature,
                    cpu_usage: sys.global_processor_info().cpu_usage(),
                    battery,
                })).ok();
                
                time::sleep(time::Duration::from_millis(500)).await;
            }
            
        });
    }
}
