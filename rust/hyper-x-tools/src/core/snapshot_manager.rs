use std::fs;
use std::path::Path;
use crate::core::kernel_manager::KernelManager;

#[derive(Debug, Clone)]
pub struct SnapshotRecord {
    pub name: String,
    pub date: String,
    pub description: String,
}

pub struct SnapshotManager;

impl SnapshotManager {
    pub fn is_btrfs_filesystem() -> bool {
        if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
            for line in mounts.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[1] == "/" {
                    return parts[2].to_lowercase() == "btrfs";
                }
            }
        }
        false
    }

    pub fn list_snapshots() -> Vec<SnapshotRecord> {
        let mut list = Vec::new();
        let snapshots_dir = "/.snapshots";

        if !Path::new(snapshots_dir).exists() {
            return list;
        }

        if let Ok(entries) = fs::read_dir(snapshots_dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.starts_with('.') {
                    continue;
                }

                let date_str = if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        let dur = modified
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default();
                        let secs = dur.as_secs();
                        format!("Timestamp: {}", secs)
                    } else {
                        "Baru saja".to_string()
                    }
                } else {
                    "Baru saja".to_string()
                };

                list.push(SnapshotRecord {
                    name: file_name,
                    date: date_str,
                    description: "Btrfs CoW Subvolume Snapshot".to_string(),
                });
            }
        }

        list.reverse();
        list
    }

    pub fn create_snapshot<F>(description: &str, log_cb: F)
    where
        F: Fn(String) + Send + 'static + Clone,
    {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let clean_desc = description.replace(' ', "_").replace('/', "-");
        let tag = if clean_desc.is_empty() {
            "manual".to_string()
        } else {
            clean_desc
        };

        let snap_name = format!("snapshot_{}_{}", now, tag);
        let dest = format!("/.snapshots/{}", snap_name);

        // Ensure /.snapshots exists
        let _ = KernelManager::execute_action("pkexec", &["mkdir", "-p", "/.snapshots"], {
            let log = log_cb.clone();
            move |s| log(s)
        });

        let _ = KernelManager::execute_action(
            "pkexec",
            &["btrfs", "subvolume", "snapshot", "-r", "/", &dest],
            log_cb,
        );
    }

    pub fn restore_snapshot<F>(name: &str, log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let src = format!("/.snapshots/{}", name);
        let _ = KernelManager::execute_action(
            "pkexec",
            &["btrfs", "subvolume", "snapshot", &src, "/"],
            log_cb,
        );
    }

    pub fn delete_snapshot<F>(name: &str, log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let target = format!("/.snapshots/{}", name);
        let _ = KernelManager::execute_action(
            "pkexec",
            &["btrfs", "subvolume", "delete", &target],
            log_cb,
        );
    }
}
