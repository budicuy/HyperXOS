use std::process::Command;
use std::thread;
use std::time::Duration;
use crate::core::disk::DiskManager;

#[derive(Debug, Clone)]
pub struct PartitionLayout {
    pub disk_path: String,
    pub boot_partition: String,
    pub root_partition: String,
}

pub struct Partitioner;

impl Partitioner {
    /// Format the target disk with a 4096 MiB FAT32 ESP (Part 1) and Btrfs root (Part 2)
    pub fn partition_disk<F>(disk_path: &str, mut progress_callback: F) -> Result<PartitionLayout, String>
    where
        F: FnMut(&str),
    {
        progress_callback(&format!("Wiping existing signatures on {}...", disk_path));
        
        // 1. Wipe existing partition signatures
        let _ = Command::new("wipefs")
            .args(["-af", disk_path])
            .output();

        // 2. Zap GPT and MBR data structures
        progress_callback("Creating new GPT partition table...");
        let zap_status = Command::new("sgdisk")
            .args(["-Z", disk_path])
            .status()
            .map_err(|e| format!("Failed to execute sgdisk -Z: {}", e))?;
        if !zap_status.success() {
            // Note: sgdisk -Z may return non-zero if disk was already clean, so we continue
        }

        let new_gpt_status = Command::new("sgdisk")
            .args(["-o", disk_path])
            .status()
            .map_err(|e| format!("Failed to create new GPT table: {}", e))?;
        if !new_gpt_status.success() {
            return Err("Failed to initialize empty GPT partition table".to_string());
        }

        // 3. Create Partition 1: EFI System Partition (4096 MiB, EF00)
        progress_callback("Creating 4096 MiB EFI Boot partition (EF00)...");
        let p1_status = Command::new("sgdisk")
            .args(["-n", "1:0:+4096M", "-t", "1:ef00", "-c", "1:HYPERXOS_BOOT", disk_path])
            .status()
            .map_err(|e| format!("Failed to create boot partition: {}", e))?;
        if !p1_status.success() {
            return Err("Failed to create 4GB EFI boot partition".to_string());
        }

        // 4. Create Partition 2: Linux Root x86-64 (Remainder of disk, 8304)
        progress_callback("Creating Root partition spanning remainder of disk...");
        let p2_status = Command::new("sgdisk")
            .args(["-n", "2:0:0", "-t", "2:8304", "-c", "2:HYPERXOS_ROOT", disk_path])
            .status()
            .map_err(|e| format!("Failed to create root partition: {}", e))?;
        if !p2_status.success() {
            return Err("Failed to create root partition".to_string());
        }

        // 5. Inform kernel of partition table updates
        progress_callback("Informing kernel of partition changes (partprobe & udevadm settle)...");
        let _ = Command::new("partprobe").arg(disk_path).status();
        let _ = Command::new("udevadm").args(["settle", "--timeout=10"]).status();
        thread::sleep(Duration::from_millis(500));

        let boot_partition = DiskManager::get_partition_path(disk_path, 1);
        let root_partition = DiskManager::get_partition_path(disk_path, 2);

        Ok(PartitionLayout {
            disk_path: disk_path.to_string(),
            boot_partition,
            root_partition,
        })
    }
}
