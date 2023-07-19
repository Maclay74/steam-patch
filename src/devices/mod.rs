pub mod device_ally;

use device_ally::DeviceAlly;
use crate::server::SettingsRequest;

pub struct Patch {
    pub text_to_find: String,
    pub replacement_text: String,
}

pub trait Device {
    fn update_settings(&self, request: SettingsRequest);
    fn set_tdp(&self, tdp: i8);
    fn get_patches(&self) -> Vec<Patch>;
}

pub fn create_device() -> Option<Box<dyn Device>> {
    let product_name_path = "/sys/devices/virtual/dmi/id/product_family";
    let product_name = std::fs::read_to_string(product_name_path)
        .expect("Ally");

    match product_name.trim().as_ref() {
        "ROG Ally" => Some(Box::new(DeviceAlly)),
        _ => None,
    }
}