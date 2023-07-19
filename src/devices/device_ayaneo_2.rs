use super::Device;
use crate::server::SettingsRequest;
use crate::utils;
use crate::devices::Patch;

pub struct DeviceAyaneo2;

impl Device for DeviceAyaneo2 {
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
    }

    fn get_patches(&self) -> Vec<Patch> {
        vec![
            // Max TDP = 28
            Patch {
                text_to_find: "return[n,t,r,e=>i((()=>p.Get().SetTDPLimit(e)))".to_string(),
                replacement_text: "return[n,t,28,e=>i((()=>p.Get().SetTDPLimit(e)))".to_string(),
            },
            // Listen to TDP changes
            Patch {
                text_to_find: "const t=c.Hm.deserializeBinary(e).toObject();Object.keys(t)".to_string(),
                replacement_text: "const t=c.Hm.deserializeBinary(e).toObject(); console.log(t); fetch(`http://localhost:1338/update_settings`, { method: 'POST',  headers: {'Content-Type': 'application/json'}, body: JSON.stringify(t.settings)}); Object.keys(t)".to_string(),
            },
            // Add more patches as needed
        ]
    }
}