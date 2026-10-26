# HyperXOS Test Matrix & Quality Assurance Plan

This test matrix outlines the verification protocol for HyperXOS ISO builds, Pure Rust native installer behavior, GPU driver detection, and post-installation stability.

## 1. Automated Verification (`make test`)
| Check Item | Acceptance Criteria | Status |
| :--- | :--- | :--- |
| ISO Image Format | Hybrid ISO 9660 with El Torito & GPT/MBR EFI system partition | PASS (1916 MB) |
| Minimum ISO Size | Must be >= 1.0 GB and < 2.0 GB (desktop image with Plasma 6) | PASS (1.91 GB) |
| Kernel Smoke Test | QEMU boots `vmlinuz-linux` and `initramfs-linux.img` headlessly without panic | PASS (QEMU 8.x) |
| Checksum Integrity | SHA256 and B2SUM checksums match ISO file | PASS |

## 2. Live Environment Verification (`make dev`)
| Check Item | Acceptance Criteria | Status |
| :--- | :--- | :--- |
| UEFI Boot | Boots cleanly using systemd-boot EFI menu entries | PASS (OVMF 4M) |
| BIOS Boot | Boots cleanly using syslinux menu | PASS |
| SDDM Autologin | Autologins into user `hyperxos` in KDE Plasma 6 Wayland session | PASS |
| Wayland Compositor | `kwin_wayland` operates with 3D virgl acceleration | PASS |
| Network | NetworkManager auto-connects to DHCP network | PASS |
| Installer Launcher | `hyper-x-installer.desktop` launches Pure Rust Installer | PASS |

## 3. Pure Rust Installer Verification (`make install`)
| Check Item | Acceptance Criteria | Status |
| :--- | :--- | :--- |
| Clean 40 GB Disk | Virtual disk `hyperxos-install.qcow2` reset and partitioned | PASS |
| ESP Layout | 4096 MiB (4 GB) FAT32 partition mounted at `/boot` (`EF00`) | PASS (`HYPERX_BOOT`) |
| Btrfs Layout | Subvolumes `@` (root) and `@home` (user files) created | PASS (`HYPERXOS_ROOT`) |
| Full Online Pacstrap | Base system + KDE Plasma 6 Wayland installed in one transaction | PASS (Base + Plasma 6) |
| Target Configuration | `/etc/fstab`, user credentials, wheel sudoers, and SDDM Wayland enabled | PASS |
| Bootloader Deployment | Limine EFI binary deployed to `/boot/EFI/BOOT/` and registered in NVRAM | PASS (`BOOTX64.EFI`) |

## 4. Installed System Verification (`make run`)
| Check Item | Acceptance Criteria | Status |
| :--- | :--- | :--- |
| Target Boot | System boots directly from virtual disk using Limine without ISO mounted | PASS (OVMF NVRAM) |
| User Authentication | SDDM Wayland greeter / Autologin logs into user session | PASS (`hyperx`) |
| Desktop Environment | KDE Plasma 6 Wayland (`plasmashell` & `kwin_wayland`) running active | PASS (Plasma 6.x) |
| Audio Stack | PipeWire & WirePlumber running under user session | PASS |
| Filesystem Hierarchy | `/` on `@` subvolume, `/home` on `@home` subvolume, `/boot` on 4GB ESP | PASS (findmnt verified) |
