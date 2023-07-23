use super::Device;
use crate::server::SettingsRequest;
use std::fs;
use std::thread;
use std::time::{Duration};
use crate::devices::device_generic::DeviceGeneric;
use crate::devices::{Patch, PatchFile};

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
        let mut patches = self.device.get_patches();
        patches.push(Patch {
            text_to_find: String::from("return a.EGamepadButton.DIR_RIGHT}return a.EGamepadButton.INVALID"),
            replacement_text: String::from("return a.EGamepadButton.DIR_RIGHT; default: if (e.keyCode === 0) return a.EGamepadButton.STEAM_QUICK_MENU; if (e.keyCode === 127) return a.EGamepadButton.STEAM_GUIDE;  }return a.EGamepadButton.INVALID"),
            destination: PatchFile::Library,
        });
        patches
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