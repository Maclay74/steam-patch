#![allow(non_snake_case)] // Allow non-snake_case identifiers

use crate::devices::create_device;
use crate::patch::{Patch, PatchFile};
use hyper::{Client, Uri};
use inotify::{EventMask, Inotify, WatchMask};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use sysinfo::{ProcessExt, SystemExt};
use tokio::time::{sleep, Duration};
use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

#[derive(Deserialize)] // Enable deserialization for the Tab struct
struct Tab {
    title: String,
    webSocketDebuggerUrl: String,
}

pub struct SteamClient {
    socket: WebSocket<MaybeTlsStream<std::net::TcpStream>>,
}

impl SteamClient {
    pub async fn patch(&mut self, patches: Vec<Patch>) -> Result<(), Error> {
        let mut opened_files: HashMap<String, String> = HashMap::new();

        //if let Some(device) = create_device() {
        //let patches = device.get_patches();
        for patch in patches {
            let path_file = patch.destination.get_file().unwrap();
            let path_str = path_file.to_str().unwrap().to_string();
            let content = opened_files
                .entry(path_str.clone())
                .or_insert_with(|| fs::read_to_string(&path_file).unwrap());
            let text_to_find = &patch.text_to_find;
            let replacement_text = &patch.replacement_text;
            *content = content.replace(text_to_find, replacement_text);
        }

        for (path, content) in &opened_files {
            fs::write(path, content)?; // write the updated content back to each file
        }
        //}

        Ok(())
    }
    async fn send_message(&mut self, message: serde_json::Value) {
        let mut retries = 3;

        while retries > 0 {
            match self.socket.send(Message::Text(message.to_string())) {
                Ok(_) => break, // the message has been successfully sent, exit the loop
                Err(_) => {
                    eprintln!("Couldn't send message to Steam, retrying...");

                    if let Some(context) = Self::get_context().await {
                        match connect(context) {
                            Ok((socket, _)) => {
                                self.socket = socket;
                            }
                            Err(_) => {
                                println!("Still can't connect to Steam");
                            }
                        };
                    }

                    retries -= 1;
                }
            }
        }
    }
    pub async fn reboot(&mut self) {
        self.send_message(serde_json::json!({
            "id": 1,
            "method": "Page.reload",
        }))
        .await;
    }
    pub async fn execute(&mut self, js_code: &str) {
        self.send_message(serde_json::json!({
            "id": 1,
            "method": "Runtime.evaluate",
            "params": {
                "expression": js_code,
            }
        }))
        .await;
    }
    async fn get_context() -> Option<String> {
        println!("Getting Steam...");

        let client = Client::new();
        let start_time = tokio::time::Instant::now();
        let uri: Uri = "http://localhost:8080/json".parse().unwrap();

        loop {
            if start_time.elapsed() > Duration::from_secs(60) {
                println!("Timeout while trying to fetch Steam data!");
                return None;
            }

            match client.get(uri.clone()).await {
                Ok(response) => {
                    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
                    let tabs: Vec<Tab> =
                        serde_json::from_slice(&bytes).unwrap_or_else(|_| Vec::new());
                    if let Some(tab) = tabs.into_iter().find(|tab| {
                        tab.title == "SharedJSContext" && !tab.webSocketDebuggerUrl.is_empty()
                    }) {
                        return Some(tab.webSocketDebuggerUrl);
                    }
                }
                Err(_) => println!("Couldn't connect to Steam"),
            }

            sleep(Duration::from_millis(50)).await;
        }
    }
    pub async fn new() -> Option<SteamClient> {
        if let Some(context) = Self::get_context().await {
            let (socket, _) = match connect(context) {
                Ok(socket) => socket,
                Err(_) => {
                    println!("Couldn't connect to Steam!");
                    return None;
                }
            };

            return Some(SteamClient { socket });
        }

        None
    }

    pub async fn watch() -> Option<tokio::task::JoinHandle<()>> {
        // If Steam client is already running, patch it and restart
        if Self::is_running() {
            let mut client = Self::new().await?;

            if let Some(device) = create_device() {
                match client.patch(device.get_patches()).await {
                    Ok(_) => println!("Steam patched"),
                    Err(_) => eprintln!("Couldn't patch Steam"),
                }
            }

            client.reboot().await;
        }

        // Watch for changes in chunk
        let mut inotify = Inotify::init().expect("Failed to initialize inotify");
        let chunk_patch = PatchFile::Chunk.get_file()?;

        inotify
            .watches()
            .add(chunk_patch.as_path(), WatchMask::DELETE_SELF)
            .unwrap();

        println!("Watching current directory for activity...");
        let task = tokio::task::spawn_blocking(move || {
            let mut buffer = [0u8; 4096];
            loop {
                if let Ok(events) = inotify.read_events_blocking(&mut buffer) {
                    for event in events {
                        if event.mask.contains(EventMask::DELETE_SELF) {
                            println!("Steam patched itself!");

                            tokio::task::block_in_place(|| {
                                let rt = tokio::runtime::Runtime::new().unwrap();
                                rt.block_on(async {
                                    if let Some(mut client) = Self::new().await {
                                        if let Some(device) = create_device() {
                                            match client.patch(device.get_patches()).await {
                                                Ok(_) => println!("Steam patched"),
                                                Err(_) => eprintln!("Couldn't patch Steam"),
                                            }
                                        }
                                        client.reboot().await;
                                    }
                                })
                            });
                        }
                    }
                }
            }
        });

        Some(task)
    }
    fn is_running() -> bool {
        let mut sys = sysinfo::System::new_all();

        // We need to update the system value to get the fresh process list
        sys.refresh_all();

        sys.processes()
            .values()
            .any(|process| process.name() == "steam")
    }
}
