#!/bin/bash
set -u

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

bash "${REPO_ROOT}/scripts/sync-version.sh"
VERSION="$(grep '^version = ' "${REPO_ROOT}/Cargo.toml" | head -n1 | cut -d '"' -f2)"

mkdir -p "${REPO_ROOT}/releases/macos" "${REPO_ROOT}/releases/linux" "${REPO_ROOT}/releases/windows"

build_ok=()
build_fail=()

build_target() {
  local target="$1"
  local output_path="$2"
  local bin_name="$3"
  local platform_label="$4"

  echo "\n==> Building ${platform_label} (${target})"

  rustup target add "${target}" >/dev/null 2>&1

  if cargo build --release --target "${target}"; then
    cp "${REPO_ROOT}/target/${target}/release/${bin_name}" "${output_path}"
    build_ok+=("${platform_label}")
    echo "Built: ${output_path}"
  else
    build_fail+=("${platform_label}")
    echo "Skipped ${platform_label}: build toolchain/linker not available on this machine"
  fi
}

cd "${REPO_ROOT}" || exit 1

build_target "aarch64-apple-darwin" "${REPO_ROOT}/releases/macos/ppmm-macos-arm64" "ppmm" "macOS ARM64"
build_target "x86_64-apple-darwin" "${REPO_ROOT}/releases/macos/ppmm-macos-x64" "ppmm" "macOS x64"
build_target "x86_64-unknown-linux-gnu" "${REPO_ROOT}/releases/linux/ppmm-linux-x64" "ppmm" "Linux x64"
build_target "x86_64-pc-windows-gnu" "${REPO_ROOT}/releases/windows/ppmm-windows-x64.exe" "ppmm.exe" "Windows x64"

if [[ -f "${REPO_ROOT}/releases/windows/ppmm-windows-x64.exe" ]]; then
  (cd "${REPO_ROOT}/releases/windows" && zip -q -r ppmm-windows-x64.zip ppmm-windows-x64.exe)
  echo "Packed: ${REPO_ROOT}/releases/windows/ppmm-windows-x64.zip"
fi

echo "\nVersion: ${VERSION}"
echo "Successful builds: ${build_ok[*]:-none}"
echo "Skipped builds: ${build_fail[*]:-none}"

if [[ ${#build_ok[@]} -eq 0 ]]; then
  exit 1
fi
