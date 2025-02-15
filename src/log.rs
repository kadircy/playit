/// A macro for logging messages at various levels (info, warning, error)
///
/// # Parameters
/// - `$level`: The log level (e.g., "info", "warning", "error")
/// - `$message`: The message to be logged
/// - `$fmt`: The format string for formatted messages (for the second variant)
/// - `$($arg:tt)*`: The format arguments for the second variant
macro_rules! log {
    // Basic log message with level and message
    ($level:expr, $message:expr) => {
        println!("{}: {}", $level, $message);
    };

    // Formatted log message with level and module-specific information
    ($level:expr, $fmt:expr, $($arg:tt)*) => {
        println!("{}: {}", $level, format!($fmt, $($arg)*));
    };
}

/// Logs an informational message
///
/// # Parameters
/// - `message`: The message to log at the "info" level
pub fn info<T: std::fmt::Display>(message: T) {
    log!("info", message);
}

/// Logs a warning message
///
/// # Parameters
/// - `message`: The message to log at the "warning" level
pub fn warning<T: std::fmt::Display>(message: T) {
    log!("warning", message);
}

/// Logs an error message
///
/// # Parameters
/// - `message`: The message to log at the "error" level
pub fn error<T: std::fmt::Display>(message: T) {
    log!("error", message);
}
