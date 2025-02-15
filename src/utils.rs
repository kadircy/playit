/// Checks if a given text is a valid URL (starts with "http://" or "https://")
///
/// # Parameters
/// - `text`: The string to check
///
/// # Returns
/// - `true` if the text starts with "http://" or "https://", otherwise `false`
pub fn is_url(text: &str) -> bool {
    text.starts_with("http://") || text.starts_with("https://")
}
