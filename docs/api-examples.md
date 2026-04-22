---
outline: deep
---

# CLI Reference

This reference is aligned with the commands implemented in the Rust CLI.

## Project Commands

### `ppmm new <name>`

Create a new project with source scaffolding.

```bash
ppmm new my-project -v 1.0.0 -d "My project" -g
```

### `ppmm init`

Initialize ppmm in the current folder.

```bash
ppmm init -g
```

## Dependency Commands

### `ppmm add <packages...>`

Add one or many packages, with optional version pins:

```bash
ppmm add requests flask
ppmm add django==5.0.4
```

### `ppmm rm <packages...>`

Remove packages from project config and environment:

```bash
ppmm rm flask
```

### `ppmm install`

Install dependencies from project config.

```bash
ppmm install
ppmm install -r requirements.txt
```

### `ppmm update [packages...]`

Update all configured packages, or selected package names.

```bash
ppmm update
ppmm update requests flask
```

### `ppmm list`

List configured packages from project metadata.

```bash
ppmm list
```

## Run and Build

### `ppmm start`

Run the configured main script.

```bash
ppmm start
```

### `ppmm run <script-name>`

Run custom scripts defined in `project.toml`.

```bash
ppmm run test
ppmm run dev
```

### `ppmm build`

Run `scripts.build` from project config.

```bash
ppmm build
```

## Version and Metadata

### `ppmm bump <patch|minor|major>`

Update semantic version in project config.

```bash
ppmm bump patch
```

### `ppmm gen`

Generate requirements.txt from configured packages.

```bash
ppmm gen
```

### `ppmm info`

Print project details, scripts, and package summary.

```bash
ppmm info
```

### `ppmm doctor`

Run diagnostic checks for config, Python, pip, and virtual environment.

```bash
ppmm doctor
```

## Lock Commands

### `ppmm lock lock`

Resolve and pin dependencies, optionally skipping hashes:

```bash
ppmm lock lock --requirements requirements.txt
ppmm lock lock --no-hashes
```

### `ppmm lock install`

Install dependencies from lock/requirements workflow:

```bash
ppmm lock install
ppmm lock install --requirements requirements.txt
```

### `ppmm lock update`

Check or apply lock updates:

```bash
ppmm lock update
ppmm lock update --apply
```

### `ppmm lock audit`

Scan dependencies for known vulnerabilities:

```bash
ppmm lock audit
ppmm lock audit --json-output
```
