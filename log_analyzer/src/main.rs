#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;
mod parser;

use parser::parse_line;
use std::fs::File;
use std::io::{self, BufRead};

#[tauri::command]
fn analyze_log_file(path: String) -> Result<Vec<String>, String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);
    let mut parsed_entries = Vec::new();

    for line in reader.lines() {
        let content = line.map_err(|e| e.to_string())?;
        let log_entry = parse_line(&content);
        parsed_entries.push(log_entry.to_string());
    }

    Ok(parsed_entries)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze_log_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
