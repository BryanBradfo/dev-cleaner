use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FoundItem {
    pub path: PathBuf,
    pub ecosystem: String,
    pub size: u64,
}

/// Calculate directory size recursively
fn calculate_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| fs::metadata(entry.path()).ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}

/// Check if a file exists in the same directory
fn has_sibling_file(dir: &Path, filename: &str) -> bool {
    if let Some(parent) = dir.parent() {
        parent.join(filename).exists()
    } else {
        false
    }
}

/// Check if directory is a Python virtual environment
fn is_python_venv(dir: &Path) -> bool {
    dir.join("pyvenv.cfg").exists()
}

/// Check if this directory should be scanned
fn should_scan(dir_name: &str, full_path: &Path) -> Option<String> {
    match dir_name {
        "node_modules" => Some("Node.js".to_string()),
        "__pycache__" => Some("Python".to_string()),
        ".venv" | "venv" | "env" | ".env" => {
            if is_python_venv(full_path) {
                Some("Python venv".to_string())
            } else {
                None
            }
        }
        "target" => {
            if has_sibling_file(full_path, "Cargo.toml") {
                Some("Rust".to_string())
            } else {
                None
            }
        }
        ".gradle" => Some("Gradle".to_string()),
        "build" => {
            if has_sibling_file(full_path, "build.gradle")
                || has_sibling_file(full_path, "build.gradle.kts")
            {
                Some("Gradle".to_string())
            } else if has_sibling_file(full_path, "CMakeLists.txt") {
                Some("CMake".to_string())
            } else {
                None
            }
        }
        ".tox" => Some("Python tox".to_string()),
        ".pytest_cache" => Some("Python pytest".to_string()),
        ".mypy_cache" => Some("Python mypy".to_string()),
        ".next" => Some("Next.js".to_string()),
        "dist" => {
            if has_sibling_file(full_path, "package.json") {
                Some("JavaScript".to_string())
            } else {
                None
            }
        }
        ".parcel-cache" => Some("Parcel".to_string()),
        "bower_components" => Some("Bower".to_string()),
        _ => None,
    }
}

/// Scan directory recursively for dev dependencies
pub fn scan_directory(root: &Path) -> Vec<FoundItem> {
    let mut found_items = Vec::new();

    // We'll use WalkDir's min_depth to control traversal depth
    // and manually skip directories we've already identified as dev dependencies
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(ecosystem) = should_scan(dir_name, path) {
                // Check if this path is already inside a found item
                let is_nested = found_items
                    .iter()
                    .any(|item: &FoundItem| path.starts_with(&item.path));

                if !is_nested {
                    // Calculate size
                    let size = calculate_dir_size(path);

                    if size > 0 {
                        found_items.push(FoundItem {
                            path: path.to_path_buf(),
                            ecosystem,
                            size,
                        });
                    }
                }
            }
        }
    }

    // Sort by size descending
    found_items.sort_by(|a, b| b.size.cmp(&a.size));

    found_items
}
