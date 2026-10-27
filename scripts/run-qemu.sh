#!/usr/bin/env bash
# HyperXOS QEMU Virtualization Runner Script
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

MODE="${1:-dev}"
DISK_IMAGE="${PROJECT_ROOT}/hyperxos-install.qcow2"
DISK_SIZE="40G"
OVMF_CODE="/usr/share/edk2/x64/OVMF_CODE.4m.fd"
OVMF_VARS_ORIG="/usr/share/edk2/x64/OVMF_VARS.4m.fd"
OVMF_VARS_LOCAL="${PROJECT_ROOT}/OVMF_VARS.4m.fd"

# Find latest ISO image in out/
shopt -s nullglob
ISO_FILES=("${PROJECT_ROOT}/out"/hyperxos-*.iso)
shopt -u nullglob

LATEST_ISO=""
if [ ${#ISO_FILES[@]} -gt 0 ]; then
    LATEST_ISO=$(ls -t "${PROJECT_ROOT}/out"/hyperxos-*.iso | head -n 1)
fi

# Ensure OVMF VARS local copy exists
if [ ! -f "${OVMF_VARS_LOCAL}" ] && [ -f "${OVMF_VARS_ORIG}" ]; then
    echo "==> [HyperXOS] Creating clean UEFI NVRAM variables (${OVMF_VARS_LOCAL})..."
    cp "${OVMF_VARS_ORIG}" "${OVMF_VARS_LOCAL}"
fi

# Base QEMU flags
QEMU_CMD=(qemu-system-x86_64)

# KVM hardware acceleration
if [ -e "/dev/kvm" ] && [ -r "/dev/kvm" ] && [ -w "/dev/kvm" ]; then
    QEMU_CMD+=(-enable-kvm -cpu host)
else
    echo "  [!] Warning: /dev/kvm not writable, falling back to TCG emulation..."
    QEMU_CMD+=(-cpu max)
fi

QEMU_CMD+=(
    -smp 4
    -m 4096
    -netdev user,id=net0
    -device virtio-net-pci,netdev=net0,romfile=
    -chardev socket,path=/tmp/qga.sock,server=on,wait=off,id=qga0
    -device virtio-serial -device virtserialport,chardev=qga0,name=org.qemu.guest_agent.0
    -monitor unix:/tmp/hmp.sock,server=on,wait=off
    -qmp unix:/tmp/qmp.sock,server=on,wait=off
)

# Robust Display Backend Detection (Prefer SDL with OpenGL 3D VirGL acceleration)
if [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; then
    # Test if SDL with OpenGL is supported
    if qemu-system-x86_64 -display sdl,gl=on -device virtio-vga-gl -help >/dev/null 2>&1; then
        echo "==> [HyperXOS] Using hardware-accelerated SDL OpenGL (virgl 3D) display backend."
        QEMU_CMD+=(-device virtio-vga-gl -display sdl,gl=on)
    else
        echo "==> [HyperXOS] Using standard GTK display backend."
        QEMU_CMD+=(-device virtio-vga -display gtk)
    fi
else
    QEMU_CMD+=(-device virtio-vga -display none)
fi

# Audio Device Setup
if command -v pipewire >/dev/null 2>&1 || [ -n "${PULSE_SERVER:-}" ]; then
    QEMU_CMD+=(-audiodev pa,id=snd0 -device intel-hda -device hda-duplex,audiodev=snd0)
fi

# Attach UEFI firmware
if [ -f "${OVMF_CODE}" ] && [ -f "${OVMF_VARS_LOCAL}" ]; then
    QEMU_CMD+=(
        -drive if=pflash,format=raw,readonly=on,file="${OVMF_CODE}"
        -drive if=pflash,format=raw,file="${OVMF_VARS_LOCAL}"
    )
fi

case "${MODE}" in
    dev|live)
        if [ -z "${LATEST_ISO}" ] || [ ! -f "${LATEST_ISO}" ]; then
            echo "Error: No HyperXOS ISO found in ${PROJECT_ROOT}/out/. Please run 'hyperx build' first." >&2
            exit 1
        fi
        # Always reset UEFI NVRAM for live dev mode so it cleanly boots the ISO
        if [ -f "${OVMF_VARS_ORIG}" ]; then
            cp "${OVMF_VARS_ORIG}" "${OVMF_VARS_LOCAL}"
        fi
        # Ensure a virtual disk is present so installer can test partitioning
        if [ ! -f "${DISK_IMAGE}" ]; then
            qemu-img create -f qcow2 "${DISK_IMAGE}" "${DISK_SIZE}"
        fi
        echo "==> [HyperXOS] Booting Live ISO in dev mode: ${LATEST_ISO}"
        "${QEMU_CMD[@]}" \
            -drive file="${DISK_IMAGE}",format=qcow2,if=virtio \
            -drive file="${LATEST_ISO}",media=cdrom,readonly=on \
            -boot order=d,menu=on
        ;;

    install|vm-install)
        if [ -z "${LATEST_ISO}" ] || [ ! -f "${LATEST_ISO}" ]; then
            echo "Error: No HyperXOS ISO found in ${PROJECT_ROOT}/out/. Please run 'hyperx build' first." >&2
            exit 1
        fi
        echo "==> [HyperXOS] Resetting virtual disk ${DISK_IMAGE} to a clean ${DISK_SIZE} image..."
        rm -f "${DISK_IMAGE}"
        qemu-img create -f qcow2 "${DISK_IMAGE}" "${DISK_SIZE}"

        # Reset UEFI NVRAM for a fresh installation
        if [ -f "${OVMF_VARS_ORIG}" ]; then
            cp "${OVMF_VARS_ORIG}" "${OVMF_VARS_LOCAL}"
        fi

        echo "==> [HyperXOS] Booting installer environment with target disk attached..."
        "${QEMU_CMD[@]}" \
            -drive file="${DISK_IMAGE}",format=qcow2,if=virtio \
            -drive file="${LATEST_ISO}",media=cdrom,readonly=on \
            -boot order=d,menu=on
        ;;

    run)
        if [ ! -f "${DISK_IMAGE}" ]; then
            echo "Error: Virtual disk ${DISK_IMAGE} not found. Please install HyperXOS first using 'hyperx install'." >&2
            exit 1
        fi
        echo "==> [HyperXOS] Booting installed system from virtual disk: ${DISK_IMAGE}"
        "${QEMU_CMD[@]}" \
            -drive file="${DISK_IMAGE}",format=qcow2,if=virtio \
            -boot order=c,menu=on
        ;;

    test)
        if [ -z "${LATEST_ISO}" ] || [ ! -f "${LATEST_ISO}" ]; then
            echo "Error: No HyperXOS ISO found in ${PROJECT_ROOT}/out/. Please run 'hyperx build' first." >&2
            exit 1
        fi
        echo "==> [HyperXOS] Running automated non-interactive boot verification test..."
        "${PROJECT_ROOT}/tests/qemu-check.sh"
        ;;

    *)
        echo "Usage: $0 [dev|install|run|test]" >&2
        exit 1
        ;;
esac
