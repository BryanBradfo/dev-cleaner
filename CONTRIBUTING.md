# Contributing to dev-cleaner

Thank you for your interest in contributing to dev-cleaner! 🎉

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)
- Git

### Setting Up the Development Environment

1. **Clone the repository**
   ```bash
   git clone https://github.com/BryanBradfo/dev-cleaner.git
   cd dev-cleaner
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run the project**
   ```bash
   cargo run -- scan
   # or
   cargo run -- clean --dry-run
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

## Development Workflow

### Code Style

This project follows the standard Rust style guidelines:

- **Format your code** with `rustfmt`:
  ```bash
  cargo fmt
  ```

- **Lint your code** with `clippy`:
  ```bash
  cargo clippy -- -D warnings
  ```

- **Check your code** before committing:
  ```bash
  cargo check
  cargo test
  cargo fmt --check
  cargo clippy
  ```

### Making Changes

1. **Create a new branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and commit them with descriptive messages:
   ```bash
   git commit -m "Add support for detecting Maven target directories"
   ```

3. **Test your changes**:
   ```bash
   cargo test
   cargo build --release
   ./target/release/dev-cleaner scan
   ```

4. **Push your branch** and create a pull request:
   ```bash
   git push origin feature/your-feature-name
   ```

## Pull Request Guidelines

- **Keep PRs focused**: Each PR should address a single feature or bug fix
- **Write clear descriptions**: Explain what your PR does and why
- **Add tests**: If applicable, add tests for new functionality
- **Update documentation**: Update the README if you're adding new features
- **Follow code style**: Ensure your code passes `cargo fmt` and `cargo clippy`

## Adding Support for New Ecosystems

To add support for a new package manager or build system:

1. **Update `src/scanner.rs`**: Add a new case to the `should_scan()` function
2. **Add validation logic**: Include any necessary file checks (like checking for `package.json`)
3. **Update README**: Add the new ecosystem to the supported ecosystems table
4. **Test thoroughly**: Ensure the scanner correctly identifies the new directory type

Example:
```rust
"maven_target" => {
    if has_sibling_file(full_path, "pom.xml") {
        Some("Maven".to_string())
    } else {
        None
    }
}
```

## Reporting Issues

### Bug Reports

When reporting bugs, please include:
- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce the issue
- Expected vs. actual behavior
- Any error messages or logs

### Feature Requests

When suggesting features, please:
- Clearly describe the feature and its use case
- Explain how it fits with the project's goals
- Consider implementation complexity

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on the code, not the person
- Help maintain a welcoming environment for all contributors

## Questions?

If you have questions about contributing, feel free to:
- Open an issue with the "question" label
- Start a discussion on GitHub Discussions

---

Thank you for contributing to dev-cleaner! 🦀
