use std::fs;
use std::path::Path;
use std::process::Command;
use sysinfo::{Components, Disks, System};

#[derive(Debug, Clone, Default)]
pub struct SystemMetrics {
    // CPU
    pub cpu_usage_percent: f32,
    pub cpu_cores: usize,
    pub cpu_model: String,
    pub cpu_temp_celsius: i32,

    // RAM & Swap
    pub ram_total_bytes: u64,
    pub ram_used_bytes: u64,
    pub ram_usage_percent: f32,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,

    // Disk
    pub disk_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub disk_usage_percent: f32,
    pub disk_fs_type: String,

    // GPU
    pub gpu_model: String,
    pub gpu_usage_percent: f32,
    pub vram_total_bytes: u64,
    pub vram_used_bytes: u64,

    // OS Info & Hardware Specifications
    pub os_name: String,
    pub kernel_version: String,
    pub uptime_string: String,
    pub display_session: String,
    pub host_name: String,
    pub hardware_board: String,
    pub desktop_env: String,
}

pub struct SystemMonitor {
    sys: System,
    cached_cpu_model: String,
    cached_gpu_model: String,
    cached_os_name: String,
    cached_kernel: String,
    cached_host_name: String,
    cached_hardware_board: String,
    cached_desktop_env: String,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        // Read static CPU model
        let cpu_model = if let Some(cpu) = sys.cpus().first() {
            cpu.brand().trim().to_string()
        } else {
            "Generic x86_64 Processor".to_string()
        };

        // Read static OS & Kernel
        let kernel_version = fs::read_to_string("/proc/sys/kernel/osrelease")
            .unwrap_or_else(|_| "6.12.0".to_string())
            .trim()
            .to_string();

        let os_name = "HyperXOS Linux 2026.10".to_string();

        let host_name = System::host_name().unwrap_or_else(|| "hyperxos".to_string());
        let hardware_board = Self::detect_hardware_board();
        let desktop_env = Self::detect_desktop_env();

        let mut monitor = Self {
            sys,
            cached_cpu_model: cpu_model,
            cached_gpu_model: String::new(),
            cached_os_name: os_name,
            cached_kernel: kernel_version,
            cached_host_name: host_name,
            cached_hardware_board: hardware_board,
            cached_desktop_env: desktop_env,
        };

