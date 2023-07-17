pub mod device_ally;

use device_ally::DeviceAlly;
use crate::server::SettingsRequest;

pub trait Device {
    fn update_settings(&self, request: SettingsRequest);
    fn set_tdp(&self, tdp: i8);
}

pub fn create_device(product_name: &str) -> Option<Box<dyn Device>> {
    match product_name {
        "Ally" => Some(Box::new(DeviceAlly)),
        _ => None,
    }
}