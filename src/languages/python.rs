use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};

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
        // TODO: Implement by running `pip list` and analyzing usage
        None
    }
}
