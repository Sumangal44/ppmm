# PPM - Python Project Manager

A fast, efficient command-line tool to create, manage, and deploy Python projects. Written in Rust with cross-platform support for Windows, macOS, and Linux.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0--alpha-brightgreen.svg)](https://github.com/Sumangal44/python-project-manager/releases)

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
- ✅ **Cross-Platform** - Works seamlessly on Windows, macOS, and Linux
- ⚡ **Performance** - Optimized Rust implementation with zero runtime dependencies

## Quick Start

### Create a New Project

```bash
ppm new my-project
cd my-project
ppm start
```

### Initialize in Existing Directory

```bash
cd existing-project
ppm init
ppm add numpy pandas
ppm start
```

### Add Packages

```bash
ppm add requests flask
ppm add beautifulsoup4==4.9.0
```

### Run Project

```bash
ppm start
```

### Run Custom Scripts

```bash
ppm run test
ppm run build
```

## Installation

### Script install (Linux/macOS)

From the repository root run:

```bash
git clone https://github.com/Sumangal44/python-project-manager.git
cd python-project-manager
bash install.sh
```

This uses [install.sh](install.sh) to check prerequisites, build, and place `ppm` in `/usr/local/bin` (prompts for sudo if needed).

### Quick one-liner (Linux/macOS)

```bash
bash <(curl -s https://raw.githubusercontent.com/Sumangal44/python-project-manager/master/quick-install.sh)
```

Or run locally: `bash quick-install.sh` after cloning. Script lives at [quick-install.sh](quick-install.sh).

### Manual install (Linux/macOS/Windows)

Requirements: Rust 1.60+, Python 3.7+, Git.

```bash
git clone https://github.com/Sumangal44/python-project-manager.git
cd python-project-manager
cargo build --release

# Linux/macOS
sudo cp target/release/ppm /usr/local/bin/

# Windows (PowerShell/CMD)
copy target\release\ppm.exe C:\\Windows\\System32\\   # or add target\release to PATH

ppm --version
```

Binary output: `target/release/ppm` (or `ppm.exe` on Windows).

## Commands

### Create & Initialize Projects

#### `ppm new <NAME>`
Create a new Python project with scaffolding.

**Options:**
- `-v, --version <VERSION>` - Project version (default: `0.1.0`)
- `-d, --description <DESC>` - Project description
- `-g, --git` - Initialize git repository
- `-e, --no-venv` - Skip virtual environment creation

**Examples:**
```bash
# Create basic project
ppm new my-project

# Create with metadata and git
ppm new my-project -v 1.0.0 -d "My awesome project" -g

# Create without venv
ppm new my-project --no-venv
```

#### `ppm init`
Initialize a Python project in the current directory.

**Options:**
- Same as `ppm new`

**Examples:**
```bash
# Initialize in current directory
ppm init

# Initialize with git
ppm init -g
```

### Package Management

#### `ppm add <PACKAGES>`
Add one or more packages to the project.

**Features:**
- Installs to virtual environment automatically
- Supports version pinning (e.g., `package==1.2.3`)
- Updates `project.toml` automatically
- Validates package names

**Examples:**
```bash
# Add multiple packages
ppm add requests flask numpy

# Add specific versions
ppm add django==3.2.0 pillow==9.0.0

# Mix and match
ppm add requests flask==2.0.0 numpy
```

#### `ppm rm <PACKAGES>`
Remove packages from the project and environment.

**Features:**
- Removes from virtual environment
- Updates `project.toml`
- Validates package existence

**Examples:**
```bash
ppm rm requests
ppm rm flask numpy pandas
```

#### `ppm update`
Update all packages to their latest versions from PyPI.

**Features:**
- Fetches latest versions from PyPI API
- Updates all packages atomically
- Reports failed updates

**Examples:**
```bash
ppm update
```

### Script Management

#### `ppm run <SCRIPT-NAME>`
Execute a custom script defined in `project.toml`.

**Features:**
- Cross-platform command execution
- Access to virtual environment
- Real-time output streaming

**Examples:**
```bash
ppm run test
ppm run build
ppm run dev
```

#### `ppm build`
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

ppm build
```

### Project Information

#### `ppm info`
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

#### `ppm gen`
Generate a `requirements.txt` file from `project.toml`.

**Features:**
- Extracts all packages and versions
- Creates standard requirements.txt format
- Overwrites existing requirements.txt

**Examples:**
```bash
ppm gen

# Equivalent to: pip freeze > requirements.txt
```

#### `ppm install`
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
ppm install

# Install from requirements.txt
ppm install -r requirements.txt
ppm install --requirements /path/to/reqs.txt
```

## Project Configuration

### `project.toml` Format

PPM uses TOML for project configuration. Here's the complete format:

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

PPM creates the following structure for new projects:

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
ppm new api-server -v 1.0.0 -d "REST API server" -g

# Add dependencies
ppm add flask flask-cors flask-sqlalchemy

# Create scripts
# Edit project.toml to add:
# [scripts]
# dev = "python -m flask run"
# prod = "gunicorn main:app"

# Start development
ppm start
```

### Example 2: Data Science Project

```bash
# Create project
ppm new data-analysis -d "Data analysis project"

# Add data science packages
ppm add pandas numpy scipy matplotlib scikit-learn jupyter

# Generate requirements for sharing
ppm gen

# Update all packages
ppm update
```

### Example 3: Migrate from pip

```bash
# Convert existing project
cd my-existing-project
ppm init -g

# Install from existing requirements
ppm install -r requirements.txt

# Generate new project config
ppm gen
```

## Cross-Platform Support

PPM is fully cross-platform and tested on:

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
git clone https://github.com/Sumangal44/python-project-manager
cd python-project-manager

# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Run tests
cargo test

# Run clippy linter
cargo clippy
```

### Output

Binary location: `target/release/ppm` (or `ppm.exe` on Windows)

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

### 🚀 Planned Features

- Dependency resolution
- Lock file support (like Cargo.lock)
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
1. Create venv: `ppm new my-project` (auto-creates)
2. Manually create: `python -m venv venv`
3. Use `--no-venv` flag if intentional

### Package Installation Failed

**Problem:** "Package 'X' failed to install"

**Solutions:**
1. Check package name spelling
2. Verify package exists: `pip search <package>`
3. Check pip version: `pip --version`
4. Update pip: `ppm run upgrade-pip`

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

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [Clap](https://docs.rs/clap) for CLI parsing
- Uses [TOML](https://docs.rs/toml) for configuration
- Uses [Reqwest](https://docs.rs/reqwest) for PyPI API
- Uses [Colored](https://docs.rs/colored) for terminal colors

## Support

For issues, questions, or suggestions:

- Open an [Issue](https://github.com/Sumangal44/python-project-manager/issues)
- Check [Discussions](https://github.com/Sumangal44/python-project-manager/discussions)
- Read the [Wiki](https://github.com/Sumangal44/python-project-manager/wiki)

## Changelog

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
