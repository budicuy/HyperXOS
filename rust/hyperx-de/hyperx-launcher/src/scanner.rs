use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use freedesktop_desktop_entry::DesktopEntry;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct AppItem {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub description: String,
    pub category: String,
}

pub fn scan_applications() -> Vec<AppItem> {
    let mut apps = Vec::new();
    let mut seen_names = HashSet::new();

    // Built-in HyperXOS system utilities and actions
    let builtins = vec![
        AppItem {
            name: "HyperX File Manager".to_string(),
            exec: "hyperx-file-manager".to_string(),
            icon: "📁".to_string(),
            description: "macOS Finder-style Miller Columns File Browser".to_string(),
            category: "System".to_string(),
        },
        AppItem {
            name: "HyperX Tools".to_string(),
            exec: "hyper-x-tools".to_string(),
            icon: "🛠".to_string(),
            description: "HyperXOS Package and Performance Tuning Suite".to_string(),
            category: "System".to_string(),
        },
        AppItem {
            name: "HyperX Installer".to_string(),
            exec: "hyper-x-installer".to_string(),
            icon: "💿".to_string(),
            description: "HyperXOS Guided System Installation Wizard".to_string(),
            category: "System".to_string(),
        },
        AppItem {
            name: "Lock Screen".to_string(),
            exec: "loginctl lock-session".to_string(),
            icon: "🔒".to_string(),
            description: "Lock the current desktop session".to_string(),
            category: "Power".to_string(),
        },
        AppItem {
            name: "Restart System".to_string(),
            exec: "systemctl reboot".to_string(),
            icon: "🔄".to_string(),
            description: "Restart computer".to_string(),
            category: "Power".to_string(),
        },
        AppItem {
            name: "Shut Down".to_string(),
            exec: "systemctl poweroff".to_string(),
            icon: "🛑".to_string(),
            description: "Power off computer completely".to_string(),
            category: "Power".to_string(),
        },
        AppItem {
            name: "Log Out".to_string(),
            exec: "loginctl terminate-user \"\"".to_string(),
            icon: "🚪".to_string(),
            description: "End current user session".to_string(),
            category: "Session".to_string(),
        },
    ];

    for b in builtins {
        seen_names.insert(b.name.to_lowercase());
        apps.push(b);
    }

    let search_dirs = get_desktop_directories();

    for dir in search_dirs {
        if !dir.exists() {
            continue;
        }

        for entry in WalkDir::new(&dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("desktop") {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(desktop_entry) = DesktopEntry::decode(path, &content) {
                        if desktop_entry.no_display() {
                            continue;
                        }

                        let name = desktop_entry
                            .name(None)
                            .map(|n| n.to_string())
                            .unwrap_or_else(|| {
                                path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("App")
                                    .to_string()
                            });

                        let name_lower = name.to_lowercase();
                        if seen_names.contains(&name_lower) {
                            continue;
                        }

                        let raw_exec = desktop_entry.exec().unwrap_or("").to_string();
                        if raw_exec.trim().is_empty() {
                            continue;
                        }

                        let cleaned_exec = clean_exec(&raw_exec);
                        let icon = desktop_entry.icon().unwrap_or("📦").to_string();
                        let desc = desktop_entry
                            .comment(None)
                            .map(|c| c.to_string())
                            .unwrap_or_else(|| "Application".to_string());

                        seen_names.insert(name_lower);
                        apps.push(AppItem {
                            name,
                            exec: cleaned_exec,
                            icon,
                            description: desc,
                            category: "Applications".to_string(),
                        });
                    }
                }
            }
        }
    }

    apps
}

fn get_desktop_directories() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
    ];

    if let Ok(home) = std::env::var("HOME") {
        dirs.push(Path::new(&home).join(".local/share/applications"));
    }

    dirs
}

fn clean_exec(exec: &str) -> String {
    // Strip field codes like %f, %F, %u, %U, %d, %D, %n, %N, %i, %c, %k, %v, %m
    let mut words = Vec::new();
    for word in exec.split_whitespace() {
        if !word.starts_with('%') {
            words.push(word);
        }
    }
    words.join(" ")
}
