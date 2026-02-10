mod cleaner;
mod scanner;
mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dev-cleaner")]
#[command(about = "A blazing-fast CLI tool to scan and clean unused dev dependencies", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan for dev dependency folders
    Scan {
        /// Root directory to scan (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Clean dev dependency folders interactively
    Clean {
        /// Root directory to scan (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Clean everything without confirmation
        #[arg(short, long, default_value_t = false)]
        all: bool,

        /// Dry run - show what would be deleted without deleting
        #[arg(short, long, default_value_t = false)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            println!("🔍 Scanning {} for dev dependencies...\n", path.display());

            let items = scanner::scan_directory(&path);

            if items.is_empty() {
                println!("✨ No dev dependency folders found!");
                return;
            }

            utils::display_scan_results(&items);
        }
        Commands::Clean { path, all, dry_run } => {
            println!("🔍 Scanning {} for dev dependencies...\n", path.display());

            let items = scanner::scan_directory(&path);

            if items.is_empty() {
                println!("✨ No dev dependency folders found!");
                return;
            }

            if dry_run {
                println!("🔍 DRY RUN - Nothing will be deleted\n");
                utils::display_scan_results(&items);
                let total_size: u64 = items.iter().map(|item| item.size).sum();
                println!(
                    "\n💾 Total space that would be reclaimed: {}",
                    utils::format_size(total_size)
                );
            } else {
                cleaner::clean_directories(items, all);
            }
        }
    }
}
