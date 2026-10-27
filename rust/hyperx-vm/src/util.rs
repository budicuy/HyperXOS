use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Dynamically locate the project root directory
pub fn get_project_root() -> PathBuf {
    // 1. Walk upwards from current working directory
    if let Ok(mut cur) = std::env::current_dir() {
        loop {
            if cur.join("archiso/profiledef.sh").exists() {
                return cur;
            }
            if !cur.pop() {
                break;
            }
        }
    }

    // 2. Walk upwards from current executable location
    if let Ok(mut exe) = std::env::current_exe() {
        while exe.pop() {
            if exe.join("archiso/profiledef.sh").exists() {
                return exe;
            }
        }
    }

    PathBuf::from(".")
}

/// Check if a binary command exists in PATH
#[allow(dead_code)]
pub fn command_exists(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Execute a command with streaming stdio
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