        monitor.cached_gpu_model = monitor.detect_gpu_name();
        monitor
    }

    fn detect_hardware_board() -> String {
        let vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor")
            .unwrap_or_default()
            .trim()
            .to_string();
        let product = fs::read_to_string("/sys/class/dmi/id/product_name")
            .unwrap_or_default()
            .trim()
            .to_string();
        let board = fs::read_to_string("/sys/class/dmi/id/board_name")
            .unwrap_or_default()
            .trim()
            .to_string();

        let mut parts = Vec::new();
        if !vendor.is_empty() && vendor != "System manufacturer" && vendor != "Default string" {
            parts.push(vendor.clone());
        }
        if !board.is_empty() && board != "Base Board Product Name" && board != "Default string" {
            if !vendor.is_empty() && !board.starts_with(&vendor) {
                parts.push(board);
            } else if vendor.is_empty() {
                parts.push(board);
            }
        } else if !product.is_empty() && product != "System Product Name" && product != "Default string" {
            parts.push(product);
        }

        if parts.is_empty() {
            "PC Kompatibel x86_64".to_string()
        } else {
            parts.join(" ")
        }
    }

    fn detect_desktop_env() -> String {
        if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
            if !de.is_empty() {
                return de;
            }
        }
        if let Ok(de) = std::env::var("DESKTOP_SESSION") {
            if !de.is_empty() {
                return de;
            }
        }
        "KDE Plasma".to_string()
    }

    pub fn collect_metrics(&mut self) -> SystemMetrics {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();

        // CPU
        let cpu_usage_percent = self.sys.global_cpu_usage();
        let cpu_cores = self.sys.cpus().len();
        let cpu_temp_celsius = self.read_cpu_temperature();

        // Memory (RAM)
        let ram_total_bytes = self.sys.total_memory();
        let ram_used_bytes = self.sys.used_memory();
        let ram_usage_percent = if ram_total_bytes > 0 {
            (ram_used_bytes as f64 / ram_total_bytes as f64 * 100.0) as f32
        } else {
            0.0
        };

        let swap_total_bytes = self.sys.total_swap();
        let swap_used_bytes = self.sys.used_swap();

        // Disks
        let (disk_total_bytes, disk_used_bytes, disk_usage_percent, disk_fs_type) =
            self.read_root_disk();

        // GPU
        let (gpu_usage_percent, vram_total_bytes, vram_used_bytes) = self.read_gpu_metrics();

        // Uptime
        let uptime_secs = System::uptime();
        let hours = uptime_secs / 3600;
        let mins = (uptime_secs % 3600) / 60;
        let uptime_string = format!("{}j {}m", hours, mins);

        let display_session = std::env::var("XDG_SESSION_TYPE")
            .unwrap_or_else(|_| "Wayland".to_string())
            .to_uppercase();

        SystemMetrics {
            cpu_usage_percent,
            cpu_cores,
            cpu_model: self.cached_cpu_model.clone(),
            cpu_temp_celsius,
            ram_total_bytes,
            ram_used_bytes,
            ram_usage_percent,
            swap_total_bytes,
            swap_used_bytes,
            disk_total_bytes,
            disk_used_bytes,
            disk_usage_percent,
            disk_fs_type,
            gpu_model: self.cached_gpu_model.clone(),
            gpu_usage_percent,
            vram_total_bytes,
            vram_used_bytes,
            os_name: self.cached_os_name.clone(),
            kernel_version: self.cached_kernel.clone(),
            uptime_string,
            display_session,
            host_name: self.cached_host_name.clone(),
            hardware_board: self.cached_hardware_board.clone(),
            desktop_env: self.cached_desktop_env.clone(),
        }
    }

    fn read_cpu_temperature(&self) -> i32 {
        let components = Components::new_with_refreshed_list();
        for component in &components {
            let label = component.label().to_lowercase();
            if label.contains("core") || label.contains("cpu") || label.contains("k10temp") || label.contains("tctl") {
                if let Some(temp) = component.temperature() {
                    if temp > 0.0 {
                        return temp as i32;
                    }
                }
            }
        }

        // Fallback to thermal_zone
        for i in 0..5 {
            let path = format!("/sys/class/thermal/thermal_zone{}/temp", i);
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(val) = content.trim().parse::<i32>() {
                    let temp = if val > 1000 { val / 1000 } else { val };
                    if temp > 0 && temp < 120 {
                        return temp;
                    }
                }
            }
        }
        0
    }

    fn read_root_disk(&self) -> (u64, u64, f32, String) {
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            if disk.mount_point() == Path::new("/") {
                let total = disk.total_space();
                let avail = disk.available_space();
                let used = if total > avail { total - avail } else { 0 };
                let pct = if total > 0 {
                    (used as f64 / total as f64 * 100.0) as f32
                } else {
                    0.0
                };
                let fs_type = disk.file_system().to_string_lossy().to_string().to_uppercase();
                return (total, used, pct, fs_type);
            }
        }

        // If no explicit / mount found, pick the first disk
        if let Some(disk) = disks.first() {
            let total = disk.total_space();
            let avail = disk.available_space();
            let used = if total > avail { total - avail } else { 0 };
            let pct = if total > 0 {
                (used as f64 / total as f64 * 100.0) as f32
            } else {
                0.0
            };
            let fs_type = disk.file_system().to_string_lossy().to_string().to_uppercase();
            return (total, used, pct, fs_type);
        }

        (0, 0, 0.0, "BTRFS".to_string())
    }

    fn detect_gpu_name(&self) -> String {
        // 1. Check NVIDIA via nvidia-smi
        if let Ok(out) = Command::new("nvidia-smi")
            .args(["--query-gpu=name", "--format=csv,noheader"])
            .output()
        {
            if out.status.success() {
                let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !name.is_empty() {
                    return name;
                }
            }
        }

        // 2. Scan DRM cards
        for i in 0..8 {
            let uevent_path = format!("/sys/class/drm/card{}/device/uevent", i);
            if let Ok(content) = fs::read_to_string(&uevent_path) {
                let mut driver = String::new();
                for line in content.lines() {
                    if line.starts_with("DRIVER=") {
                        driver = line.trim_start_matches("DRIVER=").to_string();
                    }
                }

                if driver == "amdgpu" {
                    return "AMD Radeon Graphics (amdgpu)".to_string();
                } else if driver == "i915" || driver == "xe" {
                    return "Intel HD/Iris/Arc Graphics".to_string();
                } else if driver == "virtio-gpu" || driver == "qxl" || driver == "vmwgfx" {
                    return "Akselerator Grafis Virtual (VirtIO/KVM)".to_string();
                } else if !driver.is_empty() {
                    return format!("GPU ({})", driver);
                }
            }
        }

        "Standard VGA / Virtual GPU".to_string()
    }

    fn read_gpu_metrics(&self) -> (f32, u64, u64) {
        // 1. Try NVIDIA
        if let Ok(out) = Command::new("nvidia-smi")
            .args(["--query-gpu=utilization.gpu,memory.total,memory.used", "--format=csv,noheader,nounits"])
            .output()
        {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                let parts: Vec<&str> = text.trim().split(',').map(|s| s.trim()).collect();
                if parts.len() >= 3 {
                    let usage: f32 = parts[0].parse().unwrap_or(0.0);
                    let total_mb: u64 = parts[1].parse().unwrap_or(0);
                    let used_mb: u64 = parts[2].parse().unwrap_or(0);
                    return (usage, total_mb * 1024 * 1024, used_mb * 1024 * 1024);
                }
            }
        }

        // 2. Try AMD drm cards
        for i in 0..8 {
            let busy_path = format!("/sys/class/drm/card{}/device/gpu_busy_percent", i);
            if let Ok(content) = fs::read_to_string(&busy_path) {
                let usage: f32 = content.trim().parse().unwrap_or(0.0);
                let mut vtotal = 0u64;
                let mut vused = 0u64;

                let total_path = format!("/sys/class/drm/card{}/device/mem_info_vram_total", i);
                let used_path = format!("/sys/class/drm/card{}/device/mem_info_vram_used", i);

                if let Ok(c) = fs::read_to_string(total_path) {
                    vtotal = c.trim().parse().unwrap_or(0);
                }
                if let Ok(c) = fs::read_to_string(used_path) {
                    vused = c.trim().parse().unwrap_or(0);
                }

                return (usage, vtotal, vused);
            }
        }

        // Default: Active
        (5.0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_metrics() {
        let mut monitor = SystemMonitor::new();
        let m = monitor.collect_metrics();
        println!("CPU Cores: {}, Model: {}", m.cpu_cores, m.cpu_model);
        println!("RAM Total: {} MB, Used: {} MB, Usage: {:.1}%", m.ram_total_bytes / (1024*1024), m.ram_used_bytes / (1024*1024), m.ram_usage_percent);
        println!("Disk Total: {} GB, Used: {} GB, Usage: {:.1}%", m.disk_total_bytes / (1024*1024*1024), m.disk_used_bytes / (1024*1024*1024), m.disk_usage_percent);
        println!("GPU: {}, Usage: {:.1}%", m.gpu_model, m.gpu_usage_percent);
        println!("Kernel: {}, Uptime: {}", m.kernel_version, m.uptime_string);

        assert!(m.cpu_cores > 0, "CPU cores should be > 0");
        assert!(m.ram_total_bytes > 0, "RAM total should be > 0");
        assert!(m.ram_used_bytes > 0, "RAM used should be > 0");
        assert!(m.ram_usage_percent > 0.0 && m.ram_usage_percent <= 100.0, "RAM usage % should be valid");
        assert!(m.disk_total_bytes > 0, "Disk total should be > 0");
        assert!(!m.kernel_version.is_empty(), "Kernel version should not be empty");
    }
}
