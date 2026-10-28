use std::fs;
use std::path::{Path, PathBuf};

use iced::border::Radius;
use iced::widget::{button, column, container, mouse_area, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    fluent_address_bar_style, fluent_file_item_style, fluent_input_style,
    fluent_ribbon_button_style, fluent_sidebar_container_style, fluent_sidebar_nav_item,
    fluent_window_header_bar, fluent_window_style, glass_scrollable, icons, primary_button_style,
    svg_icon, FLUENT_PRIMARY, FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED, FONT_BODY_MEDIUM,
    FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD,
};

use crate::ui::overlays::QuickLookData;
use crate::ui::{DesktopWindowId, Message};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilePlace {
    Home,
    Desktop,
    Documents,
    Downloads,
    Pictures,
    Music,
    Trash,
    Root,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileClipboardOp {
    Copy,
    Cut,
}

#[derive(Debug, Clone)]
pub struct FileEntryItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub ext: String,
}

#[derive(Debug, Clone)]
pub struct FileManagerState {
    pub current_place: FilePlace,
    pub current_dir: PathBuf,
    pub entries: Vec<FileEntryItem>,
    pub selected_index: Option<usize>,
    pub search_query: String,
    pub new_folder_name: String,
    pub rename_input: String,
    pub renaming_index: Option<usize>,
    pub file_clipboard: Option<(FileClipboardOp, PathBuf)>,
    pub show_hidden: bool,
    pub grid_mode: bool,
    pub status_message: String,
    pub trash_count: usize,
}

impl Default for FileManagerState {
    fn default() -> Self {
        let home = Self::home_dir();
        let _ = fs::create_dir_all(Self::trash_dir());
        let mut s = Self {
            current_place: FilePlace::Home,
            current_dir: home,
            entries: Vec::new(),
            selected_index: None,
            search_query: String::new(),
            new_folder_name: String::new(),
            rename_input: String::new(),
            renaming_index: None,
            file_clipboard: None,
            show_hidden: false,
            grid_mode: true,
            status_message: "Siap menjelajahi berkas sistem.".to_string(),
            trash_count: 0,
        };
        s.reload();
        s
    }
}

impl FileManagerState {
    pub fn home_dir() -> PathBuf {
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/home/hyperxos"))
    }

    pub fn trash_dir() -> PathBuf {
        let p = Self::home_dir().join(".local/share/Trash/files");
        let _ = fs::create_dir_all(&p);
        p
    }

    pub fn refresh_trash_count(&mut self) {
        let td = Self::trash_dir();
        self.trash_count = fs::read_dir(td)
            .map(|rd| rd.flatten().count())
            .unwrap_or(0);
    }

