#![allow(non_snake_case)] // Allow non-snake_case identifiers

use reqwest::blocking::get;
use std::{fs};
use serde::Deserialize;
use tungstenite::connect;
use tungstenite::Message;
use std::io::{Error};
use std::path::{Path, PathBuf};
use std::env;
use std::time::{Duration, Instant};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use sysinfo::{ProcessExt, SystemExt};

mod patches;

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
            Err(_) => println!("Failed to fetch Steam data!")
        }
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

fn apply_patches(steamChunkPath: &PathBuf) -> Result<(), Error> {
    let mut content = fs::read_to_string(&steamChunkPath)?;
    let patches = patches::get_patches();

    for patch in patches {
        let text_to_find = &patch.text_to_find;
        let replacement_text = &patch.replacement_text;
        content = content.replace(text_to_find, replacement_text);
    }

    fs::write(&steamChunkPath, content)?;

    Ok(())
}

fn is_chunk_patched(steamChunkPath: &PathBuf) -> bool  {
    let content = fs::read_to_string(&steamChunkPath).unwrap();
    let patches = patches::get_patches();

    for patch in patches {
        if !content.contains(&patch.replacement_text) {
            return false;
        }
    }

    true
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

fn get_chunk() -> Result<PathBuf, Error> {
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

    let matching_files: Vec<_> = fs::read_dir(&steamui_path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            if file_name.to_string_lossy().contains("chunk") {
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

fn on_chunk_change() {
    let steam_chunk_path = match get_chunk() {
        Ok(chunk) => chunk,
        Err(err) => {
            println!("Failed to get steam chunk: {:?}", err);
            return ;
        }
    };

    if is_chunk_patched(&steam_chunk_path) {
        return
    }

    println!("File has changed!: {:?}", steam_chunk_path);

    match apply_patches(&steam_chunk_path) {
        Ok(_) => println!("Patches applied successfully"),
        Err(err) => println!("Failed to apply patches: {:?}", err)
    };
}

pub fn patch_steam() -> Result<RecommendedWatcher, ()> {

    // Get Steam chunk link
    let steam_chunk_path = match get_chunk() {
        Ok(chunk) => chunk,
        Err(err) => {
            println!("Failed to get steam chunk: {:?}", err);
            return Err(());
        }
    };

    match apply_patches(&steam_chunk_path) {
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
        match res {
            Ok(_) => on_chunk_change(),
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();

    watcher.watch(&steam_chunk_path, RecursiveMode::Recursive).unwrap();

    Ok(watcher)
}