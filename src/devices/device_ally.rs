use super::Device;
use crate::server::SettingsRequest;
use std::fs;
use std::thread;
use std::time::{Duration};
use crate::devices::device_generic::DeviceGeneric;
use crate::devices::Patch;

pub struct DeviceAlly {
    device: DeviceGeneric,
}

impl DeviceAlly {
    pub fn new() -> Self {
        DeviceAlly {
            device: DeviceGeneric::new(30),
        }
    }
}

impl Device for DeviceAlly {
    fn update_settings(&self, request: SettingsRequest) {
        self.device.update_settings(request);
    }

    fn get_patches(&self) -> Vec<Patch> {
        self.device.get_patches()
    }

    fn set_tdp(&self, tdp: i8) -> () {
        self.device.set_tdp(tdp);

        // Update thermal policy
        let thermal_policy = match tdp {
            val if val < 12 => 0, // silent
            val if val >= 12 && val <= 25 => 1, // performance
            _ => 2, // turbo
        };

        let file_path = "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy";
        let _ = thread::spawn(move || {
            match fs::read_to_string(file_path) {
                Ok(content) if content.trim() != thermal_policy.to_string() => {
                    thread::sleep(Duration::from_millis(50));
                    fs::write(file_path, thermal_policy.to_string()).expect("Couldn't change thermal policy")
                }
                _ => {}
            }
        });
    }
}