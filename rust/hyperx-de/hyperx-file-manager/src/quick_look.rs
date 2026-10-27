use std::fs;

use crate::entry::{FileItem, FileKind};

#[derive(Debug, Clone)]
pub struct QuickLookData {
    pub item: FileItem,
    pub text_preview: Option<String>,
}

impl QuickLookData {
    pub fn load_for(item: FileItem) -> Self {
        let text_preview = match item.kind {
            FileKind::Code | FileKind::Document => {
                if item.size_bytes < 500_000 {
                    fs::read_to_string(&item.path)
                        .ok()
                        .map(|s| s.lines().take(80).collect::<Vec<_>>().join("\n"))
                } else {
                    Some("File too large for inline text preview.".to_string())
                }
            }
            FileKind::Image => Some(format!("Image file: {}", item.name)),
            FileKind::Archive => Some(format!("Archive archive: {}", item.name)),
            _ => None,
        };

        Self { item, text_preview }
    }
}
