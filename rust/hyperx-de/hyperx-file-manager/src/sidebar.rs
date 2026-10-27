use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarItem {
    pub label: String,
    pub icon: &'static str,
    pub path: PathBuf,
}

pub fn get_default_sidebar_items() -> Vec<SidebarItem> {
    let mut items = Vec::new();

    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        items.push(SidebarItem {
            label: "Home".to_string(),
            icon: "🏠",
            path: home.clone(),
        });

        let dirs = [
            ("Desktop", "🖥️", "Desktop"),
            ("Documents", "📑", "Dokumen"),
            ("Documents", "📑", "Documents"),
            ("Downloads", "📥", "Unduhan"),
            ("Downloads", "📥", "Downloads"),
            ("Pictures", "🖼️", "Gambar"),
            ("Pictures", "🖼️", "Pictures"),
            ("Music", "🎵", "Musik"),
            ("Music", "🎵", "Music"),
            ("Videos", "🎬", "Video"),
            ("Videos", "🎬", "Videos"),
        ];

        let mut seen = std::collections::HashSet::new();
        for (label, icon, sub) in dirs {
            let p = home.join(sub);
            if p.exists() && seen.insert(label) {
                items.push(SidebarItem {
                    label: label.to_string(),
                    icon,
                    path: p,
                });
            }
        }
    }

    items.push(SidebarItem {
        label: "File System".to_string(),
        icon: "💽",
        path: PathBuf::from("/"),
    });

    items
}
