use crate::log::{error, info, warning};
use crate::utils::is_url;
use crate::youtube::search;
use dirs::config_dir;
use serde_json;
use std::fs;

/// Directory path for storing playlist files.
const PLAYLISTS_DIR: &str = "{}/playit";

/// Represents a playlist with a path to the file and a list of items (URLs).
///
/// # Fields
/// - `path`: A string representing the path to the playlist file.
/// - `items`: A vector of strings containing the URLs of the playlist items.
#[derive(Clone)]
pub struct Playlist {
    pub path: String,
    pub items: Vec<String>,
}

impl Playlist {
    /// Creates a new playlist with the specified name. The playlist is saved as a `.pl` file
    /// inside the user's configuration directory.
    ///
    /// # Parameters
    /// - `name`: The name of the playlist.
    ///
    /// # Returns
    /// A new `Playlist` instance with the generated file path and an empty list of items.
    pub fn new(name: &str) -> Self {
        // Construct the path for the playlist file based on the user's configuration directory.
        let path = format!(
            "{}/{}.pl",
            PLAYLISTS_DIR.replace(
                "{}",
                &config_dir()
                    .unwrap_or_else(|| {
                        // TODO: Maybe using `$HOME/.config` as fallback will be good.
                        // But now, let's just throw errors
                        error("Unable to retrieve the configuration directory.");
                        std::process::exit(1);
                    })
                    .display()
                    .to_string()
            ),
            name
        );

        // Log the creation of the new playlist
        info(format!("Creating new playlist: {}", name));
        Playlist {
            path,
            items: Vec::new(),
        }
    }

    /// Reads the playlist file and loads its content into the `items` field as a vector of strings.
    ///
    /// # Returns
    /// - `Ok(())` if the file was successfully read and parsed.
    /// - `Err(String)` if there was an error reading or parsing the file.
    pub fn read(&mut self) -> Result<(), String> {
        // Attempt to read the playlist file into a string.
        let content = match fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(_) => {
                error(format!(
                    "Failed to read the playlist file at: {}",
                    self.path
                ));
                return Err("Error reading the playlist file.".to_string());
            }
        };

        // Deserialize the content into a vector of strings (URLs).
        match serde_json::from_str::<Vec<String>>(&content) {
            Ok(data) => {
                self.items = data;
                info(format!("Playlist loaded successfully from: {}", self.path)); // Log successful loading
                Ok(())
            }
            Err(_) => {
                error(format!(
                    "Failed to parse the playlist JSON from: {}",
                    self.path
                ));
                Err("Failed to parse the playlist JSON. It might be corrupted.".to_string())
            }
        }
    }

    /// Adds a new URL or search query to the playlist. If the query is not a valid URL,
    /// a search is performed to find the appropriate URL.
    ///
    /// # Parameters
    /// - `query`: A string slice containing either a URL or a search query.
    ///
    /// # Returns
    /// This function does not return a value. It updates the playlist items.
    pub fn add(&mut self, query: &str) {
        // If the query is not a valid URL, perform a search.
        let url = if is_url(query) {
            query.to_string() // Convert &str to String
        } else {
            match search(query) {
                Ok(result) => result,
                Err(_) => {
                    warning(format!(
                        "Search query '{}' did not return a valid result.",
                        query
                    )); // Log a warning for failed search
                    return;
                }
            }
        };

        // Add the valid URL to the playlist.
        self.items.push(url);
        info(format!("Added URL to playlist: {}", query)); // Log added URL
    }

    /// Removes a URL from the playlist based on an exact match.
    ///
    /// # Parameters
    /// - `query`: The URL to remove from the playlist.
    ///
    /// # Returns
    /// This function does not return a value. If the URL is found, it is removed from the playlist.
    pub fn remove(&mut self, query: &str) {
        // Find the index of the item that matches the query, and remove it if found.
        if let Some(index) = self.items.iter().position(|item| item == query) {
            self.items.remove(index);
            info(format!("Removed URL from playlist: {}", query)); // Log URL removal
        } else {
            warning(format!(
                "URL '{}' not found in the playlist, nothing to remove.",
                query
            )); // Log a warning for non-existent URL
        }
    }

    /// Writes the playlist items to the playlist file as a JSON string.
    ///
    /// # Returns
    /// - `Ok(())` if the file was successfully written.
    /// - `Err(String)` if there was an error writing to the file.
    pub fn write(self) -> Result<Self, String> {
        // Serialize the playlist items into a JSON string.
        let content = match serde_json::to_string(&self.items) {
            Ok(content) => content,
            Err(_) => {
                error("Failed to convert playlist items to string for writing to file.");
                return Err("Failed to convert playlist items to string.".to_string());
            }
        };

        // Write the content to the playlist file.
        match fs::write(&self.path, content) {
            Ok(_) => {
                info(format!("Playlist successfully written to: {}", self.path)); // Log successful write
                Ok(self)
            }
            Err(e) => {
                error(format!(
                    "Error writing to playlist file at {}: {}",
                    self.path, e
                )); // Log error during write
                Err("Failed to write playlist to file.".to_string())
            }
        }
    }
}
