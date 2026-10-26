use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct KernelEntry {
    pub name: String,
    pub package_name: String,
    pub description: String,
    pub installed: bool,
    pub is_running: bool,
    pub installed_version: String,
}

pub struct KernelManager;

impl KernelManager {
    pub fn get_catalog() -> Vec<KernelEntry> {
        let running_release = fs::read_to_string("/proc/sys/kernel/osrelease")
            .unwrap_or_default()
            .trim()
            .to_lowercase();

        let installed_kernels = Self::scan_installed_kernels();

        let catalog_defs = vec![
            (
                "Linux CachyOS (BORE / v3)",
                "linux-cachyos",
                "Kernel CachyOS standar dengan patch BORE scheduler & kompilasi optimasi x86-64-v3.",
            ),
            (
                "Linux CachyOS Sched-Ext",
                "linux-cachyos-sched-ext",
                "Kernel CachyOS dengan BPF extensible scheduler (SCX) untuk performa gaming latensi ultra-rendah.",
            ),
            (
                "Linux CachyOS BORE",
                "linux-cachyos-bore",
                "Burst-Oriented Response Enhancer (BORE) scheduler untuk interaktivitas desktop maksimal.",
            ),
            (
                "Linux CachyOS LTS",
                "linux-cachyos-lts",
                "Kernel Long Term Support dengan optimasi performa dan kestabilan jangka panjang CachyOS.",
            ),
            (
                "Linux CachyOS Hardened",
                "linux-cachyos-hardened",
                "Kernel CachyOS dengan proteksi keamanan tinggi tanpa mengorbankan performa komputasi.",
            ),
            (
                "Linux Zen",
                "linux-zen",
                "Kernel hasil kolaborasi komunitas Arch Linux untuk performa desktop dan multitasking harian.",
            ),
            (
                "Linux Arch Standar",
                "linux",
                "Kernel Linux upstream vanilla standar resmi dari repositori Arch Linux.",
            ),
        ];

        let mut result = Vec::new();

        for (name, pkg, desc) in catalog_defs {
            let is_installed = installed_kernels.contains_key(pkg);
            let installed_ver = installed_kernels.get(pkg).cloned().unwrap_or_default();

            // Match running release
            let is_running = if pkg == "linux-cachyos-sched-ext" {
                running_release.contains("sched-ext") || running_release.contains("scx")
            } else if pkg == "linux-cachyos-bore" {
                running_release.contains("bore") && !running_release.contains("sched-ext")
            } else if pkg == "linux-cachyos-lts" {
                running_release.contains("cachyos-lts") || (running_release.contains("cachyos") && running_release.contains("lts"))
            } else if pkg == "linux-cachyos-hardened" {
                running_release.contains("cachyos-hardened")
            } else if pkg == "linux-cachyos" {
                running_release.contains("cachyos") && !running_release.contains("sched-ext") && !running_release.contains("bore") && !running_release.contains("lts") && !running_release.contains("hardened")
            } else if pkg == "linux-zen" {
                running_release.contains("zen")
            } else if pkg == "linux" {
                running_release.starts_with("6.") && !running_release.contains("cachyos") && !running_release.contains("zen")
            } else {
                false
            };

            result.push(KernelEntry {
                name: name.to_string(),
                package_name: pkg.to_string(),
                description: desc.to_string(),
                installed: is_installed,
                is_running,
                installed_version: installed_ver,
            });
        }

        result
    }

    pub fn get_running_kernel_info() -> (String, String) {
        let release = fs::read_to_string("/proc/sys/kernel/osrelease")
            .unwrap_or_else(|_| "6.12.0".to_string())
            .trim()
            .to_string();

        let lower = release.to_lowercase();
        let name = if lower.contains("cachyos-sched-ext") {
            "linux-cachyos-sched-ext"
        } else if lower.contains("cachyos-bore") {
            "linux-cachyos-bore"
        } else if lower.contains("cachyos-lts") {
            "linux-cachyos-lts"
        } else if lower.contains("cachyos") {
            "linux-cachyos"
        } else if lower.contains("zen") {
            "linux-zen"
        } else {
            "linux"
        };

        (name.to_string(), release)
    }

    fn scan_installed_kernels() -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        let local_path = "/var/lib/pacman/local";

        if let Ok(entries) = fs::read_dir(local_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("linux") && !name.contains("-headers") && !name.contains("-firmware") && !name.contains("-docs") {
                    // split at last two dashes: pkgname-ver-rel
                    let parts: Vec<&str> = name.split('-').collect();
                    if parts.len() >= 3 {
                        let rel = parts.last().unwrap_or(&"");
                        let ver = parts.get(parts.len() - 2).unwrap_or(&"");
                        let pkg_name = parts[..parts.len() - 2].join("-");
                        let full_ver = format!("{}-{}", ver, rel);
                        map.insert(pkg_name, full_ver);
                    }
                }
            }
        }

        map
    }

    pub fn execute_action<F>(cmd: &str, args: &[&str], log_cb: F) -> Result<(), String>
    where
        F: Fn(String) + Send + 'static,
    {
        use std::io::{BufRead, BufReader};
        use std::process::Stdio;

        log_cb(format!("$ {} {}\n", cmd, args.join(" ")));

        let mut child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Gagal menjalankan {}: {}", cmd, e))?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                log_cb(format!("{}\n", line));
            }
        }

        let status = child
            .wait()
            .map_err(|e| format!("Error saat menunggu proses: {}", e))?;

        if status.success() {
            log_cb("[✓] Perintah selesai dengan sukses.\n".to_string());
            Ok(())
        } else {
            let err = format!("[✗] Perintah keluar dengan kode error: {:?}\n", status.code());
            log_cb(err.clone());
            Err(err)
        }
    }
}
