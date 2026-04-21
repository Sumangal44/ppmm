#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
VERSION="3.1.4"  # Auto-synced from Cargo.toml

bash "${REPO_ROOT}/scripts/sync-version.sh"

cd "${REPO_ROOT}/releases/windows"
zip -r ppmm-windows-x64.zip ppmm-windows-x64.exe
cd "${REPO_ROOT}"

echo "Windows zip created for version ${VERSION}!"
