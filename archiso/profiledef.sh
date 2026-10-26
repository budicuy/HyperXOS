#!/usr/bin/env bash
# shellcheck disable=SC2034
# HyperXOS Archiso Profile Definition

iso_name="hyperxos"
iso_label="HYPERXOS_$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="HyperXOS Linux <https://github.com/budicuy/HyperXOS>"
iso_application="HyperXOS Live/Installation Media"
iso_version="$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)"
install_dir="hyperxos"
bootmodes=('bios.syslinux'
           'uefi.systemd-boot')
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'zstd' '-Xcompression-level' '1' '-b' '256K' '-no-xattrs')
bootstrap_tarball_compression=('zstd' '-c' '-T0' '--auto-threads=logical' '--long' '-19')

file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/etc/gshadow"]="0:0:400"
  ["/etc/sudoers.d"]="0:0:750"
  ["/etc/sudoers.d/00-hyperxos-live"]="0:0:440"
  ["/etc/polkit-1/rules.d"]="0:0:755"
  ["/etc/polkit-1/rules.d/49-hyperxos-live.rules"]="0:0:644"
  ["/root"]="0:0:750"
  ["/root/.automated_script.sh"]="0:0:755"
  ["/root/.gnupg"]="0:0:700"
  ["/home/hyperxos"]="1000:1000:750"
  ["/home/hyperxos/.config"]="1000:1000:750"
  ["/home/hyperxos/.config/kdeglobals"]="1000:1000:644"
  ["/home/hyperxos/.config/plasma-org.kde.plasma.desktop-appletsrc"]="1000:1000:644"
  ["/home/hyperxos/Desktop/hyper-x-installer.desktop"]="1000:1000:755"
  ["/home/hyperxos/Desktop/hyperxtools.desktop"]="1000:1000:755"
  ["/usr/share/applications/hyper-x-installer.desktop"]="0:0:755"
  ["/usr/local/bin/launch-installer"]="0:0:755"
)
