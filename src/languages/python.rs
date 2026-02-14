use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};
use std::process::Command;

pub struct PythonCleaner;

impl LanguageCleaner for PythonCleaner {
    fn name(&self) -> &str {
        "Python"
    }

    fn icon(&self) -> &str {
        "🐍"
    }

    fn project_patterns(&self) -> Vec<DetectionPattern> {
        vec![
            DetectionPattern::DirectoryName("__pycache__".to_string()),
            DetectionPattern::DirectoryName(".venv".to_string()),
            DetectionPattern::DirectoryName("venv".to_string()),
            DetectionPattern::DirectoryName("env".to_string()),
            DetectionPattern::DirectoryName(".tox".to_string()),
            DetectionPattern::DirectoryName(".pytest_cache".to_string()),
            DetectionPattern::DirectoryName(".mypy_cache".to_string()),
            DetectionPattern::DirectoryName(".ruff_cache".to_string()),
            DetectionPattern::GlobPattern("*.egg-info".to_string()),
            DetectionPattern::DirectoryWithSibling {
                dir_name: "dist".to_string(),
                sibling: "setup.py".to_string(),
            },
        ]
    }

    fn global_cache_paths(&self) -> Vec<GlobalCachePath> {
        let mut paths = Vec::new();

        // Linux/macOS: ~/.cache/pip
        if let Some(home) = dirs::home_dir() {
            let pip_cache = home.join(".cache").join("pip");
            if pip_cache.exists() {
                paths.push(GlobalCachePath {
                    path: pip_cache,
                    description: "pip cache".to_string(),
                });
            }

            // Linux: ~/.local/lib/pythonX.X
            let local_lib = home.join(".local").join("lib");
            if local_lib.exists() {
                paths.push(GlobalCachePath {
                    path: local_lib,
                    description: "Python user site-packages".to_string(),
                });
            }
        }

        // Windows: %LOCALAPPDATA%\pip\Cache
        #[cfg(target_os = "windows")]
        if let Some(local_appdata) = dirs::data_local_dir() {
            let pip_cache = local_appdata.join("pip").join("Cache");
            if pip_cache.exists() {
                paths.push(GlobalCachePath {
                    path: pip_cache,
                    description: "pip cache (Windows)".to_string(),
                });
            }
        }

        paths
    }

    fn detect_orphaned_packages(&self) -> Option<Vec<OrphanedPackage>> {
        // Run `pip list` to get globally installed packages
        let output = Command::new("pip")
            .args(["list", "--format=freeze"])
            .output();

        let stdout = if let Ok(output) = output {
            if !output.status.success() {
                // Try pip3 if pip failed
                let output3 = Command::new("pip3")
                    .args(["list", "--format=freeze"])
                    .output();
                
                let Ok(output3) = output3 else {
                    return None;
                };
                
                if !output3.status.success() {
                    return None;
                }
                
                String::from_utf8_lossy(&output3.stdout).to_string()
            } else {
                String::from_utf8_lossy(&output.stdout).to_string()
            }
        } else {
            // pip command not found - try pip3
            let output3 = Command::new("pip3")
                .args(["list", "--format=freeze"])
                .output();
            
            let Ok(output3) = output3 else {
                return None;
            };
            
            if !output3.status.success() {
                return None;
            }
            
            String::from_utf8_lossy(&output3.stdout).to_string()
        };

        // Parse pip output - format is: package-name==version
        let mut packages = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            
            if line.is_empty() {
                continue;
            }

            // Split on == to get package name
            if let Some((name, _version)) = line.split_once("==") {
                packages.push(OrphanedPackage {
                    name: name.trim().to_string(),
                    size: 0, // Size calculation would require inspecting site-packages
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

