use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileKind {
    Directory,
    Image,
    Audio,
    Video,
    Document,
    Code,
    Archive,
    Executable,
    Symlink,
    Other,
}

impl FileKind {
    pub fn from_path(path: &Path, is_dir: bool, is_symlink: bool) -> Self {
        if is_dir {
            return Self::Directory;
        }
        if is_symlink {
            return Self::Symlink;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        match ext.as_str() {
            "png" | "jpg" | "jpeg" | "webp" | "svg" | "gif" | "bmp" | "ico" => Self::Image,
            "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" => Self::Audio,
            "mp4" | "mkv" | "webm" | "avi" | "mov" => Self::Video,
            "pdf" | "doc" | "docx" | "txt" | "md" | "odt" | "rtf" => Self::Document,
            "rs" | "c" | "cpp" | "h" | "py" | "js" | "ts" | "html" | "css" | "json" | "toml" | "yaml" | "sh" => Self::Code,
            "zip" | "tar" | "gz" | "zst" | "xz" | "7z" | "bz2" | "iso" => Self::Archive,
            "bin" => Self::Executable,
            _ => Self::Other,
        }
    }

    pub fn display_icon(&self) -> &'static str {
        match self {
            Self::Directory => "📁",
            Self::Image => "🖼️",
            Self::Audio => "🎵",
            Self::Video => "🎬",
            Self::Document => "📄",
            Self::Code => "🦀",
            Self::Archive => "📦",
            Self::Executable => "⚙️",
            Self::Symlink => "🔗",
            Self::Other => "📎",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileItem {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_hidden: bool,
    pub size_bytes: u64,
    pub modified: SystemTime,
    pub kind: FileKind,
}

impl FileItem {
    pub fn from_dir_entry(entry: fs::DirEntry) -> Option<Self> {
        let path = entry.path();
        let metadata = entry.metadata().ok()?;
        let file_type = metadata.file_type();

        let is_dir = file_type.is_dir();
        let is_symlink = file_type.is_symlink();
        let name = entry.file_name().to_string_lossy().to_string();
        let is_hidden = name.starts_with('.');
        let size_bytes = metadata.len();
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let kind = FileKind::from_path(&path, is_dir, is_symlink);

        Some(Self {
            path,
            name,
            is_dir,
            is_hidden,
            size_bytes,
            modified,
            kind,
        })
    }

    pub fn format_size(&self) -> String {
        if self.is_dir {
            return "--".to_string();
        }
        let b = self.size_bytes;
        if b < 1024 {
            format!("{} B", b)
        } else if b < 1024 * 1024 {
            format!("{:.1} KB", b as f64 / 1024.0)
        } else if b < 1024 * 1024 * 1024 {
            format!("{:.1} MB", b as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", b as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}
