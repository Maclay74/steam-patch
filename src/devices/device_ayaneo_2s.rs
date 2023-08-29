use std::collections::VecDeque;
use super::Device;
use crate::devices::device_generic::DeviceGeneric;
use crate::devices::Patch;
use crate::patch::PatchFile;
use crate::server::SettingsRequest;
use crate::steam::SteamClient;


pub struct DeviceAyaneo2s {
    device: DeviceGeneric,
}

impl DeviceAyaneo2s {
    pub fn new() -> Self {
        DeviceAyaneo2s {
            device: DeviceGeneric::new(30),
        }
    }
}

impl Device for DeviceAyaneo2s {
    fn update_settings(&self, request: SettingsRequest) {
        if let Some(per_app) = &request.per_app {
            // TDP changes
            if let Some(tdp) = per_app.tdp_limit {
                self.set_tdp(tdp);
            }
        }
    }

    fn get_patches(&self) -> Vec<Patch> {
        let mut patches = self.device.get_patches();
        patches.push(Patch {
            text_to_find: String::from("this.m_rgControllers=new Map,\"undefined\"!=typeof SteamClient&&(this.m_hUnregisterControllerDigitalInput"),
            replacement_text: String::from("this.m_rgControllers=new Map; window.HandleSystemKeyEvents = this.HandleSystemKeyEvents; \"undefined\"!=typeof SteamClient&&(this.m_hUnregisterControllerDigitalInput"),
            destination: PatchFile::Library,
        });
        patches
    }

    fn set_tdp(&self, tdp: i8) {
        self.device.set_tdp(tdp);
    }

    fn get_key_mapper(&self) -> Option<tokio::task::JoinHandle<()>> {
        start_mapper()
    }
}

pub fn pick_device() -> Option<evdev::Device> {
    let target_vendor_id = 0x0001u16;
    let target_product_id = 0x0001u16;

    let devices = evdev::enumerate();
    for (_, device) in devices {
        let input_id = device.input_id();

        if input_id.vendor() == target_vendor_id && input_id.product() == target_product_id {
            return Some(device);
        }
    }

    None
}

pub fn check_combo_key(queue: &mut VecDeque<i32>, new_val: i32, check_vals_1: &[i32], check_vals_2: &[i32]) -> i32 {
    if queue.len() == 12 {
        queue.pop_front();
    }

    queue.push_back(new_val);

    // QAM button pressed
    if check_vals_1.iter().all(|&x| queue.contains(&x)) {
        queue.clear();
        1
    // Main menu button pressed
    } else if check_vals_2.iter().all(|&x| queue.contains(&x)) {
        queue.clear();
        2
    } else {
        0
    }

}


pub fn start_mapper() -> Option<tokio::task::JoinHandle<()>> {
    let device = pick_device();

    match device {
        Some(device) => Some(tokio::spawn(async {
            if let Ok(mut events) = device.into_event_stream() {
                let mut steam = SteamClient::new();
                steam.connect().await;

                let mut queue = VecDeque::new();
                let menu_combo = vec![97, 125, 187];
                let qam_combo = vec![32, 125];

                loop {
                    if let Ok(event) = events.next_event().await {
                        if let evdev::InputEventKind::Key(key) = event.kind() {

                            let result = check_combo_key(&mut queue, key.code() as i32, &menu_combo, &qam_combo);
                            match result {
                                1 => { steam.execute("window.HandleSystemKeyEvents({eKey: 0})").await; }
                                2 => { steam.execute("window.HandleSystemKeyEvents({eKey: 1})").await; }
                                _ => {}
                            }
                            // // QAM button pressed
                            // if key == evdev::Key::KEY_PROG1 && event.value() == 0 {
                            //     println!("Show QAM");
                            //     steam
                            //         .execute("window.HandleSystemKeyEvents({eKey: 1})")
                            //         .await;
                            // }
                            //
                            // // Main menu button pressed
                            // if key == evdev::Key::KEY_F16 && event.value() == 0 {
                            //     println!("Show Menu");
                            //     steam
                            //         .execute("window.HandleSystemKeyEvents({eKey: 0})")
                            //         .await;
                            // }
                        }
                    }
                }
            }
        })),
        None => {
            println!("No Ayaneo2S-specific found");
            None
        }
    }
}
