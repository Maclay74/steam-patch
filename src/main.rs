mod server;
mod steam;
mod devices;
mod utils;
mod key_mapper;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut threads = Vec::new();

    if let Some(handle) = key_mapper::start_mapper() {
        threads.push(handle);
    }

    /*threads.push(server::start_server());

    let _watcher = match steam::patch_steam() {
        Ok(watcher) => watcher,
        Err(_) => {
            eprintln!("Error setting up file watcher. Exiting...");
            std::process::exit(1);
        },
    };*/

    for thread in threads {
        thread.join().unwrap();
    }

    Ok(())
}