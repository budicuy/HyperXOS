use std::fs;
use std::process::Command;
use crate::core::kernel_manager::KernelManager;

pub struct SystemTweaker;

impl SystemTweaker {
    pub fn get_current_governor() -> String {
        fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
            .unwrap_or_else(|_| "powersave".to_string())
            .trim()
            .to_string()
    }

    pub fn set_governor<F>(gov: &str, log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let _ = KernelManager::execute_action(
            "pkexec",
            &["cpupower", "frequency-set", "-g", gov],
            log_cb,
        );
    }

    pub fn get_max_map_count() -> (String, bool) {
        let val = fs::read_to_string("/proc/sys/vm/max_map_count")
            .unwrap_or_else(|_| "65530".to_string())
            .trim()
            .to_string();
        let num: u64 = val.parse().unwrap_or(0);
        let is_opt = num >= 1048576;
        (val, is_opt)
    }

    pub fn optimize_max_map_count<F>(log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let _ = KernelManager::execute_action(
            "pkexec",
            &["sysctl", "-w", "vm.max_map_count=2147483642"],
            log_cb,
        );
    }

    pub fn is_split_lock_disabled() -> bool {
        if let Ok(content) = fs::read_to_string("/proc/sys/kernel/split_lock_mitigate") {
            return content.trim() == "0";
        }
        false
    }

    pub fn toggle_split_lock<F>(disable: bool, log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let val = if disable { "0" } else { "1" };
        let arg = format!("kernel.split_lock_mitigate={}", val);
        let _ = KernelManager::execute_action("pkexec", &["sysctl", "-w", &arg], log_cb);
    }

    pub fn is_gamemode_active() -> bool {
        if let Ok(out) = Command::new("systemctl")
            .args(["--user", "is-active", "gamemoded"])
            .output()
        {
            return String::from_utf8_lossy(&out.stdout).trim() == "active";
        }
        false
    }

    pub fn toggle_gamemode<F>(enable: bool, log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let action = if enable { "start" } else { "stop" };
        let _ = KernelManager::execute_action(
            "systemctl",
            &["--user", action, "gamemoded"],
            log_cb,
        );
    }

    pub fn install_gaming_runtimes<F>(log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let _ = KernelManager::execute_action(
            "pkexec",
            &[
                "pacman",
                "-S",
                "--needed",
                "--noconfirm",
                "wine-staging",
                "winetricks",
                "vulkan-tools",
                "mangohud",
                "goverlay",
                "gamemode",
            ],
            log_cb,
        );
    }
}
