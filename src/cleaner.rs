use crate::scanner::FoundItem;
use crate::utils;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

/// Delete a directory with all its contents
fn delete_directory(path: &std::path::Path) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path)
}

/// Display directories and allow interactive selection for deletion
pub fn clean_directories(items: Vec<FoundItem>, all: bool) {
    if items.is_empty() {
        println!("✨ No dev dependency folders found!");
        return;
    }

    // Display summary
    utils::display_scan_results(&items);
    println!();

    let selections = if all {
        // Select all items
        (0..items.len()).collect::<Vec<_>>()
    } else {
        // Interactive selection
        let item_labels: Vec<String> = items
            .iter()
            .map(|item| {
                format!(
                    "{} - {} ({})",
                    item.path.display(),
                    item.ecosystem,
                    utils::format_size(item.size)
                )
            })
            .collect();

        match MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select directories to delete (use Space to select, Enter to confirm)")
            .items(&item_labels)
            .interact()
        {
            Ok(selections) => selections,
            Err(_) => {
                println!("❌ Selection cancelled");
                return;
            }
        }
    };

    if selections.is_empty() {
        println!("✨ No directories selected for deletion");
        return;
    }

    // Calculate total space to reclaim
    let total_size: u64 = selections.iter().map(|&i| items[i].size).sum();

    println!(
        "\n💾 Total space to be reclaimed: {}",
        utils::format_size(total_size).bold().green()
    );

    // Final confirmation
    if !all {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Are you sure you want to delete {} directories?",
                selections.len()
            ))
            .default(false)
            .interact();

        match confirm {
            Ok(true) => {}
            _ => {
                println!("❌ Deletion cancelled");
                return;
            }
        }
    }

    // Delete selected directories
    println!("\n🗑️  Deleting directories...\n");

    let pb = ProgressBar::new(selections.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut success_count = 0;
    let mut error_count = 0;

    for &idx in &selections {
        let item = &items[idx];
        pb.set_message(format!("Deleting {}", item.path.display()));

        match delete_directory(&item.path) {
            Ok(_) => {
                success_count += 1;
            }
            Err(e) => {
                error_count += 1;
                pb.println(format!(
                    "❌ Failed to delete {}: {}",
                    item.path.display(),
                    e
                ));
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("Done!");

    println!("\n✅ Successfully deleted {} directories", success_count);
    if error_count > 0 {
        println!("⚠️  Failed to delete {} directories", error_count);
    }
    println!(
        "💾 Reclaimed approximately {}",
        utils::format_size(total_size).bold().green()
    );
}
