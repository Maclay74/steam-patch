use regex::Regex;

use crate::utils::get_username;
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Patch {
    pub text_to_find: String,
    pub replacement_text: String,
    pub destination: PatchFile,
}

#[derive(Debug)]
pub enum PatchFile {
    Chunk,
    Library,
}

impl PatchFile {
    pub fn get_regex(&self) -> &str {
        match self {
            PatchFile::Chunk => "^chunk",
            PatchFile::Library => "^library",
        }
    }
}

impl PatchFile {
    pub fn get_file(&self) -> Result<Option<PathBuf>, &'static str> {
        let username = get_username();
        println!("Username: {}", username );
        let steamui_path = dirs::home_dir()
            .ok_or("Home directory not found")?
            .join(format!("/home/{}/.local/share/Steam/steamui", username));
    
        if !steamui_path.exists() {
            println!("Steam UI path does not exist: {:?}", steamui_path);
            return Ok(None);
        }
    
        let regex = Regex::new(self.get_regex()).map_err(|_| "Failed to create regex")?;
        
        let mut matching_files = Vec::new();

        let entries = fs::read_dir(&steamui_path).map_err(|_| "Failed to read Steam UI directory")?;
        for entry in entries {
            let entry = entry.map_err(|_| "Failed to read an entry in the Steam UI directory")?;
            let file_name = entry.file_name().into_string().map_err(|_| "Failed to convert OsString to String")?;
            if regex.is_match(&file_name) {
                matching_files.push(entry.path());
                // If there's more than one match, no need to continue
                if matching_files.len() > 1 {
                    println!("Expected one matching file, found multiple.");
                    return Ok(None);
                }
            }
        }
    
        if matching_files.len() == 1 {
            Ok(matching_files.pop())
        } else {
            println!("Expected one matching file, found {}: {:?}", matching_files.len(), matching_files);
            Ok(None)
        }
    }
    
}
