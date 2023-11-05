use super::Device;
use crate::devices::device_generic::DeviceGeneric;
use crate::devices::Patch;
use crate::patch::PatchFile;
use crate::server::SettingsRequest;
use crate::steam::SteamClient;
use std::fs;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

pub struct DeviceAlly {
    device: DeviceGeneric,
}

impl DeviceAlly {
    pub fn new() -> Self {
        DeviceAlly {
            device: DeviceGeneric::new(30, 800,2700),
        }
        
    }
}

impl Device for DeviceAlly {
    fn set_thermalpolicy(&self, thermal_policy: i32) {
        println!("Setting new thermal policy: {}", thermal_policy);
        
        let file_path = "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy";
    
        // Attempt to write the thermal policy to the file.
        match fs::write(file_path, thermal_policy.to_string()) {
            Ok(_) => {
                // Optionally, add a small delay to give the system time to apply the setting.
                thread::sleep(Duration::from_millis(50));
    
                // Read the file back to confirm.
                match fs::read_to_string(file_path) {
                    Ok(content) if content.trim() == thermal_policy.to_string() => {
                        println!("Thermal policy set successfully.");
                    },
                    _ => {
                        eprintln!("Failed to set thermal policy. Value could not be confirmed.");
                    }
                }
            },
            Err(e) => {
                // Handle the error, but don't propagate it.
                eprintln!("Failed to write thermal policy: {}", e);
            }
        };
    }

