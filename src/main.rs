mod checks;
mod clean;
mod config;
mod lint;
mod reporter;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Validates and maintains a Markdown + YAML-frontmatter knowledge base against a declarative _types.yml config")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check frontmatter, status values, related-links, staleness, secret encryption, attachments
    Lint {
        /// Path to the knowledge directory
        #[arg(default_value = "knowledge")]
        knowledge_dir: PathBuf,

        /// Files above this size (bytes) get a warning suggesting _attachments.yml instead
        #[arg(long, default_value_t = 500_000)]
        max_file_size: u64,
    },
    /// Remove example content from every category, keep structure and _template.md
    Clean {
        /// Path to the knowledge directory
        #[arg(default_value = "knowledge")]
        knowledge_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Lint {
            knowledge_dir,
            max_file_size,
        } => lint::run_lint(&knowledge_dir, max_file_size),
        Command::Clean { knowledge_dir } => clean::run_clean(&knowledge_dir),
    }
}
