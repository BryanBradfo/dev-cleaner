use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};
use std::process::Command;

pub struct RustCleaner;

impl LanguageCleaner for RustCleaner {
    fn name(&self) -> &str {
        "Rust"
    }

    fn icon(&self) -> &str {
        "🦀"
    }

    fn project_patterns(&self) -> Vec<DetectionPattern> {
        vec![DetectionPattern::DirectoryWithSibling {
            dir_name: "target".to_string(),
            sibling: "Cargo.toml".to_string(),
        }]
    }

    fn global_cache_paths(&self) -> Vec<GlobalCachePath> {
        let mut paths = Vec::new();

        if let Some(home) = dirs::home_dir() {
            // ~/.cargo/registry
            let cargo_registry = home.join(".cargo").join("registry");
            if cargo_registry.exists() {
                paths.push(GlobalCachePath {
                    path: cargo_registry,
                    description: "Cargo registry cache".to_string(),
                });
            }

            // ~/.cargo/git
            let cargo_git = home.join(".cargo").join("git");
            if cargo_git.exists() {
                paths.push(GlobalCachePath {
                    path: cargo_git,
                    description: "Cargo git cache".to_string(),
                });
            }

            // ~/.rustup/toolchains
            let rustup_toolchains = home.join(".rustup").join("toolchains");
            if rustup_toolchains.exists() {
                paths.push(GlobalCachePath {
                    path: rustup_toolchains,
                    description: "Rustup toolchains".to_string(),
                });
            }
        }

        paths
    }

    fn detect_orphaned_packages(&self) -> Option<Vec<OrphanedPackage>> {
        // Run `cargo install --list` to get globally installed packages
        let output = Command::new("cargo").args(["install", "--list"]).output();

        let Ok(output) = output else {
            // cargo command not found or failed
            return None;
        };

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut packages = Vec::new();

        // Parse cargo output - format is:
        // package-name v0.1.0:
        //     binary1
        //     binary2
        // another-package v0.2.0 (path):
        //     binary3
        for line in stdout.lines() {
            let line = line.trim();

            // Package lines don't start with whitespace and contain ' v'
            if line.starts_with(' ') || line.is_empty() {
                continue;
            }

            // Extract package name (everything before ' v')
            if let Some(v_pos) = line.find(" v") {
                let name = &line[..v_pos];
                packages.push(OrphanedPackage {
                    name: name.trim().to_string(),
                    size: 0, // Size calculation would require inspecting .cargo/bin
                    last_used: None,
                });
            }
        }

        if packages.is_empty() {
            None
        } else {
            Some(packages)
        }
    }
}
