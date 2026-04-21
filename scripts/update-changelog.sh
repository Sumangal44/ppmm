#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CHANGELOG="${REPO_ROOT}/CHANGELOG.md"
CARGO_TOML="${REPO_ROOT}/Cargo.toml"

if [[ ! -f "${CHANGELOG}" ]]; then
  echo "CHANGELOG.md not found at ${CHANGELOG}"
  exit 1
fi

if [[ ! -f "${CARGO_TOML}" ]]; then
  echo "Cargo.toml not found at ${CARGO_TOML}"
  exit 1
fi

VERSION="$(grep '^version = ' "${CARGO_TOML}" | head -n1 | cut -d '"' -f2)"
if [[ -z "${VERSION}" ]]; then
  echo "Failed to read version from Cargo.toml"
  exit 1
fi

if grep -q "^## \[${VERSION}\]" "${CHANGELOG}"; then
  echo "CHANGELOG already contains version ${VERSION}"
  exit 0
fi

DATE_UTC="$(date -u +%Y-%m-%d)"
TMP_FILE="$(mktemp)"

awk -v version="${VERSION}" -v date_utc="${DATE_UTC}" '
BEGIN { inserted = 0 }
{
  print $0
  if (!inserted && $0 ~ /^## \[Unreleased\]/) {
    print ""
    print "## [" version "] - " date_utc
    print ""
    print "### Added"
    print "- TODO: describe user-visible additions"
    print ""
    print "### Changed"
    print "- TODO: describe behavior or API changes"
    print ""
    print "### Fixed"
    print "- TODO: describe bug fixes"
    print ""
    inserted = 1
  }
}
END {
  if (!inserted) {
    exit 2
  }
}
' "${CHANGELOG}" > "${TMP_FILE}"

if [[ $? -eq 2 ]]; then
  rm -f "${TMP_FILE}"
  echo "Could not find '## [Unreleased]' section in CHANGELOG.md"
  exit 1
fi

mv "${TMP_FILE}" "${CHANGELOG}"
echo "Inserted CHANGELOG entry for version ${VERSION} (${DATE_UTC})"
