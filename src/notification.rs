use crate::log::{error, info, warning};
use std::process::Command;

/// Sends a desktop notification with the provided message.
///
/// # Parameters
/// - `body`: The message to be displayed in the notification.
///
/// This function triggers the 'notify-send' command to display a notification and logs its status.
/// Note that this function depends on `libnotify` library.
/// TODO: Using an crate instead of running an child process will be more good
pub fn send_notification(body: &str) {
    // Create a new Command to run the 'notify-send' program with the provided message
    let mut cmd = Command::new("notify-send");
    cmd.arg(body);

    // Attempt to execute the command and handle success or failure
    match cmd.output() {
        Ok(_) => {
            info("Notification displayed successfully");
        }
        Err(e) => {
            error("Failed to display notification");
            error(&e);
            warning("The notification couldn't be shown, but the audio will still play");
        }
    }
}
