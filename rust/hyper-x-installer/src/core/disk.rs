use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, Deserialize)]
pub struct LsblkResponse {
    pub blockdevices: Vec<LsblkDevice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LsblkDevice {
    pub name: String,
    pub model: Option<String>,
    pub size: Option<u64>,
    pub tran: Option<String>,
    #[serde(rename = "type")]
    pub dev_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub path: String,
    pub model: String,
    pub size_gb: f64,
    pub transport: String,
}

pub struct DiskManager;

impl DiskManager {
    pub fn list_disks() -> Vec<DiskInfo> {
        let output = match Command::new("lsblk")
            .args(["-J", "-b", "-d", "-o", "NAME,MODEL,SIZE,TRAN,TYPE"])
            .output()
        {
            Ok(out) if out.status.success() => out.stdout,
            _ => return Vec::new(),
        };

        let parsed: LsblkResponse = match serde_json::from_slice(&output) {
            Ok(res) => res,
            Err(_) => return Vec::new(),
        };

        let mut disks = Vec::new();

        for dev in parsed.blockdevices {
            let name = dev.name;
            // Ignore zram, loop, ram, sr, dm, fd (floppy), nbd devices
            if name.starts_with("loop")
                || name.starts_with("zram")
                || name.starts_with("ram")
                || name.starts_with("sr")
                || name.starts_with("dm-")
                || name.starts_with("fd")
                || name.starts_with("nbd")
            {
                continue;
            }

            // Only consider disks
            if let Some(ref t) = dev.dev_type {
                if t != "disk" {
                    continue;
                }
            }

            // Check if read-only (e.g. CD-ROM or ISO mount)
            let ro_path = format!("/sys/block/{}/ro", name);
            if let Ok(ro_val) = fs::read_to_string(ro_path) {
                if ro_val.trim() == "1" {
                    continue;
                }
            }

            let path = format!("/dev/{}", name);
            let model = dev.model.unwrap_or_else(|| "Unknown Drive".to_string()).trim().to_string();
            let size_bytes = dev.size.unwrap_or(0);
            let size_gb = (size_bytes as f64) / (1024.0 * 1024.0 * 1024.0);

            // Ignore drives smaller than 10 GB (e.g. floppy or small virtual disks)
            if size_gb < 10.0 {
                continue;
            }

            let transport = dev.tran.unwrap_or_else(|| "Internal".to_string()).to_uppercase();

            disks.push(DiskInfo {
                path,
                model,
                size_gb,
                transport,
            });
        }

        // Sort disks by size descending so the primary/largest drive is prioritized
        disks.sort_by(|a, b| b.size_gb.partial_cmp(&a.size_gb).unwrap_or(std::cmp::Ordering::Equal));

        disks
    }

    /// Calculate partition device node names based on whether disk is nvme/mmcblk or sdX.
    /// E.g., /dev/nvme0n1 -> /dev/nvme0n1p1, /dev/sda -> /dev/sda1
    pub fn get_partition_path(disk_path: &str, part_num: u32) -> String {
        if disk_path.ends_with(|c: char| c.is_ascii_digit()) {
            format!("{}p{}", disk_path, part_num)
        } else {
            format!("{}{}", disk_path, part_num)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_path_generation() {
        assert_eq!(DiskManager::get_partition_path("/dev/vda", 1), "/dev/vda1");
        assert_eq!(DiskManager::get_partition_path("/dev/sda", 2), "/dev/sda2");
        assert_eq!(DiskManager::get_partition_path("/dev/nvme0n1", 1), "/dev/nvme0n1p1");
        assert_eq!(DiskManager::get_partition_path("/dev/mmcblk0", 2), "/dev/mmcblk0p2");
    }

    #[test]
    fn test_list_disks_filters_invalid_devices() {
        let disks = DiskManager::list_disks();
        for disk in &disks {
            assert!(!disk.path.starts_with("/dev/fd"), "Floppy disks must be filtered out: {}", disk.path);
            assert!(!disk.path.starts_with("/dev/loop"), "Loop devices must be filtered out: {}", disk.path);
            assert!(!disk.path.starts_with("/dev/zram"), "Zram devices must be filtered out: {}", disk.path);
            assert!(disk.size_gb >= 10.0, "Disks < 10GB must be filtered out: {} ({} GB)", disk.path, disk.size_gb);
        }
    }
}
