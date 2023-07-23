use std::thread;

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


pub fn start_mapper() -> Option<thread::JoinHandle<()>> {

    let mut device = pick_device();

    match device {
        Some(ref mut device) => {
            // Use the device
            println!("Device found: {}", device.name().unwrap_or("Unnamed device"));

            Some(thread::spawn(move || {
                loop {
                    for ev in device.fetch_events().unwrap() {
                        println!("{:?}", ev);
                    }
                }
            }))
        }
        None => {
            println!("No device found");
            None
        }
    }
}