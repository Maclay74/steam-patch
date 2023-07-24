use super::Device;
use crate::devices::device_generic::DeviceGeneric;
use crate::devices::Patch;
use crate::patch::PatchFile;
use crate::server::SettingsRequest;
//use crate::steam::{execute, get_context};
use std::fs;
use std::thread;
use std::time::Duration;

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
            text_to_find: String::from("this.m_rgControllers=new Map,\"undefined\"!=typeof SteamClient&&(this.m_hUnregisterControllerDigitalInput"),
            replacement_text: String::from("this.m_rgControllers=new Map; window.HandleSystemKeyEvents = this.HandleSystemKeyEvents; \"undefined\"!=typeof SteamClient&&(this.m_hUnregisterControllerDigitalInput"),
            destination: PatchFile::Library,
        });
        patches
    }

    fn set_tdp(&self, tdp: i8) {
        self.device.set_tdp(tdp);

        // Update thermal policy
        let thermal_policy = match tdp {
            val if val < 12 => 0,                 // silent
            val if (12..=25).contains(&val) => 1, // performance
            _ => 2,                               // turbo
        };

        let file_path = "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy";
        let _ = thread::spawn(move || match fs::read_to_string(file_path) {
            Ok(content) if content.trim() != thermal_policy.to_string() => {
                thread::sleep(Duration::from_millis(50));
                fs::write(file_path, thermal_policy.to_string())
                    .expect("Couldn't change thermal policy")
            }
            _ => {}
        });
    }

    fn get_key_mapper(&self) -> Option<tokio::task::JoinHandle<()>> {
        start_mapper()
    }
}

pub fn pick_device() -> Option<evdev::Device> {
    let target_vendor_id = 0xb05u16;
    let target_product_id = 0x1abeu16;

    let devices = evdev::enumerate();
    for (_, device) in devices {
        let input_id = device.input_id();

        if input_id.vendor() == target_vendor_id && input_id.product() == target_product_id {
            return Some(device);
        }
    }

    None
}

pub fn start_mapper() -> Option<tokio::task::JoinHandle<()>> {
    let device = pick_device();

    match device {
        Some(mut device) => {
            println!(
                "Ally-specific device found: {}",
                device.name().unwrap_or("Unnamed device")
            );

            Some(thread::spawn(move || {
                let context = Arc::new(get_context().unwrap());

                let context_clone = Arc::clone(&context);
                println!("Starting loop for keys");

                loop {
                    for event in device.fetch_events().unwrap() {
                        if let evdev::InputEventKind::Key(key) = event.kind() {
                            if key == evdev::Key::KEY_PROG1 && event.value() == 0 {
                                println!("Show QAM");
                                let context_str = context_clone.deref().clone();
                                execute(
                                    context_str,
                                    String::from("window.HandleSystemKeyEvents({eKey: 1})"),
                                );
                            }

                            if key == evdev::Key::KEY_F16 && event.value() == 0 {
                                println!("Show Menu");
                                let context_str = context_clone.deref().clone();
                                execute(
                                    context_str,
                                    String::from("window.HandleSystemKeyEvents({eKey: 0})"),
                                );
                            }
                        }
                    }
                }
            }))
        }
        None => {
            println!("No Ally-specific found");
            None
        }
    }
}
