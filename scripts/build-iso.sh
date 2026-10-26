#!/usr/bin/env bash
# HyperXOS ISO Build Automation Script
# Supports:
#   - dev: Ultra-fast development builds with build caching & fast compression (zstd -1)
#   - prod: Strict zero-warning production builds with maximum compression (zstd -22 1M, xz -9e initramfs, < 2.0GB size enforcement)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

MODE="${1:-dev}"
WORK_DIR="/var/tmp/hyperxos-work"
OUT_DIR="${PROJECT_ROOT}/out"
PROFILE_DIR="${PROJECT_ROOT}/archiso"
PROFILEDEF="${PROFILE_DIR}/profiledef.sh"
MKINITCPIO_CONF="${PROFILE_DIR}/airootfs/etc/mkinitcpio.conf.d/archiso.conf"
PACKAGES_FILE="${PROFILE_DIR}/packages.x86_64"
HASH_FILE="${WORK_DIR}/.packages.hash"
MAX_PROD_ISO_BYTES=2000000000 # 2.0 GB decimal limit

echo "=================================================="
echo "  HyperXOS ISO Build System"
echo "  Mode: ${MODE}"
echo "  Time: $(date)"
echo "=================================================="

# Check requirements
if ! command -v mkarchiso >/dev/null 2>&1; then
    echo "Error: mkarchiso is not installed. Please run 'sudo pacman -S archiso' or 'make setup'." >&2
    exit 1
fi

# Ensure output directory exists
mkdir -p "${OUT_DIR}"

# Step 1: Synchronize and build local repo packages
echo "==> [1/4] Ensuring HyperXOS custom packages repository is updated..."
"${PROJECT_ROOT}/repo/build-repo.sh"

echo "  -> Mirroring HyperXOS custom packages to archiso/airootfs/opt/hyperxos-repo/x86_64..."
mkdir -p "${PROFILE_DIR}/airootfs/opt/hyperxos-repo/x86_64"
cp -af "${PROJECT_ROOT}/repo/x86_64/"* "${PROFILE_DIR}/airootfs/opt/hyperxos-repo/x86_64/"

# Step 2: Configure compression and workspace caching based on build mode
echo "==> [2/4] Configuring build profile for mode '${MODE}'..."

if [ ! -f "${PROFILEDEF}" ]; then
    echo "Error: ${PROFILEDEF} not found. Please ensure archiso profile is set up." >&2
    exit 1
fi

CURRENT_PKG_HASH="$(cat "${PACKAGES_FILE}" "${PROJECT_ROOT}/repo/x86_64/hyperxos.db.tar.gz" 2>/dev/null | sha256sum | awk '{print $1}')"

case "${MODE}" in
    prod|production)
        echo "  -> Applying PRODUCTION settings: Ultra-compressed (SquashFS zstd -22 1M, Initramfs xz -9e)"
        echo "  -> Strict zero-tolerance quality policy: no errors, no conflicts, strict < 2.0GB size."

        # Configure maximum squashfs compression
        sed -i "s/airootfs_image_tool_options=.*/airootfs_image_tool_options=('-comp' 'zstd' '-Xcompression-level' '22' '-b' '1M' '-no-xattrs')/" "${PROFILEDEF}"

        # Configure ultra-compact initramfs with xz -9e and decompress modules
        mkdir -p "$(dirname "${MKINITCPIO_CONF}")"
        cat << 'EOF' > "${MKINITCPIO_CONF}"
HOOKS=(base udev microcode modconf memdisk archiso archiso_loop_mnt block filesystems keyboard)
COMPRESSION="xz"
COMPRESSION_OPTIONS=(-9e --threads=0)
MODULES_DECOMPRESS="yes"
EOF

        # Full workspace clean for 100% clean production environment
        if [ -d "${WORK_DIR}" ]; then
            echo "  -> [PROD] Cleaning entire workspace ${WORK_DIR} for clean production compile..."
            sudo umount -R "${WORK_DIR}" 2>/dev/null || true
            sudo rm -rf "${WORK_DIR}"
        fi
        sudo install -d -m 0777 "${WORK_DIR}"
        ;;

    dev|development|*)
        echo "  -> Applying DEVELOPMENT settings: Blazing fast build with build caching"
        echo "  -> Fast compression: SquashFS zstd level 1 (fast pack), Initramfs zstd level 1"

        # Fast squashfs compression level 1 with 256K block size
        sed -i "s/airootfs_image_tool_options=.*/airootfs_image_tool_options=('-comp' 'zstd' '-Xcompression-level' '1' '-b' '256K' '-no-xattrs')/" "${PROFILEDEF}"

        # Fast initramfs compression level 1
        mkdir -p "$(dirname "${MKINITCPIO_CONF}")"
        cat << 'EOF' > "${MKINITCPIO_CONF}"
