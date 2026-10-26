#!/usr/bin/env bash
# HyperXOS Mirror Ranker & Package Availability Pre-Flight Engine
set -euo pipefail

log() {
    echo "==> [HyperXOS-Preflight] $*"
}

log "Memeriksa status konektivitas internet..."
ONLINE=false
if ping -c 1 -W 2 archlinux.org >/dev/null 2>&1 || curl -s --max-time 2 -I https://archlinux.org >/dev/null 2>&1; then
    ONLINE=true
fi

if [ "${ONLINE}" = "true" ]; then
    log "Koneksi internet aktif. Memilih dan meranking mirror tercepat dengan Reflector (fast timeout)..."
    if command -v reflector >/dev/null 2>&1; then
        reflector --latest 10 --protocol https --sort rate --download-timeout 5 --connection-timeout 3 --save /etc/pacman.d/mirrorlist 2>/dev/null || true
    fi
    log "Menyinkronkan database paket repositori resmi..."
    pacman -Sy --noconfirm 2>/dev/null || true
else
    log "Mode Offline terdeteksi: Memanfaatkan repositori lokal ISO."
fi

log "Memverifikasi ketersediaan paket katalog dan menghasilkan netinstall.yaml dinamis..."
TEMPLATE="/etc/calamares/modules/netinstall.yaml.template"
OUTPUT="/etc/calamares/modules/netinstall.yaml"

if [ ! -f "${TEMPLATE}" ]; then
    TEMPLATE="/usr/share/calamares/modules/netinstall.yaml.template"
fi

if [ -f "${TEMPLATE}" ]; then
    python3 /usr/local/bin/hyperxos-generate-netinstall.py "${TEMPLATE}" "${OUTPUT}" || true
    cp -f "${OUTPUT}" /usr/share/calamares/modules/netinstall.yaml 2>/dev/null || true
fi

log "Verifikasi selesai."
