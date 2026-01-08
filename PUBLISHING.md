# Publishing Guide for PPMM

This guide explains how to publish PPMM to crates.io using GitHub Actions with Trusted Publisher configuration.

## Table of Contents

- [Setup Trusted Publisher](#setup-trusted-publisher)
- [Configure GitHub Actions Workflow](#configure-github-actions-workflow)
- [Publishing Process](#publishing-process)
- [Troubleshooting](#troubleshooting)

## Setup Trusted Publisher

### Prerequisites

- Owner/admin access to the repository on GitHub
- Owner account on crates.io (https://crates.io)
- The repository must be public on GitHub
- crates.io API token configured locally (for initial setup)

### Step 1: Create crates.io Trusted Publisher

1. Go to [crates.io Account Settings](https://crates.io/me)
2. Navigate to **API Tokens** section
3. Click **Create New Token**
4. Or set up Trusted Publisher:
   - Go to **Publishing** or **Trusted Publishers** section
   - Click **Add a new Trusted Publisher**

### Step 2: Configure Publisher Details

Fill in the following information:

#### CI/CD Platform
- **Select:** `GitHub`

#### Repository Owner
- **Value:** `Sumangal44`
- This is the GitHub organization or username that owns the repository

#### Repository Name
- **Value:** `python-project-manager`
- The name of the GitHub repository containing the publishing workflow

#### Workflow Filename
- **Value:** `publish.yml` or `release.yml`
- This file should be located in `.github/workflows/` directory
- Example: `.github/workflows/publish.yml`

#### Environment Name (Optional)
- **Value:** `release` (recommended)
- Recommended: Create a dedicated GitHub Actions environment for publishing
- This adds an extra layer of security

### Step 3: Verify Configuration

After entering all fields:
- Click "Add trusted publisher"
- The system will verify the workflow file exists in the specified repository
- Once verified, the Trusted Publisher is active
- No static API tokens needed - uses OIDC tokens from GitHub Actions

## Configure GitHub Actions Workflow

### Create Workflow File

Create `.github/workflows/publish.yml`:

```yaml
name: Publish to crates.io

on:
  release:
    types: [published]
  workflow_dispatch:

jobs:
  publish:
    runs-on: ubuntu-latest
    environment: release
    permissions:
      id-token: write
      contents: read

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Verify project builds
        run: cargo build --release

      - name: Test before publishing
        run: cargo test

      - name: Publish to crates.io
        run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

### Workflow Explanation

- **Trigger:** Releases published on GitHub or manual workflow dispatch
- **Environment:** `release` (matches crates.io Trusted Publisher setup)
- **Permissions:** `id-token: write` required for Trusted Publisher OIDC authentication
- **Rust Setup:** Uses stable toolchain via `dtolnay/rust-toolchain`
- **Build Verification:** Ensures project compiles before publishing
- **Testing:** Runs tests to catch issues before publication
- **Publishing:** Uses `cargo publish` with credentials from Trusted Publisher

## Publishing Process

### Method 1: Create GitHub Release (Recommended)

1. Go to repository **Releases** page
2. Click **Draft a new release**
3. Fill in:
   - **Tag version:** `v1.1.0` (matches version in Cargo.toml)
   - **Release title:** `Release v1.1.0 - Description`
   - **Description:** Include changelog entries and features
4. Check **This is a pre-release** if publishing beta/rc versions
5. Click **Publish release**
6. GitHub Actions automatically triggers `publish.yml` workflow
7. Workflow:
   - Pulls code at the release tag
   - Runs `cargo build --release` to verify compilation
   - Runs `cargo test` to ensure tests pass
   - Executes `cargo publish` which:
     - Compresses package
     - Verifies manifest (Cargo.toml)
     - Uploads to crates.io
     - Makes crate publicly available

### Method 2: Manual Publishing

If GitHub Actions is not available or you need manual control:

```bash
# 1. Ensure you are on the version tag
git checkout v1.1.0

# 2. Verify the build locally
cargo build --release
cargo test

# 3. Verify the package (dry-run)
cargo publish --dry-run

# 4. Check crates.io for any existing publication
# Go to https://crates.io/crates/ppmm

# 5. Publish to crates.io
cargo publish

# 6. Verify on crates.io (wait a few seconds for indexing)
# Visit: https://crates.io/crates/ppmm/1.1.0
```

### Pre-Publication Checklist

- [ ] Version number bumped in `Cargo.toml`
- [ ] `CHANGELOG.md` updated with release notes
- [ ] All tests passing: `cargo test`
- [ ] No clippy warnings: `cargo clippy --all-targets`
- [ ] Code formatted: `cargo fmt`
- [ ] Binary builds: `cargo build --release`
- [ ] Git tag created: `git tag v1.1.0`
- [ ] Git changes committed: `git commit -m "Release v1.1.0"`
- [ ] Git pushed: `git push origin main && git push origin v1.1.0`
   - **Description:** Changelog and features
4. Click **Publish release**
5. GitHub Actions workflow automatically:
   - Builds the package
   - Publishes to PyPI
   - You can watch progress in **Actions** tab

### Method 2: Manual Workflow Dispatch

1. Go to repository **Actions** tab
2. Select **Publish to PyPI** workflow
3. Click **Run workflow**
4. Select branch (usually `master` or `main`)
5. Click **Run workflow**

## Workflow Environment Setup

### Create GitHub Actions Environment

For enhanced security, create a dedicated environment:

1. Go to repository **Settings**
2. Navigate to **Environments**
3. Click **New environment**
4. Name: `release`
5. Configure (optional):
   - **Protection rules:** Require approval before workflow runs
   - **Deployment branches:** Restrict to `main` or `master` only
6. Click **Configure environment**

## Version Management

### Update Version in Cargo.toml

Edit `Cargo.toml` and update the version field:

```toml
[package]
name = "ppmm"
version = "1.1.0"  # Update this
edition = "2021"
```

Use semantic versioning:
- **MAJOR.MINOR.PATCH** (e.g., `1.1.0`)
- **MAJOR:** Breaking API changes
- **MINOR:** New features, backwards compatible
- **PATCH:** Bug fixes only

### Create Git Tag

```bash
# Update version in Cargo.toml
vim Cargo.toml

# Commit the version change
git add Cargo.toml
git commit -m "Bump version to 1.1.0"

# Create annotated tag
git tag -a v1.1.0 -m "Release version 1.1.0"

# Push both commit and tag
git push origin main
git push origin v1.1.0
```

## Troubleshooting

### Workflow File Not Found

**Error:** "The workflow filename will be verified once all necessary fields are filled"

**Solution:**
1. Ensure workflow file exists at `.github/workflows/publish.yml`
2. Verify filename matches exactly (case-sensitive)
3. Commit and push the file to GitHub repository
4. Ensure `.github/workflows/publish.yml` uses YAML syntax
5. Wait a few minutes for GitHub to index the file

### Crate Already Published

**Error:** "crate version `1.1.0` is already uploaded"

**Solution:**
1. Check if version already exists on crates.io: `https://crates.io/crates/ppmm/1.1.0`
2. Use a new version number in `Cargo.toml`
3. Cannot re-publish same version (crates.io is immutable)
4. If unpublishing needed, use `cargo yank --vers 1.1.0` (requires ownership)

### Crate Not Found

**Error:** "crate `ppmm` not found on crates.io"

**Solution:**
1. Verify crate name in `Cargo.toml`: should be `ppmm`
2. First publish requires account on crates.io.io
3. Account must have publish rights to the crate
4. If crate doesn't exist, first publish will create it
5. Ensure `Cargo.toml` is valid TOML format

### Authentication Failed on crates.io

**Error:** "authentication failed" or "invalid token"

**Solution:**
1. Verify Trusted Publisher is configured in crates.io account settings
2. Check `permissions: id-token: write` in workflow YAML
3. Ensure `environment: release` matches crates.io setup
4. Verify workflow runs from correct GitHub repository
5. If using static token, ensure it has `publish-new` and `publish-update` scopes
6. Tokens expire - check token creation date in crates.io account

### Manifest Error

**Error:** `cargo publish` fails with "manifest validation failed"

**Solution:**
1. Run `cargo build` to verify project compiles
2. Verify `Cargo.toml` has required fields:
   ```toml
   [package]
   name = "ppmm"
   version = "X.X.X"
   edition = "2021"
   authors = ["..."]
   license = "..."
   description = "..."
   ```
3. Check documentation string: `#![doc = "..."]` is valid
4. Ensure all dependencies are published on crates.io (no git/path dependencies in release)

## Security Best Practices

1. **Use Dedicated Environment:** Create `release` environment with approval requirements
2. **Restrict Branches:** Limit publishing to specific branches (usually `main`)
3. **Require Reviews:** Enable "Require a pull request before merging" for main branch
4. **Audit Logs:** Monitor GitHub Actions logs for all publishing activity
5. **Limit Permissions:** Only grant `id-token: write` to publishing workflow
6. **Monitor Releases:** Verify each published version on crates.io and docs.rs
7. **Dependency Security:** Run `cargo audit` before publishing to check for known vulnerabilities

## Monitoring and Verification

### Check crates.io Publishing Status

1. Go to [crates.io package page](https://crates.io/crates/ppmm)
2. Verify latest version matches your release
3. Check download statistics and recent activity
4. Review all published versions and their contents

### Check docs.rs Documentation

1. Go to [docs.rs for ppmm](https://docs.rs/ppmm/)
2. Verify documentation built correctly
3. Check for any warnings in build logs
4. Ensure all public API is documented

### GitHub Actions Logs

1. Go to repository **Actions** tab
2. Select the `Publish to crates.io` workflow run
3. Expand job steps to see detailed output
4. Check for any warnings or errors in:
   - `Verify project builds` step
   - `Test before publishing` step
   - `Publish to crates.io` step
5. Save logs for audit trail

### Version History

Check published versions and availability:

```bash
# View crates.io metadata
curl https://crates.io/api/v1/crates/ppmm

# Install specific version
cargo install ppmm --version 1.1.0

# Check locally installed version
ppmm --version
```

## Rollback Procedure

If a broken version is published and users have installed it:

1. **Yank Release:** Mark version as "yanked" on crates.io (prevents new installs)
   ```bash
   cargo yank --vers 1.1.0
   ```

2. **Communicate:** Notify users via:
   - GitHub Release notes
   - GitHub Discussions
   - CHANGELOG.md update
   - Social media/announcements

3. **Fix & Republish:** Create patch release with fixes
   ```bash
   # Increment patch version
   vim Cargo.toml  # v1.1.1
   cargo publish
   ```

4. **Verify:** Confirm new version is available on crates.io

## Additional Resources

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Trusted Publishers](https://doc.rust-lang.org/cargo/reference/publishing.html#using-trusted-publishing)
- [Semantic Versioning](https://semver.org/)
- [docs.rs Documentation](https://docs.rs/)
- [Cargo Security Guidelines](https://doc.rust-lang.org/cargo/reference/security.html)
- [GitHub Actions - Trusted Publishing](https://github.blog/changelog/2023-06-27-github-actions-trusted-publishing-for-rust-crates/)



## Contact & Support

For publishing issues:
- Open an [Issue](https://github.com/Sumangal44/ppmm/issues)
- Check [Discussions](https://github.com/Sumangal44/ppmm/discussions)

---

**Last Updated:** January 2026
**Maintained By:** Sumangal44
