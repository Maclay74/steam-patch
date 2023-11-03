#![allow(non_snake_case)] // Allow non-snake_case identifiers

use crate::devices::create_device;
use crate::patch::Patch;
use crate::utils::get_username;
use hyper::{Client, Uri, body};
use inotify::{Inotify, WatchMask};
use serde::{Deserialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;
use sysinfo::{ProcessExt, SystemExt};
use tokio::time::{sleep, Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use futures::StreamExt; // Required for 'next' method on streams
use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};
use std::option::Option;

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

        for patch in &patches {
            // Use `match` or `if let` to handle the Option returned by `get_file()`
            println!("Applying patch: {:?}", patch);
            if let Ok(Some(path_file)) = patch.destination.get_file() {
                // Convert the `Path` to a string and handle the potential `None` case
                if let Some(path_str) = path_file.to_str() {
                    let path_string = path_str.to_string();
                    println!("Processing file: {}", path_string);
                    // Handle the Result of `read_to_string` with `unwrap_or_else`
                    let content = opened_files
                        .entry(path_string.clone())
                        .or_insert_with(|| {
                            match fs::read_to_string(&path_file) {
                                Ok(content) => {
                                    println!("File read successfully: {}", path_string);
                                    content
                                }
                                Err(e) => {
                                    // Handle the error, e.g., by logging and continuing with an empty string
                                    eprintln!("Error reading the file '{}': {}", path_string, e);
                                    String::new() // return an empty string on error
                                }
                            }
                        });
                    let text_to_find = &patch.text_to_find;
                    let replacement_text = &patch.replacement_text;
                    if content.contains(text_to_find) {
                        println!("Found text to replace in {}: '{}'", path_string, text_to_find);
                        *content = content.replace(text_to_find, replacement_text);
                    } else {
                        println!("Text not found in {}: '{}'", path_string, text_to_find);
                    }
                } else {
                    // Handle the error if path_str is None
                    eprintln!("Failed to convert the path to a string for file: {:?}", path_file);
                }
            } else {
                // Handle the error if get_file() returns None
                eprintln!("Failed to get the file from destination or the path is not valid for patch: {:?}", patch);
            }
        }

        println!("Writing changes to disk...");
        for (path, content) in &opened_files {
            match fs::write(path, content) {
                Ok(_) => println!("File written successfully: {}", path),
                Err(e) => eprintln!("Failed to write to file '{}': {}", path, e),
            };
        }
        println!("Patching complete.");

        Ok(())
    }

    pub fn unpatch(&mut self, patches: Vec<Patch>) -> Result<(), Error> {
        let mut opened_files: HashMap<String, String> = HashMap::new();

       
        for patch in &patches {
            // Use `match` or `if let` to handle the Option returned by `get_file()`
            println!("Removing patch: {:?}", patch);
            if let Ok(Some(path_file)) = patch.destination.get_file() {
                // Convert the `Path` to a string and handle the potential `None` case
                if let Some(path_str) = path_file.to_str() {
                    let path_string = path_str.to_string();
                    println!("Processing file: {}", path_string);
                    // Handle the Result of `read_to_string` with `unwrap_or_else`
                    let content = opened_files
                        .entry(path_string.clone())
                        .or_insert_with(|| {
                            match fs::read_to_string(&path_file) {
                                Ok(content) => {
                                    println!("File read successfully: {}", path_string);
                                    content
                                }
                                Err(e) => {
                                    // Handle the error, e.g., by logging and continuing with an empty string
                                    eprintln!("Error reading the file '{}': {}", path_string, e);
                                    String::new() // return an empty string on error
                                }
                            }
                        });
                    let text_to_find = &patch.text_to_find;
                    let replacement_text = &patch.replacement_text;
                    if content.contains(replacement_text) {
                        println!("Found text to replace in {}: '{}'", path_string, text_to_find);
                        *content = content.replace(replacement_text, text_to_find);
                    } else {
                        println!("Text not found in {}: '{}'", path_string, text_to_find);
                    }
                } else {
                    // Handle the error if path_str is None
                    eprintln!("Failed to convert the path to a string for file: {:?}", path_file);
                }
            } else {
                // Handle the error if get_file() returns None
                eprintln!("Failed to get the file from destination or the path is not valid for patch: {:?}", patch);
            }
        }

        println!("Writing changes to disk...");
        for (path, content) in &opened_files {
            match fs::write(path, content) {
                Ok(_) => println!("File written successfully: {}", path),
                Err(e) => eprintln!("Failed to write to file '{}': {}", path, e),
            };
        }
        println!("Patching complete.");

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
        let start_time = Instant::now();
        let uri = match "http://localhost:8080/json".parse::<Uri>() {
            Ok(uri) => uri,
            Err(_) => {
                println!("Error: Invalid URI.");
                return None;
            }
        };

        println!("URI: {}", uri);
        //Retry time
        while start_time.elapsed() < Duration::from_secs(60) {
            match client.get(uri.clone()).await {
                Ok(response) => {
                    match body::to_bytes(response.into_body()).await {
                        Ok(bytes) => {
                            match serde_json::from_slice::<Vec<Tab>>(&bytes) {
                                Ok(tabs) => {
                                    if let Some(tab) = tabs.into_iter().find(|tab| {
                                        tab.title == "SharedJSContext" && !tab.webSocketDebuggerUrl.is_empty()
                                    }) {
                                        return Some(tab.webSocketDebuggerUrl);
                                    }
                                }
                                Err(_) => println!("Error: Failed to deserialize the response."),
                            }
                        }
                        Err(_) => println!("Error: Failed to read the response body."),
                    }
                }
                Err(_) => println!("Couldn't connect to Steam, retrying..."),
            }
    
            sleep(Duration::from_millis(50)).await;
        }
        println!("Timeout while trying to fetch Steam data!");
        None
        
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
                    Ok(_) => println!("Steam was running and patched"),
                    Err(_) => eprintln!("Couldn't patch Steam"),
                }
            }

            client.reboot().await;
        }

        // Watch for changes in log
        // let mut inotify = Inotify::init().expect("Failed to initialize inotify");
        // Initialize inotify outside of the if-let to ensure it exists for the lifetime of the function
        let mut inotify = match Inotify::init() {
            Ok(inotify) => inotify,
            Err(e) => {
                eprintln!("Failed to initialize inotify: {:?}", e);
                return None;
            }
        };

        // if let Some(log_path) = Self::get_log_path() {
        //     inotify.watches().add(log_path, WatchMask::MODIFY).unwrap();
        // }
        // Get the log path using the existing function
        let log_path = match Self::get_log_path() {
            Some(path) => path,
            None => {
                eprintln!("Log path could not be determined.");
                return None;
            }
        };
        // Add a watch to the log path
        match inotify.watches().add(&log_path, WatchMask::MODIFY) {
            Ok(_) => println!("Watching log path: {:?}", log_path),
            Err(e) => {
                eprintln!("Failed to add a watch to the log path: {:?}", e);
                return None;
            }
        };

        println!("Watching Steam log...");
        let task = tokio::task::spawn_blocking(move || {
            let mut buffer = [0u8; 4096];
            let mut client = Self::new();

            let mut process_flow = 0;
            loop {
                if let Ok(events) = inotify.read_events_blocking(&mut buffer) {
                    for _ in events {
                        let file = File::open(Self::get_log_path().unwrap()).unwrap();
                        let reader = BufReader::new(file);

                        match reader.lines().last() {
                            Some(Ok(line)) => {
                                if line.contains("BVerifyInstalledFiles") {
                                    println!("BVerifyInstalledFiles - {}", process_flow);
                                    process_flow = 1;
                                }
                                if process_flow == 2 {
                                    if line.contains("Verification complete") {
                                        println!("Verification comeplete - {}", process_flow);

                                        if let Some(device) = create_device() {
                                            match client.patch(device.get_patches()) {
                                                Ok(_) => println!("Steam patched"),
                                                Err(_) => eprintln!("Couldn't patch Steam"),
                                            }
                                        }
                                        process_flow = 0
                                    }
                                }
                                if process_flow == 1 {
                                    if line.contains("Update complete") {
                                        println!("Update complete - {}", process_flow);
                                        process_flow = 2;
                                    }
                                }
                                if process_flow == 0 {
                                    if line.contains("Shutdown") {
                                        println!("Shutdown - {}", process_flow);
                                        if let Some(device) = create_device() {
                                            match client.unpatch(device.get_patches()) {
                                                Ok(_) => println!("Steam unpatched"),
                                                Err(_) => eprintln!("Couldn't unpatch Steam"),
                                            }
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
