use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};
use std::fs;

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
            DetectionPattern::DirectoryWithSibling {
                dir_name: "build".to_string(),
                sibling: "build.gradle.kts".to_string(),
            },
            DetectionPattern::DirectoryWithSibling {
                dir_name: "build".to_string(),
                sibling: "pom.xml".to_string(),
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
        // Detect multiple Gradle wrapper versions installed
        if let Some(home) = dirs::home_dir() {
            let gradle_wrapper = home.join(".gradle").join("wrapper").join("dists");

            if !gradle_wrapper.exists() {
                return None;
            }

            let mut versions = Vec::new();

            // Read the wrapper distributions directory
            if let Ok(entries) = fs::read_dir(&gradle_wrapper) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if let Some(name) = entry.file_name().to_str() {
                                // Gradle wrapper dirs are typically named like "gradle-7.6-bin"
                                if name.starts_with("gradle-") {
                                    versions.push(OrphanedPackage {
                                        name: name.to_string(),
                                        size: 0, // Could calculate by walking the directory
                                        last_used: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            if versions.is_empty() {
                None
            } else {
                Some(versions)
            }
        } else {
            None
        }
    }
}
