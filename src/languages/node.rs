use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};
use std::process::Command;

pub struct NodeCleaner;

impl LanguageCleaner for NodeCleaner {
    fn name(&self) -> &str {
        "Node.js"
    }

    fn icon(&self) -> &str {
        "🟢"
    }

    fn project_patterns(&self) -> Vec<DetectionPattern> {
        vec![
            DetectionPattern::DirectoryName("node_modules".to_string()),
            DetectionPattern::DirectoryName(".next".to_string()),
            DetectionPattern::DirectoryName(".nuxt".to_string()),
            DetectionPattern::DirectoryName(".parcel-cache".to_string()),
            DetectionPattern::DirectoryName("bower_components".to_string()),
            DetectionPattern::DirectoryWithSibling {
                dir_name: "dist".to_string(),
                sibling: "package.json".to_string(),
            },
        ]
    }

    fn global_cache_paths(&self) -> Vec<GlobalCachePath> {
        let mut paths = Vec::new();

        if let Some(home) = dirs::home_dir() {
            // ~/.npm/_cacache
            let npm_cache = home.join(".npm").join("_cacache");
            if npm_cache.exists() {
                paths.push(GlobalCachePath {
                    path: npm_cache,
                    description: "npm cache".to_string(),
                });
            }

            // ~/.yarn/cache
            let yarn_cache = home.join(".yarn").join("cache");
            if yarn_cache.exists() {
                paths.push(GlobalCachePath {
                    path: yarn_cache,
                    description: "Yarn cache".to_string(),
                });
            }
        }

        // Windows: %APPDATA%\npm-cache
        #[cfg(target_os = "windows")]
        if let Some(appdata) = dirs::data_dir() {
            let npm_cache = appdata.join("npm-cache");
            if npm_cache.exists() {
                paths.push(GlobalCachePath {
                    path: npm_cache,
                    description: "npm cache (Windows)".to_string(),
                });
            }
        }

        paths
    }

    fn detect_orphaned_packages(&self) -> Option<Vec<OrphanedPackage>> {
        // Run `npm list -g --depth=0` to get globally installed packages
        let output = Command::new("npm")
            .args(["list", "-g", "--depth=0"])
            .output();

        let Ok(output) = output else {
            // npm command not found or failed
            return None;
        };

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut packages = Vec::new();

        // Parse npm output - format is typically:
        // /usr/local/lib
        // ├── package1@version
        // ├── package2@version
        // └── package3@version
        for line in stdout.lines() {
            let line = line.trim();
            
            // Skip empty lines and the header line (path to global node_modules)
            if line.is_empty() || !line.contains('@') {
                continue;
            }

            // Remove tree characters (├──, └──, │) and whitespace
            let cleaned = line
                .trim_start_matches("├──")
                .trim_start_matches("└──")
                .trim_start_matches("│")
                .trim();

            // Extract package name (everything before @version)
            if let Some(at_pos) = cleaned.find('@') {
                // Handle scoped packages like @types/node@1.0.0
                let name = if cleaned.starts_with('@') {
                    // For scoped packages, find the second @
                    if let Some(second_at) = cleaned[1..].find('@') {
                        &cleaned[..second_at + 1]
                    } else {
                        continue;
                    }
                } else {
                    &cleaned[..at_pos]
                };

                packages.push(OrphanedPackage {
                    name: name.trim().to_string(),
                    size: 0, // Size calculation would require traversing node_modules
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
