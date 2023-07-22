mod server;
mod steam;
mod devices;
mod utils;
use uinput::event::controller;
use std::thread;
use std::time::Duration;

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

    let device = uinput::default().unwrap()
        .name("test").unwrap()
        .event(uinput::event::controller::Controller::All).unwrap()
        .create().unwrap();

    thread::sleep(Duration::from_secs(1));
    device.synchronize().unwrap();

    Ok(())
}