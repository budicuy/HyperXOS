use std::fs;
use std::path::{Path, PathBuf};

use crate::entry::FileItem;

#[derive(Debug, Clone)]
pub struct ColumnState {
    pub dir_path: PathBuf,
    pub items: Vec<FileItem>,
    pub selected_index: Option<usize>,
}

impl ColumnState {
    pub fn load_directory(path: &Path, show_hidden: bool) -> Self {
        let mut items = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(item) = FileItem::from_dir_entry(entry) {
                    if !show_hidden && item.is_hidden {
                        continue;
                    }
                    items.push(item);
                }
            }
        }

        // Sort directories first, then alphabetical case-insensitive
        items.sort_by(|a, b| {
            b.is_dir.cmp(&a.is_dir).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        Self {
            dir_path: path.to_path_buf(),
            items,
            selected_index: None,
        }
    }
}
