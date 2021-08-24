
use std::error::Error;
use std::thread;

use common::{msg::{Broadcaster, Msg}, types::SystemStatus};
use sysinfo::{ComponentExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use std::{boxed::Box};
use tokio::{self, sync::mpsc, time};


#[derive(Debug)]
pub struct SystemInfo {
}

impl SystemInfo {
    pub fn run(broadcaster: &Broadcaster) {
        let sys = System::new();

        let publisher = broadcaster.publisher();
        
        tokio::spawn(async move {
            let mut sys = System::new_all();
            loop {
                sys.refresh_system();

                let mut cpu_temperature = 0.0;
                for component in sys.components() {
                    if component.label() == "CPU" {
                        cpu_temperature = component.temperature();
                        break;
                    }
                }

                publisher.send(Msg::SystemStatus(SystemStatus {
                    total_memory: sys.total_memory(),
                    used_memory: sys.used_memory(),
                    total_swap: sys.total_swap(),
                    used_swap: sys.used_swap(),
                    cpu_temperature,
                })).ok();
                
                time::sleep(time::Duration::from_millis(500)).await;
            }
            
        });
    }
}
