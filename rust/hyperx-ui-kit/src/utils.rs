use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::icons::LOGO_PNG_BYTES;

/// Loads the embedded HyperXOS window icon (RGBA) for Iced windows.
pub fn load_window_icon() -> Option<iced::window::Icon> {
    let img = image::load_from_memory(LOGO_PNG_BYTES).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
}

/// Opens an external URL in the user's default browser via `xdg-open`.
pub fn open_external_url(url: &str) {
    let _ = Command::new("xdg-open").arg(url).spawn();
}

/// Prints a formatted CLI header banner for HyperXOS terminal tools (`hyper-x-hw`, `hyper-x-installer`, etc.).
pub fn print_cli_banner(title: &str, subtitle: Option<&str>) {
    println!("============================================================");
    println!("  {}", title);
    if let Some(sub) = subtitle {
        println!("  {}", sub);
    }
    println!("============================================================");
}

/// Automatically detects and sets `WAYLAND_DISPLAY` / `XDG_RUNTIME_DIR` or `DISPLAY`
/// when a GUI app is launched via root (`pkexec`/`sudo`) or from a bare session.
pub fn ensure_display_environment() {
    let mut has_wayland =
        std::env::var("WAYLAND_DISPLAY").is_ok() || std::env::var("WAYLAND_SOCKET").is_ok();

    if !has_wayland {
        let euid = nix::unistd::geteuid().as_raw();
        let runtimes = [
            format!("/run/user/{}", euid),
            "/run/user/1000".to_string(),
        ];

        for rdir in &runtimes {
            if let Ok(entries) = std::fs::read_dir(rdir) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("wayland-") && !name.ends_with(".lock") {
                        std::env::set_var("WAYLAND_DISPLAY", &name);
                        std::env::set_var("XDG_RUNTIME_DIR", rdir);
                        has_wayland = true;
                        break;
                    }
                }
            }
            if has_wayland {
                break;
            }
        }
    }

    if !has_wayland && std::env::var("DISPLAY").is_err() {
        if Path::new("/tmp/.X11-unix/X0").exists() {
            std::env::set_var("DISPLAY", ":0");
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        } else if Path::new("/tmp/.X11-unix/X1").exists() {
            std::env::set_var("DISPLAY", ":1");
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        } else {
            std::env::set_var("DISPLAY", ":0");
        }
    }
}

/// Locate project root containing `archiso/profiledef.sh` dynamically.
pub fn get_project_root() -> PathBuf {
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

    if let Ok(mut exe) = std::env::current_exe() {
        while exe.pop() {
            if exe.join("archiso/profiledef.sh").exists() {
                return exe;
            }
        }
    }

    PathBuf::from(".")
}

/// Execute a command streaming its output directly to stdout/stderr in real-time.
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

/// Check if an executable exists in `PATH` without spawning a subprocess.
pub fn command_exists(tool: &str) -> bool {
    let tool_path = Path::new(tool);
    if tool_path.components().count() > 1 {
        return tool_path.is_file();
    }
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let full = dir.join(tool);
            if full.is_file() {
                return true;
            }
        }
    }
    false
}

/// Find the newest `hyperxos-*.iso` image in `<root>/out`.
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

    isos.sort_by_key(|a| std::cmp::Reverse(a.1));
    isos.first().map(|(p, _)| p.clone())
}
