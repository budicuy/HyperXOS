#!/usr/bin/env bash
# HyperXOS Project Build Cleanup Script
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "==> [HyperXOS] Cleaning temporary build artifacts..."

# Remove local work directory
if [ -d "${PROJECT_ROOT}/work" ]; then
    echo "  -> Removing ${PROJECT_ROOT}/work..."
    sudo rm -rf "${PROJECT_ROOT}/work"
fi

# Remove /var/tmp/hyperxos-work
if [ -d "/var/tmp/hyperxos-work" ]; then
    echo "  -> Removing /var/tmp/hyperxos-work..."
    sudo umount -R "/var/tmp/hyperxos-work" 2>/dev/null || true
    sudo rm -rf "/var/tmp/hyperxos-work"
fi

# Remove /tmp/hyperxos-tmp if present
if [ -d "/tmp/hyperxos-tmp" ]; then
    echo "  -> Removing /tmp/hyperxos-tmp..."
    sudo rm -rf "/tmp/hyperxos-tmp"
fi

# If argument --all or -a is passed, also clean virtual disk and out/ directory
if [[ "${1:-}" == "--all" || "${1:-}" == "-a" ]]; then
    echo "  -> Cleaning out/ directory and virtual test disks..."
    rm -rf "${PROJECT_ROOT}/out"
    rm -f "${PROJECT_ROOT}/hyperxos-install.qcow2"
    rm -f "${PROJECT_ROOT}/hyperxos-test.qcow2"
    rm -f "${PROJECT_ROOT}/OVMF_VARS.4m.fd"
fi

echo "==> [HyperXOS] Cleanup completed successfully."
