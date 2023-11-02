use super::Device;
use crate::devices::device_generic::DeviceGeneric;
use crate::devices::Patch;
use crate::server::SettingsRequest;
use std::fs;
use std::thread;
use std::time::Duration;

pub struct DeviceAllyTDPOnly {
    device: DeviceGeneric,
}

impl DeviceAllyTDPOnly {
    pub fn new() -> Self {
        Self {
            device: DeviceGeneric::new(30),
        }
    }
}

impl Device for DeviceAllyTDPOnly {
    fn update_settings(&self, request: SettingsRequest) {
        if let Some(per_app) = &request.per_app {
            // TDP changes
            if let Some(tdp) = per_app.tdp_limit {
                self.set_tdp(tdp);
            }
        }
    }

    fn get_patches(&self) -> Vec<Patch> {
        self.device.get_patches()
    }

    fn set_tdp(&self, tdp: i8) {
        // Update thermal policy
        let thermal_policy = match tdp {
            val if val < 12 => 2,                 // silent
            val if (12..=25).contains(&val) => 0, // performance
            _ => 1,                               // turbo
        };

        println!("New Policy: {}", thermal_policy);

        let file_path = "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy";
        let _ = thread::spawn(move || match fs::read_to_string(file_path) {
            Ok(content) if content.trim() != thermal_policy.to_string() => {
                thread::sleep(Duration::from_millis(50));
                fs::write(file_path, thermal_policy.to_string())
                    .expect("Couldn't change thermal policy")
            }
            _ => {}
        });

        self.device.set_tdp(tdp);
    }

    fn get_key_mapper(&self) -> Option<tokio::task::JoinHandle<()>> {
        self.device.get_key_mapper()
    }
}
