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

        let clean_desc: String = description
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
            .collect::<String>()
            .replace(' ', "_");
        let tag = if clean_desc.is_empty() {
            "manual".to_string()
        } else {
            clean_desc
        };

        let snap_name = format!("snapshot_{}_{}", now, tag);
        let cmd = format!(
            "mkdir -p /.snapshots && btrfs subvolume snapshot -r / '/.snapshots/{}'",
            snap_name
        );

        let _ = KernelManager::execute_action(
            "pkexec",
            &["sh", "-c", &cmd],
            log_cb,
        );
    }

    pub fn restore_snapshot<F>(name: &str, log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        if name.is_empty() || name.contains('/') || name.contains("..") {
            log_cb("[✗] Nama snapshot tidak valid.\n".to_string());
            return;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Find root block device from /proc/mounts and swap @ subvolume on top-level Btrfs (subvolid=5)
        let restore_script = format!(
            concat!(
                "set -e; ",
                "ROOT_DEV=$(awk '$2 == \"/\" {{print $1; exit}}' /proc/mounts); ",
                "if [ -z \"$ROOT_DEV\" ]; then echo 'Gagal mendeteksi partisi root Btrfs' >&2; exit 1; fi; ",
                "mkdir -p /tmp/hyperxos_btrfs_top; ",
                "mount -o subvolid=5 \"$ROOT_DEV\" /tmp/hyperxos_btrfs_top; ",
                "trap 'umount /tmp/hyperxos_btrfs_top 2>/dev/null || true' EXIT; ",
                "if [ -d \"/tmp/hyperxos_btrfs_top/@/.snapshots/{name}\" ]; then ",
                "  SNAP_SRC=\"/tmp/hyperxos_btrfs_top/@/.snapshots/{name}\"; ",
                "elif [ -d \"/.snapshots/{name}\" ]; then ",
                "  btrfs subvolume snapshot \"/.snapshots/{name}\" \"/tmp/hyperxos_btrfs_top/@_restored_{now}\"; ",
                "  mv /tmp/hyperxos_btrfs_top/@ \"/tmp/hyperxos_btrfs_top/@_old_{now}\"; ",
                "  mv \"/tmp/hyperxos_btrfs_top/@_restored_{now}\" /tmp/hyperxos_btrfs_top/@; ",
                "  echo 'Snapshot berhasil dipulihkan ke subvolume @! Silakan reboot sistem untuk menerapkan.'; ",
                "  exit 0; ",
                "else ",
                "  echo 'Snapshot tidak ditemukan' >&2; exit 1; ",
                "fi; ",
                "btrfs subvolume snapshot \"$SNAP_SRC\" \"/tmp/hyperxos_btrfs_top/@_restored_{now}\"; ",
                "if [ -d /tmp/hyperxos_btrfs_top/@/.snapshots ]; then ",
                "  mv /tmp/hyperxos_btrfs_top/@/.snapshots \"/tmp/hyperxos_btrfs_top/@_restored_{now}/.snapshots\" 2>/dev/null || true; ",
                "fi; ",
                "mv /tmp/hyperxos_btrfs_top/@ \"/tmp/hyperxos_btrfs_top/@_old_{now}\"; ",
                "mv \"/tmp/hyperxos_btrfs_top/@_restored_{now}\" /tmp/hyperxos_btrfs_top/@; ",
                "echo 'Snapshot berhasil dipulihkan ke subvolume @! Silakan reboot sistem untuk menerapkan.'"
            ),
            name = name,
            now = now
        );

        let _ = KernelManager::execute_action(
            "pkexec",
            &["sh", "-c", &restore_script],
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
