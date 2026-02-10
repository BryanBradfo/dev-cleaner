use std::path::PathBuf;

pub mod cpp;
pub mod java;
pub mod node;
pub mod python;
pub mod rust_lang;

#[derive(Debug, Clone)]
pub enum DetectionPattern {
    /// Match exact directory name
    DirectoryName(String),
    /// Match directory name with a required sibling file
    DirectoryWithSibling { dir_name: String, sibling: String },
    /// Match glob pattern for directories (e.g., "*.egg-info")
    GlobPattern(String),
}

#[derive(Debug, Clone)]
pub struct GlobalCachePath {
    pub path: PathBuf,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct OrphanedPackage {
    pub name: String,
    pub size: u64,
    pub last_used: Option<String>,
}

pub trait LanguageCleaner: Send + Sync {
    /// Display name for the ecosystem (e.g., "Python", "Node.js")
    fn name(&self) -> &str;

    /// Emoji/icon for display
    fn icon(&self) -> &str;

    /// Return a list of directory names/patterns this module detects in project directories
    fn project_patterns(&self) -> Vec<DetectionPattern>;

    /// Return a list of global cache locations to scan
    fn global_cache_paths(&self) -> Vec<GlobalCachePath>;

    /// Optional: Detect orphaned global packages
    fn detect_orphaned_packages(&self) -> Option<Vec<OrphanedPackage>> {
        None
    }
}

/// Get all registered language cleaners
pub fn get_all_cleaners() -> Vec<Box<dyn LanguageCleaner>> {
    vec![
        Box::new(python::PythonCleaner),
        Box::new(node::NodeCleaner),
        Box::new(rust_lang::RustCleaner),
        Box::new(java::JavaCleaner),
        Box::new(cpp::CppCleaner),
    ]
}

/// Check if a language name matches an alias
fn is_language_alias(input: &str, cleaner_name: &str) -> bool {
    // Special case mappings for common aliases
    match (input, cleaner_name) {
        ("cpp", "c++") => true,
        _ => false,
    }
}

/// Get a specific language cleaner by name (case-insensitive)
pub fn get_cleaner_by_name(name: &str) -> Option<Box<dyn LanguageCleaner>> {
    let name_lower = name.to_lowercase();
    get_all_cleaners()
        .into_iter()
        .find(|cleaner| {
            let cleaner_name_lower = cleaner.name().to_lowercase();
            // Direct match
            if cleaner_name_lower == name_lower {
                return true;
            }
            // Check for special aliases
            if is_language_alias(&name_lower, &cleaner_name_lower) {
                return true;
            }
            // Normalized match (remove dots, hyphens for node.js, etc.)
            let name_normalized = name_lower.replace(".", "").replace("-", "");
            let cleaner_normalized = cleaner_name_lower.replace(".", "").replace("-", "");
            if cleaner_normalized == name_normalized {
                return true;
            }
            // Starts with match for normalized version
            cleaner_normalized.starts_with(&name_normalized)
        })
}
