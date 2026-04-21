#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
VERSION="3.1.2"  # Auto-synced from Cargo.toml

bash "${REPO_ROOT}/scripts/sync-version.sh"

SHA256=$(sha256sum "${REPO_ROOT}/releases/windows/ppmm-windows-x64.zip" | awk '{print $1}')

cat > "${REPO_ROOT}/packaging/winget/ppmm.yaml" <<EOL
Id: Sumangal44.ppmm
Name: ppmm
Version: $VERSION
Publisher: Sumangal44
License: MIT
ShortDescription: Python Project Manager CLI
Installers:
  - Architecture: x64
    InstallerType: portable
    InstallerUrl: https://github.com/Sumangal44/ppmm/releases/download/v$VERSION/ppmm-windows-x64.zip
    InstallerSha256: $SHA256
EOL

echo "Winget manifest generated!"
