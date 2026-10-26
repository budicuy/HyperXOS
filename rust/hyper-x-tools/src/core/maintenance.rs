use std::process::Command;
use crate::core::kernel_manager::KernelManager;

pub struct Maintenance;

impl Maintenance {
    pub fn clean_cache<F>(log_cb: F)
    where
        F: Fn(String) + Send + 'static + Clone,
    {
        let log_clone = log_cb.clone();
        let _ = KernelManager::execute_action("pkexec", &["paccache", "-rk2"], move |s| {
            log_clone(s);
        });
        let _ = KernelManager::execute_action("pkexec", &["paccache", "-ruk0"], log_cb);
    }

    pub fn clean_journal<F>(log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let _ = KernelManager::execute_action(
            "pkexec",
            &["journalctl", "--vacuum-size=100M"],
            log_cb,
        );
    }

    pub fn remove_orphans<F>(log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        if let Ok(out) = Command::new("pacman").args(["-Qtdq"]).output() {
            let orphans = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if orphans.is_empty() {
                log_cb("[✓] Tidak ada paket yatim (orphans) yang ditemukan. Sistem sudah bersih.\n".to_string());
                return;
            }

            let mut args = vec!["pacman", "-Rns", "--noconfirm"];
            let pkgs: Vec<&str> = orphans.split_whitespace().collect();
            args.extend(pkgs);

            let _ = KernelManager::execute_action("pkexec", &args, log_cb);
        } else {
            log_cb("[!] Gagal menjalankan pengecekan pacman -Qtdq\n".to_string());
        }
    }

    pub fn rank_mirrors<F>(log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let _ = KernelManager::execute_action(
            "pkexec",
            &[
                "reflector",
                "--latest",
                "15",
                "--protocol",
                "https",
                "--sort",
                "rate",
                "--save",
                "/etc/pacman.d/mirrorlist",
            ],
            log_cb,
        );
    }

    pub fn fix_keyrings<F>(log_cb: F)
    where
        F: Fn(String) + Send + 'static + Clone,
    {
        let log_clone = log_cb.clone();
        let _ = KernelManager::execute_action("pkexec", &["pacman-key", "--init"], move |s| {
            log_clone(s);
        });
        let _ = KernelManager::execute_action(
            "pkexec",
            &["pacman-key", "--populate", "archlinux", "cachyos", "hyperxos"],
            log_cb,
        );
    }
}
