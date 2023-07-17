use super::Device;
use crate::server::SettingsRequest;
use crate::utils;

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
        let target_tdp = tdp as i32 * 1000;
        let boost_tdp = target_tdp + 2000;

        let command = ["ryzenadj", &format!("--stapm-limit={}", target_tdp), &format!("--fast-limit={}", boost_tdp), &format!("--slow-limit={}", target_tdp)];
        match utils::run_command(&command) {
            Ok(_) => println!("Set TDP successfully!"),
            Err(_) => println!("Couldn't set TDP")
        }
    }
}