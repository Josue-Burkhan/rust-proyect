use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fmt;
use colored::*;

// Enum to represent the severity level of a log message.
// Using an enum makes our code safer and more readable than using simple strings.
// `derive(Debug)` allows us to easily print this enum for debugging purposes.
#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
    Debug,
}

// We implement the `Display` trait to define how `LogLevel` should be
// converted to a string for printing. This is where we add colors.
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "{}", "INFO".cyan()),
            LogLevel::Warning => write!(f, "{}", "WARN".yellow()),
            LogLevel::Error => write!(f, "{}", "ERROR".red()),
            LogLevel::Critical => write!(f, "{}", "CRITICAL".bright_red().bold()),
            LogLevel::Debug => write!(f, "{}", "DEBUG".purple()),
        }
    }
}

// The standardized structure for a single log entry.
// All parsers will aim to produce this structure.
#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

// Implementation block for LogEntry.
impl LogEntry {
    // A constructor function to create a new LogEntry.
    // We'll make this more robust later.
    pub fn new(level: LogLevel, source: &str, message: &str) -> Self {
        Self {
            timestamp: Utc::now(), // For now, we just use the current time.
            level,
            source: source.to_string(),
            message: message.to_string(),
            metadata: HashMap::new(),
        }
    }
}

// We also implement `Display` for `LogEntry` to define the standard
// way a log entry is printed to the console.
impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] [{}] [{}] - {}",
            self.timestamp.to_rfc2822().dimmed(),
            self.source.green(),
            self.level, // This will use the Display impl for LogLevel
            self.message
        )
    }
}