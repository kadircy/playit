use crate::utils::is_url;
use crate::youtube::search;
use dirs::config_dir;
use serde_json;
use std::fs;

const PLAYLISTS_DIR: &str = "{}/playit";

#[derive(Clone)]
pub struct Playlist {
    pub path: String,
    pub items: Vec<String>,
}

impl Playlist {
    pub fn new(name: &str) -> Self {
        let path = format!(
            "{}/{}.pl",
            PLAYLISTS_DIR.replace("{}", &config_dir().unwrap().display().to_string()),
            name
        );
        Playlist {
            path,
            items: vec![],
        }
    }

    pub fn read(&mut self) -> Result<(), String> {
        let content = match fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(_) => {
                return Err("An error occurred while reading content of playlist".to_string());
            }
        };
        let data: Vec<String> = serde_json::from_str(&content).expect(
            "Unexpected Error: Unable to resolve playlist JSON correctly. Maybe invalid format?",
        );
        self.items = data;
        Ok(())
    }

    pub fn add(&mut self, query: String) {
        let url = match is_url(&query) {
            true => query,
            false => search(&query).unwrap(),
        };
        self.items.push(url);
    }

    pub fn remove(&mut self, query: String) {
        if let Some(index) = self.items.iter().position(|item| *item == query) {
            self.items.remove(index);
        }
    }

    pub fn write(&self) -> Result<(), String> {
        let content: String = match serde_json::to_string(&self.items) {
            Ok(content) => content,
            Err(_) => {
                return Err(
                    "An error occured while translating playlist items to string".to_string(),
                );
            }
        };
        match fs::write(&self.path, content) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Err("An error occured while writing contents to playlist file".to_string())
            }
        }
    }
}
