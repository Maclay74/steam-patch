use super::Device;
use crate::server::SettingsRequest;
use crate::utils;
use std::fs;
use std::thread;
use std::time::{Duration};

pub struct DeviceAlly;

impl Device for DeviceAlly {
    fn update_settings(&self, request: SettingsRequest) {
        if let Some(per_app) = &request.per_app {

            // TDP changes
            if let Some(tdp) = per_app.tdp_limit {
                self.set_tdp(tdp);
            }
        }
    }

    fn set_tdp(&self, tdp: i8) -> () {

        // Update TDP
        let target_tdp = tdp as i32 * 1000;
        let boost_tdp = target_tdp + 2000;

        let command = ["ryzenadj", &format!("--stapm-limit={}", target_tdp), &format!("--fast-limit={}", boost_tdp), &format!("--slow-limit={}", target_tdp)];
        match utils::run_command(&command) {
            Ok(_) => println!("Set TDP successfully!"),
            Err(_) => println!("Couldn't set TDP")
        }

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