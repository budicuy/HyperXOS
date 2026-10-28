use std::process::Command;

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

fn get_euid() -> u32 {
    extern "C" {
        fn geteuid() -> u32;
    }
    unsafe { geteuid() }
}

/// Automatically detects and sets `WAYLAND_DISPLAY` / `XDG_RUNTIME_DIR` or `DISPLAY`
/// when a GUI app is launched via root (`pkexec`/`sudo`) or from a bare session.
pub fn ensure_display_environment() {
    let has_wayland =
        std::env::var("WAYLAND_DISPLAY").is_ok() || std::env::var("WAYLAND_SOCKET").is_ok();
    let has_x11 = std::env::var("DISPLAY").is_ok();

    if !has_wayland && !has_x11 {
        let euid = get_euid();
        let mut resolved = false;

        let runtimes = vec![
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
                        resolved = true;
                        break;
                    }
                }
            }
            if resolved {
                break;
            }
        }

        if !resolved {
            if std::path::Path::new("/tmp/.X11-unix/X0").exists() {
                std::env::set_var("DISPLAY", ":0");
                std::env::set_var("WINIT_UNIX_BACKEND", "x11");
                resolved = true;
            } else if std::path::Path::new("/tmp/.X11-unix/X1").exists() {
                std::env::set_var("DISPLAY", ":1");
                std::env::set_var("WINIT_UNIX_BACKEND", "x11");
                resolved = true;
            }
        }

        if !resolved {
            std::env::set_var("DISPLAY", ":0");
        }
    }
}
