use crate::log::error;
use std::collections::HashMap;
use std::process::Command;

/// Type alias for a collection of MPV command-line arguments
pub type MpvArgs = HashMap<String, Option<String>>;

/// Represents an MPV instance with media and associated arguments
pub struct Mpv {
    audio: String,         // The media URL or file path
    args: Option<MpvArgs>, // Optional MPV arguments
}

impl Mpv {
    /// Creates a new `Mpv` instance
    ///
    /// # Parameters
    /// - `audio`: The media URL or file path to be played
    /// - `args`: Optional MPV arguments (e.g., video settings)
    ///
    /// # Returns
    /// A new `Mpv` instance
    pub fn new(audio: String, args: Option<MpvArgs>) -> Self {
        Mpv { audio, args }
    }

    /// Spawns the MPV player with the specified audio and arguments
    pub fn spawn(&self) -> u32 {
        let mut command = Command::new("mpv");

        // Add the audio file or URL to the MPV command
        command.arg(self.audio.clone());

        // Add optional arguments if provided
        if let Some(args) = &self.args {
            for (key, value) in args.iter() {
                command.arg(key);
                if value.is_some() {
                    command.arg(value.as_ref().unwrap());
                }
            }
        }

        // Execute the command and handle errors if any
        match command.spawn() {
            Ok(child) => {
                return child.id();
            }
            Err(e) => {
                error("An error occurred while spawning the MPV command");
                error(e); // Log the error details
                return 0;
            }
        };
    }
}
