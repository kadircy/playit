use std::process::Command;

/// Searches for a media URL using `yt-dlp` based on a given query.
///
/// # Parameters
/// - `query`: The search query (e.g., song name or video title).
///
/// # Returns
/// - `Ok(String)`: The media URL if found successfully.
/// - `Err(String)`: An error message if the search fails.
pub fn search(query: &str) -> Result<String, String> {
    // Run the `yt-dlp` command with the provided search query
    let output = Command::new("yt-dlp")
        .arg("--no-playlist") // Avoid playlist downloads
        .arg("--quiet") // Suppress unnecessary output
        .arg("--simulate") // Simulate the download process (no actual download)
        .arg("--print") // Print the output (in this case, the URL)
        .arg("\"%(webpage_url)s\"") // Format to print the webpage URL
        .arg(format!("ytsearch:{}", query)) // Construct the search query for yt-dlp
        .output(); // Capture the output of the command

    // Handle potential errors in running the command
    let output = output
        .map_err(|_| "Unable to create process for searching URL with 'yt-dlp'".to_string())?;

    // Convert the command's stdout (output) from bytes to a String
    let stdout = String::from_utf8(output.stdout)
        .map_err(|_| "Unable to convert u8 bytes to string".to_string())?;

    // Return the URL after removing any extra quotes and newlines
    Ok(stdout.replace("\"", "").replace("\n", ""))
}
