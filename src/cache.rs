use crate::log::{error, info, warning};
use dirs::cache_dir;
use std::collections::HashMap;
use std::fs;

/// Cache file for storing querys and urls. 
const CACHE_FILE: &str = "{}/playit";

/// Represents url cache for entire program.
///
/// # Fields
/// - `path`: A string representing the path to the cache file.
/// - `items`: A HashMap<String, String> containing querys and URLs.
pub struct Cache {
    pub path: String,
    pub items: HashMap<String, String>,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    /// Creates a new cache object. The cache file is saved as a '.json' file inside the user's
    /// cache directory
    ///
    /// # Returns
    /// A new `Cache` instance with the cache file path and an empty HashMap.
    pub fn new() -> Self {
        // Construct the path for the cache file.
        let path: String = match cache_dir() {
            Some(pathbuf) => CACHE_FILE.replace("{}", &pathbuf.display().to_string()),
            None => {
                warning("Unable to retrieve the cache directory");
                String::new()
            }
        };
        Cache {
            path,
            items: HashMap::new(),
        }
    }

    /// Reads the cache file and loads its content into `items` as a HashMap<String, String>
    ///
    /// # Returns
    /// - `Ok(())` if the file was successfully read and parsed.
    /// - `Err(String)` if there was an error reading or parsing the file
    pub fn read(&mut self) -> Result<(), String> {
        // Check if file exists on the path
        // If not create the file for avoiding errors
        if !fs::exists(&self.path).unwrap_or(false) {
            match fs::write(&self.path, "{}") {
                Ok(()) => {
                    info("Cache file created for playit.");
                }
                Err(e) => {
                    warning("Unable to create cache file for playit.");
                    error(&e);
                    return Err("Unable to create cache file".to_string());
                }
            }
        }
        // Attempt to read the cache file into a string.
        let content = match fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(_) => {
                error(format!("Failed to read the cache file at: {}", self.path));
                return Err("Error reading the cache file.".to_string());
            }
        };
        // Deserialize the content into a HashMap<String, String>
        //                                       (query)  (url)
        match serde_json::from_str::<HashMap<String, String>>(&content) {
            Ok(data) => {
                self.items = data;
                info(format!("Cache loaded successfully from: {}", self.path));
                Ok(())
            }
            Err(_) => {
                error(format!(
                    "Failed to parse the cache JSON from: {}",
                    self.path
                ));
                Err("Failed to parse the cache JSON. It might be corrupted.".to_string())
            }
        }
    }

    /// Adds a new item to cache.
    ///
    /// # Parameters
    /// - `query`: A string slice containing user given query for key.
    /// - `url`: A string slice containing search result of query for value.
    ///
    /// # Returns
    /// This function does not return a value. It updates the cache items.
    pub fn add(&mut self, query: &str, url: &str) {
        self.items.insert(query.to_string(), url.to_string());
        info(format!("Added url with query {} to cache: {}", query, url));
    }

    /// Writes the cache items to the cache file as a JSON string.
    ///
    /// # Returns
    /// - `Ok(())` if the file was successfully written.
    /// - `Err(String)` if there was an error writing the file.
    pub fn write(self) -> Result<Self, String> {
        // Serialize the cache items into a JSON string.
        let content = match serde_json::to_string(&self.items) {
            Ok(content) => content,
            Err(_) => {
                error("Failed to convert cache items to string for writing to file.");
                return Err("Failed to convert cache items to string.".to_string());
            }
        };

        // Write the cache to the playlist file.
        match fs::write(&self.path, content) {
            Ok(_) => {
                info(format!("Cache successfully written to: {}", self.path)); // Log successful write
                Ok(self)
            }
            Err(e) => {
                error(format!(
                    "Error writing to cache file at {}: {}",
                    self.path, e
                )); // Log error during write
                Err("Failed to write cache to file.".to_string())
            }
        }
    }
}
