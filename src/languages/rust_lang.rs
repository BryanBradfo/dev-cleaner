use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};

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
        // TODO: Implement by running `cargo install --list`
        None
    }
}
