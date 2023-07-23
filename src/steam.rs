#![allow(non_snake_case)] // Allow non-snake_case identifiers

use reqwest::blocking::get;
use std::{fs, thread};
use serde::Deserialize;
use tungstenite::connect;
use tungstenite::Message;
use std::io::{Error};
use std::path::{Path, PathBuf};
use std::env;
use std::time::{Duration, Instant};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use sysinfo::{ProcessExt, SystemExt};
use crate::devices::{create_device, PatchFile};
use std::collections::HashMap;
use regex::Regex;

#[derive(Deserialize)] // Enable deserialization for the Tab struct
struct Tab {
    title: String,
    webSocketDebuggerUrl: String,
}

fn get_context() -> Option<String> {
    println!("Getting Steam...");

    let start_time = Instant::now();
    loop {
        if start_time.elapsed() > Duration::from_secs(60) {
            println!("Timeout while trying to fetch Steam data!");
            return None;
        }

        match get("http://localhost:8080/json") {
            Ok(response) => {
                match response.json::<Vec<Tab>>() {
                    Ok(tabs) => {
                        if let Some(tab) = tabs.into_iter().find(|tab| tab.title == "SharedJSContext" && !tab.webSocketDebuggerUrl.is_empty()) {
                            return Some(tab.webSocketDebuggerUrl);
                        }
                    }
                    Err(_) => println!("Failed to deserialize response!")
                }
            }
            Err(_) => {}
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
        Err(err) => println!("Failed to reboot Steam: {:?}", err)
    }
}

fn apply_patches() -> Result<(), Error> {
    let mut opened_files: HashMap<String, String> = HashMap::new();

    if let Some(device) = create_device() {
        let patches = device.get_patches();
        for patch in patches {
            let path_file = patch.destination.get_file().unwrap();
            let path_str = path_file.to_str().unwrap().to_string();
            let content = opened_files.entry(path_str.clone()).or_insert_with(|| fs::read_to_string(&path_file).unwrap());
            let text_to_find = &patch.text_to_find;
            let replacement_text = &patch.replacement_text;
            *content = content.replace(text_to_find, replacement_text);
        }

        for (path, content) in &opened_files {
            fs::write(path, content)?;  // write the updated content back to each file
        }
    }

    Ok(())
}

fn get_username() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return String::from("gamer");
    }

    let arg = &args[1];

    if arg.starts_with("--user=") {
        let username = arg.trim_start_matches("--user=");
        String::from(username)
    } else {
        String::from("gamer")
    }
}

fn is_steam_running() -> bool {
    let mut sys = sysinfo::System::new_all();

    // We need to update the system value to get the fresh process list
    sys.refresh_all();

    for (_, process) in sys.processes() {
        if process.name() == "steam" {
            return true;
        }
    }

    false
}

pub fn patch_steam() -> Result<RecommendedWatcher, ()> {

  /*  // Get Steam chunk link
    let steam_chunk_path = match get_patch_file(PatchFile::Chunk) {
        Ok(chunk) => chunk,
        Err(err) => {
            println!("Failed to get steam chunk: {:?}", err);
            return Err(());
        }
    };
*/
    match apply_patches() {
        Ok(_) =>  {
            //*is_chunk_patched_clone.lock().unwrap() = true;
            if is_steam_running() {
                match get_context() {
                    Some(link) =>reboot(link),
                    None => println!("Can't get Steam context")
                }
            }
        }
        Err(_) => println!("Couldn't patch chunk")
    }
    
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
                            /*match get_patch_file(PatchFile::Chunk) {
                                Ok(chunk) => {
                                    apply_patches(&chunk).expect("Failed to apply patches");
                                    reboot(link);
                                },
                                Err(err) => println!("Failed to get steam chunk: {:?}", err),
                            }*/
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
}

impl PatchFile {
    pub fn get_file(&self) -> Result<PathBuf, Error> {
        let username = get_username();

        // Depending on the system, different path
        let steamui_path = if cfg!(windows) {
            env::var_os("PROGRAMFILES(X86)")
                .map(|path| Path::new(&path).join("Steam").join("steamui"))
        } else {
            dirs::home_dir().map(|home| home.join(format!("/home/{}/.local/share/Steam/steamui", username)))
        };

        // Steam folder not found
        let steamui_path = match steamui_path {
            Some(path) => path,
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Path doesn't exist",
                ));
            }
        };

        if !steamui_path.exists() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Path doesn't exist",
            ));
        }

        let regex = Regex::new(self.get_regex()).unwrap();
        let matching_files: Vec<_> = fs::read_dir(&steamui_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_name = entry.file_name();
                if regex.is_match(file_name.to_str().unwrap()) { // handle this unwrap as appropriate
                    Some(entry)
                } else {
                    None
                }
            })
            .collect();

        if matching_files.is_empty() || matching_files.len() > 1 {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Chunk not found or multiple chunks found",
            ));
        }

        let first_matching_file = matching_files[0].file_name();
        Ok(steamui_path.join(first_matching_file))
    }
}