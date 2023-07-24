#![allow(non_snake_case)] // Allow non-snake_case identifiers

use crate::devices::{create_device, PatchFile};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{fs, thread};
use tungstenite::connect;
use tungstenite::Message;

#[derive(Deserialize)] // Enable deserialization for the Tab struct
struct Tab {
    title: String,
    webSocketDebuggerUrl: String,
}

pub fn get_context() -> Option<String> {
    println!("Getting Steam...");

    let client = Client::new();
    let start_time = Instant::now();

    loop {
        if start_time.elapsed() > Duration::from_secs(60) {
            println!("Timeout while trying to fetch Steam data!");
            return None;
        }

        match client.get("http://localhost:8080/json").send() {
            Ok(response) => match response.json::<Vec<Tab>>() {
                Ok(tabs) => {
                    if let Some(tab) = tabs.into_iter().find(|tab| {
                        tab.title == "SharedJSContext" && !tab.webSocketDebuggerUrl.is_empty()
                    }) {
                        return Some(tab.webSocketDebuggerUrl);
                    }
                }
                Err(_) => println!("Failed to deserialize response!"),
            },
            Err(_) => {
                println!("Couldn't connect to Steam");
            }
        }

        thread::sleep(Duration::from_millis(50));
    }
}

fn reboot(link: String) {
    let (mut socket, _) = match connect(link) {
        Ok(socket) => socket,
        Err(_) => {
            println!("Couldn't reload Steam!");
            return;
        }
    };

    let message = serde_json::json!({
        "id": 1,
        "method": "Page.reload",
    });
    match socket.write_message(Message::Text(message.to_string())) {
        Ok(_) => println!("Steam Rebooted"),
        Err(err) => println!("Failed to reboot Steam: {:?}", err),
    }
}

pub fn execute(link: String, js_code: String) {
    let (mut socket, _) = match connect(link) {
        Ok(socket) => socket,
        Err(_) => {
            println!("Couldn't reload Steam!");
            return;
        }
    };

    let message = serde_json::json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": {
            "expression": js_code,
        }
    });
    socket
        .write_message(Message::Text(message.to_string()))
        .expect("TODO: panic message");
}

fn apply_patches() -> Result<(), Error> {
    let mut opened_files: HashMap<String, String> = HashMap::new();

    if let Some(device) = create_device() {
        let patches = device.get_patches();
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
    }

    Ok(())
}

pub fn patch_steam() {
    match apply_patches() {
        Ok(_) => {
            if is_steam_running() {
                match get_context() {
                    Some(link) => reboot(link),
                    None => println!("Can't get Steam context"),
                }
            }
        }
        Err(_) => println!("Couldn't patch chunk"),
    }

    /*

    // Watch for changes in the chunk.
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
        let event = match res {
            Ok(event) => event,
            Err(e) => {
                println!("watch error: {:?}", e);
                return;
            }
        };

        if let notify::EventKind::Remove(_) = event.kind {
            if let Some(path) = event.paths.get(0) {
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().contains("chunk") {
                        println!("Steam patched itself! {:?}", event.kind);
                        if let Some(link) = get_context() {
                            apply_patches().expect("Failed to apply patches");
                            reboot(link);
                        } else {
                            println!("Can't get Steam context");
                        }
                    }
                }
            }
        }
    }).unwrap();

    watcher.watch(PatchFile::Chunk.get_file().unwrap().as_path(), RecursiveMode::NonRecursive).unwrap();

    Ok(watcher)

     */
}
