use regex::Regex;

use crate::utils::get_username;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub struct Patch {
    pub text_to_find: String,
    pub replacement_text: String,
    pub destination: PatchFile,
}

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
    pub fn get_file(&self) -> Option<PathBuf> {
        let username = get_username();
        let steamui_path = dirs::home_dir()
            .map(|home| home.join(format!("/home/{}/.local/share/Steam/steamui", username)));

        let steamui_path = match steamui_path {
            Some(path) => path,
            None => return None,
        };

        if !steamui_path.exists() {
            return None;
        }

        let regex = Regex::new(self.get_regex()).unwrap();
        let matching_files: Vec<_> = match fs::read_dir(&steamui_path).ok() {
            Some(dir) => dir
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let file_name = entry.file_name();
                    if regex.is_match(file_name.to_str().unwrap()) {
                        Some(entry)
                    } else {
                        None
                    }
                })
                .collect(),
            None => return None,
        };

        if matching_files.is_empty() || matching_files.len() > 1 {
            return None;
        }

        let first_matching_file = matching_files[0].file_name();
        Some(steamui_path.join(first_matching_file))
    }
}
