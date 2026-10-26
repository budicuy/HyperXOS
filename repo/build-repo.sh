#!/usr/bin/env bash
# HyperXOS Custom Local Repository Builder
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="${SCRIPT_DIR}/x86_64"
DB_NAME="hyperxos"

echo "==> [HyperXOS] Updating local pacman repository database in ${REPO_DIR}..."

mkdir -p "${REPO_DIR}"
cd "${REPO_DIR}"

# Check if there are any packages in the directory
shopt -s nullglob
PKGS=(*.pkg.tar.zst)
shopt -u nullglob

if [ ${#PKGS[@]} -eq 0 ]; then
    echo "  [i] No .pkg.tar.zst packages found in ${REPO_DIR} yet."
    echo "  [i] Creating empty repository database: ${DB_NAME}.db.tar.gz"
    # Create empty archive if needed so pacman doesn't fail on missing db
    tar -czf "${DB_NAME}.db.tar.gz" -T /dev/null
    tar -czf "${DB_NAME}.files.tar.gz" -T /dev/null
    ln -sf "${DB_NAME}.db.tar.gz" "${DB_NAME}.db"
    ln -sf "${DB_NAME}.files.tar.gz" "${DB_NAME}.files"
    echo "==> [HyperXOS] Empty repository database initialized."
    exit 0
fi

echo "  -> Found ${#PKGS[@]} package(s). Adding to ${DB_NAME}.db.tar.gz..."
rm -f "${DB_NAME}.db"* "${DB_NAME}.files"*
repo-add "${DB_NAME}.db.tar.gz" "${PKGS[@]}"

# Ensure symlinks exist
ln -sf "${DB_NAME}.db.tar.gz" "${DB_NAME}.db"
ln -sf "${DB_NAME}.files.tar.gz" "${DB_NAME}.files"

echo "==> [HyperXOS] Repository database successfully updated with ${#PKGS[@]} package(s)."
