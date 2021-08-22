
use std::error::Error;
use std::thread;

use common::msg::Broadcaster;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use std::{boxed::Box};
use tokio::{self, sync::mpsc, time};


#[derive(Debug)]
pub struct SystemInfo {
}

impl SystemInfo {
    pub fn run(broadcaster: &Broadcaster) {
        let sys = System::new();
        
        tokio::spawn(async move {
            let mut sys = System::new_all();
            loop {
                sys.refresh_all();
                println!("total memory: {} KB", sys.total_memory());
                println!("used memory : {} KB", sys.used_memory());
                println!("total swap  : {} KB", sys.total_swap());
                println!("used swap   : {} KB", sys.used_swap());
                time::sleep(time::Duration::from_millis(500));
            }
            
        });
    }
}
