pub mod device_ally;
pub mod device_generic;

use crate::{patch::Patch, server::SettingsRequest};
use device_ally::DeviceAlly;
use device_generic::DeviceGeneric;
use regex::Regex;
use std::fs;

pub trait Device {
    fn update_settings(&self, request: SettingsRequest);
    fn set_thermalpolicy(&self, thermal_policy: i32);
    fn set_tdp(&self, tdp: i8);
    fn set_gpu(&self, gpu: i16);
    fn get_patches(&self) -> Vec<Patch>;
    fn get_key_mapper(&self) -> Option<tokio::task::JoinHandle<()>>;
}

pub fn create_device() -> Option<Box<dyn Device>> {
    match get_device_name() {
        Some(device_name) => {
            match device_name.trim() {
                // Asus Rog Ally
                "AMD Ryzen Z1 Extreme ASUSTeK COMPUTER INC. RC71L" => {
                    Some(Box::new(DeviceAlly::new()))
                }

                // Ayaneo 2
                "AMD Ryzen 7 6800U with Radeon Graphics AYANEO AYANEO 2" => {
                    Some(Box::new(DeviceGeneric::new(28,800, 1500)))
                }

                // Ayaneo Geek
                "AMD Ryzen 7 6800U with Radeon Graphics AYANEO GEEK" => {
                    Some(Box::new(DeviceGeneric::new(28, 800,1500)))
                }

                // Ayaneo 2S
                "AMD Ryzen 7 7840U w/ Radeon 780M Graphics AYANEO AYANEO 2S" => {
                    Some(Box::new(DeviceGeneric::new(30,800, 1500)))
                }

                // Ayaneo Geek 1S
                "AMD Ryzen 7 7840U w/ Radeon 780M Graphics AYANEO GEEK 1S" => {
                    Some(Box::new(DeviceGeneric::new(30,800, 1500)))
                }

                // GPD WM2
                "AMD Ryzen 7 6800U with Radeon Graphics GPD G1619-04" => {
                    Some(Box::new(DeviceGeneric::new(28,800, 1500)))
                }

                // AOKZOE A1
                "AMD Ryzen 7 6800U with Radeon Graphics AOKZOE AOKZOE A1 AR07" => {
                    Some(Box::new(DeviceGeneric::new(28,800, 1500)))
                }

                // Any other device
                _ => Some(Box::new(DeviceGeneric::new(25,800, 1500))),
            }
        }
        None => None,
    }
}

fn get_device_name() -> Option<String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").expect("Unknown");

    let model_re = Regex::new(r"model name\s*:\s*(.*)").unwrap();
    let model = model_re.captures_iter(&cpuinfo).next().unwrap()[1]
        .trim()
        .to_string();

    let board_vendor = match fs::read_to_string("/sys/devices/virtual/dmi/id/board_vendor") {
        Ok(str) => str.trim().to_string(),
        Err(_) => return None,
    };

    let board_name = match fs::read_to_string("/sys/devices/virtual/dmi/id/board_name") {
        Ok(str) => str.trim().to_string(),
        Err(_) => return None,
    };

    Some(format!("{} {} {}", model, board_vendor, board_name))
}
