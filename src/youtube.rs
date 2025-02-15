use std::process::Command;

/// Searches for a media URL using `yt-dlp` based on a given query
///
/// # Parameters
/// - `query`: The search query (e.g., song name or video title)
///
/// # Returns
/// - `Ok(String)`: The media URL if found successfully
/// - `Err(String)`: An error message if the search fails
pub fn search(query: &str) -> Result<String, String> {
    // Run the `yt-dlp` command with the provided search query
    let command = Command::new("yt-dlp")
        .arg("--no-playlist") // Avoid playlist downloads
        .arg("--quiet") // Suppress unnecessary output
        .arg("--simulate") // Simulate the download process (no actual download)
        .arg("--print") // Print the output (in this case, the URL)
        .arg("\"%(webpage_url)s\"") // Format to print the webpage URL
        .arg(format!("ytsearch:{}", query)) // Construct the search query for yt-dlp
        .output(); // Capture the output of the command

    // Check if the command failed to start
    if command.is_err() {
        return Err("Unable to create process for searching URL with 'yt-dlp'".to_string());
    }

    // Convert the command's stdout (output) from bytes to a String
    let stdout = String::from_utf8(command.unwrap().stdout);

    // Check if the conversion failed
    if stdout.is_err() {
        return Err("Unable to convert u8 bytes to string".to_string());
    }

    // Return the URL after removing any extra quotes
    Ok(stdout.unwrap().replace("\"", ""))
}
