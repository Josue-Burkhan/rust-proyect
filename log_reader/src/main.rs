use iced::widget::{column, container, scrollable, text, Text};
use iced::{executor, Application, Color, Command, Element, Settings, Theme};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;

lazy_static! {
    // This regex finds common HTTP methods followed by a status code of 4xx or 5xx
    static ref HTTP_ERROR_REGEX: Regex =
        Regex::new(r"\s(GET|POST|PUT|DELETE|HEAD)\s.*\s(4\d{2}|5\d{2})\s").unwrap();
}

// --- Defines the importance of a log message ---
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Critical,
    Error,
    HttpError, // New level for HTTP errors
    Warning,
}

impl LogLevel {
    // --- Determines the log level from a line of text (IMPROVED) ---
    fn from_line(line: &str) -> Option<Self> {
        let upper_line = line.to_uppercase();

        // Check in order of severity
        if upper_line.contains("CRITICAL") {
            Some(LogLevel::Critical)
        } else if upper_line.contains("ERROR") || upper_line.contains("FAIL") {
            Some(LogLevel::Error)
        } else if HTTP_ERROR_REGEX.is_match(line) {
            Some(LogLevel::HttpError)
        } else if upper_line.contains("WARN") || upper_line.contains("WARNING") {
            Some(LogLevel::Warning)
        } else {
            None
        }
    }

    // --- Here assigns a color to each log level ---
    fn to_color(&self) -> Color {
        match self {
            LogLevel::Critical => Color::from([1.0, 0.0, 0.5]),
            LogLevel::Error => Color::from([1.0, 0.4, 0.4]),
            LogLevel::HttpError => Color::from([0.0, 0.7, 1.0]),
            LogLevel::Warning => Color::from([1.0, 1.0, 0.4]),
        }
    }
}

// --- A structured representation of a single log line ---
#[derive(Debug, Clone, Eq, PartialEq)]
struct LogEntry {
    level: LogLevel,
    message: String,
}

// --- Allows sorting of LogEntry based on level, then message ---
impl Ord for LogEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level).then_with(|| self.message.cmp(&other.message))
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn main() -> iced::Result {
    LogViewer::run(Settings::default())
}

// --- Application State ---
struct LogViewer {
    logs: Vec<LogEntry>,
}

// --- Messages for events ---
#[derive(Debug, Clone)]
enum Message {
    LogsLoaded(Vec<LogEntry>),
}

impl Application for LogViewer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    // --- Starts the application and loads the logs ---
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            LogViewer { logs: vec![] },
            Command::perform(load_logs(), Message::LogsLoaded),
        )
    }

    // --- Window Title ---
    fn title(&self) -> String {
        String::from("Log Viewer")
    }

    // --- Updates state when messages arrive ---
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LogsLoaded(loaded_logs) => {
                self.logs = loaded_logs;
            }
        }
        Command::none()
    }

    // --- Draws the GUI ---
    fn view(&self) -> Element<Message> {
        let log_list = if self.logs.is_empty() {
            column![text("No important logs found in the 'logs' folder.")]
        } else {
            self.logs.iter().fold(column![].spacing(5), |col, log_entry| {
                let styled_text =
                    Text::new(&log_entry.message).style(log_entry.level.to_color());
                col.push(styled_text)
            })
        };

        let scroll = scrollable(log_list);

        container(scroll)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .into()
    }

    // --- Set the application theme ---
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

// --- Loads, filters, and sorts logs from the 'logs' folder ---
async fn load_logs() -> Vec<LogEntry> {
    let mut important_logs = Vec::new();
    let log_dir = Path::new("logs");

    if let Ok(entries) = fs::read_dir(log_dir) {
        for entry in entries.flatten() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                for line in content.lines() {
                    if let Some(level) = LogLevel::from_line(line) {
                        important_logs.push(LogEntry {
                            level,
                            message: line.to_string(),
                        });
                    }
                }
            }
        }
    }

    important_logs.sort();
    important_logs
}