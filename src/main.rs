use crate::devices::create_device;

mod devices;
mod patch;
mod server;
mod steam;
mod utils;

#[tokio::main]
async fn main() {
    let mut tasks = vec![];

    tasks.push(tokio::spawn(server::run()));

    if let Some(device) = create_device() {
        println!("Device created");
        if let Some(mapper) = device.get_key_mapper() {
            tasks.push(mapper);
        }
    }

    if let Some(steam) = steam::SteamClient::watch().await {
        tasks.push(steam);
    }

    let _ = futures::future::join_all(tasks).await;
}
