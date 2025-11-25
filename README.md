# mdfiles

Find files by modification date and suffix, output as markdown links.

## Features

- 🔍 **Date Filtering**: Find files modified on a specific date (defaults to today)
- 📁 **Suffix Matching**: Filter files by extension (defaults to `.go`)
- 🌳 **Directory Traversal**: Recursively search from any root directory
- 📝 **Markdown Output**: Results formatted as clickable markdown links
- ⚡ **Fast**: Built in Rust with efficient directory walking
- 🧪 **Well Tested**: Comprehensive test suite with 34 tests

## Installation

### From Pre-built Binaries

Download the latest release for your platform:

- **Linux (AMD64)**: `mdfiles-linux-amd64`
- **Windows (AMD64)**: `mdfiles-windows-amd64.exe`
- **macOS (ARM64)**: `mdfiles-darwin-arm64`

### From Source

```bash
git clone https://github.com/tebeka/mdfiles.git
cd mdfiles
cargo build --release
```

The binary will be at `target/release/mdfiles`.

## Usage

### Basic Examples

```bash
# Find all .go files modified today in current directory
mdfiles

# Find .rs files modified today
mdfiles --suffix .rs

# Find files modified on a specific date
mdfiles --date 2025-11-25

# Search in a specific directory
mdfiles --root ./src

# Combine all options
mdfiles -d 2025-11-25 -s .md -r ./docs
```

### Output Format

Results are formatted as markdown links:

```markdown
- [main.rs](./src/main.rs)
- [lib.rs](./src/lib.rs)
- [cli.rs](./tests/cli.rs)
```

This makes it easy to:
- Copy and paste into markdown documents
- Create file lists for documentation
- Generate project status reports
- Track recent changes

### Options

```
Options:
  -d, --date <DATE>      Date in YYYY-MM-DD format [default: today]
  -s, --suffix <SUFFIX>  File suffix to match [default: .go]
  -r, --root <ROOT>      Root directory to start search from [default: .]
  -h, --help             Print help
  -V, --version          Print version
```

## Use Cases

- **Find Go files changed today**: `mdfiles` (default behavior)
- **Code review prep**: `mdfiles -d 2025-11-24 -s .rs` - list all Rust files from yesterday
- **Documentation**: Generate file lists for README files
- **Project tracking**: Monitor files modified on specific dates
- **Release notes**: Track changes for release preparation

## Building

### Requirements

- Rust 1.70 or later
- Cargo

### Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

All 34 tests (18 unit tests + 16 integration tests) should pass.

## CI/CD

The project includes GitHub Actions workflows for:

- **CI**: Automated testing on Linux, Windows, and macOS
- **Release**: Automated binary builds for multiple platforms

Releases are automatically created when you push a tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

## Development

This project was created using AI pair programming with Claude Code. The git commit history contains the prompts used to create each feature, providing insight into the development process. Check the commit messages to see how each feature was implemented!

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Project Structure

```
mdfiles/
├── src/
│   └── main.rs           # Main application code
├── tests/
│   └── cli.rs            # Integration tests
├── .github/
│   └── workflows/
│       ├── ci.yml        # Continuous integration
│       └── release.yml   # Release automation
├── Cargo.toml            # Project metadata and dependencies
├── LICENSE               # MIT License
└── README.md             # This file
```

## Technical Details

- **Language**: Rust (2024 edition)
- **CLI Framework**: clap 4.5 with derive macros
- **Date Handling**: chrono 0.4
- **File Walking**: walkdir 2.5
- **Testing**: assert_cmd + predicates

---

Made with 🦀 Rust
