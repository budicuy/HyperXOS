#!/usr/bin/env bash
# ==============================================================================
# HyperXOS GitHub Release Automation Script
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
OUT_DIR="${PROJECT_ROOT}/out"
RELEASE_NOTES="${PROJECT_ROOT}/RELEASE_NOTES.md"

echo "======================================================================"
echo "               HyperXOS GitHub Release Manager                        "
echo "======================================================================"

# 1. Locate the latest built ISO
LATEST_ISO="$(ls -t "${OUT_DIR}"/hyperxos-*.iso 2>/dev/null | head -n 1 || true)"
if [ -z "${LATEST_ISO}" ] || [ ! -f "${LATEST_ISO}" ]; then
    echo "[-] ERROR: Berkas ISO tidak ditemukan di direktori '${OUT_DIR}'!"
    echo "    Harap jalankan terlebih dahulu: hyperx build --prod"
    exit 1
fi

ISO_BASENAME="$(basename "${LATEST_ISO}")"
VERSION="$(echo "${ISO_BASENAME}" | sed -E 's/^hyperxos-([0-9.]+)-x86_64\.iso$/\1/')"
TAG="v${VERSION}"

echo "[+] Berkas ISO terdeteksi: ${ISO_BASENAME}"
echo "[+] Versi Rilis:          ${VERSION}"
echo "[+] Git Tag Target:       ${TAG}"

# 2. Check checksum files
SHA256_FILE="${LATEST_ISO}.sha256"
B2SUM_FILE="${LATEST_ISO}.b2sum"
PKGLIST_FILE="${OUT_DIR}/hyperxos-${VERSION}-x86_64.pkglist"

if [ ! -f "${SHA256_FILE}" ] || [ ! -f "${B2SUM_FILE}" ]; then
    echo "[!] Checksum belum lengkap. Membuat checksum sekarang..."
    hyperx checksum
fi

ASSETS=("${LATEST_ISO}" "${SHA256_FILE}" "${B2SUM_FILE}")
if [ -f "${PKGLIST_FILE}" ]; then
    ASSETS+=("${PKGLIST_FILE}")
fi

# 3. Ensure Git Tag exists locally
if ! git rev-parse "${TAG}" >/dev/null 2>&1; then
    echo "[+] Membuat git tag lokal '${TAG}'..."
    git tag -a "${TAG}" -m "HyperXOS ${VERSION} Production Release"
else
    echo "[✓] Git tag '${TAG}' sudah ada secara lokal."
fi

# 4. Check if GitHub CLI (gh) is available
if command -v gh >/dev/null 2>&1; then
    echo "[+] Memeriksa autentikasi GitHub CLI (gh)..."
    if gh auth status >/dev/null 2>&1; then
        echo "[+] Mengunggah rilis ke GitHub dengan 'gh'..."
        
        # Check if release already exists on GitHub
        if gh release view "${TAG}" --repo "budicuy/HyperXOS" >/dev/null 2>&1; then
            echo "[!] Rilis '${TAG}' sudah ada di GitHub. Mengunggah ulang aset..."
            gh release upload "${TAG}" "${ASSETS[@]}" --clobber --repo "budicuy/HyperXOS"
        else
            echo "[+] Membuat rilis baru di GitHub..."
            gh release create "${TAG}" "${ASSETS[@]}" \
                --repo "budicuy/HyperXOS" \
                --title "HyperXOS ${VERSION} — Official Release" \
                --notes-file "${RELEASE_NOTES}"
        fi
        echo "[✓] Rilis GitHub berhasil dipublikasikan!"
        echo "    URL: https://github.com/budicuy/HyperXOS/releases/tag/${TAG}"
        exit 0
    fi
fi

# 5. Fallback instructions if gh is not installed/authenticated
echo "----------------------------------------------------------------------"
echo "[i] GitHub CLI (gh) belum terpasang atau belum login."
echo "    Anda dapat mempublikasikan rilis dengan salah satu opsi berikut:"
echo ""
echo "    Opsi 1 (Otomatis via GitHub CLI):"
echo "      sudo pacman -S github-cli"
echo "      gh auth login"
echo "      ./scripts/publish-release.sh"
echo ""
echo "    Opsi 2 (Push Git Tag & Unggah via Web):"
echo "      1. Push komit dan tag ke GitHub:"
echo "         git push origin main"
echo "         git push origin ${TAG}"
echo "      2. Buka browser ke URL rilis:"
echo "         https://github.com/budicuy/HyperXOS/releases/new?tag=${TAG}"
echo "      3. Judul: HyperXOS ${VERSION} — Official Release"
echo "      4. Salin isi berkas: RELEASE_NOTES.md"
echo "      5. Lampirkan berkas dari direktori out/:"
for f in "${ASSETS[@]}"; do
    echo "         - $(basename "$f")"
done
echo "======================================================================"
