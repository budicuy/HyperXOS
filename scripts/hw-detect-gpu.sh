#!/usr/bin/env bash
# HyperXOS Multi-Tier GPU and Hardware Detection Engine
set -euo pipefail

TARGET_ROOT="${1:-/}"
DRY_RUN="${DRY_RUN:-false}"

log() {
    echo "==> [HyperXOS-HW] $*"
}

run_in_target() {
    if [ "${TARGET_ROOT}" == "/" ]; then
        if [ "${DRY_RUN}" == "true" ]; then
            log "[DRY-RUN] $*"
        else
            "$@"
        fi
    else
        if [ "${DRY_RUN}" == "true" ]; then
            log "[DRY-RUN in ${TARGET_ROOT}] $*"
        else
            chroot "${TARGET_ROOT}" "$@"
        fi
    fi
}

log "Starting hardware and GPU auto-detection (Target: ${TARGET_ROOT})..."

# ---------------------------------------------------------
# 1. Virtualization Detection
# ---------------------------------------------------------
VIRT_TYPE="none"
if command -v systemd-detect-virt >/dev/null 2>&1; then
    VIRT_TYPE=$(systemd-detect-virt || echo "none")
fi

log "Virtualization environment detected: ${VIRT_TYPE}"

case "${VIRT_TYPE}" in
    kvm|qemu)
        log "Configuring KVM/QEMU guest utilities (spice-vdagent, qemu-guest-agent)..."
        run_in_target pacman -S --needed --noconfirm qemu-guest-agent spice-vdagent || true
        run_in_target systemctl enable qemu-guest-agent.service spice-vdagentd.service || true
        ;;
    oracle)
        log "Configuring VirtualBox guest utilities..."
        run_in_target pacman -S --needed --noconfirm virtualbox-guest-utils-nox || true
        run_in_target systemctl enable vboxservice.service || true
        ;;
    vmware)
        log "Configuring VMware guest utilities..."
        run_in_target pacman -S --needed --noconfirm open-vm-tools || true
        run_in_target systemctl enable vmtoolsd.service || true
        ;;
    *)
        log "Physical machine or standard environment."
        ;;
esac

# ---------------------------------------------------------
# 2. GPU Detection via sysfs / lspci
# ---------------------------------------------------------
HAS_NVIDIA=false
HAS_INTEL=false
HAS_AMD=false
GPU_COUNT=0

if command -v lspci >/dev/null 2>&1; then
    while IFS= read -r line; do
        GPU_COUNT=$((GPU_COUNT + 1))
        if echo "${line}" | grep -qi "NVIDIA"; then
            HAS_NVIDIA=true
        fi
        if echo "${line}" | grep -qi "Intel"; then
            HAS_INTEL=true
        fi
        if echo "${line}" | grep -qi -E "AMD|Advanced Micro Devices|ATI"; then
            HAS_AMD=true
        fi
    done < <(lspci -nn | grep -E "VGA compatible controller|3D controller" || true)
fi

log "GPU Discovery Summary: Count=${GPU_COUNT}, Intel=${HAS_INTEL}, AMD=${HAS_AMD}, NVIDIA=${HAS_NVIDIA}"

# ---------------------------------------------------------
# 3. Intel / AMD GPU Stack
# ---------------------------------------------------------
if [ "${HAS_INTEL}" == "true" ]; then
    log "Configuring Intel Graphics stack (Mesa, Vulkan, Media Driver)..."
    run_in_target pacman -S --needed --noconfirm mesa lib32-mesa vulkan-intel lib32-vulkan-intel intel-media-driver || true
fi

if [ "${HAS_AMD}" == "true" ]; then
    log "Configuring AMD Graphics stack (Mesa, RADV Vulkan)..."
    run_in_target pacman -S --needed --noconfirm mesa lib32-mesa vulkan-radeon lib32-vulkan-radeon || true
fi

