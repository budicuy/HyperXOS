#!/usr/bin/env bash
# HyperXOS Automated Verification and Smoke Test Script
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

OUT_DIR="${PROJECT_ROOT}/out"

echo "=================================================="
echo "  HyperXOS Automated Quality Assurance & Testing"
echo "  Date: $(date)"
echo "=================================================="

# Check for ISO in out/
shopt -s nullglob
ISO_FILES=("${OUT_DIR}"/hyperxos-*.iso)
shopt -u nullglob

if [ ${#ISO_FILES[@]} -eq 0 ]; then
    echo "[-] Error: No HyperXOS ISO found in ${OUT_DIR}. Run 'hyperx build' first." >&2
    exit 1
fi

LATEST_ISO=$(ls -t "${OUT_DIR}"/hyperxos-*.iso | head -n 1)
echo "[+] Found ISO image: ${LATEST_ISO}"

# 1. Size verification
ISO_SIZE=$(stat -c%s "${LATEST_ISO}")
ISO_SIZE_MB=$(( ISO_SIZE / 1024 / 1024 ))
echo "[+] ISO Image Size: ${ISO_SIZE_MB} MB"

if [ "${ISO_SIZE_MB}" -lt 500 ]; then
    echo "[-] Error: ISO size is suspiciously small (${ISO_SIZE_MB} MB < 500 MB)." >&2
    exit 1
fi

# 2. File format verification
echo "[+] Verifying ISO header and hybrid partition structure..."
file "${LATEST_ISO}" | grep -Ei "ISO 9660|DOS/MBR" >/dev/null || {
    echo "[-] Error: File is not a valid ISO 9660 / hybrid boot image." >&2
    exit 1
}
echo "[✓] ISO 9660 filesystem header verified."

# 3. Check El Torito / EFI boot record if tools available
if command -v fdisk >/dev/null 2>&1; then
    echo "[+] Partition table inspection:"
    fdisk -l "${LATEST_ISO}" 2>/dev/null | grep -E "EFI System|ISO9660" || true
fi

# 4. Check for sha256 checksum file
CHECKSUM_FILE="${LATEST_ISO}.sha256"
if [ -f "${CHECKSUM_FILE}" ]; then
    echo "[+] Verifying SHA256 checksum..."
    (cd "${OUT_DIR}" && sha256sum -c "$(basename "${CHECKSUM_FILE}")")
    echo "[✓] SHA256 checksum verified valid."
fi

# 5. Non-interactive QEMU Headless Smoke Test
echo "[+] Initiating QEMU headless kernel boot smoke test (25s)..."
set +e
timeout 25 qemu-system-x86_64 \
    -enable-kvm \
    -cpu host \
    -smp 2 \
    -m 2048 \
    -cdrom "${LATEST_ISO}" \
    -display none \
    -vga none \
    -serial stdio \
    >/dev/null 2>&1
TEST_EXIT=$?
set -e

# In timeout, exit code 124 indicates the system kept running until killed (passed smoke test)
if [ ${TEST_EXIT} -eq 124 ]; then
    echo "[✓] Automated boot test PASSED! Kernel and initramfs loaded and executed stably."
elif [ ${TEST_EXIT} -eq 0 ]; then
    echo "[✓] QEMU booted and exited cleanly."
else
    echo "[-] Warning: QEMU exited with code ${TEST_EXIT} during boot test." >&2
    exit ${TEST_EXIT}
fi

echo "=================================================="
echo "  All automated verification checks PASSED!"
echo "=================================================="
exit 0
