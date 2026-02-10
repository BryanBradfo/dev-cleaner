# 🧹 dev-cleaner

**A blazing-fast CLI tool to scan and clean unused dev dependencies across all ecosystems.**

Built in Rust 🦀 for maximum performance.

---

## 🚀 The Problem

As developers, we accumulate **gigabytes** of dev dependencies over time:
- `node_modules` folders scattered everywhere
- Python `__pycache__`, `.venv`, `.tox` directories
- Rust `target` build folders
- Gradle `.gradle` and `build` directories
- ...and many more

**dev-cleaner** finds them all, shows you how much space they're eating, and lets you clean them up — fast.

## ✨ Features

- 🔍 **Multi-ecosystem scanning** — Node.js, Python, Rust, Java/Gradle, C++, and more
- 🎯 **Per-language filtering** — Focus on specific ecosystems with `--language` flag
- 📊 **Size analysis** — See exactly how much space each directory is wasting
- 🗑️ **Interactive cleanup** — Select what to delete with checkboxes
- ⚡ **Blazing fast** — Built in Rust for lightning-speed filesystem traversal
- 🪟 **Cross-platform** — Works on Windows, macOS, and Linux
- 🔒 **Safe by default** — Always asks for confirmation, supports dry-run mode
- 🧩 **Extensible** — Easy to add new language modules

## 📦 Installation

### From source (requires Rust toolchain)
```bash
git clone https://github.com/BryanBradfo/dev-cleaner.git
cd dev-cleaner
cargo install --path .
```

### From crates.io (coming soon)
```bash
cargo install dev-cleaner
```

## 🛠️ Usage

### Scan the current directory
```bash
dev-cleaner scan
```

### Scan a specific directory
```bash
dev-cleaner scan --path ~/projects
```

### Filter by language/ecosystem
```bash
# Only scan for Python dependencies
dev-cleaner scan --language python

# Only scan for Node.js dependencies
dev-cleaner scan --language node

# Only scan for Rust dependencies
dev-cleaner scan --language rust
```

### Clean up (interactive)
```bash
dev-cleaner clean
```

### Clean with language filter
```bash
# Only clean Python dependencies
dev-cleaner clean --language python
```

### Clean with dry-run (preview only)
```bash
dev-cleaner clean --dry-run
```

### Clean everything without confirmation
```bash
dev-cleaner clean --all
```

## 🎯 Supported Ecosystems

dev-cleaner uses a modular architecture with dedicated language modules for each ecosystem. Each module provides smart detection with context-aware scanning.

| Ecosystem | Icon | Directories Detected |
|-----------|------|---------------------|
| 🐍 Python | `python` | `__pycache__`, `.venv`, `venv`, `env`, `.tox`, `.pytest_cache`, `.mypy_cache`, `.ruff_cache`, `*.egg-info`, `dist` (with `setup.py`) |
| 🟢 Node.js | `node` | `node_modules`, `.next`, `.nuxt`, `.parcel-cache`, `dist` (with `package.json`), `bower_components`, `.cache` |
| 🦀 Rust | `rust` | `target` (with `Cargo.toml`) |
| ☕ Java | `java` | `.gradle`, `build` (with `build.gradle` or `build.gradle.kts`) |
| ⚙️ C++ | `cpp` | `build` (with `CMakeLists.txt` or `Makefile`), `cmake-build-*`, `out` |

### Context-Aware Detection

dev-cleaner is smart about what it detects:
- **Sibling file validation**: Only detects `target` when `Cargo.toml` is present, `build` when build files exist, etc.
- **No false positives**: Won't flag generic directory names without proper context
- **Glob patterns**: Supports patterns like `*.egg-info` for Python packages

## 🏗️ Architecture

dev-cleaner features a **modular, extensible architecture**:

```
src/
├── languages/          # Language-specific modules
│   ├── mod.rs         # LanguageCleaner trait definition
│   ├── python.rs      # Python ecosystem
│   ├── node.rs        # Node.js ecosystem
│   ├── rust_lang.rs   # Rust ecosystem
│   ├── java.rs        # Java/Gradle ecosystem
│   └── cpp.rs         # C/C++ ecosystem
├── scanner.rs         # Filesystem scanning engine
├── cleaner.rs         # Interactive cleanup logic
├── utils.rs           # Display & formatting utilities
└── main.rs            # CLI interface
```

### Adding a New Language

To add support for a new language:

1. Create a new file in `src/languages/` (e.g., `go.rs`)
2. Implement the `LanguageCleaner` trait
3. Register it in `src/languages/mod.rs` in the `get_all_cleaners()` function

Example:
```rust
pub struct GoCleaner;

impl LanguageCleaner for GoCleaner {
    fn name(&self) -> &str { "Go" }
    fn icon(&self) -> &str { "🐹" }
    
    fn project_patterns(&self) -> Vec<DetectionPattern> {
        vec![
            DetectionPattern::DirectoryName("vendor".to_string()),
            DetectionPattern::DirectoryWithSibling {
                dir_name: "bin".to_string(),
                sibling: "go.mod".to_string(),
            },
        ]
    }
    
    fn global_cache_paths(&self) -> Vec<GlobalCachePath> {
        // Implementation for global Go cache
        vec![]
    }
}
```

## 🗺️ Roadmap

- [x] Core scanning engine
- [x] Interactive cleanup with confirmations
- [x] Per-language cleaner modules
- [x] Language filtering (`--language` flag)
- [ ] Global cache scanning (pip, npm, cargo, etc.)
- [ ] Orphaned package detection
- [ ] Docker image cleanup integration
- [ ] Scheduled scanning (cron/Task Scheduler)
- [ ] TUI dashboard with charts
- [ ] Config file (`.dev-cleaner.toml`) for custom ignore rules
- [ ] GitHub Actions integration for CI cache cleanup

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⭐ Star this repo!

If you find this useful, give it a ⭐ — it helps others discover the project!
