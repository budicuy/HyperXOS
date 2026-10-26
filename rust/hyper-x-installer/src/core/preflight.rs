use std::net::TcpStream;
use std::path::Path;
use std::time::Duration;
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct PreflightStatus {
    pub is_uefi: bool,
    pub total_ram_gb: f64,
    pub has_min_ram: bool,
    pub has_internet: bool,
    pub is_plugged_in: bool,
    pub has_min_disk: bool,
}

impl PreflightStatus {
    pub fn check() -> Self {
        // 1. UEFI Firmware Check
        let is_uefi = Path::new("/sys/firmware/efi").exists();

        // 2. RAM Check (min ~3.5 GB for 4GB systems)
        let mut sys = System::new_all();
        sys.refresh_memory();
        let total_ram_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let has_min_ram = total_ram_gb >= 3.5;

        // 3. Internet Connectivity (TCP to Cloudflare DNS 1.1.1.1:53 with 2s timeout)
        let has_internet = TcpStream::connect_timeout(
            &"1.1.1.1:53".parse().unwrap(),
            Duration::from_millis(2000),
        )
        .is_ok();

        // 4. Power / Battery Check
        let is_plugged_in = Self::check_power_source();

        // 5. Target Storage Check (at least one non-loop disk >= 20GB)
        let has_min_disk = crate::core::disk::DiskManager::list_disks()
            .iter()
            .any(|d| d.size_gb >= 20.0);

        Self {
            is_uefi,
            total_ram_gb,
            has_min_ram,
            has_internet,
            is_plugged_in,
            has_min_disk,
        }
    }

    pub fn can_proceed(&self) -> bool {
        self.is_uefi && self.has_min_ram && self.has_internet && self.has_min_disk
    }

    fn check_power_source() -> bool {
        // Check /sys/class/power_supply/ for AC adapters
        if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("AC") || name.starts_with("ADP") {
                    let online_file = entry.path().join("online");
                    if let Ok(content) = std::fs::read_to_string(online_file) {
                        return content.trim() == "1";
                    }
                }
            }
        }
        // Default to true if desktop (no battery found)
        true
    }
}
