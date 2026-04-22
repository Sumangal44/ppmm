---
outline: deep
---

# Getting Started

ppmm is a Rust-powered CLI for creating, managing, and shipping Python projects with a consistent workflow.

## Install

Choose one install path:

```bash
# Cargo
cargo install ppmm

# Homebrew
brew tap Sumangal44/ppmm
brew install ppmm
```

## Create a New Project

```bash
ppmm new my-project -g
cd my-project
ppmm start
```

## Initialize Existing Folder

```bash
cd existing-project
ppmm init
ppmm add requests flask
ppmm start
```

## Everyday Commands

| Command | Purpose |
| --- | --- |
| `ppmm add <package>` | Add dependencies |
| `ppmm rm <package>` | Remove dependencies |
| `ppmm install` | Install dependencies from config |
| `ppmm update` | Update all packages to latest |
| `ppmm run <script>` | Run a custom script |
| `ppmm build` | Run build script |
| `ppmm bump patch|minor|major` | Bump semantic version |
| `ppmm gen` | Generate requirements.txt |
| `ppmm info` | Print project metadata |
| `ppmm doctor` | Run environment diagnostics |

## Lockfile Workflow

Use lock subcommands for reproducible and security-aware dependency handling:

```bash
# Resolve and pin dependencies
ppmm lock lock

# Install from lock or requirements
ppmm lock install

# Check updates
ppmm lock update

# Audit vulnerabilities
ppmm lock audit
```

## Project Layout

```text
my-project/
├── project.toml
├── requirements.txt
├── ppmm.lock
├── src/
│   └── main.py
└── venv/
```

## Next Step

Continue to [CLI Reference](/api-examples) for full command details and examples.
