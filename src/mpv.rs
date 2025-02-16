use crate::log::{error, info};
use std::collections::HashMap;
use std::process::Command;

/// Type alias for a collection of MPV command-line arguments.
pub type MpvArgs = HashMap<String, Option<String>>;

/// Represents an MPV instance with media and associated arguments.
pub struct Mpv {
    audio: String,         // The media URL or file path
    args: Option<MpvArgs>, // Optional MPV arguments
}

impl Mpv {
    /// Creates a new `Mpv` instance.
    ///
    /// # Parameters
    /// - `audio`: The media URL or file path to be played.
    /// - `args`: Optional MPV arguments (e.g., video settings).
    ///
    /// # Returns
    /// A new `Mpv` instance.
    pub fn new(audio: String, args: Option<MpvArgs>) -> Self {
        Mpv { audio, args }
    }

    /// Spawns the MPV player with the specified audio and arguments.
    ///
    /// # Returns
    /// - `u32`: The process ID of the spawned MPV process, or `0` if an error occurred.
    pub fn spawn(&self) -> u32 {
        let mut command = Command::new("mpv");

        // Add the audio file or URL to the MPV command
        command.arg(&self.audio);

        // Add optional arguments if provided
        if let Some(args) = &self.args {
            for (key, value) in args.iter() {
                let mut arg: String = String::from(key);
                if let Some(val) = value {
                    arg.push_str("=");
                    arg.push_str(val);
                }
                command.arg(&arg);
            }
        }

        // Execute the command and handle errors if any
        match command.spawn() {
            Ok(child) => {
                // Log the spawn details for debugging purposes
                log_spawn_details(&self.audio, &self.args);
                child.id()
            }
            Err(e) => {
                error(&format!(
                    "An error occurred while spawning the MPV command: {}",
                    e
                ));
                0
            }
        }
    }
}

/// Logs the details of the spawned MPV process for debugging purposes.
fn log_spawn_details(audio: &str, args: &Option<MpvArgs>) {
    let args_str = match args {
        Some(arguments) => format!("{:?}", arguments),
        None => String::from("No arguments provided"),
    };
    info(&format!(
        "mpv spawn details:\n  Audio: {}\n  Arguments: {}",
        audio, args_str
    ));
}
