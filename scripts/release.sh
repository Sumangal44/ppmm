#!/bin/bash
set -e

# Validate all bash scripts before running the release flow.
bash ./scripts/check-scripts.sh

# Sync all packaging metadata to Cargo.toml version.
bash ./scripts/sync-version.sh

# Build binaries
./scripts/build-release.sh

# Build zip for winget
./scripts/build-win-zip.sh

# Generate winget manifest
./scripts/generate-winget.sh

# Build deb
./scripts/build-deb.sh

echo "All packages built successfully!"
