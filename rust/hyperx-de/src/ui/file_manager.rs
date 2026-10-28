use std::fs;
use std::path::{Path, PathBuf};

use iced::widget::{button, column, container, mouse_area, row, text, text_input};
use iced::{Alignment, Background, Color, Element, Length};

use hyperx_ui_kit::{
    active_toggle_button_style, brand_sidebar_header, danger_button_style, glass_action_button,
    glass_card_accent_style, glass_card_style, glass_input_style, glass_scrollable,
    glass_window_style, icons, primary_button_style, selection_card_style, sidebar_container_style,
    sidebar_nav_item, sidebar_section_header, svg_icon, window_header_bar, ACCENT_AMBER,
    ACCENT_CYAN, ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED, ACCENT_TEAL, FONT_BODY_MEDIUM,
    FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD, TEXT_MAIN, TEXT_MUTED, TEXT_SUB,
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
        (icons::FOLDER, ACCENT_CYAN)
    } else {
        match item.ext.as_str() {
            "rs" | "toml" | "sh" | "py" | "js" | "ts" | "c" | "cpp" | "json" | "yml" | "yaml" => {
                (icons::FILE_CODE, ACCENT_GREEN)
            }
            "png" | "jpg" | "jpeg" | "svg" | "webp" | "gif" => (icons::IMAGE_ICON, ACCENT_PURPLE),
            "mp3" | "flac" | "wav" | "ogg" => (icons::MUSIC_ICON, ACCENT_AMBER),
            "mp4" | "mkv" | "webm" => (icons::VIDEO_ICON, ACCENT_RED),
            "zst" | "gz" | "xz" | "zip" | "tar" | "iso" | "qcow2" => {
                (icons::FILE_ARCHIVE, ACCENT_TEAL)
            }
            _ => (icons::FILE_TEXT, TEXT_SUB),
        }
    }
}

