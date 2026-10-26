use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::gpu::GpuDiagnostic;
use crate::virt::VirtType;

pub struct ConfigApplier {
    target_root: PathBuf,
}

impl ConfigApplier {
    pub fn new<P: AsRef<Path>>(target_root: P) -> Self {
        Self {
            target_root: target_root.as_ref().to_path_buf(),
        }
    }

    /// Apply all configurations for virtualization and GPU stack
    pub fn apply_all(
        &self,
        virt: &VirtType,
        gpu: &GpuDiagnostic,
        install_pkgs: bool,
    ) -> Result<(), String> {
        println!("==> [HyperXOS-HW] Applying hardware stack configurations (Target: {:?})...", self.target_root);

        // 1. Install required packages
        let mut all_pkgs = virt.required_packages();
        for p in &gpu.recommended_packages {
            all_pkgs.push(p.as_str());
        }
        all_pkgs.sort();
        all_pkgs.dedup();

        if install_pkgs && !all_pkgs.is_empty() {
            println!("  -> Installing driver packages: {:?}", all_pkgs);
            self.run_pacman(&all_pkgs)?;
        }

        // 2. Early KMS Kernel Modules
        if !gpu.kernel_modules_early_kms.is_empty() {
            println!("  -> Configuring early KMS modules: {:?}", gpu.kernel_modules_early_kms);
            self.configure_early_kms(&gpu.kernel_modules_early_kms)?;
        }

        // 3. Limine Bootloader Parameters
        if !gpu.kernel_cmdline_params.is_empty() {
            println!("  -> Injecting kernel cmdline parameters: {:?}", gpu.kernel_cmdline_params);
            self.configure_limine_bootloader(&gpu.kernel_cmdline_params)?;
        }

        // 4. Laptop Hybrid Dynamic Power Management
        if gpu.is_hybrid_laptop {
            println!("  -> Configuring NVIDIA Dynamic Power Management for hybrid laptop...");
            self.configure_hybrid_power_management()?;
        }

        // 5. Systemd Services
        let mut all_services = virt.required_services();
        for s in &gpu.enable_services {
            all_services.push(s.as_str());
        }
        all_services.sort();
        all_services.dedup();

        for srv in all_services {
            println!("  -> Enabling systemd service: {}", srv);
            let _ = self.enable_service(srv);
        }

        println!("[✓] Hardware and GPU configuration successfully applied.");
        Ok(())
    }

    fn run_pacman(&self, pkgs: &[&str]) -> Result<(), String> {
        let is_chroot = self.target_root != Path::new("/");
        let mut cmd = if is_chroot {
            let mut c = Command::new("arch-chroot");
            c.arg(&self.target_root);
            c.arg("pacman");
            c
        } else {
            Command::new("pacman")
        };

        cmd.args(["-S", "--needed", "--noconfirm"]);
        for p in pkgs {
            cmd.arg(p);
        }

        let status = cmd.status().map_err(|e| format!("Failed to run pacman: {}", e))?;
        if !status.success() {
            eprintln!("[WARN] Pacman returned non-zero status when installing drivers (may be offline or already up to date).");
        }
        Ok(())
    }

    fn enable_service(&self, service: &str) -> Result<(), String> {
        let is_chroot = self.target_root != Path::new("/");
        let mut cmd = if is_chroot {
            let mut c = Command::new("arch-chroot");
            c.arg(&self.target_root);
            c.arg("systemctl");
            c
        } else {
            Command::new("systemctl")
        };

        cmd.args(["enable", service]);
        let _ = cmd.status();
        Ok(())
    }

    fn configure_early_kms(&self, modules: &[String]) -> Result<(), String> {
        // Method 1: Drop-in config in /etc/mkinitcpio.conf.d/hyperxos-gpu.conf
        let conf_d = self.target_root.join("etc/mkinitcpio.conf.d");
        let _ = fs::create_dir_all(&conf_d);
        let dropin_file = conf_d.join("hyperxos-gpu.conf");
        let dropin_content = format!("MODULES+=({})\n", modules.join(" "));
        let _ = fs::write(&dropin_file, dropin_content);

        // Method 2: Ensure main /etc/mkinitcpio.conf also has it if conf.d not supported
        let mkinit_path = self.target_root.join("etc/mkinitcpio.conf");
        if mkinit_path.exists() {
            if let Ok(content) = fs::read_to_string(&mkinit_path) {
                let mut modified = false;
                let mut new_lines = Vec::new();

                for line in content.lines() {
                    if line.trim_start().starts_with("MODULES=(") && !line.contains(&modules[0]) {
                        // Insert modules safely inside MODULES=(...)
                        if let Some(close_idx) = line.rfind(')') {
                            let (start, _) = line.split_at(close_idx);
                            let added = format!(" {})\n", modules.join(" "));
                            let mut full = start.to_string();
                            full.push_str(&added);
                            new_lines.push(full);
                            modified = true;
                            continue;
                        }
                    }
                    new_lines.push(line.to_string());
                }

                if modified {
                    let _ = fs::write(&mkinit_path, new_lines.join("\n"));
                }
            }
        }

        Ok(())
    }

    fn configure_limine_bootloader(&self, params: &[String]) -> Result<(), String> {
        let candidate_paths = [
            self.target_root.join("boot/limine.conf"),
            self.target_root.join("boot/EFI/BOOT/limine.conf"),
            self.target_root.join("boot/EFI/limine/limine.conf"),
            self.target_root.join("boot/limine/limine.conf"),
        ];

        for path in &candidate_paths {
            if !path.exists() {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path) {
                let mut modified = false;
                let mut new_lines = Vec::new();

                for line in content.lines() {
                    if line.trim_start().starts_with("cmdline:") {
                        let mut line_str = line.to_string();
                        for param in params {
                            if !line_str.contains(param) {
                                line_str.push(' ');
                                line_str.push_str(param);
                                modified = true;
                            }
                        }
                        new_lines.push(line_str);
                    } else {
                        new_lines.push(line.to_string());
                    }
                }

                if modified {
                    let _ = fs::write(path, new_lines.join("\n"));
                }
            }
        }

        Ok(())
    }

    fn configure_hybrid_power_management(&self) -> Result<(), String> {
        let modprobe_d = self.target_root.join("etc/modprobe.d");
        let _ = fs::create_dir_all(&modprobe_d);
        let conf_path = modprobe_d.join("nvidia-pm.conf");
        let content = "options nvidia \"NVreg_DynamicPowerManagement=0x02\"\n";
        fs::write(conf_path, content).map_err(|e| format!("Failed to write nvidia-pm.conf: {}", e))
    }
}
