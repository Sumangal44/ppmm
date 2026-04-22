#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
VERSION="3.1.6"  # Auto-synced from Cargo.toml

bash "${REPO_ROOT}/scripts/sync-version.sh"

RELEASE_DIR="${REPO_ROOT}/releases/windows"
TARGET_EXE="${REPO_ROOT}/target/x86_64-pc-windows-gnu/release/ppmm.exe"
RELEASE_EXE="${RELEASE_DIR}/ppmm-windows-x64.exe"

mkdir -p "${RELEASE_DIR}"

# If a cross-compiled Windows binary exists in target/, stage it into releases/windows.
if [[ -f "${TARGET_EXE}" ]]; then
	cp "${TARGET_EXE}" "${RELEASE_EXE}"
fi

if [[ ! -f "${RELEASE_EXE}" ]]; then
	echo "Error: ${RELEASE_EXE} not found."
	echo "Build the Windows binary first, for example:"
	echo "  cargo build --release --target x86_64-pc-windows-gnu"
	echo "or run:"
	echo "  bash ${REPO_ROOT}/scripts/build-all-platforms.sh"
	exit 1
fi

(cd "${RELEASE_DIR}" && zip -q -r ppmm-windows-x64.zip ppmm-windows-x64.exe)

echo "Windows zip created for version ${VERSION}!"
