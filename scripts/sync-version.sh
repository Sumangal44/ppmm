#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
VERSION="$(grep '^version = ' "${REPO_ROOT}/Cargo.toml" | head -n1 | cut -d '"' -f2)"

if [[ -z "${VERSION}" ]]; then
  echo "Failed to read version from Cargo.toml"
  exit 1
fi

# Keep packaging metadata in sync with Cargo.toml version.
perl -i -pe "s/^pkgver=.*/pkgver=${VERSION}/" "${REPO_ROOT}/packaging/aur/PKGBUILD"
perl -i -pe "s/^Version: .*/Version: ${VERSION}/" "${REPO_ROOT}/packaging/deb/control"
perl -i -pe "s/^version: .*/version: ${VERSION}/" "${REPO_ROOT}/packaging/winget/ppmm.yaml"
perl -i -pe "s/^  version \".*\"/  version \"${VERSION}\"/" "${REPO_ROOT}/packaging/homebrew/ppmm.rb"

# Keep helper scripts aligned too.
perl -i -pe "s/^VERSION=\"[^\"]*\".*/VERSION=\"${VERSION}\"  # Auto-synced from Cargo.toml/" "${REPO_ROOT}/scripts/build-aur.sh"
perl -i -pe "s/^VERSION=\"[^\"]*\".*/VERSION=\"${VERSION}\"  # Auto-synced from Cargo.toml/" "${REPO_ROOT}/scripts/build-deb.sh"
perl -i -pe "s/^VERSION=\"[^\"]*\".*/VERSION=\"${VERSION}\"  # Auto-synced from Cargo.toml/" "${REPO_ROOT}/scripts/build-win-zip.sh"
perl -i -pe "s/^VERSION=\"[^\"]*\".*/VERSION=\"${VERSION}\"  # Auto-synced from Cargo.toml/" "${REPO_ROOT}/scripts/generate-winget.sh"

# Update the top README badge version.
perl -i -pe "s/version-[0-9]+\.[0-9]+\.[0-9]+latest/version-${VERSION}latest/" "${REPO_ROOT}/README.md"

echo "Synced project version to ${VERSION}"
