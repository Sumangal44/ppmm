#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
VERSION="3.1.4"  # Auto-synced from Cargo.toml

bash "${REPO_ROOT}/scripts/sync-version.sh"

rm -rf "${REPO_ROOT}/build"
mkdir -p "${REPO_ROOT}/build/DEBIAN" "${REPO_ROOT}/build/usr/bin"

# Build binary
cargo build --release
cp "${REPO_ROOT}/target/release/ppmm" "${REPO_ROOT}/build/usr/bin/ppmm"

# Copy control file
cp "${REPO_ROOT}/packaging/deb/control" "${REPO_ROOT}/build/DEBIAN/control"

# Build deb package
dpkg-deb --build "${REPO_ROOT}/build" "${REPO_ROOT}/ppmm_${VERSION}_amd64.deb"

echo "Deb package created: ppmm_${VERSION}_amd64.deb"