HOOKS=(base udev microcode modconf memdisk archiso archiso_loop_mnt block filesystems keyboard)
COMPRESSION="zstd"
COMPRESSION_OPTIONS=(-1 -T0)
MODULES_DECOMPRESS="no"
EOF

        # Incremental Caching Check:
        CACHED_HASH=""
        if [ -f "${HASH_FILE}" ]; then
            CACHED_HASH="$(cat "${HASH_FILE}" 2>/dev/null || echo "")"
        fi

        if [ -d "${WORK_DIR}/x86_64/airootfs" ] && [ -f "${WORK_DIR}/base._make_packages" ] && [ "${CACHED_HASH}" == "${CURRENT_PKG_HASH}" ]; then
            echo "  -> [CACHE HIT] Menggunakan build cache airootfs untuk pembangunan super cepat..."
            echo "  -> Menyinkronkan perubahan overlay archiso/airootfs..."
            sudo cp -af --no-preserve=ownership,mode "${PROFILE_DIR}/airootfs/." "${WORK_DIR}/x86_64/airootfs/"

            # If efiboot.img is missing, ensure its bootmode generator runs
            if [ ! -f "${WORK_DIR}/efiboot.img" ]; then
                sudo rm -f "${WORK_DIR}"/base._make_bootmode_uefi.systemd-boot \
                           "${WORK_DIR}"/base._make_boot_on_fat \
                           "${WORK_DIR}"/base._make_boot_on_iso9660
            fi

            # Invalidate downstream packaging markers only
            sudo rm -f "${WORK_DIR}"/base._mkairootfs_squashfs \
                       "${WORK_DIR}"/base._prepare_airootfs_image \
                       "${WORK_DIR}"/base._make_iso_image \
                       "${WORK_DIR}"/iso._build_iso_image \
                       "${WORK_DIR}"/build._build_buildmode_iso \
                       "${WORK_DIR}"/iso/hyperxos/x86_64/airootfs.sfs*
        else
            echo "  -> [CACHE MISS] Inisialisasi workspace baru (perubahan paket atau build pertama)..."
            if [ -d "${WORK_DIR}" ]; then
                sudo umount -R "${WORK_DIR}" 2>/dev/null || true
                sudo rm -rf "${WORK_DIR}"
            fi
            sudo install -d -m 0777 "${WORK_DIR}"
        fi
        ;;
esac

# Step 3: Run mkarchiso
echo "==> [3/4] Building ISO with mkarchiso..."
echo "  -> Workdir: ${WORK_DIR}"
echo "  -> Output:  ${OUT_DIR}"
echo "  -> Profile: ${PROFILE_DIR}"

BUILD_LOG="${WORK_DIR}/mkarchiso-${MODE}.log"

# Run mkarchiso with sudo and log output
if [ "${MODE}" == "prod" ] || [ "${MODE}" == "production" ]; then
    # In production mode: enforce strict zero-error, zero-conflict policy
    sudo mkarchiso -v -w "${WORK_DIR}" -o "${OUT_DIR}" "${PROFILE_DIR}" 2>&1 | tee "${BUILD_LOG}"

    # Check for package conflict warnings or fatal errors in log
    if grep -qi -E "conflicting files|error: failed to commit transaction|unresolvable dependencies" "${BUILD_LOG}"; then
        echo "[-] ERROR: Production build failed strict check! Detected conflicts or transaction errors in log." >&2
        exit 1
    fi
else
    # In development mode: fast build
    sudo mkarchiso -v -w "${WORK_DIR}" -o "${OUT_DIR}" "${PROFILE_DIR}"
    # Save package hash for cache validation on future dev builds
    echo "${CURRENT_PKG_HASH}" | sudo tee "${HASH_FILE}" >/dev/null 2>&1 || true
fi

# Fix file ownership of output directory
CURRENT_USER="$(id -u -n)"
CURRENT_GROUP="$(id -g -n)"
echo "==> [4/4] Setting ownership of built artifacts in ${OUT_DIR}..."
sudo chown -R "${CURRENT_USER}:${CURRENT_GROUP}" "${OUT_DIR}"

# Step 4: Verification and Reporting
LATEST_ISO="$(ls -t "${OUT_DIR}"/hyperxos-*.iso 2>/dev/null | head -n 1 || true)"

if [ -z "${LATEST_ISO}" ] || [ ! -f "${LATEST_ISO}" ]; then
    echo "[-] ERROR: Expected ISO was not created in ${OUT_DIR}!" >&2
    exit 1
fi

ISO_SIZE="$(stat -c%s "${LATEST_ISO}")"
ISO_SIZE_MB=$(( ISO_SIZE / 1024 / 1024 ))
echo "=================================================="
echo "  HyperXOS ISO Build Completed Successfully!"
echo "  Mode:      ${MODE}"
echo "  Artifact:  ${LATEST_ISO}"
echo "  File Size: ${ISO_SIZE_MB} MB (${ISO_SIZE} bytes)"

if [ "${MODE}" == "prod" ] || [ "${MODE}" == "production" ]; then
    if [ "${ISO_SIZE}" -ge "${MAX_PROD_ISO_BYTES}" ]; then
        echo "[-] ERROR: Production ISO size (${ISO_SIZE} bytes / ${ISO_SIZE_MB} MB) EXCEEDS 2.0 GB limit (${MAX_PROD_ISO_BYTES} bytes)!" >&2
        exit 1
    fi
    echo "[✓] Production size check PASSED: strictly under 2.0 GB (${ISO_SIZE} < ${MAX_PROD_ISO_BYTES})."
fi

echo "=================================================="
ls -lh "${LATEST_ISO}"
