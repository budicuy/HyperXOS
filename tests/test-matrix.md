# HyperXOS Test Matrix & Quality Assurance Plan

This test matrix outlines the verification protocol for HyperXOS ISO builds, Calamares installer behavior, GPU driver detection, and post-installation stability.

## 1. Automated Verification (`make test`)
| Check Item | Acceptance Criteria | Status |
| :--- | :--- | :--- |
| ISO Image Format | Hybrid ISO 9660 with El Torito & GPT/MBR EFI system partition | Ready |
| Minimum ISO Size | Must be >= 1.0 GB (desktop image with Plasma 6) | Ready |
| Kernel Smoke Test | QEMU boots `vmlinuz-linux` and `initramfs-linux.img` headlessly without panic | Ready |
| Checksum Integrity | SHA256 and B2SUM checksums match ISO file | Ready |

## 2. Live Environment Verification (`make dev`)
| Check Item | Acceptance Criteria | Status |
| :--- | :--- | :--- |
| UEFI Boot | Boots cleanly using systemd-boot EFI menu entries | Pending build |
| BIOS Boot | Boots cleanly using syslinux menu | Pending build |
| SDDM Autologin | Autologins into user `hyperxos` in KDE Plasma 6 Wayland session | Pending build |
| Wayland Compositor | `kwin_wayland` operates with 3D virgl acceleration | Pending build |
| Network | NetworkManager auto-connects to DHCP network | Pending build |
| Installer Launcher | `calamares.desktop` launches Calamares via `pkexec` wrapper | Pending build |

## 3. Calamares Installer Verification (`make install`)
| Check Item | Acceptance Criteria | Status |
| :--- | :--- | :--- |
| Clean 40 GB Disk | Virtual disk `hyperxos-install.qcow2` reset and partitioned | Ready |
| Btrfs Layout | Subvolumes `@`, `@home`, `@log`, `@cache`, `@snapshots` created | Configured |
| ESP Layout | FAT32 EFI partition mounted at `/boot` >= 1024 MiB | Configured |
| Dual Unpack | `airootfs.sfs` unpacked + `/boot/vmlinuz-linux` copied to target | Configured |
| Target Sanitization | Live autologin and passwordless sudo purged from target | Configured |
| Bootloader Selection | Chooser offers `systemd-boot` (default), `GRUB`, and `Limine` before summary | Configured |

## 4. Installed System Verification (`make run`)
| Check Item | Acceptance Criteria | Status |
| :--- | :--- | :--- |
| Target Boot | System boots directly from virtual disk without ISO mounted | Pending install |
| User Authentication | SDDM prompts for user credentials configured during install | Pending install |
| AUR Helper | `paru --version` runs cleanly and connects to AUR | Pending install |
| PipeWire Audio | `wpctl status` shows active PipeWire audio node | Pending install |
| GPU Driver State | `hw-detect-gpu` selects correct driver stack for virtual or physical GPU | Ready |
