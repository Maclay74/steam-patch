#![allow(non_snake_case)] // Allow non-snake_case identifiers

use crate::devices::create_device;
use crate::patch::Patch;
use crate::utils::get_username;
use hyper::{Client, Uri};
use inotify::{Inotify, WatchMask};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;
use sysinfo::{ProcessExt, SystemExt};
use tokio::time::{sleep, Duration};
use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

#[derive(Deserialize)]
struct Tab {
    title: String,
    webSocketDebuggerUrl: String,
}

pub struct SteamClient {
    socket: Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>>,
}

impl SteamClient {
    pub fn patch(&mut self, patches: Vec<Patch>) -> Result<(), Error> {
        let mut opened_files: HashMap<String, String> = HashMap::new();

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
            fs::write(path, content)?;
        }

        Ok(())
    }

    pub fn unpatch(&mut self, patches: Vec<Patch>) -> Result<(), Error> {
        let mut opened_files: HashMap<String, String> = HashMap::new();

        for patch in patches {
            let path_file = patch.destination.get_file().unwrap();
            let path_str = path_file.to_str().unwrap().to_string();
            let content = opened_files
                .entry(path_str.clone())
                .or_insert_with(|| fs::read_to_string(&path_file).unwrap());
            let text_to_find = &patch.text_to_find;
            let replacement_text = &patch.replacement_text;
            *content = content.replace(replacement_text, text_to_find);
        }

        for (path, content) in &opened_files {
            fs::write(path, content)?;
        }

        Ok(())
    }

    async fn send_message(&mut self, message: serde_json::Value) {
        let mut retries = 3;

        while retries > 0 {
            match self.socket.as_mut() {
                Some(socket) => match socket.send(Message::Text(message.to_string())) {
                    Ok(_) => break,
                    Err(_) => {
                        eprintln!("Couldn't send message to Steam, retrying...");

                        self.connect().await;

                        retries -= 1;
                    }
                },
                None => {
                    self.connect().await;
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

    pub fn new() -> SteamClient {
        return SteamClient { socket: None };
    }

    pub async fn connect(&mut self) {
        if let Some(context) = Self::get_context().await {
            self.socket = match connect(context) {
                Ok((socket, _)) => Some(socket),
                Err(_) => None,
            };
        }
    }

    pub fn get_log_path() -> Option<PathBuf> {
        let username = get_username();
        dirs::home_dir().map(|home| {
            home.join(format!(
                "/home/{}/.local/share/Steam/logs/bootstrap_log.txt",
                username
            ))
        })
    }

    pub async fn watch() -> Option<tokio::task::JoinHandle<()>> {
        // If Steam client is already running, patch it and restart
        if Self::is_running() {
            let mut client = Self::new();
            client.connect().await;

            if let Some(device) = create_device() {
                match client.patch(device.get_patches()) {
                    Ok(_) => println!("Steam patched"),
                    Err(_) => eprintln!("Couldn't patch Steam"),
                }
            }

            client.reboot().await;
        }

        // Watch for changes in log
        let mut inotify = Inotify::init().expect("Failed to initialize inotify");

        if let Some(log_path) = Self::get_log_path() {
            inotify.watches().add(log_path, WatchMask::MODIFY).unwrap();
        }

        println!("Watching Steam log...");
        let task = tokio::task::spawn_blocking(move || {
            let mut buffer = [0u8; 4096];
            let mut client = Self::new();
            loop {
                if let Ok(events) = inotify.read_events_blocking(&mut buffer) {
                    for _ in events {
                        let file = File::open(Self::get_log_path().unwrap()).unwrap();
                        let reader = BufReader::new(file);

                        match reader.lines().last() {
                            Some(Ok(line)) => {
                                if line.contains("Verification complete") {
                                    if let Some(device) = create_device() {
                                        match client.patch(device.get_patches()) {
                                            Ok(_) => println!("Steam patched"),
                                            Err(_) => eprintln!("Couldn't patch Steam"),
                                        }
                                    }
                                }

                                if line.contains("Shutdown") {
                                    if let Some(device) = create_device() {
                                        match client.unpatch(device.get_patches()) {
                                            Ok(_) => println!("Steam unpatched"),
                                            Err(_) => eprintln!("Couldn't unpatch Steam"),
                                        }
                                    }
                                }
                            }
                            Some(Err(err)) => println!("Error reading line: {}", err),
                            None => println!("The file is empty"),
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
