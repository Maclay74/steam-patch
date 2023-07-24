use crate::devices::create_device;

mod devices;
mod server;
//mod steam;
mod patch;
mod utils;

#[tokio::main]
async fn main() {
    let mut tasks = vec![];

    tasks.push(tokio::spawn(server::run()));

    tasks.push(tokio::spawn(async {
        // Here goes your file watcher code
        println!("file watcher")
    }));

    if let Some(device) = create_device() {
        println!("Device created");
        if let Some(mapper) = device.get_key_mapper() {
            tasks.push(mapper);
        }
    }

    let _ = futures::future::join_all(tasks).await;
}
