#![allow(non_snake_case)] // Allow non-snake_case identifiers

use reqwest::blocking::get;
use std::{fs, thread};
use serde::Deserialize;
use tungstenite::connect;
use tungstenite::Message;
use std::io::{Error};
use std::path::{Path, PathBuf};
use std::env;

mod patches;

#[derive(Deserialize)] // Enable deserialization for the Tab struct
struct Tab {
    title: String,
    webSocketDebuggerUrl: String,
}

fn get_context() -> Option<String> {
    println!("Getting Steam...");

    loop {
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
    match connect(link) {
        Ok((mut socket, _)) => {
            let message = serde_json::json!({
                "id": 1,
                "method": "Page.reload",
            });
            match socket.write_message(Message::Text(message.to_string())) {
                Ok(_) => println!("Steam Rebooted"),
                Err(err) => println!("Failed to reboot Steam: {:?}", err)
            }
        }
        Err(_) => {
            println!("Couldn't reload Steam!")
        }
    }
}

fn apply_patches(steamChunkPath: PathBuf) -> Result<(), Error> {
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

fn get_chunk() -> Result<PathBuf, Error> {
    // Depending on the system, different path
    let steamui_path = if cfg!(windows) {
        env::var_os("PROGRAMFILES(X86)")
            .map(|path| Path::new(&path).join("Steam").join("steamui"))
    } else {
        dirs::home_dir().map(|home| home.join(".local/share/Steam/steamui"))
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

    if matching_files.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Chunk not found",
        ));
    }

    let first_matching_file = matching_files[0].file_name();
    Ok(steamui_path.join(first_matching_file))
}

pub fn patch_steam() -> thread::JoinHandle<()> {
    let steamChunkPath = get_chunk().unwrap();

    thread::spawn(move || {
        let link = get_context().unwrap();
        apply_patches(steamChunkPath).unwrap();
        reboot(link);
    })
}