    fn update_settings(&self, request: SettingsRequest) {
        if let Some(per_app) = &request.per_app {
            println!("{:#?}",per_app);
            // TDP changes
            if let Some(true) = per_app.is_tdp_limit_enabled {
                if let Some(tdp) = per_app.tdp_limit {
                    self.set_tdp(tdp);
                }
            }  else {
                self.set_thermalpolicy(1);
            }

            if let Some(gpu) = per_app.gpu_performance_manual_mhz {
                self.set_gpu(gpu);
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
        // Update thermal policy
        let thermal_policy = match tdp {
            val if val < 12 => 2,                 // silent
            val if (12..=25).contains(&val) => 0, // performance
            _ => 1,                               // turbo
        };
        self.set_thermalpolicy(thermal_policy);
        self.device.set_tdp(tdp);
    }
    fn set_gpu(&self, gpu: i16) {
        //Placeholder for later implementations
        println!("New GPU clock: {}", gpu);
    }

    fn get_key_mapper(&self) -> Option<tokio::task::JoinHandle<()>> {
        tokio::spawn(async move {
            let mut steam = SteamClient::new();
            steam.connect().await;
            start_mapper(steam);
        });
        None
    }
}

pub fn pick_device() -> Option<evdev::Device> {
    let target_vendor_id = 0xb05u16;
    let target_product_id = 0x1abeu16;

    let devices = evdev::enumerate();
    for (_, device) in devices {
        let input_id = device.input_id();

        if input_id.vendor() == target_vendor_id && input_id.product() == target_product_id {
            if device.supported_keys().map_or(false, |keys| keys.contains(evdev::Key::KEY_PROG1)) {
                return Some(device);   
            }
        }
    }

    None
}

pub fn start_mapper(mut steam:SteamClient) -> Option<tokio::task::JoinHandle<()>> {
    let device = pick_device();
    
    match device {
        Some(device) => Some(tokio::spawn(async move {
            if let Ok(mut events) = device.into_event_stream() {
                loop {
                    match events.next_event().await {
                        Ok(event) => {
                            if let evdev::InputEventKind::Key(key) = event.kind() {
                                //Useful to get status on event, saved for future use.
                                //Another method of sending keys to steam:
                                // GamepadNavTree.m_Controller.OnButtonActionInternal(true, 28)
                                // isButtonDown, // originally 'e'
                                // gamepadButton, // originally 't'
                                // source, // originally 'n'
                                // r, // purpose unclear, original parameter 'r' - could be a timestamp or specific event data
                                // isRepeat, // originally 'o'

                                //             list of values found
                                //     e[e.INVALID = 0] = "INVALID",
                                //     e[e.OK = 1] = "OK",
                                //     e[e.CANCEL = 2] = "CANCEL",
                                //     e[e.SECONDARY = 3] = "SECONDARY",
                                //     e[e.OPTIONS = 4] = "OPTIONS",
                                //     e[e.BUMPER_LEFT = 5] = "BUMPER_LEFT",
                                //     e[e.BUMPER_RIGHT = 6] = "BUMPER_RIGHT",
                                //     e[e.TRIGGER_LEFT = 7] = "TRIGGER_LEFT",
                                //     e[e.TRIGGER_RIGHT = 8] = "TRIGGER_RIGHT",
                                //     e[e.DIR_UP = 9] = "DIR_UP",
                                //     e[e.DIR_DOWN = 10] = "DIR_DOWN",
                                //     e[e.DIR_LEFT = 11] = "DIR_LEFT",
                                //     e[e.DIR_RIGHT = 12] = "DIR_RIGHT",
                                //     e[e.SELECT = 13] = "SELECT",
                                //     e[e.START = 14] = "START",
                                //     e[e.LSTICK_CLICK = 15] = "LSTICK_CLICK",
                                //     e[e.RSTICK_CLICK = 16] = "RSTICK_CLICK",
                                //     e[e.LSTICK_TOUCH = 17] = "LSTICK_TOUCH",
                                //     e[e.RSTICK_TOUCH = 18] = "RSTICK_TOUCH",
                                //     e[e.LPAD_TOUCH = 19] = "LPAD_TOUCH",
                                //     e[e.LPAD_CLICK = 20] = "LPAD_CLICK",
                                //     e[e.RPAD_TOUCH = 21] = "RPAD_TOUCH",
                                //     e[e.RPAD_CLICK = 22] = "RPAD_CLICK",
                                //     e[e.REAR_LEFT_UPPER = 23] = "REAR_LEFT_UPPER",
                                //     e[e.REAR_LEFT_LOWER = 24] = "REAR_LEFT_LOWER",
                                //     e[e.REAR_RIGHT_UPPER = 25] = "REAR_RIGHT_UPPER",
                                //     e[e.REAR_RIGHT_LOWER = 26] = "REAR_RIGHT_LOWER",
                                //     e[e.STEAM_GUIDE = 27] = "STEAM_GUIDE",
                                //     e[e.STEAM_QUICK_MENU = 28] = "STEAM_QUICK_MENU"

                                //     e[e.UNKNOWN = 0] = "UNKNOWN",
                                //     e[e.GAMEPAD = 1] = "GAMEPAD",
                                //     e[e.KEYBOARD = 2] = "KEYBOARD",
                                //     e[e.MOUSE = 3] = "MOUSE",
                                //     e[e.TOUCH = 4] = "TOUCH",
                                //     e[e.LPAD = 5] = "LPAD",
                                //     e[e.RPAD = 6] = "RPAD"

                                // QAM button pressed
                                if key == evdev::Key::KEY_PROG1 && event.value() == 0 {
                                    println!("Show QAM");
                                    steam
                                        .execute("GamepadNavTree.m_Controller.OnButtonActionInternal(true, 28, 2)")
                                        .await;
                                }

                                // Main menu button pressed
                                if key == evdev::Key::KEY_F16 && event.value() == 0 {
                                    println!("Show Menu");
                                    steam
                                        .execute("GamepadNavTree.m_Controller.OnButtonActionInternal(true, 27, 2); console.log(\"Show Menu\");")
                                        .await;
                                }
                                
                                // Back button(s) (unified) Revisit once separated
                                if key == evdev::Key::KEY_F15 && event.value() == 0 {
                                    
                                    steam
                                        .execute("GamepadNavTree.m_Controller.OnButtonActionInternal(true, 26, 2); console.log(\"Simulating Rear right lower SteamDeck button\");")
                                        .await;
                                }
                            }
                        },
                        Err(_) => {
                            print!("Error reading event stream, retrying in 1 second");
                            thread::sleep(Duration::from_secs(1));
                            tokio::spawn(async move {
                                start_mapper(steam)
                            });
                            break
                        }
                    };
                }
            }
        })),
        None => {
            println!("No Ally-specific found, retrying in 2 seconds");
            thread::sleep(Duration::from_secs(2));
            tokio::spawn(async move {
                start_mapper(steam)
            });
            None
        }
    }
}
