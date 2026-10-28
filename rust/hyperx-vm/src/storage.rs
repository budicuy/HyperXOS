use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::VmConfig;
use crate::util::run_stream;

pub struct StorageManager;

impl StorageManager {
    /// Find latest built ISO image in out/
    pub fn find_latest_iso(root: &Path) -> Option<PathBuf> {
        let out_dir = root.join("out");
        if !out_dir.exists() {
            return None;
        }

        let mut isos: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();
        if let Ok(entries) = fs::read_dir(&out_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("hyperxos-") && name.ends_with(".iso") {
                        let mtime = entry
                            .metadata()
                            .and_then(|m| m.modified())
                            .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                        isos.push((path, mtime));
                    }
                }
            }
        }

        isos.sort_by_key(|a| std::cmp::Reverse(a.1));
        isos.first().map(|(p, _)| p.clone())
    }

    /// Ensure 40GB virtual disk exists
    pub fn ensure_disk_exists(config: &VmConfig) -> Result<(), String> {
        if !config.disk_image.exists() {
            println!(
                "==> [hyperx-vm] Creating virtual target disk {} ({}) ...",
                config.disk_image.display(),
                config.disk_size
            );
            let mut cmd = Command::new(&config.qemu_img_bin);
            cmd.args([
                "create",
                "-f",
                "qcow2",
                config.disk_image.to_str().unwrap(),
                &config.disk_size,
            ]);
            run_stream(&mut cmd, "qemu-img create")?;
        }
        Ok(())
    }

    /// Reset 40GB virtual disk to clean state
    pub fn reset_disk(config: &VmConfig) -> Result<(), String> {
        println!(
            "==> [hyperx-vm] Resetting virtual disk {} to clean {} image...",
            config.disk_image.display(),
            config.disk_size
        );
        let _ = fs::remove_file(&config.disk_image);

        let mut cmd = Command::new(&config.qemu_img_bin);
        cmd.args([
            "create",
            "-f",
            "qcow2",
            config.disk_image.to_str().unwrap(),
            &config.disk_size,
        ]);
        run_stream(&mut cmd, "qemu-img create")?;
        Ok(())
    }

    /// Clean socket or temp runtime files in /tmp
    pub fn clean_temp_files() {
        let temp_sockets = ["/tmp/qga.sock", "/tmp/hmp.sock", "/tmp/qmp.sock"];
        for s in temp_sockets {
            let _ = fs::remove_file(s);
        }
    }

    /// Ensure local UEFI NVRAM variables file exists
    pub fn ensure_nvram(config: &VmConfig) {
        if !config.ovmf_vars_local.exists() {
            if let Some(ref orig) = config.ovmf_vars_orig {
                println!(
                    "==> [hyperx-vm] Initializing local UEFI NVRAM ({}) ...",
                    config.ovmf_vars_local.display()
                );
                let _ = fs::copy(orig, &config.ovmf_vars_local);
            }
        }
    }

    /// Reset local UEFI NVRAM variables to original template
    pub fn reset_nvram(config: &VmConfig) {
        if let Some(ref orig) = config.ovmf_vars_orig {
            println!("==> [hyperx-vm] Resetting local UEFI NVRAM template...");
            let _ = fs::copy(orig, &config.ovmf_vars_local);
        }
    }
}
