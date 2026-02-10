use crate::scanner::FoundItem;
use bytesize::ByteSize;
use colored::Colorize;

/// Format byte size to human-readable string
pub fn format_size(bytes: u64) -> String {
    ByteSize::b(bytes).to_string()
}

/// Get color for ecosystem type
fn get_ecosystem_color(ecosystem: &str) -> colored::Color {
    match ecosystem {
        "Node.js" | "Next.js" | "JavaScript" | "Parcel" | "Bower" => colored::Color::Yellow,
        "Python" | "Python venv" | "Python tox" | "Python pytest" | "Python mypy" => {
            colored::Color::Blue
        }
        "Rust" => colored::Color::Red,
        "Gradle" | "CMake" => colored::Color::Green,
        _ => colored::Color::White,
    }
}

/// Display scan results in a formatted table
pub fn display_scan_results(items: &[FoundItem]) {
    println!("{}", "Found dev dependency folders:".bold());
    println!("{}", "━".repeat(80).dimmed());

    let mut total_size: u64 = 0;

    for (idx, item) in items.iter().enumerate() {
        let ecosystem_display = format!("{} {}", item.icon, item.ecosystem);
        let ecosystem = ecosystem_display
            .color(get_ecosystem_color(&item.ecosystem))
            .bold();
        let size = format_size(item.size).cyan().bold();
        let path = item.path.display().to_string().dimmed();

        println!("{:3}. {} {} - {}", idx + 1, ecosystem, size, path);

        total_size += item.size;
    }

    println!("{}", "━".repeat(80).dimmed());
    println!(
        "{}  {} directories found",
        "📊".bold(),
        items.len().to_string().bold()
    );
    println!(
        "{}  Total size: {}",
        "💾".bold(),
        format_size(total_size).bold().green()
    );
}