# ---------------------------------------------------------
# 4. NVIDIA Multi-Tier Driver Classification
# ---------------------------------------------------------
if [ "${HAS_NVIDIA}" == "true" ]; then
    log "Analyzing NVIDIA Architecture..."

    # Check Device IDs via lspci
    NVIDIA_INFO=$(lspci -d 10de: -nn || true)
    log "NVIDIA Hardware Signature: ${NVIDIA_INFO}"

    # Determine architecture: Turing (0x1e00-0x21ff, 0x1f00-0x1fff), Ampere (0x2200-0x26ff),
    # Ada Lovelace (0x2700-0x28ff), Blackwell (0x2900+)
    # Pascal: 0x1b00-0x1dff (GTX 10xx, Titan Xp)
    # Maxwell: 0x1300-0x17ff (GTX 9xx, GTX 750Ti)
    # Kepler: 0x0fc0-0x11ff (GTX 6xx, GTX 7xx)

    ARCH="turing_or_newer"
    if echo "${NVIDIA_INFO}" | grep -qE "\[10de:(1b[0-9a-f]{2}|1c[0-9a-f]{2}|1d[0-9a-f]{2}|13[0-9a-f]{2}|14[0-9a-f]{2}|16[0-9a-f]{2}|17[0-9a-f]{2})\]"; then
        ARCH="pascal_maxwell"
    elif echo "${NVIDIA_INFO}" | grep -qE "\[10de:(0f[0-9a-f]{2}|10[0-9a-f]{2}|11[0-9a-f]{2}|12[0-9a-f]{2})\]"; then
        ARCH="kepler_or_older"
    fi

    log "Classified NVIDIA Architecture: ${ARCH}"

    case "${ARCH}" in
        turing_or_newer)
            log "Installing official open-source kernel modules: nvidia-open stack..."
            run_in_target pacman -S --needed --noconfirm nvidia-open nvidia-utils lib32-nvidia-utils nvidia-settings opencl-nvidia || true

            # Configure early KMS in mkinitcpio.conf
            log "Configuring early KMS for NVIDIA Wayland in mkinitcpio..."
            if [ -f "${TARGET_ROOT}/etc/mkinitcpio.conf" ]; then
                sed -i 's/^MODULES=(\(.*\))/MODULES=(\1 nvidia nvidia_modeset nvidia_uvm nvidia_drm)/' "${TARGET_ROOT}/etc/mkinitcpio.conf" || true
            fi

            # Add kernel parameters for DRM KMS and framebuffer device
            log "Configuring kernel parameters: nvidia_drm.modeset=1 nvidia_drm.fbdev=1..."
            if [ -d "${TARGET_ROOT}/boot/loader/entries" ]; then
                for entry in "${TARGET_ROOT}/boot/loader/entries"/*.conf; do
                    [ -f "${entry}" ] || continue
                    if ! grep -q "nvidia_drm.modeset=1" "${entry}"; then
                        sed -i '/^options/ s/$/ nvidia_drm.modeset=1 nvidia_drm.fbdev=1/' "${entry}"
                    fi
                done
            fi

            if [ -f "${TARGET_ROOT}/etc/default/grub" ]; then
                sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="\(.*\)"/GRUB_CMDLINE_LINUX_DEFAULT="\1 nvidia_drm.modeset=1 nvidia_drm.fbdev=1"/' "${TARGET_ROOT}/etc/default/grub" || true
            fi

            # Limine bootloader support for NVIDIA
            for limine_cfg in "${TARGET_ROOT}/boot/limine.conf" \
                              "${TARGET_ROOT}/boot/EFI/BOOT/limine.conf" \
                              "${TARGET_ROOT}/boot/EFI/limine/limine.conf"; do
                if [ -f "${limine_cfg}" ]; then
                    if ! grep -q "nvidia_drm.modeset=1" "${limine_cfg}"; then
                        sed -i '/cmdline:/ s/$/ nvidia_drm.modeset=1 nvidia_drm.fbdev=1/' "${limine_cfg}"
                    fi
                fi
            done
            ;;

        pascal_maxwell|kepler_or_older)
            log "Legacy NVIDIA GPU detected (${ARCH})."
            log "Applying pure open-source Nouveau graphics stack (Mesa Gallium3D + Wayland native)..."
            run_in_target pacman -S --needed --noconfirm mesa lib32-mesa || true
            ;;
    esac

    # Laptop Hybrid (Optimus/PRIME) Handling
    if [ ${GPU_COUNT} -ge 2 ] && { [ "${HAS_INTEL}" == "true" ] || [ "${HAS_AMD}" == "true" ]; }; then
        log "Dual-GPU Laptop configuration detected! Installing PRIME utilities..."
        run_in_target pacman -S --needed --noconfirm nvidia-prime switcheroo-control || true
        run_in_target systemctl enable switcheroo-control.service || true

        # Dynamic power management
        mkdir -p "${TARGET_ROOT}/etc/modprobe.d"
        cat << 'EOF' > "${TARGET_ROOT}/etc/modprobe.d/nvidia-pm.conf"
options nvidia "NVreg_DynamicPowerManagement=0x02"
EOF
    fi
fi

log "Hardware and GPU auto-detection completed successfully."
