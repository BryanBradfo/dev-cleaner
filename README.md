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
- 📊 **Size analysis** — See exactly how much space each directory is wasting
- 🗑️ **Interactive cleanup** — Select what to delete with checkboxes
- ⚡ **Blazing fast** — Built in Rust for lightning-speed filesystem traversal
- 🪟 **Windows-first** — Designed with Windows in mind (works on macOS/Linux too)
- 🔒 **Safe by default** — Always asks for confirmation, supports dry-run mode

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

### Clean up (interactive)
```bash
dev-cleaner clean
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

| Ecosystem | Directories Detected |
|-----------|---------------------|
| Node.js   | `node_modules`, `.next`, `.parcel-cache`, `dist`, `bower_components` |
| Python    | `__pycache__`, `.venv`, `venv`, `.tox`, `.pytest_cache`, `.mypy_cache` |
| Rust      | `target` (when `Cargo.toml` is present) |
| Java      | `.gradle`, `build` (when `build.gradle` is present) |
| C++       | `build` (when `CMakeLists.txt` is present) |

## 🗺️ Roadmap

- [x] Core scanning engine
- [x] Interactive cleanup with confirmations
- [ ] Global pip/npm package analysis (detect unused global packages)
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
