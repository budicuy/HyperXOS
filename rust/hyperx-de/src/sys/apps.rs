use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopAppEntry {
    pub id: String,
    pub name: String,
    pub comment: String,
    pub exec: String,
    pub icon_key: String,
    pub category: String,
}

fn translate_xdg_category(categories_line: &str) -> String {
    let lower = categories_line.to_lowercase();
    if lower.contains("network") || lower.contains("webbrowser") || lower.contains("instantmessaging") || lower.contains("chat") {
        "Internet & Chat".to_string()
    } else if lower.contains("game") {
        "Gaming".to_string()
    } else if lower.contains("audio") || lower.contains("video") || lower.contains("music") {
        "Multimedia".to_string()
    } else if lower.contains("development") || lower.contains("ide") {
        "Pemrograman".to_string()
    } else if lower.contains("graphics") {
        "Desain & Grafis".to_string()
    } else if lower.contains("system") || lower.contains("settings") {
        "Sistem".to_string()
    } else if lower.contains("utility") || lower.contains("accessories") {
        "Utilitas".to_string()
    } else {
        "Aplikasi Pacman".to_string()
    }
}

pub fn scan_desktop_apps() -> Vec<DesktopAppEntry> {
    let mut apps = vec![
        DesktopAppEntry {
            id: "hyper-x-installer".to_string(),
            name: "Install HyperXOS".to_string(),
            comment: "Penginstal Sistem Operasi Native Pure Rust".to_string(),
            exec: "launch-installer".to_string(),
            icon_key: "hyper-x-installer".to_string(),
            category: "Sistem".to_string(),
        },
        DesktopAppEntry {
            id: "hyper-x-tools".to_string(),
            name: "HyperXTools".to_string(),
            comment: "Pusat Kendali Sistem, Kernel & Gaming Tweaker".to_string(),
            exec: "hyper-x-tools".to_string(),
            icon_key: "hyper-x-tools".to_string(),
            category: "Sistem".to_string(),
        },
        DesktopAppEntry {
            id: "firefox".to_string(),
            name: "Firefox Browser".to_string(),
            comment: "Peramban Web Cepat & Aman".to_string(),
            exec: "firefox".to_string(),
            icon_key: "firefox".to_string(),
            category: "Internet & Chat".to_string(),
        },
        DesktopAppEntry {
            id: "terminal".to_string(),
            name: "Terminal".to_string(),
            comment: "Emulator Terminal Wayland".to_string(),
            exec: "terminal".to_string(),
            icon_key: "terminal".to_string(),
            category: "Utilitas".to_string(),
        },
    ];

    let mut search_dirs = vec![PathBuf::from("/usr/share/applications")];
    if let Ok(home) = std::env::var("HOME") {
        search_dirs.push(PathBuf::from(home).join(".local/share/applications"));
    }

    let mut discovered_pacman_apps: Vec<DesktopAppEntry> = Vec::new();

    for app_dir in search_dirs {
        let Ok(entries) = fs::read_dir(&app_dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }
            if let Ok(content) = fs::read_to_string(&path) {
                let mut name = String::new();
                let mut comment = String::new();
                let mut exec = String::new();
                let mut icon = String::new();
                let mut categories = String::new();
                let mut no_display = false;

                for line in content.lines() {
                    if line.starts_with("NoDisplay=true") || line.starts_with("Hidden=true") {
                        no_display = true;
                        break;
                    }
                    if name.is_empty() && line.starts_with("Name=") {
                        name = line.trim_start_matches("Name=").trim().to_string();
                    } else if comment.is_empty() && line.starts_with("Comment=") {
                        comment = line.trim_start_matches("Comment=").trim().to_string();
                    } else if exec.is_empty() && line.starts_with("Exec=") {
                        let raw = line.trim_start_matches("Exec=").trim();
                        exec = raw
                            .split_whitespace()
                            .filter(|part| !part.starts_with('%'))
                            .collect::<Vec<_>>()
                            .join(" ");
                    } else if icon.is_empty() && line.starts_with("Icon=") {
                        icon = line.trim_start_matches("Icon=").trim().to_string();
                    } else if categories.is_empty() && line.starts_with("Categories=") {
                        categories = line.trim_start_matches("Categories=").trim().to_string();
                    }
                }

                if no_display || name.is_empty() || exec.is_empty() {
                    continue;
                }

                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("app")
                    .to_string();

                if apps
                    .iter()
                    .chain(discovered_pacman_apps.iter())
                    .any(|a| a.name.eq_ignore_ascii_case(&name) || a.id == stem)
                {
                    continue;
                }

                discovered_pacman_apps.push(DesktopAppEntry {
                    id: stem,
                    name,
                    comment: if comment.is_empty() {
                        "Aplikasi Desktop Linux (Pacman)".to_string()
                    } else {
                        comment
                    },
                    exec,
                    icon_key: icon,
                    category: translate_xdg_category(&categories),
                });
            }
        }
    }

    discovered_pacman_apps.sort_by_key(|a| a.name.to_lowercase());
    apps.extend(discovered_pacman_apps);

    apps
}

pub fn spawn_external_command(cmd_line: &str) -> Result<String, String> {
    if cmd_line == "terminal" {
        for term in ["foot", "alacritty", "kitty", "xterm"] {
            if Command::new("which")
                .arg(term)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                Command::new(term)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|e| e.to_string())?;
                return Ok(format!("Terminal '{}' diluncurkan", term));
            }
        }
    }

    if cmd_line == "launch-installer" || cmd_line == "hyper-x-installer" {
        for bin in [
            "launch-installer",
            "/usr/bin/launch-installer",
            "/usr/local/bin/launch-installer",
            "hyper-x-installer-launcher",
            "/usr/bin/hyper-x-installer-launcher",
            "hyper-x-installer",
            "/usr/bin/hyper-x-installer",
        ] {
            if Command::new(bin)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .is_ok()
            {
                return Ok("HyperXOS Native Installer diluncurkan".to_string());
            }
        }
    }

    Command::new("sh")
        .arg("-c")
        .arg(cmd_line)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Gagal menjalankan '{}': {}", cmd_line, e))?;

    Ok(format!("Aplikasi '{}' diluncurkan", cmd_line))
}
