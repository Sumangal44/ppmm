#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
VERSION="3.1.5"  # Auto-synced from Cargo.toml

bash "${REPO_ROOT}/scripts/sync-version.sh"

# Create temp folder
rm -rf "${REPO_ROOT}/aur-build"
mkdir -p "${REPO_ROOT}/aur-build"
cp -r "${REPO_ROOT}/packaging/aur/"* "${REPO_ROOT}/aur-build/"

# Build AUR package
cd "${REPO_ROOT}/aur-build"
makepkg -si

echo "AUR package installed for version ${VERSION}!"
