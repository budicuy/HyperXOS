use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;
use crate::core::partitioner::PartitionLayout;

#[derive(Debug, Clone)]
pub struct FilesystemContext {
    pub boot_uuid: String,
    pub root_uuid: String,
}

pub struct FilesystemManager;

impl FilesystemManager {
    /// Format partitions, create Btrfs subvolumes, and mount target tree under /mnt
    pub fn setup_filesystems<F>(layout: &PartitionLayout, mut progress_callback: F) -> Result<FilesystemContext, String>
    where
        F: FnMut(&str),
    {
        // 1. Format ESP Partition as FAT32 (4GB) - FAT labels must be <= 11 characters
        progress_callback(&format!("Formatting {} as FAT32 (HYPERX_BOOT)...", layout.boot_partition));
        let fat_res = Command::new("mkfs.fat")
            .args(["-F32", "-n", "HYPERX_BOOT", &layout.boot_partition])
            .status()
            .map_err(|e| format!("Failed to run mkfs.fat: {}", e))?;
        if !fat_res.success() {
            return Err(format!("mkfs.fat failed on {}", layout.boot_partition));
        }

        // 2. Format Root Partition as Btrfs
        progress_callback(&format!("Formatting {} as Btrfs (HYPERXOS_ROOT)...", layout.root_partition));
        let btrfs_res = Command::new("mkfs.btrfs")
            .args(["-f", "-L", "HYPERXOS_ROOT", &layout.root_partition])
            .status()
            .map_err(|e| format!("Failed to run mkfs.btrfs: {}", e))?;
        if !btrfs_res.success() {
            return Err(format!("mkfs.btrfs failed on {}", layout.root_partition));
        }

        // 3. Create temporary mount point for creating Btrfs subvolumes (@, @home)
        let tmp_mount = "/tmp/hyperxos_btrfs_temp";
        let _ = fs::create_dir_all(tmp_mount);

        // Ensure clean mount state
        let _ = Command::new("umount").arg(tmp_mount).status();

        progress_callback("Mounting Btrfs root volume to initialize subvolumes...");
        let mnt_temp = Command::new("mount")
            .args([&layout.root_partition, tmp_mount])
            .status()
            .map_err(|e| format!("Failed to mount Btrfs temp volume: {}", e))?;
        if !mnt_temp.success() {
            return Err("Failed to mount Btrfs temp volume".to_string());
        }

        // Create subvolume @ (for root /)
        progress_callback("Creating subvolume '@' for root filesystem...");
        let subvol_root = Command::new("btrfs")
            .args(["subvolume", "create", &format!("{}/@", tmp_mount)])
            .status()
            .map_err(|e| format!("Failed to create subvolume @: {}", e))?;
        if !subvol_root.success() {
            let _ = Command::new("umount").arg(tmp_mount).status();
            return Err("Failed to create subvolume @".to_string());
        }

        // Create subvolume @home (for /home)
        progress_callback("Creating subvolume '@home' for user directories...");
        let subvol_home = Command::new("btrfs")
            .args(["subvolume", "create", &format!("{}/@home", tmp_mount)])
            .status()
            .map_err(|e| format!("Failed to create subvolume @home: {}", e))?;
        if !subvol_home.success() {
            let _ = Command::new("umount").arg(tmp_mount).status();
            return Err("Failed to create subvolume @home".to_string());
        }

        // Unmount temporary mount point
        let _ = Command::new("umount").arg(tmp_mount).status();
        let _ = fs::remove_dir(tmp_mount);

        // 4. Mount target hierarchy under /mnt
        progress_callback("Mounting root subvolume '@' to /mnt (noatime,compress=zstd:1,space_cache=v2)...");
        // Ensure /mnt is unmounted first
        let _ = Command::new("umount").args(["-R", "/mnt"]).status();
        let _ = fs::create_dir_all("/mnt");

        let mnt_root = Command::new("mount")
            .args([
                "-o",
                "noatime,compress=zstd:1,space_cache=v2,subvol=@",
                &layout.root_partition,
                "/mnt",
            ])
            .status()
            .map_err(|e| format!("Failed to mount /mnt: {}", e))?;
        if !mnt_root.success() {
            return Err("Failed to mount root subvolume to /mnt".to_string());
        }

        // Mount /mnt/home
        let _ = fs::create_dir_all("/mnt/home");
        progress_callback("Mounting /mnt/home subvolume '@home'...");
        let mnt_home = Command::new("mount")
            .args([
                "-o",
                "noatime,compress=zstd:1,space_cache=v2,subvol=@home",
                &layout.root_partition,
                "/mnt/home",
            ])
            .status()
            .map_err(|e| format!("Failed to mount /mnt/home: {}", e))?;
        if !mnt_home.success() {
            return Err("Failed to mount /mnt/home".to_string());
        }

        // Mount /mnt/boot
        let _ = fs::create_dir_all("/mnt/boot");
        progress_callback(&format!("Mounting ESP partition {} to /mnt/boot...", layout.boot_partition));
        let mnt_boot = Command::new("mount")
            .args([&layout.boot_partition, "/mnt/boot"])
            .status()
            .map_err(|e| format!("Failed to mount /mnt/boot: {}", e))?;
        if !mnt_boot.success() {
            return Err("Failed to mount /mnt/boot".to_string());
        }

        // 5. Query UUIDs using blkid
        let boot_uuid = Self::get_uuid(&layout.boot_partition)?;
        let root_uuid = Self::get_uuid(&layout.root_partition)?;

        Ok(FilesystemContext {
            boot_uuid,
            root_uuid,
        })
    }

    pub fn get_uuid(device_node: &str) -> Result<String, String> {
        let output = Command::new("blkid")
            .args(["-s", "UUID", "-o", "value", device_node])
            .output()
            .map_err(|e| format!("Failed to run blkid: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to retrieve UUID for {}", device_node));
        }

        let uuid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if uuid.is_empty() {
            return Err(format!("Empty UUID returned for {}", device_node));
        }

        Ok(uuid)
    }

    /// Safely unmount all mounts under /mnt
    pub fn unmount_all() -> Result<(), String> {
        let _ = Command::new("sync").status();
        thread::sleep(Duration::from_millis(500));
        let status = Command::new("umount")
            .args(["-R", "/mnt"])
            .status()
            .map_err(|e| format!("Failed to execute umount: {}", e))?;

        if !status.success() {
            // Force or lazy unmount if busy
            let _ = Command::new("umount").args(["-l", "-R", "/mnt"]).status();
        }
        Ok(())
    }
}