pub fn view_file_manager<'a>(
    state: &'a FileManagerState,
    w: Length,
    h: Length,
) -> Element<'a, Message> {
    let brand = brand_sidebar_header("HYPERX FILES", "Manajer Berkas Finder");

    let sep = container("")
        .width(Length::Fill)
        .height(1)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.20))),
            ..container::Style::default()
        });

    let nav_links = column![
        sidebar_section_header("LOKASI FAVORIT"),
        sidebar_nav_item(
            "Beranda (Home)",
            icons::HOME_ICON,
            state.current_place == FilePlace::Home,
            Message::FileSelectPlace(FilePlace::Home),
        ),
        sidebar_nav_item(
            "Desktop",
            icons::DASHBOARD,
            state.current_place == FilePlace::Desktop,
            Message::FileSelectPlace(FilePlace::Desktop),
        ),
        sidebar_nav_item(
            "Dokumen",
            icons::DOCS,
            state.current_place == FilePlace::Documents,
            Message::FileSelectPlace(FilePlace::Documents),
        ),
        sidebar_nav_item(
            "Unduhan",
            icons::DOWNLOAD,
            state.current_place == FilePlace::Downloads,
            Message::FileSelectPlace(FilePlace::Downloads),
        ),
        sidebar_nav_item(
            "Gambar & Screenshot",
            icons::IMAGE_ICON,
            state.current_place == FilePlace::Pictures,
            Message::FileSelectPlace(FilePlace::Pictures),
        ),
        sidebar_nav_item(
            "Musik & Audio",
            icons::MUSIC_ICON,
            state.current_place == FilePlace::Music,
            Message::FileSelectPlace(FilePlace::Music),
        ),
        sidebar_section_header("SAMPAH & SISTEM"),
        sidebar_nav_item(
            "Tempat Sampah (Trash)",
            icons::TRASH,
            state.current_place == FilePlace::Trash,
            Message::FileSelectPlace(FilePlace::Trash),
        ),
        sidebar_nav_item(
            "Sistem Root (/)",
            icons::HARD_DRIVE,
            state.current_place == FilePlace::Root,
            Message::FileSelectPlace(FilePlace::Root),
        ),
    ]
    .spacing(5);

    let sidebar_footer = container(
        column![
            text(format!("Trash: {} item", state.trash_count))
                .size(11)
                .font(FONT_DISPLAY_BOLD),
            text(&state.status_message)
                .size(9)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                }),
        ]
        .spacing(3),
    )
    .padding([9, 12])
    .width(Length::Fill)
    .style(glass_card_style);

    let sidebar = container(
        column![
            brand,
            sep,
            nav_links,
            container("").height(Length::Fill),
            sidebar_footer,
        ]
        .spacing(10)
        .padding(14),
    )
    .width(Length::Fixed(220.0))
    .height(Length::Fill)
    .style(sidebar_container_style);

    let path_str = state.current_dir.to_string_lossy().to_string();
    let top_header = mouse_area(window_header_bar(
        if state.current_place == FilePlace::Trash {
            "HyperX Files — Tempat Sampah (macOS Trash)"
        } else {
            "HyperX Files — Manajer Berkas Finder"
        },
        &path_str,
        if state.current_place == FilePlace::Trash {
            icons::TRASH
        } else {
            icons::FOLDER
        },
        if state.current_place == FilePlace::Trash {
            ACCENT_AMBER
        } else {
            ACCENT_CYAN
        },
        Message::MinimizeWindow(DesktopWindowId::Files),
        Message::ToggleMaximizeWindow(DesktopWindowId::Files),
        Message::CloseWindow(DesktopWindowId::Files),
    ))
    .on_press(Message::StartWindowDrag(DesktopWindowId::Files))
    .on_release(Message::EndWindowDrag);

    // Toolbar adapts if user is in Trash or has a File in Clipboard
    let mut toolbar = row![
        button(
            row![
                svg_icon(icons::ARROW_UP, 13.0, ACCENT_CYAN),
                text("Naik").size(11).font(FONT_BODY_SEMIBOLD),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .padding([6, 9])
        .style(glass_action_button)
        .on_press(Message::FileNavigateUp),
        button(
            row![
                svg_icon(icons::REFRESH, 13.0, ACCENT_GREEN),
                text("Segarkan").size(11).font(FONT_BODY_SEMIBOLD),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .padding([6, 9])
        .style(glass_action_button)
        .on_press(Message::FileRefresh),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    if state.file_clipboard.is_some() {
        toolbar = toolbar.push(
            button(
                row![
                    svg_icon(icons::PASTE, 13.0, ACCENT_GREEN),
                    text("Tempel / Paste (Ctrl+V)")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .padding([6, 10])
            .style(primary_button_style(ACCENT_GREEN))
            .on_press(Message::FilePasteClipboard),
        );
    }

    if state.current_place == FilePlace::Trash {
        if let Some(sel) = state.selected_index {
            toolbar = toolbar.push(
                button(
                    row![
                        svg_icon(icons::ROTATE_CCW, 13.0, ACCENT_GREEN),
                        text("Pulihkan (Restore)").size(11).font(FONT_DISPLAY_BOLD),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                )
                .padding([6, 10])
                .style(primary_button_style(ACCENT_GREEN))
                .on_press(Message::FileRestoreFromTrash(sel)),
            );
        }
        toolbar = toolbar.push(
            button(
                row![
                    svg_icon(icons::TRASH, 13.0, ACCENT_RED),
                    text("Kosongkan Sampah (Empty Trash)")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .padding([6, 12])
            .style(danger_button_style())
            .on_press(Message::FileEmptyTrash),
        );
    } else {
        toolbar = toolbar
            .push(
                button(
                    text(if state.grid_mode { "Grid" } else { "Daftar" })
                        .size(11)
                        .font(FONT_BODY_SEMIBOLD),
                )
                .padding([6, 9])
                .style(active_toggle_button_style(state.grid_mode, ACCENT_CYAN))
                .on_press(Message::FileToggleViewMode),
            )
            .push(
                text_input("Cari berkas...", &state.search_query)
                    .on_input(Message::FileSearchChanged)
                    .padding([6, 10])
                    .size(11)
                    .width(Length::Fixed(135.0))
                    .style(glass_input_style),
            )
            .push(
                text_input("Nama folder baru...", &state.new_folder_name)
                    .on_input(Message::FileNewFolderNameChanged)
                    .on_submit(Message::FileCreateFolder)
                    .padding([6, 10])
                    .size(11)
                    .width(Length::Fixed(125.0))
                    .style(glass_input_style),
            )
            .push(
                button(
                    row![
                        svg_icon(icons::FOLDER_PLUS, 13.0, ACCENT_AMBER),
                        text("Folder Baru").size(11).font(FONT_BODY_SEMIBOLD),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                )
                .padding([6, 10])
                .style(primary_button_style(ACCENT_AMBER))
                .on_press(Message::FileCreateFolder),
            );
    }

    let toolbar = toolbar.padding([6, 14]);

    let q = state.search_query.trim().to_lowercase();
    let filtered: Vec<(usize, &FileEntryItem)> = state
        .entries
        .iter()
        .enumerate()
        .filter(|(_, item)| q.is_empty() || item.name.to_lowercase().contains(&q))
        .collect();

    let files_view: Element<'a, Message> = if state.grid_mode {
        let mut grid_col = column![].spacing(10);
        for chunk in filtered.chunks(4) {
            let mut r = row![].spacing(10);
            for (idx, item) in chunk {
                let is_sel = state.selected_index == Some(*idx);
                let (ic, col) = file_icon_and_color(item);
                let sub = if item.is_dir {
                    "Direktori".to_string()
                } else {
                    format_size(item.size_bytes)
                };

                let card_btn = button(
                    column![
                        row![
                            svg_icon(ic, 24.0, col),
                            container("").width(Length::Fill),
                            text(sub)
                                .size(9)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(|_| text::Style {
                                    color: Some(TEXT_MUTED),
                                }),
                        ]
                        .align_y(Alignment::Center),
                        text(&item.name)
                            .size(12)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style {
                                color: Some(TEXT_MAIN),
                            }),
                    ]
                    .spacing(8)
                    .padding(6),
                )
                .width(Length::Fill)
                .style(active_toggle_button_style(is_sel, col))
                .on_press(Message::FileEntryClicked(*idx));

                let interactive_card = mouse_area(card_btn)
                    .on_right_press(Message::OpenFileEntryContextMenu(*idx));

                r = r.push(interactive_card);
            }
            grid_col = grid_col.push(r);
        }
        glass_scrollable(grid_col.padding(14))
            .height(Length::Fill)
            .into()
    } else {
        let mut list_col = column![].spacing(6);
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
                    text(&item.name).size(12).font(FONT_DISPLAY_BOLD),
                    container("").width(Length::Fill),
                    text(sub).size(11).font(FONT_BODY_MEDIUM),
                ]
                .spacing(10)
                .align_y(Alignment::Center)
                .padding([4, 8]),
            )
            .width(Length::Fill)
            .style(active_toggle_button_style(is_sel, col))
            .on_press(Message::FileEntryClicked(idx));

            let interactive_row =
                mouse_area(row_btn).on_right_press(Message::OpenFileEntryContextMenu(idx));

            list_col = list_col.push(interactive_row);
        }
        glass_scrollable(list_col.padding(14))
            .height(Length::Fill)
            .into()
    };

    let rename_bar: Option<Element<'a, Message>> = if state.renaming_index.is_some() {
        Some(
            container(
                row![
                    svg_icon(icons::TAG, 15.0, ACCENT_AMBER),
                    text("Ganti Nama:")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD),
                    text_input("Ketik nama baru...", &state.rename_input)
                        .on_input(Message::FileRenameInputChanged)
                        .on_submit(Message::FileCommitRename)
                        .padding([6, 10])
                        .size(11)
                        .width(Length::Fill)
                        .style(glass_input_style),
                    button(text("Simpan Nama").size(11).font(FONT_DISPLAY_BOLD))
                        .padding([6, 12])
                        .style(primary_button_style(ACCENT_GREEN))
                        .on_press(Message::FileCommitRename),
                    button(text("Batal").size(11).font(FONT_BODY_SEMIBOLD))
                        .padding([6, 10])
                        .style(glass_action_button)
                        .on_press(Message::FileCancelRename),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
                .padding([8, 14]),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_AMBER))
            .into(),
        )
    } else {
        None
    };

    let inspector_bar: Element<'a, Message> = if let Some(sel_idx) = state.selected_index {
        if let Some(item) = state.entries.get(sel_idx) {
            let (ic, col) = file_icon_and_color(item);
            let mut action_row = row![
                svg_icon(ic, 18.0, col),
                column![
                    text(&item.name).size(12).font(FONT_DISPLAY_BOLD),
                    text(item.path.to_string_lossy().to_string())
                        .size(9)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(TEXT_MUTED),
                        }),
                ]
                .spacing(1),
                container("").width(Length::Fill),
            ]
            .spacing(7)
            .align_y(Alignment::Center);

            if is_image_ext(&item.ext) {
                action_row = action_row.push(
                    button(
                        row![
                            svg_icon(icons::SPARKLES, 12.0, ACCENT_PURPLE),
                            text("Jadikan Wallpaper").size(10).font(FONT_DISPLAY_BOLD),
                        ]
                        .spacing(5)
                        .align_y(Alignment::Center),
                    )
                    .padding([5, 10])
                    .style(primary_button_style(ACCENT_PURPLE))
                    .on_press(Message::SetCustomWallpaperFromFile(item.path.clone())),
                );
            }

            action_row = action_row
                .push(
                    button(
                        row![
                            svg_icon(icons::COPY, 12.0, ACCENT_CYAN),
                            text("Copy (Ctrl+C)").size(10).font(FONT_BODY_SEMIBOLD),
                        ]
                        .spacing(4)
                        .align_y(Alignment::Center),
                    )
                    .padding([5, 9])
                    .style(glass_action_button)
                    .on_press(Message::FileCopySelected(sel_idx)),
                )
                .push(
                    button(
                        row![
                            svg_icon(icons::SCISSORS_CUT, 12.0, ACCENT_AMBER),
                            text("Cut (Ctrl+X)").size(10).font(FONT_BODY_SEMIBOLD),
                        ]
                        .spacing(4)
                        .align_y(Alignment::Center),
                    )
                    .padding([5, 9])
                    .style(glass_action_button)
                    .on_press(Message::FileCutSelected(sel_idx)),
                )
                .push(
                    button(
                        row![
                            svg_icon(icons::EYE, 12.0, ACCENT_PURPLE),
                            text("Quick Look").size(10).font(FONT_BODY_SEMIBOLD),
                        ]
                        .spacing(4)
                        .align_y(Alignment::Center),
                    )
                    .padding([5, 9])
                    .style(primary_button_style(ACCENT_PURPLE))
                    .on_press(Message::FileContextQuickLook(sel_idx)),
                )
                .push(
                    button(
                        row![
                            svg_icon(icons::TRASH, 12.0, ACCENT_RED),
                            text(if state.current_place == FilePlace::Trash {
                                "Hapus Permanen"
                            } else {
                                "Ke Sampah"
                            })
                            .size(10)
                            .font(FONT_BODY_SEMIBOLD),
                        ]
                        .spacing(4)
                        .align_y(Alignment::Center),
                    )
                    .padding([5, 9])
                    .style(danger_button_style())
                    .on_press(Message::FileDeleteSelected),
                );

            container(action_row.padding([8, 14]))
                .width(Length::Fill)
                .style(glass_card_accent_style(col))
                .into()
        } else {
            container("").into()
        }
    } else {
        container(
            row![
                svg_icon(icons::INFO, 14.0, ACCENT_CYAN),
                text("Ctrl+C (Copy), Ctrl+X (Cut), Ctrl+V (Paste), Super+V (Clipboard HUD), Delete (Trash), atau Klik Kanan.")
                    .size(10)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(TEXT_MUTED),
                    }),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .padding([7, 14]),
        )
        .width(Length::Fill)
        .style(selection_card_style(false))
        .into()
    };

    let mut right_content = column![top_header, toolbar, files_view].spacing(6);
    if let Some(rb) = rename_bar {
        right_content = right_content.push(container(rb).padding([0, 14]));
    }
    right_content = right_content.push(container(inspector_bar).padding([0, 14]));

    container(
        row![
            sidebar,
            right_content.width(Length::Fill).height(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(w)
    .height(h)
    .style(glass_window_style)
    .into()
}

pub fn open_file_or_dir(path: &Path) -> Result<String, String> {
    std::process::Command::new("xdg-open")
        .arg(path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(format!("Membuka '{}'", path.display()))
}
