use std::fs;
use std::path::Path;
use std::process::Command;
use crate::core::filesystem::{FilesystemContext, FilesystemManager};

pub struct LimineBootloader;

impl LimineBootloader {
    /// Deploy Limine EFI binaries, write theme & boot entries, register NVRAM, and unmount
    pub fn install<F>(
        disk_path: &str,
        fs_ctx: &FilesystemContext,
        mut progress_callback: F,
    ) -> Result<(), String>
    where
        F: FnMut(&str),
    {
        progress_callback("Setting up Limine Bootloader in /boot (ESP)...");

        // 1. Ensure target EFI directories exist
        let efi_boot_dir = "/mnt/boot/EFI/BOOT";
        let efi_limine_dir = "/mnt/boot/EFI/limine";
        let boot_limine_dir = "/mnt/boot/limine";
        fs::create_dir_all(efi_boot_dir).map_err(|e| format!("Failed to create EFI/BOOT: {}", e))?;
        fs::create_dir_all(efi_limine_dir).map_err(|e| format!("Failed to create EFI/limine: {}", e))?;
        fs::create_dir_all(boot_limine_dir).map_err(|e| format!("Failed to create boot/limine: {}", e))?;

        // 2. Deploy BOOTX64.EFI binary from target or host
        progress_callback("Deploying Limine EFI binaries (BOOTX64.EFI)...");
        let candidates = [
            "/mnt/usr/share/limine/BOOTX64.EFI",
            "/usr/share/limine/BOOTX64.EFI",
        ];

        let mut deployed = false;
        for src in candidates {
            if Path::new(src).exists() {
                let _ = fs::copy(src, format!("{}/BOOTX64.EFI", efi_boot_dir));
                let _ = fs::copy(src, format!("{}/BOOTX64.EFI", efi_limine_dir));
                deployed = true;
                break;
            }
        }
        if !deployed {
            return Err("Limine EFI binary BOOTX64.EFI not found in /usr/share/limine".to_string());
        }

        // Deploy limine-bios.sys if present
        let bios_candidates = [
            "/mnt/usr/share/limine/limine-bios.sys",
            "/usr/share/limine/limine-bios.sys",
        ];
        for src in bios_candidates {
            if Path::new(src).exists() {
                let _ = fs::copy(src, "/mnt/boot/limine-bios.sys");
                let _ = fs::copy(src, format!("{}/limine-bios.sys", boot_limine_dir));
                break;
            }
        }

        // 3. Compose limine.conf
        progress_callback("Generating Limine configuration with HyperXOS styling...");
        let mut conf = String::new();
        conf.push_str("timeout: 5\n");
        conf.push_str("default_entry: 1\n");
        conf.push_str("remember_last_entry: yes\n\n");

        conf.push_str("# HyperXOS Limine Theme\n");
        conf.push_str("term_palette: 1e1e2e;f38ba8;a6e3a1;f9e2af;89b4fa;f5c2e7;94e2d5;cdd6f4\n");
        conf.push_str("term_palette_bright: 585b70;f38ba8;a6e3a1;f9e2af;89b4fa;f5c2e7;94e2d5;cdd6f4\n");
        conf.push_str("term_background: 1e1e2e\n");
        conf.push_str("term_foreground: cdd6f4\n");
        conf.push_str("term_background_bright: 313244\n");
        conf.push_str("term_foreground_bright: cdd6f4\n\n");

        let cmdline = format!(
            "root=UUID={} rootflags=subvol=@ rw quiet splash zswap.enabled=0",
            fs_ctx.root_uuid
        );

        // Check for ucode
        let intel_ucode = Path::new("/mnt/boot/intel-ucode.img").exists();
        let amd_ucode = Path::new("/mnt/boot/amd-ucode.img").exists();

        // Standard Entry
        conf.push_str("/HyperXOS Linux\n");
        conf.push_str("    protocol: linux\n");
        conf.push_str("    kernel_path: boot():/vmlinuz-linux\n");
        if intel_ucode {
            conf.push_str("    module_path: boot():/intel-ucode.img\n");
        }
        if amd_ucode {
            conf.push_str("    module_path: boot():/amd-ucode.img\n");
        }
        conf.push_str("    module_path: boot():/initramfs-linux.img\n");
        conf.push_str(&format!("    cmdline: {}\n\n", cmdline));

        // Fallback Entry
        conf.push_str("/HyperXOS Linux (fallback initramfs)\n");
        conf.push_str("    protocol: linux\n");
        conf.push_str("    kernel_path: boot():/vmlinuz-linux\n");
        if intel_ucode {
            conf.push_str("    module_path: boot():/intel-ucode.img\n");
        }
        if amd_ucode {
            conf.push_str("    module_path: boot():/amd-ucode.img\n");
        }
        conf.push_str("    module_path: boot():/initramfs-linux-fallback.img\n");
        conf.push_str(&format!("    cmdline: {}\n\n", cmdline));

        // Write configuration to all standard Limine search paths
        let conf_paths = [
            "/mnt/boot/limine.conf",
            "/mnt/boot/EFI/BOOT/limine.conf",
            "/mnt/boot/EFI/limine/limine.conf",
            "/mnt/boot/limine/limine.conf",
        ];
        for path in conf_paths {
            let _ = fs::write(path, &conf);
        }

        // Write /etc/default/limine
        let _ = fs::create_dir_all("/mnt/etc/default");
        let default_limine = format!(
            "ESP_PATH=\"/boot\"\nKERNEL_CMDLINE[default]+=\"{}\"\nBOOT_ORDER=\"*, *lts, *fallback, Snapshots\"\nSKIP_UEFI=yes\nENABLE_LIMINE_FALLBACK=yes\n",
            cmdline
        );
        let _ = fs::write("/mnt/etc/default/limine", default_limine);

        // 4. Register Limine in UEFI NVRAM via efibootmgr
        progress_callback("Registering HyperXOS UEFI boot entry in NVRAM (efibootmgr)...");
        let _ = Command::new("efibootmgr")
            .args([
                "--create",
                "--disk",
                disk_path,
                "--part",
                "1",
                "--label",
                "HyperXOS",
                "--loader",
                r"\EFI\BOOT\BOOTX64.EFI",
            ])
            .status();

        // 5. Sync and Clean Unmount
        progress_callback("Flushing caches (sync) and unmounting /mnt...");
        FilesystemManager::unmount_all()?;

        progress_callback("Limine bootloader installation and system unmount complete!");
        Ok(())
    }
}