    pub fn navigate_place(&mut self, place: FilePlace) {
        self.current_place = place;
        let home = Self::home_dir();
        let target = match place {
            FilePlace::Home => home,
            FilePlace::Desktop => {
                let p = home.join("Desktop");
                if p.exists() { p } else { home }
            }
            FilePlace::Documents => {
                let p = home.join("Documents");
                let p_id = home.join("Dokumen");
                if p_id.exists() {
                    p_id
                } else if p.exists() {
                    p
                } else {
                    home
                }
            }
            FilePlace::Downloads => {
                let p = home.join("Downloads");
                let p_id = home.join("Unduhan");
                if p_id.exists() {
                    p_id
                } else if p.exists() {
                    p
                } else {
                    home
                }
            }
            FilePlace::Pictures => {
                let p = home.join("Pictures");
                let p_id = home.join("Gambar");
                if p_id.exists() {
                    p_id
                } else if p.exists() {
                    p
                } else {
                    home
                }
            }
            FilePlace::Music => {
                let p = home.join("Music");
                if p.exists() { p } else { home }
            }
            FilePlace::Trash => Self::trash_dir(),
            FilePlace::Root => PathBuf::from("/"),
        };
        self.current_dir = target;
        self.selected_index = None;
        self.renaming_index = None;
        self.reload();
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        if path.is_dir() {
            if path == Self::trash_dir() {
                self.current_place = FilePlace::Trash;
            }
            self.current_dir = path;
            self.selected_index = None;
            self.renaming_index = None;
            self.reload();
        }
    }

    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected_index = None;
            self.renaming_index = None;
            self.reload();
        }
    }

    pub fn reload(&mut self) {
        self.refresh_trash_count();
        let mut items = Vec::new();
        if let Ok(rd) = fs::read_dir(&self.current_dir) {
            for entry in rd.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if !self.show_hidden && name.starts_with('.') {
                    continue;
                }
                let meta = entry.metadata().ok();
                let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);
                let size_bytes = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                items.push(FileEntryItem {
                    name,
                    path,
                    is_dir,
                    size_bytes,
                    ext,
                });
            }
        }

        items.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        self.status_message = format!(
            "{} item di {}",
            items.len(),
            self.current_dir.display()
        );
        self.entries = items;
    }

    pub fn create_folder(&mut self) -> Result<String, String> {
        let trimmed = self.new_folder_name.trim();
        let folder_name = if trimmed.is_empty() {
            format!("Folder-Baru-{}", self.entries.len() + 1)
        } else {
            trimmed.to_string()
        };
        let target = self.current_dir.join(&folder_name);
        fs::create_dir_all(&target).map_err(|e| e.to_string())?;
        self.new_folder_name.clear();
        self.reload();
        Ok(format!("Folder '{}' berhasil dibuat", folder_name))
    }

    pub fn start_rename(&mut self, idx: usize) {
        if let Some(entry) = self.entries.get(idx) {
            self.selected_index = Some(idx);
            self.renaming_index = Some(idx);
            self.rename_input = entry.name.clone();
        }
    }

    pub fn commit_rename(&mut self) -> Result<String, String> {
        let Some(idx) = self.renaming_index else {
            return Err("Tidak ada item yang sedang diganti namanya.".to_string());
        };
        let Some(entry) = self.entries.get(idx).cloned() else {
            self.renaming_index = None;
            return Err("Item tidak ditemukan.".to_string());
        };
        let new_name = self.rename_input.trim();
        if new_name.is_empty() || new_name == entry.name {
            self.renaming_index = None;
            return Err("Nama baru tidak berubah.".to_string());
        }
        let dest = self.current_dir.join(new_name);
        fs::rename(&entry.path, &dest).map_err(|e| e.to_string())?;
        let msg = format!("'{}' -> '{}'", entry.name, new_name);
        self.renaming_index = None;
        self.rename_input.clear();
        self.reload();
        Ok(msg)
    }

    pub fn copy_entry_to_clipboard(&mut self, idx: usize) -> Option<(String, String)> {
        let entry = self.entries.get(idx)?;
        self.file_clipboard = Some((FileClipboardOp::Copy, entry.path.clone()));
        Some((
            entry.name.clone(),
            format!("COPY: {}", entry.path.display()),
        ))
    }

    pub fn cut_entry_to_clipboard(&mut self, idx: usize) -> Option<(String, String)> {
        let entry = self.entries.get(idx)?;
        self.file_clipboard = Some((FileClipboardOp::Cut, entry.path.clone()));
        Some((
            entry.name.clone(),
            format!("CUT: {}", entry.path.display()),
        ))
    }

    pub fn paste_from_clipboard(&mut self) -> Result<String, String> {
        let Some((op, src_path)) = self.file_clipboard.clone() else {
            return Err("Clipboard berkas masih kosong.".to_string());
        };
        if !src_path.exists() {
            self.file_clipboard = None;
            return Err("Berkas sumber sudah tidak ditemukan.".to_string());
        }
        let file_name = src_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "item-tempel".to_string());

        let mut dest_path = self.current_dir.join(&file_name);
        if dest_path == src_path || dest_path.exists() {
            dest_path = self.current_dir.join(format!("Salinan-{}", file_name));
        }

        match op {
            FileClipboardOp::Copy => {
                copy_recursively(&src_path, &dest_path).map_err(|e| e.to_string())?;
                self.reload();
                Ok(format!("'{}' berhasil ditempel (Copy)", file_name))
            }
            FileClipboardOp::Cut => {
                if fs::rename(&src_path, &dest_path).is_err() {
                    copy_recursively(&src_path, &dest_path).map_err(|e| e.to_string())?;
                    if src_path.is_dir() {
                        let _ = fs::remove_dir_all(&src_path);
                    } else {
                        let _ = fs::remove_file(&src_path);
                    }
                }
                self.file_clipboard = None;
                self.reload();
                Ok(format!("'{}' berhasil dipindahkan (Cut -> Paste)", file_name))
            }
        }
    }

    pub fn duplicate_entry(&mut self, idx: usize) -> Result<String, String> {
        let Some(entry) = self.entries.get(idx).cloned() else {
            return Err("Item tidak valid.".to_string());
        };
        let copy_name = format!("Salinan-{}", entry.name);
        let dest = self.current_dir.join(&copy_name);
        copy_recursively(&entry.path, &dest).map_err(|e| e.to_string())?;
        self.reload();
        Ok(format!("'{}' diduplikasi", copy_name))
    }

    pub fn move_to_trash_or_delete(&mut self, idx: usize) -> Result<String, String> {
        let Some(item) = self.entries.get(idx).cloned() else {
            return Err("Item tidak valid.".to_string());
        };
        let trash_dir = Self::trash_dir();
        if self.current_dir == trash_dir {
            // Inside Trash -> Permanent delete
            if item.is_dir {
                fs::remove_dir_all(&item.path).map_err(|e| e.to_string())?;
            } else {
                fs::remove_file(&item.path).map_err(|e| e.to_string())?;
            }
            self.selected_index = None;
            self.reload();
            Ok(format!("'{}' dihapus permanen dari Sampah", item.name))
        } else {
            let mut dest = trash_dir.join(&item.name);
            if dest.exists() {
                dest = trash_dir.join(format!("{}-{}", self.trash_count + 1, item.name));
            }
            if fs::rename(&item.path, &dest).is_err() {
                copy_recursively(&item.path, &dest).map_err(|e| e.to_string())?;
                if item.is_dir {
                    let _ = fs::remove_dir_all(&item.path);
                } else {
                    let _ = fs::remove_file(&item.path);
                }
            }
            self.selected_index = None;
            self.reload();
            Ok(format!("'{}' dipindahkan ke Tempat Sampah (Trash)", item.name))
        }
    }

    pub fn restore_from_trash(&mut self, idx: usize) -> Result<String, String> {
        let Some(item) = self.entries.get(idx).cloned() else {
            return Err("Pilih berkas di Tempat Sampah untuk dipulihkan.".to_string());
        };
        let home = Self::home_dir();
        let mut dest = home.join(&item.name);
        if dest.exists() {
            dest = home.join(format!("Pulihan-{}", item.name));
        }
        fs::rename(&item.path, &dest).map_err(|e| e.to_string())?;
        self.selected_index = None;
        self.reload();
        Ok(format!("'{}' dipulihkan ke {}", item.name, home.display()))
    }

    pub fn empty_trash(&mut self) -> Result<String, String> {
        let td = Self::trash_dir();
        let mut count = 0;
        if let Ok(rd) = fs::read_dir(&td) {
            for entry in rd.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let _ = fs::remove_dir_all(&p);
                } else {
                    let _ = fs::remove_file(&p);
                }
                count += 1;
            }
        }
        self.selected_index = None;
        self.reload();
        Ok(format!("Tempat Sampah dikosongkan ({} item dihapus)", count))
    }

    pub fn build_quick_look(&self, idx: usize) -> Option<QuickLookData> {
        let item = self.entries.get(idx)?;
        let size_label = if item.is_dir {
            "Direktori Sistem".to_string()
        } else {
            format_size(item.size_bytes)
        };
        let preview_text = if item.is_dir {
            let mut listing = Vec::new();
            if let Ok(rd) = fs::read_dir(&item.path) {
                for sub in rd.flatten().take(40) {
                    listing.push(format!("• {}", sub.file_name().to_string_lossy()));
                }
            }
            if listing.is_empty() {
                "(Folder ini kosong)".to_string()
            } else {
                format!("Isi Folder ({}):\n\n{}", item.name, listing.join("\n"))
            }
        } else {
            match fs::read_to_string(&item.path) {
                Ok(txt) => {
                    let preview: String = txt.lines().take(50).collect::<Vec<_>>().join("\n");
                    if preview.trim().is_empty() {
                        "(Berkas teks kosong)".to_string()
                    } else {
                        preview
                    }
                }
                Err(_) => format!(
                    "Berkas Biner / Citra / Paket Sistem\nNama: {}\nLokasi: {}\nUkuran: {}\nEkstensi: .{}",
                    item.name,
                    item.path.display(),
                    size_label,
                    item.ext
                ),
            }
        };

        Some(QuickLookData {
            name: item.name.clone(),
            path: item.path.to_string_lossy().to_string(),
            is_dir: item.is_dir,
            size_label,
            preview_text,
        })
    }
}

