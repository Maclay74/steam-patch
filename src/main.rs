mod server;
mod steam;
mod devices;
mod utils;
use evdev_rs::enums::EV_KEY;
use evdev_rs::enums::EventCode;
use std::time::Duration;
use uinput::{event::keyboard, CreateDevice, Device, Key};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut threads = Vec::new();

    threads.push(server::start_server());

    let _watcher = match steam::patch_steam() {
        Ok(watcher) => watcher,
        Err(_) => {
            eprintln!("Error setting up file watcher. Exiting...");
            std::process::exit(1);
        },
    };

    for thread in threads {
        thread.join().unwrap();
    }

    let mut device = CreateDevice::new()?
        .name("virtual_keyboard")?
        .with_events(vec![
            // Add the keys that you want to emit events for.
            keyboard::Key::A.into(),
        ])?
        .create()?;

    // Simulate a key press.
    device.emit(&Key::A, 1)?;
    device.synchronize()?;

    // Wait a bit.
    std::thread::sleep(Duration::from_millis(100));

    // Simulate a key release.
    device.emit(&Key::A, 0)?;
    device.synchronize()?;

    Ok(())
}