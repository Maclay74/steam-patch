use crate::devices::create_device;

mod server;
mod steam;
mod devices;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut threads = Vec::new();

    if let Some(device) = create_device() {
        println!("Device created");
        if let Some(mapper_thread) = device.get_key_mapper() {
            println!("Mapper is there");
            threads.push(mapper_thread);
        }
    }

    threads.push(server::start_server());

   /* let _watcher = match steam::patch_steam() {
        Ok(watcher) => watcher,
        Err(_) => {
            println!("Error setting up file watcher. Exiting...");
            std::process::exit(1);
        },
    };*/

    for thread in threads {
        thread.join().unwrap();
    }

    Ok(())
}