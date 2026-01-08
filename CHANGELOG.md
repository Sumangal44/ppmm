# Changelog

All notable changes to PPMM (Python Project Manager) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.2] - 2026-01-08

### Added
- GitHub Actions workflows (publish.yml and release-checklist.yml)
- Complete CHANGELOG.md with versioning guidelines
- Code formatting with rustfmt applied
- Release validation and pre-publication checklist

### Changed
- Applied cargo fmt to all source files for consistent formatting
- Improved code organization and import ordering
- Enhanced line wrapping for better readability

### Fixed
- Fixed Cargo.toml edition from 2024 to 2021

## [1.1.1] - 2026-01-08

### Added
- GitHub Actions workflow for automated publishing to crates.io
- `.github/workflows/publish.yml` for Trusted Publisher setup
- Complete PUBLISHING.md guide for crate distribution

### Changed
- Fixed edition field in Cargo.toml (2024 → 2021)

### Security
- Implemented OIDC-based Trusted Publisher authentication
- No static tokens required in repository

## [1.1.0] - 2025-12-15

### Added
- Comprehensive README.md (598 lines)
- CONTRIBUTING.md with development guidelines
- SECURITY.md with vulnerability policy
- Cargo documentation generation support
- Command standardization across all markdown files

### Changed
- All command references updated from `ppm` to `ppmm`
- Enhanced error handling across modules
- Improved cross-platform compatibility

### Fixed
- Multiple code quality issues identified during analysis
- Clippy warnings resolved
- Import organization and formatting

## [1.0.0] - 2025-11-01

### Added
- Initial release of PPMM (Python Project Manager)
- Core project management functionality
- Virtual environment management
- Cross-platform CLI support
- Basic configuration system

### Features
- `ppmm new` - Create new Python projects
- `ppmm add` - Add dependencies to projects
- `ppmm remove` - Remove dependencies
- `ppmm list` - List installed packages
- `ppmm search` - Search for packages
- `ppmm update` - Update dependencies
- `ppmm info` - Show project information
- `ppmm config` - Manage configuration
- `ppmm clean` - Clean project cache
- `ppmm --version` - Display version

---

## How to Release

### Version Bump Procedure

1. **Update version in Cargo.toml:**
   ```toml
   version = "X.Y.Z"
   ```

2. **Update CHANGELOG.md:**
   - Add new `[X.Y.Z] - YYYY-MM-DD` section
   - List all changes under Added, Changed, Fixed, Removed, Deprecated, Security
   - Keep Unreleased section for future changes

3. **Commit changes:**
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release v1.1.0"
   ```

4. **Create git tag:**
   ```bash
   git tag -a vX.Y.Z -m "Release version X.Y.Z"
   ```

5. **Push to repository:**
   ```bash
   git push origin main
   git push origin vX.Y.Z
   ```

6. **Create GitHub Release:**
   - Go to GitHub Releases page
   - Click "Draft a new release"
   - Select the tag just created
   - Use changelog entries as release notes
   - Publish the release
   - GitHub Actions workflow automatically publishes to crates.io

### Semantic Versioning Guidelines

- **MAJOR** (X.0.0): Breaking changes to API or functionality
- **MINOR** (0.Y.0): New features, backwards compatible
- **PATCH** (0.0.Z): Bug fixes only

### Changelog Entry Guidelines

- Use past tense ("Added", "Changed", not "Adds", "Changes")
- Focus on user-visible changes
- Group changes by category: Added, Changed, Fixed, Removed, Deprecated, Security
- Be concise but descriptive
- Reference issue numbers where applicable (#123)

### Pre-Release Checklist

- [ ] All tests passing: `cargo test`
- [ ] No clippy warnings: `cargo clippy --all-targets`
- [ ] Code formatted: `cargo fmt`
- [ ] Documentation updated: README, CHANGELOG, comments
- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG.md updated with all changes
- [ ] Commit and tag created locally
- [ ] No uncommitted changes: `git status`

