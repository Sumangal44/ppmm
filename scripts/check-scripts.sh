#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Checking bash script syntax in ${SCRIPT_DIR}..."

status=0
for script in "${SCRIPT_DIR}"/*.sh; do
  if bash -n "${script}"; then
    echo "OK: $(basename "${script}")"
  else
    echo "FAIL: $(basename "${script}")"
    status=1
  fi
done

if [[ ${status} -ne 0 ]]; then
  echo "One or more scripts failed bash syntax check."
  exit 1
fi

echo "All scripts passed bash syntax check."
