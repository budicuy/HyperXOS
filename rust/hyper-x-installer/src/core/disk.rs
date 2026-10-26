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
    pub name: String,
    pub path: String,
    pub model: String,
    pub size_bytes: u64,
    pub size_gb: f64,
    pub transport: String,
    pub is_removable: bool,
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
            // Ignore zram, loop, ram, sr, dm devices
            if name.starts_with("loop")
                || name.starts_with("zram")
                || name.starts_with("ram")
                || name.starts_with("sr")
                || name.starts_with("dm-")
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
            let transport = dev.tran.unwrap_or_else(|| "Internal".to_string()).to_uppercase();

            let removable_path = format!("/sys/block/{}/removable", name);
            let is_removable = fs::read_to_string(removable_path)
                .map(|v| v.trim() == "1")
                .unwrap_or(false);

            disks.push(DiskInfo {
                name,
                path,
                model,
                size_bytes,
                size_gb,
                transport,
                is_removable,
            });
        }

        disks
    }

    /// Calculate partition device node names based on whether disk is nvme/mmcblk or sdX.
    /// E.g., /dev/nvme0n1 -> /dev/nvme0n1p1, /dev/sda -> /dev/sda1
    pub fn get_partition_path(disk_path: &str, part_num: u32) -> String {
        if disk_path.chars().last().map_or(false, |c| c.is_ascii_digit()) {
            format!("{}p{}", disk_path, part_num)
        } else {
            format!("{}{}", disk_path, part_num)
        }
    }
}
