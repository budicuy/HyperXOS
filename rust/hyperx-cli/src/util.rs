use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Locate project root containing archiso/profiledef.sh
pub fn get_project_root() -> PathBuf {
    if let Ok(cur) = std::env::current_dir() {
        if cur.join("archiso/profiledef.sh").exists() {
            return cur;
        }
        if cur.join("../../archiso/profiledef.sh").exists() {
            if let Ok(p) = cur.join("../..").canonicalize() {
                return p;
            }
        }
    }
    PathBuf::from("/home/budicuy/Dokumen/HyperXOS")
}

/// Execute a command streaming its output directly to stdout/stderr in real-time
pub fn run_stream(cmd: &mut Command, desc: &str) -> Result<(), String> {
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());
    cmd.stdin(Stdio::inherit());

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute {}: {}", desc, e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Command '{}' exited with status code {}",
            desc,
            status.code().unwrap_or(-1)
        ))
    }
}

/// Check if a tool exists in PATH
pub fn command_exists(tool: &str) -> bool {
    Command::new("which")
        .arg(tool)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Find the latest ISO file in out/
pub fn find_latest_iso(root: &Path) -> Option<PathBuf> {
    let out_dir = root.join("out");
    if !out_dir.exists() {
        return None;
    }

    let mut isos: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&out_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("hyperxos-") && name.ends_with(".iso") {
                    let mtime = entry
                        .metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    isos.push((path, mtime));
                }
            }
        }
    }

    isos.sort_by(|a, b| b.1.cmp(&a.1));
    isos.first().map(|(p, _)| p.clone())
}
