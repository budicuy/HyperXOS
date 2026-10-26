use std::fs;
use crate::core::kernel_manager::KernelManager;

#[derive(Debug, Clone)]
pub struct StorePackage {
    pub package_name: String,
    pub display_name: String,
    pub category: String,
    pub description: String,
    pub installed: bool,
}

pub struct PackageManager;

impl PackageManager {
    pub fn get_catalog() -> Vec<StorePackage> {
        let catalog_defs = vec![
            // Browsers
            ("firefox", "Mozilla Firefox", "Browsers", "Peramban web open-source cepat dan fokus pada privasi."),
            ("chromium", "Chromium", "Browsers", "Peramban web open-source berbasis Chromium engine."),
            ("brave-bin", "Brave Browser", "Browsers", "Peramban web cepat dengan pemblokir iklan & pelacak bawaan."),

            // Gaming
            ("steam", "Steam Gaming Client", "Gaming", "Platform distribusi game digital terbesar di PC."),
            ("lutris", "Lutris Gaming Platform", "Gaming", "Pengelola game terpadu untuk Wine, Epic Games, GOG, dan emulator."),
            ("heroic-games-launcher-bin", "Heroic Games Launcher", "Gaming", "Klien open source untuk Epic Games dan GOG."),
            ("discord", "Discord", "Gaming", "Aplikasi obrolan suara dan teks untuk komunitas gamer."),

            // Development
            ("code", "Visual Studio Code (OSS)", "Development", "Editor kode sumber modern dengan ekosistem ekstensi yang kaya."),
            ("git", "Git Version Control", "Development", "Sistem pengontrol versi kode sumber terdistribusi standar industri."),
            ("neovim", "Neovim", "Development", "Editor teks berbasis terminal modern penerus Vim."),
            ("docker", "Docker Container", "Development", "Platform kontainerisasi aplikasi untuk pengembang."),

            // Multimedia
            ("vlc", "VLC Media Player", "Multimedia", "Pemutar multimedia serbaguna pendukung berbagai macam format."),
            ("mpv", "MPV Player", "Multimedia", "Pemutar video ultra-ringan dan bertenaga tinggi."),
            ("obs-studio", "OBS Studio", "Multimedia", "Perangkat lunak open source untuk perekaman layar dan live streaming."),
            ("gimp", "GNU Image Manipulation Program", "Multimedia", "Editor grafis dan manipulasi foto profesional."),

            // Utilities
            ("btop", "Btop Resource Monitor", "Utilities", "Monitor sistem berbasis TUI dengan grafik interaktif yang indah."),
            ("fastfetch", "Fastfetch", "Utilities", "Utilitas penampil spesifikasi sistem secepat kilat."),
            ("timeshift", "Timeshift", "Utilities", "Alat backup dan pemulihan sistem untuk filesystem Linux."),
            ("htop", "Htop Process Viewer", "Utilities", "Penampil dan pengelola proses sistem interaktif."),
        ];

        let installed_set = Self::get_installed_packages();

        catalog_defs
            .into_iter()
            .map(|(pkg, name, cat, desc)| {
                let is_inst = installed_set.contains(pkg);
                StorePackage {
                    package_name: pkg.to_string(),
                    display_name: name.to_string(),
                    category: cat.to_string(),
                    description: desc.to_string(),
                    installed: is_inst,
                }
            })
            .collect()
    }

    fn get_installed_packages() -> std::collections::HashSet<String> {
        let mut set = std::collections::HashSet::new();
        if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let parts: Vec<&str> = name.split('-').collect();
                if parts.len() >= 3 {
                    let pkg_name = parts[..parts.len() - 2].join("-");
                    set.insert(pkg_name);
                }
            }
        }
        set
    }

    pub fn filter_catalog(
        catalog: &[StorePackage],
        category: &str,
        search_query: &str,
    ) -> Vec<StorePackage> {
        let q = search_query.to_lowercase();
        catalog
            .iter()
            .filter(|p| {
                let matches_cat = if category == "Semua" {
                    true
                } else {
                    p.category.eq_ignore_ascii_case(category)
                };

                let matches_query = if q.is_empty() {
                    true
                } else {
                    p.package_name.to_lowercase().contains(&q)
                        || p.display_name.to_lowercase().contains(&q)
                        || p.description.to_lowercase().contains(&q)
                };

                matches_cat && matches_query
            })
            .cloned()
            .collect()
    }

    pub fn install_package<F>(package_name: &str, log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let _ = KernelManager::execute_action(
            "pkexec",
            &["pacman", "-S", "--needed", "--noconfirm", package_name],
            log_cb,
        );
    }

    pub fn remove_package<F>(package_name: &str, log_cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        let _ = KernelManager::execute_action(
            "pkexec",
            &["pacman", "-Rns", "--noconfirm", package_name],
            log_cb,
        );
    }
}
