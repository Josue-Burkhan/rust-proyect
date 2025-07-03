use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};


mod cli;
mod model;
mod parser;

// Import the necessary items from our modules into the current scope.
use cli::{Cli, Commands};
use parser::parse_line;

fn main() {
    // Parse the command-line arguments using the structure defined in `cli.rs`.
    let cli_args = Cli::parse();

    // Handle the subcommand provided by the user.
    match &cli_args.command {
        Commands::Analyze { file } => {
            println!("Analyzing file: {}", file);
            if let Err(e) = process_log_file(file) {
                eprintln!("Error processing file: {}", e);
            }
        }
    }
}

// This function contains the logic for processing the log file.
fn process_log_file(path: &str) -> io::Result<()> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Create a buffered reader for efficiency. It reads the file line by line
    // instead of loading the whole file into memory.
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file.
    for line in reader.lines() {
        // The `line` is a `Result`, so we handle potential errors.
        match line {
            Ok(content) => {
                // If the line is read successfully, parse it.
                let log_entry = parse_line(&content);
                // Print the structured log entry to the console.
                // This will use the `Display` implementation we wrote in `model.rs`.
                println!("{}", log_entry);
            }
            Err(e) => {
                eprintln!("Error reading a line: {}", e);
            }
        }
    }

    Ok(())
}
