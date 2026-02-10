use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};

pub struct JavaCleaner;

impl LanguageCleaner for JavaCleaner {
    fn name(&self) -> &str {
        "Java"
    }

    fn icon(&self) -> &str {
        "☕"
    }

    fn project_patterns(&self) -> Vec<DetectionPattern> {
        vec![
            DetectionPattern::DirectoryName(".gradle".to_string()),
            DetectionPattern::DirectoryWithSibling {
                dir_name: "build".to_string(),
                sibling: "build.gradle".to_string(),
            },
        ]
    }

    fn global_cache_paths(&self) -> Vec<GlobalCachePath> {
        let mut paths = Vec::new();

        if let Some(home) = dirs::home_dir() {
            // ~/.m2/repository (Maven)
            let m2_repo = home.join(".m2").join("repository");
            if m2_repo.exists() {
                paths.push(GlobalCachePath {
                    path: m2_repo,
                    description: "Maven repository".to_string(),
                });
            }

            // ~/.gradle/caches
            let gradle_caches = home.join(".gradle").join("caches");
            if gradle_caches.exists() {
                paths.push(GlobalCachePath {
                    path: gradle_caches,
                    description: "Gradle caches".to_string(),
                });
            }

            // ~/.gradle/wrapper/dists
            let gradle_wrapper = home.join(".gradle").join("wrapper").join("dists");
            if gradle_wrapper.exists() {
                paths.push(GlobalCachePath {
                    path: gradle_wrapper,
                    description: "Gradle wrapper distributions".to_string(),
                });
            }
        }

        paths
    }

    fn detect_orphaned_packages(&self) -> Option<Vec<OrphanedPackage>> {
        // TODO: Implement Gradle wrapper version detection
        None
    }
}
