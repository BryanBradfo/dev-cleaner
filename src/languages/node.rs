use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};

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
        // TODO: Implement by running `npm list -g --depth=0`
        None
    }
}
