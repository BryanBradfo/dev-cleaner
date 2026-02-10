use super::{DetectionPattern, GlobalCachePath, LanguageCleaner, OrphanedPackage};

pub struct CppCleaner;

impl LanguageCleaner for CppCleaner {
    fn name(&self) -> &str {
        "C++"
    }

    fn icon(&self) -> &str {
        "⚙️"
    }

    fn project_patterns(&self) -> Vec<DetectionPattern> {
        vec![
            DetectionPattern::DirectoryWithSibling {
                dir_name: "build".to_string(),
                sibling: "CMakeLists.txt".to_string(),
            },
            DetectionPattern::DirectoryWithSibling {
                dir_name: "build".to_string(),
                sibling: "Makefile".to_string(),
            },
            DetectionPattern::GlobPattern("cmake-build-*".to_string()),
            DetectionPattern::DirectoryName("out".to_string()),
        ]
    }

    fn global_cache_paths(&self) -> Vec<GlobalCachePath> {
        let mut paths = Vec::new();

        if let Some(home) = dirs::home_dir() {
            // ~/.conan/data (Conan package manager)
            let conan_data = home.join(".conan").join("data");
            if conan_data.exists() {
                paths.push(GlobalCachePath {
                    path: conan_data,
                    description: "Conan package cache".to_string(),
                });
            }

            // vcpkg is typically installed in a user-chosen location
            // We'll check common locations but this is less standardized
            let vcpkg_installed = home.join("vcpkg").join("installed");
            if vcpkg_installed.exists() {
                paths.push(GlobalCachePath {
                    path: vcpkg_installed,
                    description: "vcpkg installed packages".to_string(),
                });
            }

            let vcpkg_buildtrees = home.join("vcpkg").join("buildtrees");
            if vcpkg_buildtrees.exists() {
                paths.push(GlobalCachePath {
                    path: vcpkg_buildtrees,
                    description: "vcpkg build trees".to_string(),
                });
            }
        }

        paths
    }

    fn detect_orphaned_packages(&self) -> Option<Vec<OrphanedPackage>> {
        None
    }
}
