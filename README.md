![OSCG26 Label](https://github.com/user-attachments/assets/9ae968b0-3aaf-49a6-9e8b-aa77f0270eba)
# PPMM - Python Project Manager 

A fast, efficient command-line tool to create, manage, and deploy Python projects. Written in Rust with cross-platform support for Windows, macOS, and Linux.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-3.1.6-brightgreen.svg)](https://github.com/Sumangal44/ppmm/releases)
[![Netlify Status](https://api.netlify.com/api/v1/badges/d9564fd3-7ca3-4b77-a04f-3c03097d4036/deploy-status)](https://app.netlify.com/projects/ppmmdocs/deploys)
## Table of Contents

- [Features](#features)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Commands](#commands)
  - [Create & Initialize Projects](#create--initialize-projects)
  - [Package Management](#package-management)
  - [Script Management](#script-management)
  - [Project Information](#project-information)
  - [Requirements Management](#requirements-management)
- [Project Configuration](#project-configuration)
- [Project Structure](#project-structure)
- [Examples](#examples)
- [Cross-Platform Support](#cross-platform-support)
- [Build From Source](#build-from-source)
- [Contributing](#contributing)
- [License](#license)

## Features

✨ **Core Features:**
- 🗂️ **Project Creation** - Scaffold new Python projects with proper structure
- 📦 **Package Management** - Add, remove, and update Python packages with pip integration
- 🔧 **Virtual Environment Management** - Automatic venv creation and management
- 📝 **Configuration Management** - TOML-based project configuration
- 🚀 **Script Management** - Create and run custom project scripts
- 📋 **Requirements Generation** - Auto-generate requirements.txt from project configuration
- 🔄 **Package Updates** - Update all packages to their latest versions
- 🏷️ **Version Control** - Automatic semantic version bumping (major, minor, patch)
- ✅ **Cross-Platform** - Works seamlessly on Windows, macOS, and Linux
- ⚡ **Performance** - Optimized Rust implementation with zero runtime dependencies
- 🔒 **Lock File Support** - Automatic `ppmm.lock` generation for reproducible builds


## Quick Start

### Create a New Project

```bash
ppmm new my-project
cd my-project
ppmm start
```

### Initialize in Existing Directory

```bash
cd existing-project
ppmm init
ppmm add numpy pandas
ppmm start
```

### Add Packages

```bash
ppmm add requests flask
ppmm add beautifulsoup4==4.9.0
```

### Run Project

```bash
ppmm start
```

### Run Custom Scripts

```bash
ppmm run test
ppmm run build
```

## ⚡ Quick Command Cheat Sheet

Here are the most commonly used `ppmm` commands:

| Command | Description |
|----------|------------|
| `ppmm new <name>` | Create a new Python project |
| `ppmm init` | Initialize ppmm in current directory |
| `ppmm add <package>` | Add and install packages |
| `ppmm rm <package>` | Remove installed packages |
| `ppmm update` | Update all packages |
| `ppmm list` | List configured packages |
| `ppmm start` | Run the main project script |
| `ppmm run <script>` | Run custom script from project.toml |
| `ppmm build` | Run build script |
| `ppmm bump patch/minor/major` | Bump project version |
| `ppmm gen` | Generate requirements.txt |
| `ppmm install` | Install dependencies |
| `ppmm lock lock` | Resolve and pin dependencies |
| `ppmm lock install` | Install via lock/requirements workflow |
| `ppmm lock update` | Check or apply dependency updates |
| `ppmm lock audit` | Scan dependencies for vulnerabilities |
| `ppmm info` | Show project details |
| `ppmm doctor` | Diagnose environment issues |


## Installation

### Using Cargo (Recommended)

Install directly from crates.io:

```bash
cargo install ppmm
```

### Using Homebrew (macOS/Linux)

```bash
brew tap Sumangal44/ppmm
brew install ppmm
```

### Using Scoop (Windows)

```bash
# Coming soon - waiting for bucket submission
scoop bucket add ppmm https://github.com/Sumangal44/ppmm
scoop install ppmm
```

### From Binary Releases

Download pre-built binaries from [GitHub Releases](https://github.com/Sumangal44/ppmm/releases):

**Linux:**
```bash
# Download and install
curl -L https://github.com/Sumangal44/ppmm/releases/latest/download/ppmm-linux-x64 -o ppmm
chmod +x ppmm
sudo mv ppmm /usr/local/bin/
ppmm --version
```

**macOS (Intel x86_64):**
```bash
curl -L https://github.com/Sumangal44/ppmm/releases/latest/download/ppmm-macos-x64 -o ppmm
chmod +x ppmm
sudo mv ppmm /usr/local/bin/
ppmm --version
```

**macOS (Apple Silicon / ARM64):**
```bash
curl -L https://github.com/Sumangal44/ppmm/releases/latest/download/ppmm-macos-arm64 -o ppmm
chmod +x ppmm
sudo mv ppmm /usr/local/bin/
ppmm --version
```

**Windows:**
```powershell
# Download from releases page and add to PATH
# Or use the installer executable
```

### Script install (Linux/macOS)

From the repository root run:

```bash
git clone https://github.com/Sumangal44/ppmm.git
cd ppmm
bash install.sh
```

This uses [install.sh](install.sh) to check prerequisites, build, and place `ppmm` in `/usr/local/bin` (prompts for sudo if needed).

### Quick one-liner (Linux/macOS)

```bash
bash <(curl -s https://raw.githubusercontent.com/Sumangal44/ppmm/master/quick-install.sh)
```

Or run locally: `bash quick-install.sh` after cloning. Script lives at [quick-install.sh](quick-install.sh).

### Manual install (Linux/macOS/Windows)

Requirements: Rust 1.60+, Python 3.7+, Git.

```bash
git clone https://github.com/Sumangal44/ppmm.git
cd ppmm
cargo build --release

# Linux/macOS
sudo cp target/release/ppmm /usr/local/bin/

# Windows (PowerShell/CMD)
copy target\release\ppmm.exe C:\\Windows\\System32\\   # or add target\release to PATH

ppmm --version
```

Binary output: `target/release/ppmm` (or `ppmm.exe` on Windows).

## Commands

### Create & Initialize Projects

#### `ppmm new <NAME>`
Create a new Python project with scaffolding.

**Options:**
- `-v, --version <VERSION>` - Project version (default: `0.1.0`)
- `-d, --description <DESC>` - Project description
- `-g, --git` - Initialize git repository
- `-e, --no-venv` - Skip virtual environment creation

**Examples:**
```bash
# Create basic project
ppmm new my-project

# Create with metadata and git
ppmm new my-project -v 1.0.0 -d "My awesome project" -g

# Create without venv
ppmm new my-project --no-venv
```

#### `ppmm init`
Initialize a Python project in the current directory.

**Options:**
- Same as `ppmm new`

**Examples:**
```bash
# Initialize in current directory
ppmm init

# Initialize with git
ppmm init -g
```

### Package Management

#### `ppmm add <PACKAGES>`
Add one or more packages to the project.

**Features:**
- Installs to virtual environment automatically
- Supports version pinning (e.g., `package==1.2.3`)
- Updates `project.toml` automatically
- Validates package names

**Examples:**
```bash
# Add multiple packages
ppmm add requests flask numpy

# Add specific versions
ppmm add django==3.2.0 pillow==9.0.0

# Mix and match
ppmm add requests flask==2.0.0 numpy
```

#### `ppmm rm <PACKAGES>`
Remove packages from the project and environment.

**Features:**
- Removes from virtual environment
- Updates `project.toml`
- Validates package existence

**Examples:**
```bash
ppmm rm requests
ppmm rm flask numpy pandas
```

#### `ppmm update`
Update all packages to their latest versions from PyPI.

**Features:**
- Fetches latest versions from PyPI API
- Updates all packages atomically
- Reports failed updates

**Examples:**
```bash
ppmm update
```

### Script Management

#### `ppmm run <SCRIPT-NAME>`
Execute a custom script defined in `project.toml`.

**Features:**
- Cross-platform command execution
- Access to virtual environment
- Real-time output streaming

**Examples:**
```bash
ppmm run test
ppmm run build
ppmm run dev
```

#### `ppmm build`
Run the `build` script defined in the `[scripts]` section of `project.toml`.

**Features:**
- Uses the project's virtual environment on PATH
- Cross-platform execution (`cmd` on Windows, `sh -c` on Linux/macOS)
- Warns if `scripts.build` is not defined

**Examples:**
```bash
# Ensure project.toml contains:
# [scripts]
# build = "python setup.py build"

ppmm build
```

#### `ppmm bump <TYPE>`
Automatically bump the project version following semantic versioning.

**Arguments:**
- `major` - Increment major version (1.0.0 → 2.0.0)
- `minor` - Increment minor version (1.0.0 → 1.1.0)
- `patch` - Increment patch version (1.0.0 → 1.0.1)

This command updates the version field in `project.toml` automatically. 

**Features:**
- Parses semantic versions (major.minor.patch)
- Strips alpha/beta suffixes before bumping
- Updates project.toml automatically
- Shows colored version bump info

**Examples:**
```bash
# Bump patch version
ppmm bump patch

# Bump minor version
ppmm bump minor

# Bump major version
ppmm bump major
```

### Project Information

#### `ppmm info`
Display comprehensive project information.

**Shows:**
- Project name, version, description
- Python version in use
- All configured scripts
- All installed packages (up to 10 with count)

**Example Output:**
```
Python: 3.9.0

Project: my-project
Version: 1.0.0
Description: An awesome project

-- 4 Scripts --
test: python -m pytest tests/
build: python setup.py build
dev: python -m flask run
upgrade: python -m pip install --upgrade pip

-- 5 Packages --
flask==2.1.0
numpy==1.21.0
pandas==1.3.0
requests==2.26.0
pytest==6.2.0

```

### Requirements Management

#### `ppmm gen`
Generate a `requirements.txt` file from `project.toml`.

**Features:**
- Extracts all packages and versions
- Creates standard requirements.txt format
- Overwrites existing requirements.txt

**Examples:**
```bash
ppmm gen

# Equivalent to: pip freeze > requirements.txt
```

#### `ppmm install`
Install all packages from `project.toml`.

**Features:**
- Creates venv if missing
- Batch installs all packages
- Validates all packages exist

**Options:**
- `-r, --requirements <FILE>` - Install from requirements.txt instead

**Examples:**
```bash
# Install from project.toml
ppmm install

# Install from requirements.txt
ppmm install -r requirements.txt
ppmm install --requirements /path/to/reqs.txt
```
#### `ppmm doctor`

Run diagnostics to verify your project and environment setup.

**Checks:**
- Presence of `project.toml`
- Virtual environment (`venv`) and its Python
- System Python installation
- pip availability

**Usage:**
```bash
ppmm doctor
```

## Project Configuration

### `project.toml` Format

PPMM uses TOML for project configuration. Here's the complete format:

```toml
[project]
name = "my-project"
version = "1.0.0"
description = "An awesome Python project"
main_script = "./src/main.py"

[packages]
# Production dependencies
requests = "2.28.0"
flask = "2.1.0"
numpy = "1.21.0"

[scripts]
# Custom scripts
test = "python -m pytest tests/ -v"
lint = "python -m pylint src/"
format = "python -m black src/"
build = "python setup.py build"
dev = "python -m flask run --debug"
upgrade-pip = "python -m pip install --upgrade pip"
```

### Configuration Details

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project.name` | String | Yes | Project name |
| `project.version` | String | Yes | Project version (semver) |
| `project.description` | String | No | Project description |
| `project.main_script` | String | Yes | Entry point script |
| `packages.<name>` | String | No | Package with version |
| `scripts.<name>` | String | No | Command to execute |

## Project Structure

PPMM creates the following structure for new projects:

```
my-project/
├── project.toml          # Project configuration
├── requirements.txt      # Auto-generated dependencies
├── venv/                 # Virtual environment
│   ├── bin/             # Executables (Linux/macOS)
│   ├── Scripts/         # Executables (Windows)
│   └── lib/             # Installed packages
├── src/
│   └── main.py          # Entry point
└── .gitignore           # Git ignore (if -g flag used)
```

## Examples

### Example 1: Web API Project

```bash
# Create project
ppmm new api-server -v 1.0.0 -d "REST API server" -g

# Add dependencies
ppmm add flask flask-cors flask-sqlalchemy

# Create scripts
# Edit project.toml to add:
# [scripts]
# dev = "python -m flask run"
# prod = "gunicorn main:app"

# Start development
ppmm start
```

### Example 2: Data Science Project

```bash
# Create project
ppmm new data-analysis -d "Data analysis project"

# Add data science packages
ppmm add pandas numpy scipy matplotlib scikit-learn jupyter

# Generate requirements for sharing
ppmm gen

# Update all packages
ppmm update
```

### Example 3: Migrate from pip

```bash
# Convert existing project
cd my-existing-project
ppmm init -g

# Install from existing requirements
ppmm install -r requirements.txt

# Generate new project config
ppmm gen
```

## Cross-Platform Support

PPMM is fully cross-platform and tested on:

- **Windows** - Full support, `.exe` extensions handled automatically
- **macOS** - Full support, uses `bin/` for venv
- **Linux** - Full support, uses `bin/` for venv

The tool automatically detects your platform and uses the correct paths and commands.

### Platform-Specific Paths

| Platform | Python Path | Pip Path |
|----------|-------------|----------|
| Windows | `./venv/Scripts/python.exe` | `./venv/Scripts/pip.exe` |
| Linux/macOS | `./venv/bin/python` | `./venv/bin/pip` |

## Build From Source

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python 3.7+
# From: https://www.python.org/downloads/
```

### Building

```bash
git clone https://github.com/Sumangal44/ppmm
cd ppmm

# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Run tests
cargo test

# Check all bash scripts for syntax errors
bash ./scripts/check-scripts.sh

# Run clippy linter
cargo clippy
```

### Output

Binary location: `target/release/ppmm` (or `ppmm.exe` on Windows)

### Development

```bash
# Run directly from source
cargo run -- new my-project

# Debug mode with verbose output
RUST_LOG=debug cargo run -- new my-project

# Watch mode (requires cargo-watch)
cargo watch -x build
```

## Requirements

### Runtime

- Python 3.7 or newer
- pip (comes with Python)

### Development

- Rust 1.60 or newer
- Cargo (comes with Rust)
- Git (for version control)

## Features Status

### ✅ Implemented

- Project scaffolding with automatic structure
- Virtual environment management
- Package installation/removal
- Package update checking from PyPI
- Script execution
- Requirements.txt generation
- Cross-platform support (Windows, macOS, Linux)
- TOML configuration
- Git integration (optional)
- Error handling and validation
- Package name validation
- Improved error messages
- Cross-platform path handling
- Lock file support (`ppmm.lock`)

### 🚀 Planned Features

- Dependency resolution
- Dev dependencies separation
- Python version management
- Project templates
- Virtual environment isolation validation
- Package conflict detection
- Installation progress bar
- Caching of PyPI responses

## Troubleshooting

### Virtual Environment Not Found

**Problem:** "Virtual Environment Not Found"

**Solutions:**
1. Create venv: `ppmm new my-project` (auto-creates)
2. Manually create: `python -m venv venv`
3. Use `--no-venv` flag if intentional

### Package Installation Failed

**Problem:** "Package 'X' failed to install"

**Solutions:**
1. Check package name spelling
2. Verify package existence on PyPI: visit `https://pypi.org/project/<package>/` in your browser
3. Check pip version: `pip --version`
4. Update pip: `ppmm run upgrade-pip`

### Python Not Found

**Problem:** "python command not found"

**Solutions:**
1. Ensure Python is installed
2. Add Python to PATH
3. Use absolute path in scripts

### Cross-Platform Issues

**Windows:**
- Use forward slashes in `project.toml` (scripts: `"python -m pytest tests/"`)
- Paths are normalized automatically

**Linux/macOS:**
- Ensure execute permissions: `chmod +x venv/bin/python`

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust conventions (rustfmt)
- Pass clippy linter checks
- Add tests for new features
- Update documentation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**sumangal44** - Original Creator

Based on the PPM concept for streamlined Python project management.

## 👥 Contributors
Thanks to these amazing people for improving **ppmm**! 🚀

<a href="https://github.com/sumangal44/ppmm/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=sumangal44/ppmm" />
</a>

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [Clap](https://docs.rs/clap) for CLI parsing
- Uses [TOML](https://docs.rs/toml) for configuration
- Uses [Reqwest](https://docs.rs/reqwest) for PyPI API
- Uses [Colored](https://docs.rs/colored) for terminal colors

## Support

For issues, questions, or suggestions:

- Open an [Issue](https://github.com/Sumangal44/ppmm/issues)
- Check [Discussions](https://github.com/Sumangal44/ppmm/discussions)
- Read the [Wiki](https://github.com/Sumangal44/ppmm/wiki)

## Changelog

### Version 1.1.0

- ✅ Added `ppmm build` command for project builds
- ✅ Added `ppmm bump` command for semantic versioning (major, minor, patch)
- ✅ Automatic version control and management
- ✅ Install script with prerequisites checking
- ✅ Quick one-liner installer support
- ✅ Updated documentation and examples

### Version 1.0.0-alpha

- ✅ Initial release
- ✅ Cross-platform support
- ✅ Package management
- ✅ Virtual environment management
- ✅ Script execution
- ✅ Requirements generation
- ✅ Improved error handling
- ✅ TOML configuration
- ✅ Optimized Rust codebase
- ✅ Zero clippy warnings
- ✅ Production-ready

---

**Made with ❤️ in Rust**