fn copy_recursively(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_recursively(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.join(entry.file_name()))?;
            }
        }
    } else {
        fs::copy(src, dst)?;
    }
    Ok(())
}

pub fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

pub fn is_image_ext(ext: &str) -> bool {
    matches!(ext, "png" | "jpg" | "jpeg" | "webp" | "bmp")
}

fn file_icon_and_color(item: &FileEntryItem) -> (&'static str, Color) {
    if item.is_dir {
        (icons::FOLDER, Color::from_rgb(0.96, 0.62, 0.04)) // Windows 11 Fluent Amber Folder
    } else {
        match item.ext.as_str() {
            "rs" | "toml" | "sh" | "py" | "js" | "ts" | "c" | "cpp" | "json" | "yml" | "yaml" => {
                (icons::FILE_CODE, Color::from_rgb(0.06, 0.65, 0.38))
            }
            "png" | "jpg" | "jpeg" | "svg" | "webp" | "gif" => {
                (icons::IMAGE_ICON, Color::from_rgb(0.55, 0.36, 0.96))
            }
            "mp3" | "flac" | "wav" | "ogg" => {
                (icons::MUSIC_ICON, Color::from_rgb(0.92, 0.28, 0.60))
            }
            "mp4" | "mkv" | "webm" => (icons::VIDEO_ICON, Color::from_rgb(0.92, 0.26, 0.21)),
            "zst" | "gz" | "xz" | "zip" | "tar" | "iso" | "qcow2" => {
                (icons::FILE_ARCHIVE, Color::from_rgb(0.08, 0.68, 0.70))
            }
            _ => (icons::FILE_TEXT, Color::from_rgb(0.45, 0.50, 0.58)),
        }
    }
}

