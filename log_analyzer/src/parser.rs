use crate::model::{LogEntry, LogLevel};
use regex::Regex;

// A simple parsing function. It takes a string slice (a line from a file)
// and returns a `LogEntry`. This is the heart of the "Módulo de Origen".
pub fn parse_line(line: &str) -> LogEntry {
    // We use `lazy_static` or `once_cell` in real apps to compile regex only once.
    // For this first version, compiling it every time is fine.
    let re_error = Regex::new(r"(?i)error|fail|failed").unwrap();
    let re_warn = Regex::new(r"(?i)warn|warning").unwrap();
    let re_debug = Regex::new(r"(?i)debug").unwrap();

    let level = if re_error.is_match(line) {
        LogLevel::Error
    } else if re_warn.is_match(line) {
        LogLevel::Warning
    } else if re_debug.is_match(line) {
        LogLevel::Debug
    } else {
        LogLevel::Info // Default level if no keyword is found
    };

    // We create a new LogEntry with the determined level.
    // The source is hardcoded for now.
    LogEntry::new(level, "GenericFile", line)
}
