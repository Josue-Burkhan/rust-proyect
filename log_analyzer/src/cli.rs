// File: src/cli.rs

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "A universal log analyzer built in Rust")]
pub struct Cli {
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {

    Analyze {
        #[arg(short, long)]
        file: String,
    }
}