pub fn view_file_manager<'a>(
    state: &'a FileManagerState,
    w: Length,
    h: Length,
) -> Element<'a, Message> {
    let tab_title = match state.current_place {
        FilePlace::Home => "Beranda",
        FilePlace::Desktop => "Desktop",
        FilePlace::Documents => "Dokumen",
        FilePlace::Downloads => "Unduhan",
        FilePlace::Pictures => "Gambar",
        FilePlace::Music => "Musik",
        FilePlace::Trash => "Tempat Sampah",
        FilePlace::Root => "Sistem (Root)",
    };

    let top_header = mouse_area(fluent_window_header_bar(
        "HyperX Files",
        tab_title,
        if state.current_place == FilePlace::Trash {
            icons::TRASH
        } else {
            icons::FOLDER
        },
        if state.current_place == FilePlace::Trash {
            Color::from_rgb(0.91, 0.07, 0.14)
        } else {
            Color::from_rgb(0.96, 0.62, 0.04)
        },
        Message::MinimizeWindow(DesktopWindowId::Files),
        Message::ToggleMaximizeWindow(DesktopWindowId::Files),
        Message::CloseWindow(DesktopWindowId::Files),
    ))
    .on_press(Message::StartWindowDrag(DesktopWindowId::Files))
    .on_release(Message::EndWindowDrag);

    // Fluent Command Ribbon Bar
    let make_ribbon_sep = || {
        container("")
            .width(Length::Fixed(1.0))
            .height(Length::Fixed(16.0))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.10))),
                ..container::Style::default()
            })
    };

    let new_folder_btn = button(
        row![
            svg_icon(icons::FOLDER_PLUS, 14.0, FLUENT_PRIMARY),
            text("Baru").size(12).font(FONT_BODY_SEMIBOLD).style(|_| text::Style {
                color: Some(FLUENT_TEXT_DARK),
            }),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
    )
    .padding([5, 9])
    .style(fluent_ribbon_button_style)
    .on_press(Message::FileCreateFolder);

    let has_selection = state.selected_index.is_some();
    let sel_idx = state.selected_index.unwrap_or(0);

    let mut cut_btn = button(
        row![
            svg_icon(
                icons::SCISSORS_CUT,
                14.0,
                if has_selection { FLUENT_TEXT_DARK } else { FLUENT_TEXT_MUTED },
            ),
            text("Gunting").size(11).font(FONT_BODY_MEDIUM).style(move |_| text::Style {
                color: Some(if has_selection { FLUENT_TEXT_DARK } else { FLUENT_TEXT_MUTED }),
            }),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding([5, 8])
    .style(fluent_ribbon_button_style);
    if has_selection {
        cut_btn = cut_btn.on_press(Message::FileCutSelected(sel_idx));
    }

    let mut copy_btn = button(
        row![
            svg_icon(
                icons::COPY,
                14.0,
                if has_selection { FLUENT_TEXT_DARK } else { FLUENT_TEXT_MUTED },
            ),
            text("Salin").size(11).font(FONT_BODY_MEDIUM).style(move |_| text::Style {
                color: Some(if has_selection { FLUENT_TEXT_DARK } else { FLUENT_TEXT_MUTED }),
            }),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding([5, 8])
    .style(fluent_ribbon_button_style);
    if has_selection {
        copy_btn = copy_btn.on_press(Message::FileCopySelected(sel_idx));
    }

    let has_clipboard = state.file_clipboard.is_some();
    let mut paste_btn = button(
        row![
            svg_icon(
                icons::PASTE,
                14.0,
                if has_clipboard { FLUENT_PRIMARY } else { FLUENT_TEXT_MUTED },
            ),
            text("Tempel").size(11).font(FONT_BODY_MEDIUM).style(move |_| text::Style {
                color: Some(if has_clipboard { FLUENT_PRIMARY } else { FLUENT_TEXT_MUTED }),
            }),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding([5, 8])
    .style(fluent_ribbon_button_style);
    if has_clipboard {
        paste_btn = paste_btn.on_press(Message::FilePasteClipboard);
    }

    let mut rename_btn = button(
        row![
            svg_icon(
                icons::EDIT_PEN,
                14.0,
                if has_selection { FLUENT_TEXT_DARK } else { FLUENT_TEXT_MUTED },
            ),
            text("Ganti Nama").size(11).font(FONT_BODY_MEDIUM).style(move |_| text::Style {
                color: Some(if has_selection { FLUENT_TEXT_DARK } else { FLUENT_TEXT_MUTED }),
            }),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding([5, 8])
    .style(fluent_ribbon_button_style);
    if has_selection {
        rename_btn = rename_btn.on_press(Message::FileContextStartRename(sel_idx));
    }

    let mut del_btn = button(
        row![
            svg_icon(
                icons::TRASH,
                14.0,
                if has_selection { Color::from_rgb(0.91, 0.07, 0.14) } else { FLUENT_TEXT_MUTED },
            ),
            text("Hapus").size(11).font(FONT_BODY_MEDIUM).style(move |_| text::Style {
                color: Some(if has_selection { Color::from_rgb(0.91, 0.07, 0.14) } else { FLUENT_TEXT_MUTED }),
            }),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding([5, 8])
    .style(fluent_ribbon_button_style);
    if has_selection {
        del_btn = del_btn.on_press(Message::FileDeleteSelected);
    }

    let view_btn = button(
        row![
            svg_icon(
                if state.grid_mode { icons::GRID_VIEW } else { icons::LIST_VIEW },
                14.0,
                FLUENT_PRIMARY,
            ),
            text(if state.grid_mode { "Grid" } else { "Daftar" })
                .size(11)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding([5, 8])
    .style(fluent_ribbon_button_style)
    .on_press(Message::FileToggleViewMode);

    let mut ribbon = row![new_folder_btn, make_ribbon_sep(), cut_btn, copy_btn, paste_btn, rename_btn, del_btn]
        .spacing(3)
        .align_y(Alignment::Center);

    if state.current_place == FilePlace::Trash {
        ribbon = ribbon.push(make_ribbon_sep());
        if let Some(sel) = state.selected_index {
            ribbon = ribbon.push(
                button(
                    row![
                        svg_icon(icons::ROTATE_CCW, 13.0, Color::from_rgb(0.06, 0.65, 0.38)),
                        text("Pulihkan").size(11).font(FONT_BODY_SEMIBOLD),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                )
                .padding([5, 9])
                .style(fluent_ribbon_button_style)
                .on_press(Message::FileRestoreFromTrash(sel)),
            );
        }
        ribbon = ribbon.push(
            button(
                row![
                    svg_icon(icons::TRASH, 13.0, Color::from_rgb(0.91, 0.07, 0.14)),
                    text("Kosongkan Sampah").size(11).font(FONT_BODY_SEMIBOLD),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .padding([5, 9])
            .style(fluent_ribbon_button_style)
            .on_press(Message::FileEmptyTrash),
        );
    }

    ribbon = ribbon.push(make_ribbon_sep()).push(view_btn);

    let command_bar = container(ribbon)
        .padding([4, 12])
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.60))),
            border: Border {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                width: 1.0,
                radius: Radius::default(),
            },
            ..container::Style::default()
        });

    // Breadcrumb Navigation & Search Row
    let nav_up_btn = button(
        container(svg_icon(icons::ARROW_UP, 14.0, FLUENT_TEXT_DARK))
            .width(Length::Fixed(28.0))
            .height(Length::Fixed(28.0))
            .center_x(Length::Fixed(28.0))
            .center_y(Length::Fixed(28.0)),
    )
    .padding(0)
    .style(fluent_ribbon_button_style)
    .on_press(Message::FileNavigateUp);

    let refresh_btn = button(
        container(svg_icon(icons::REFRESH, 14.0, FLUENT_TEXT_DARK))
            .width(Length::Fixed(28.0))
            .height(Length::Fixed(28.0))
            .center_x(Length::Fixed(28.0))
            .center_y(Length::Fixed(28.0)),
    )
    .padding(0)
    .style(fluent_ribbon_button_style)
    .on_press(Message::FileRefresh);

    let address_breadcrumb = container(
        row![
            svg_icon(icons::COMPUTER, 14.0, FLUENT_PRIMARY),
            text("PC Ini")
                .size(12)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.35, 0.38, 0.44)),
                }),
            svg_icon(icons::CHEVRON_RIGHT, 11.0, Color::from_rgb(0.60, 0.63, 0.68)),
            svg_icon(
                if state.current_place == FilePlace::Trash {
                    icons::TRASH
                } else {
                    icons::FOLDER
                },
                14.0,
                if state.current_place == FilePlace::Trash {
                    Color::from_rgb(0.91, 0.07, 0.14)
                } else {
                    Color::from_rgb(0.96, 0.62, 0.04)
                },
            ),
            text(tab_title)
                .size(12)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 10])
    .width(Length::Fill)
    .style(fluent_address_bar_style);

    let search_input = text_input(&format!("Cari di {}...", tab_title), &state.search_query)
        .on_input(Message::FileSearchChanged)
        .padding([5, 10])
        .size(12)
        .width(Length::Fixed(210.0))
        .style(fluent_input_style);

    let breadcrumb_row = container(
        row![nav_up_btn, refresh_btn, address_breadcrumb, search_input]
            .spacing(8)
            .align_y(Alignment::Center),
    )
    .padding([5, 14])
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.40))),
        border: Border {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            width: 1.0,
            radius: Radius::default(),
        },
        ..container::Style::default()
    });

    let make_subtle_divider = || {
        container("")
            .width(Length::Fill)
            .height(Length::Fixed(1.0))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06))),
                ..container::Style::default()
            })
    };

    let section_label = |name: &'static str| {
        text(name)
            .size(10)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(Color::from_rgb(0.55, 0.58, 0.64)),
            })
    };

    let sidebar = container(
        column![
            section_label("AKSES CEPAT"),
            fluent_sidebar_nav_item(
                "Beranda",
                icons::HOME_ICON,
                state.current_place == FilePlace::Home,
                Color::from_rgb(0.0, 0.47, 0.83),
                Message::FileSelectPlace(FilePlace::Home),
            ),
            fluent_sidebar_nav_item(
                "Desktop",
                icons::DASHBOARD,
                state.current_place == FilePlace::Desktop,
                Color::from_rgb(0.96, 0.62, 0.04),
                Message::FileSelectPlace(FilePlace::Desktop),
            ),
            fluent_sidebar_nav_item(
                "Dokumen",
                icons::DOCS,
                state.current_place == FilePlace::Documents,
                Color::from_rgb(0.96, 0.62, 0.04),
                Message::FileSelectPlace(FilePlace::Documents),
            ),
            fluent_sidebar_nav_item(
                "Unduhan",
                icons::DOWNLOAD,
                state.current_place == FilePlace::Downloads,
                Color::from_rgb(0.0, 0.47, 0.83),
                Message::FileSelectPlace(FilePlace::Downloads),
            ),
            fluent_sidebar_nav_item(
                "Gambar",
                icons::IMAGE_ICON,
                state.current_place == FilePlace::Pictures,
                Color::from_rgb(0.55, 0.36, 0.96),
                Message::FileSelectPlace(FilePlace::Pictures),
            ),
            fluent_sidebar_nav_item(
                "Musik",
                icons::MUSIC_ICON,
                state.current_place == FilePlace::Music,
                Color::from_rgb(0.92, 0.28, 0.60),
                Message::FileSelectPlace(FilePlace::Music),
            ),
            make_subtle_divider(),
            section_label("PC INI"),
            fluent_sidebar_nav_item(
                "Sistem Root (/)",
                icons::HARD_DRIVE,
                state.current_place == FilePlace::Root,
                Color::from_rgb(0.40, 0.45, 0.52),
                Message::FileSelectPlace(FilePlace::Root),
            ),
            fluent_sidebar_nav_item(
                "Tempat Sampah",
                icons::TRASH,
                state.current_place == FilePlace::Trash,
                Color::from_rgb(0.91, 0.07, 0.14),
                Message::FileSelectPlace(FilePlace::Trash),
            ),
            container("").height(Length::Fill),
            container(
                column![
                    text(format!("Sampah: {} item", state.trash_count))
                        .size(11)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(FLUENT_TEXT_DARK),
                        }),
                    text(&state.status_message)
                        .size(9)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(FLUENT_TEXT_MUTED),
                        }),
                ]
                .spacing(2),
            )
            .padding([6, 8])
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.03))),
                border: Border::default().rounded(Radius::from(6.0)),
                ..container::Style::default()
            }),
        ]
        .spacing(4)
        .padding(10),
    )
    .width(Length::Fixed(190.0))
    .height(Length::Fill)
    .style(fluent_sidebar_container_style);

    let q = state.search_query.trim().to_lowercase();
    let filtered: Vec<(usize, &FileEntryItem)> = state
        .entries
        .iter()
        .enumerate()
        .filter(|(_, item)| q.is_empty() || item.name.to_lowercase().contains(&q))
        .collect();

    let files_view: Element<'a, Message> = if state.grid_mode {
        let mut grid_col = column![].spacing(8);
        for chunk in filtered.chunks(5) {
            let mut r = row![].spacing(8);
            for (idx, item) in chunk {
                let is_sel = state.selected_index == Some(*idx);
                let (ic, col) = file_icon_and_color(item);
                let sub = if item.is_dir {
                    "Folder file".to_string()
                } else {
                    format_size(item.size_bytes)
                };

                let card_btn = button(
                    column![
                        container(svg_icon(ic, 38.0, col))
                            .width(Length::Fill)
                            .center_x(Length::Fill),
                        text(&item.name)
                            .size(11)
                            .font(FONT_BODY_SEMIBOLD)
                            .style(|_| text::Style {
                                color: Some(FLUENT_TEXT_DARK),
                            }),
                        text(sub)
                            .size(9)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(FLUENT_TEXT_MUTED),
                            }),
                    ]
                    .spacing(3)
                    .align_x(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([8, 6])
                .style(fluent_file_item_style(is_sel))
                .on_press(Message::FileEntryClicked(*idx));

                let interactive_card = mouse_area(card_btn)
                    .on_right_press(Message::OpenFileEntryContextMenu(*idx));

                r = r.push(interactive_card);
            }
            for _ in chunk.len()..5 {
                r = r.push(container("").width(Length::Fill));
            }
            grid_col = grid_col.push(r);
        }
        glass_scrollable(grid_col.padding(14))
            .height(Length::Fill)
            .into()
    } else {
        let mut list_col = column![].spacing(4);
        for (idx, item) in filtered {
            let is_sel = state.selected_index == Some(idx);
            let (ic, col) = file_icon_and_color(item);
            let sub = if item.is_dir {
                "Folder".to_string()
            } else {
                format_size(item.size_bytes)
            };

            let row_btn = button(
                row![
                    svg_icon(ic, 16.0, col),
                    text(&item.name)
                        .size(12)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(FLUENT_TEXT_DARK),
                        }),
                    container("").width(Length::Fill),
                    text(sub)
                        .size(11)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(FLUENT_TEXT_MUTED),
                        }),
                ]
                .spacing(10)
                .align_y(Alignment::Center)
                .padding([5, 10]),
            )
            .width(Length::Fill)
            .style(fluent_file_item_style(is_sel))
            .on_press(Message::FileEntryClicked(idx));

            let interactive_row =
                mouse_area(row_btn).on_right_press(Message::OpenFileEntryContextMenu(idx));

            list_col = list_col.push(interactive_row);
        }
        glass_scrollable(list_col.padding(12))
            .height(Length::Fill)
            .into()
    };

    let rename_bar: Option<Element<'a, Message>> = if state.renaming_index.is_some() {
        Some(
            container(
                row![
                    svg_icon(icons::EDIT_PEN, 15.0, FLUENT_PRIMARY),
                    text("Ganti Nama:")
                        .size(12)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(FLUENT_TEXT_DARK),
                        }),
                    text_input("Ketik nama baru...", &state.rename_input)
                        .on_input(Message::FileRenameInputChanged)
                        .on_submit(Message::FileCommitRename)
                        .padding([5, 10])
                        .size(12)
                        .width(Length::Fill)
                        .style(fluent_input_style),
                    button(text("Simpan").size(11).font(FONT_BODY_SEMIBOLD))
                        .padding([5, 12])
                        .style(primary_button_style(FLUENT_PRIMARY))
                        .on_press(Message::FileCommitRename),
                    button(text("Batal").size(11).font(FONT_BODY_MEDIUM))
                        .padding([5, 10])
                        .style(fluent_ribbon_button_style)
                        .on_press(Message::FileCancelRename),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
                .padding([6, 12]),
            )
            .width(Length::Fill)
            .style(fluent_address_bar_style)
            .into(),
        )
    } else {
        None
    };

    let status_footer: Element<'a, Message> = if let Some(sel_idx) = state.selected_index {
        if let Some(item) = state.entries.get(sel_idx) {
            let (ic, col) = file_icon_and_color(item);
            let mut action_row = row![
                svg_icon(ic, 16.0, col),
                text(&item.name)
                    .size(11)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
                container("").width(Length::Fill),
            ]
            .spacing(8)
            .align_y(Alignment::Center);

            if is_image_ext(&item.ext) {
                action_row = action_row.push(
                    button(
                        row![
                            svg_icon(icons::SPARKLES, 12.0, Color::from_rgb(0.55, 0.36, 0.96)),
                            text("Jadikan Wallpaper").size(11).font(FONT_BODY_SEMIBOLD),
                        ]
                        .spacing(4)
                        .align_y(Alignment::Center),
                    )
                    .padding([4, 8])
                    .style(fluent_ribbon_button_style)
                    .on_press(Message::SetCustomWallpaperFromFile(item.path.clone())),
                );
            }

            action_row = action_row.push(
                button(
                    row![
                        svg_icon(icons::EYE, 12.0, FLUENT_PRIMARY),
                        text("Quick Look").size(11).font(FONT_BODY_SEMIBOLD),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                )
                .padding([4, 8])
                .style(fluent_ribbon_button_style)
                .on_press(Message::FileContextQuickLook(sel_idx)),
            );

            container(action_row.padding([4, 12]))
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.50))),
                    border: Border {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                        width: 1.0,
                        radius: Radius::default().bottom_right(12.0),
                    },
                    ..container::Style::default()
                })
                .into()
        } else {
            container("").into()
        }
    } else {
        container(
            row![
                text(format!("{} item", state.entries.len()))
                    .size(11)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
                text("|").size(11).style(|_| text::Style {
                    color: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.15)),
                }),
                text(&state.status_message)
                    .size(11)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .padding([4, 12]),
        )
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.50))),
            border: Border {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                width: 1.0,
                radius: Radius::default().bottom_right(12.0),
            },
            ..container::Style::default()
        })
        .into()
    };

    let mut right_pane = column![
        top_header,
        command_bar,
        breadcrumb_row,
        files_view,
    ]
    .spacing(0);

    if let Some(rb) = rename_bar {
        right_pane = right_pane.push(container(rb).padding([4, 12]));
    }
    right_pane = right_pane.push(status_footer);

    container(
        row![
            sidebar,
            right_pane.width(Length::Fill).height(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(w)
    .height(h)
    .style(fluent_window_style)
    .into()
}

pub fn open_file_or_dir(path: &Path) -> Result<String, String> {
    std::process::Command::new("xdg-open")
        .arg(path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(format!("Membuka '{}'", path.display()))
}
