use std::path::PathBuf;

use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Alignment, Background, Color, Element, Length, Point};

use hyperx_ui_kit::{
    context_menu_glass_style, context_menu_item_style, icons, svg_icon, ACCENT_AMBER, ACCENT_CYAN,
    ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD,
    FONT_DISPLAY_BOLD, TEXT_MUTED,
};

use crate::ui::settings::SettingsTab;
use crate::ui::window_manager::WindowSnapMode;
use crate::ui::{DesktopWindowId, Message};

#[derive(Debug, Clone, PartialEq)]
pub enum ContextMenuKind {
    Desktop,
    DesktopIcon(usize),
    FileEntry(usize),
    DockItem(DesktopWindowId),
    PinnedDockApp(usize),
    TrashDock,
}

#[derive(Debug, Clone)]
pub struct ActiveContextMenu {
    pub kind: ContextMenuKind,
    pub pos: Point,
}

fn menu_separator<'a>() -> Element<'a, Message> {
    container("")
        .width(Length::Fill)
        .height(1)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.14))),
            ..container::Style::default()
        })
        .into()
}

fn menu_item<'a>(
    label: &'a str,
    shortcut: &'a str,
    icon_svg: &'static str,
    accent: Color,
    is_danger: bool,
    on_press: Message,
) -> Element<'a, Message> {
    button(
        row![
            svg_icon(icon_svg, 14.0, if is_danger { ACCENT_RED } else { accent }),
            text(label).size(12).font(FONT_BODY_SEMIBOLD),
            container("").width(Length::Fill),
            text(shortcut)
                .size(10)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED),
                }),
        ]
        .spacing(10)
        .align_y(Alignment::Center)
        .padding([5, 8]),
    )
    .width(Length::Fill)
    .style(context_menu_item_style(is_danger))
    .on_press(on_press)
    .into()
}

pub fn view_context_menu<'a>(
    menu: &'a ActiveContextMenu,
    icon_title: Option<&'a str>,
    file_name: Option<&'a str>,
    file_path: Option<PathBuf>,
    is_image: bool,
    has_file_clipboard: bool,
) -> Element<'a, Message> {
    let menu_col: Element<'a, Message> = match &menu.kind {
        ContextMenuKind::Desktop => column![
            container(
                text("MENU DESKTOP HYPERXOS")
                    .size(9)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_CYAN),
                    }),
            )
            .padding([4, 8]),
            menu_item(
                "Buat Folder Baru di Desktop",
                "Ctrl+Shift+N",
                icons::FOLDER_PLUS,
                ACCENT_AMBER,
                false,
                Message::DesktopCreateNewFolder,
            ),
            menu_item(
                "Buka HyperX Files",
                "Super+E",
                icons::FOLDER,
                ACCENT_CYAN,
                false,
                Message::OpenWindow(DesktopWindowId::Files),
            ),
            menu_item(
                "Buka Terminal & Monitor",
                "Super+T",
                icons::TERMINAL,
                ACCENT_GREEN,
                false,
                Message::OpenWindow(DesktopWindowId::Terminal),
            ),
            menu_item(
                "Ambil Screenshot Layar...",
                "Super+Shift+S",
                icons::SNAPSHOT,
                ACCENT_PURPLE,
                false,
                Message::ToggleScreenshotHud,
            ),
            menu_item(
                "Riwayat Clipboard Manager",
                "Super+V",
                icons::CLIPBOARD,
                ACCENT_CYAN,
                false,
                Message::ToggleClipboardHud,
            ),
            menu_separator(),
            menu_item(
                "Rapikan Ikon ke Grid Otomatis",
                "Grid",
                icons::LAUNCHPAD_GRID,
                ACCENT_CYAN,
                false,
                Message::DesktopAutoArrangeIcons,
            ),
            menu_item(
                "Ganti Tema / Custom Wallpaper",
                "Cycle",
                icons::SPARKLES,
                ACCENT_PURPLE,
                false,
                Message::CycleWallpaperTheme,
            ),
            menu_item(
                "Mission Control (Semua Jendela)",
                "Super+Tab",
                icons::LAYERS,
                ACCENT_GREEN,
                false,
                Message::ToggleMissionControl,
            ),
            menu_separator(),
            menu_item(
                "Pengaturan Wallpaper & OS...",
                "Super+I",
                icons::SETTINGS_GEAR,
                ACCENT_AMBER,
                false,
                Message::OpenSettingsPage(SettingsTab::Appearance),
            ),
            menu_item(
                "Kunci Layar (Lock Screen)",
                "Super+L",
                icons::LOCK,
                ACCENT_RED,
                true,
                Message::ToggleLockScreen,
            ),
        ]
        .spacing(3)
        .padding(8)
        .width(Length::Fixed(285.0))
        .into(),

        ContextMenuKind::DesktopIcon(idx) => {
            let title = icon_title.unwrap_or("Pintasan Desktop");
            column![
                container(
                    text(title)
                        .size(10)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style {
                            color: Some(ACCENT_CYAN),
                        }),
                )
                .padding([4, 8]),
                menu_item(
                    "Buka Aplikasi / Pintasan",
                    "Enter",
                    icons::ROCKET,
                    ACCENT_CYAN,
                    false,
                    Message::DesktopOpenIcon(*idx),
                ),
                menu_item(
                    "Rapikan Semua Ikon ke Grid",
                    "Snap",
                    icons::LAUNCHPAD_GRID,
                    ACCENT_GREEN,
                    false,
                    Message::DesktopAutoArrangeIcons,
                ),
                menu_separator(),
                menu_item(
                    "Buka Pengaturan Sistem",
                    "Super+I",
                    icons::SETTINGS_GEAR,
                    ACCENT_AMBER,
                    false,
                    Message::OpenWindow(DesktopWindowId::Settings),
                ),
            ]
            .spacing(4)
            .padding(8)
            .width(Length::Fixed(245.0))
            .into()
        }

        ContextMenuKind::FileEntry(idx) => {
            let fname = file_name.unwrap_or("Berkas Terpilih");
            let mut col = column![
                container(
                    text(fname)
                        .size(10)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style {
                            color: Some(ACCENT_CYAN),
                        }),
                )
                .padding([4, 8]),
                menu_item(
                    "Buka Item",
                    "Enter",
                    icons::ROCKET,
                    ACCENT_CYAN,
                    false,
                    Message::FileContextOpen(*idx),
                ),
                menu_item(
                    "Quick Look Pratinjau",
                    "Space",
                    icons::EYE,
                    ACCENT_PURPLE,
                    false,
                    Message::FileContextQuickLook(*idx),
                ),
            ]
            .spacing(3);

            if is_image {
                if let Some(p) = file_path {
                    col = col.push(menu_item(
                        "Jadikan Wallpaper Desktop",
                        "Set BG",
                        icons::SPARKLES,
                        ACCENT_PURPLE,
                        false,
                        Message::SetCustomWallpaperFromFile(p),
                    ));
                }
            }

            col = col
                .push(menu_separator())
                .push(menu_item(
                    "Salin Berkas (Copy)",
                    "Ctrl+C",
                    icons::COPY,
                    ACCENT_CYAN,
                    false,
                    Message::FileCopySelected(*idx),
                ))
                .push(menu_item(
                    "Potong Berkas (Cut)",
                    "Ctrl+X",
                    icons::SCISSORS_CUT,
                    ACCENT_AMBER,
                    false,
                    Message::FileCutSelected(*idx),
                ));

            if has_file_clipboard {
                col = col.push(menu_item(
                    "Tempel Berkas di Sini (Paste)",
                    "Ctrl+V",
                    icons::PASTE,
                    ACCENT_GREEN,
                    false,
                    Message::FilePasteClipboard,
                ));
            }

            col = col
                .push(menu_separator())
                .push(menu_item(
                    "Ganti Nama (Rename)...",
                    "F2",
                    icons::TAG,
                    ACCENT_AMBER,
                    false,
                    Message::FileContextStartRename(*idx),
                ))
                .push(menu_item(
                    "Duplikat Berkas",
                    "Ctrl+D",
                    icons::COPY,
                    ACCENT_CYAN,
                    false,
                    Message::FileContextDuplicate(*idx),
                ))
                .push(menu_separator())
                .push(menu_item(
                    "Pindahkan ke Sampah (Trash)",
                    "Delete",
                    icons::TRASH,
                    ACCENT_RED,
                    true,
                    Message::FileContextDelete(*idx),
                ));

            col.padding(8).width(Length::Fixed(265.0)).into()
        }

        ContextMenuKind::DockItem(win_id) => column![
            container(
                text("AKSI JENDELA & UKURAN (RESIZE)")
                    .size(9)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_CYAN),
                    }),
            )
            .padding([4, 8]),
            menu_item(
                "Buka / Fokuskan Jendela",
                "Klik",
                icons::ROCKET,
                ACCENT_CYAN,
                false,
                Message::OpenWindow(*win_id),
            ),
            menu_item(
                "Perbesar Penuh (Maximize)",
                "Super+↑",
                icons::MAXIMIZE_ARROWS,
                ACCENT_GREEN,
                false,
                Message::SnapWindow(*win_id, WindowSnapMode::Maximized),
            ),
            menu_item(
                "Tempel Separuh Kiri (50%)",
                "Super+←",
                icons::LAYOUT_SPLIT,
                ACCENT_PURPLE,
                false,
                Message::SnapWindow(*win_id, WindowSnapMode::LeftHalf),
            ),
            menu_item(
                "Tempel Separuh Kanan (50%)",
                "Super+→",
                icons::LAYOUT_SPLIT,
                ACCENT_PURPLE,
                false,
                Message::SnapWindow(*win_id, WindowSnapMode::RightHalf),
            ),
            menu_item(
                "Resize Jendela Kompak (780x460)",
                "Resize",
                icons::RESIZE_CORNER,
                ACCENT_CYAN,
                false,
                Message::PresetResizeWindow(*win_id, 780.0, 460.0),
            ),
            menu_item(
                "Resize Jendela Lebar (1120x600)",
                "Resize",
                icons::RESIZE_CORNER,
                ACCENT_GREEN,
                false,
                Message::PresetResizeWindow(*win_id, 1120.0, 600.0),
            ),
            menu_item(
                "Sembunyikan ke Dock (Minimize)",
                "Super+↓",
                icons::MINUS,
                ACCENT_AMBER,
                false,
                Message::MinimizeWindow(*win_id),
            ),
            menu_separator(),
            menu_item(
                "Tutup Aplikasi (Quit)",
                "Alt+F4",
                icons::CLOSE,
                ACCENT_RED,
                true,
                Message::CloseWindow(*win_id),
            ),
        ]
        .spacing(3)
        .padding(8)
        .width(Length::Fixed(275.0))
        .into(),

        ContextMenuKind::PinnedDockApp(idx) => {
            let app_label = icon_title.unwrap_or("Aplikasi Dock Pacman");
            column![
                container(
                    text(app_label)
                        .size(10)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style {
                            color: Some(ACCENT_PURPLE),
                        }),
                )
                .padding([4, 8]),
                menu_item(
                    "Buka Aplikasi",
                    "Klik",
                    icons::ROCKET,
                    ACCENT_CYAN,
                    false,
                    Message::LaunchPinnedDockApp(*idx),
                ),
                menu_item(
                    "Tambahkan ke Desktop",
                    "Shortcut",
                    icons::PLUS,
                    ACCENT_AMBER,
                    false,
                    Message::AddPinnedDockAppToDesktop(*idx),
                ),
                menu_separator(),
                menu_item(
                    "Lepas Pin dari Dock (Unpin)",
                    "Hapus",
                    icons::TRASH,
                    ACCENT_RED,
                    true,
                    Message::UnpinDockApp(*idx),
                ),
            ]
            .spacing(4)
            .padding(8)
            .width(Length::Fixed(255.0))
            .into()
        }

        ContextMenuKind::TrashDock => column![
            container(
                text("TEMPAT SAMPAH MACOS (TRASH)")
                    .size(9)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_AMBER),
                    }),
            )
            .padding([4, 8]),
            menu_item(
                "Buka Tempat Sampah (Trash)",
                "Files",
                icons::FOLDER,
                ACCENT_CYAN,
                false,
                Message::OpenTrashInFiles,
            ),
            menu_separator(),
            menu_item(
                "Kosongkan Sampah (Empty Trash)",
                "Permanen",
                icons::TRASH,
                ACCENT_RED,
                true,
                Message::FileEmptyTrash,
            ),
        ]
        .spacing(4)
        .padding(8)
        .width(Length::Fixed(265.0))
        .into(),
    };

    let card = container(menu_col).style(context_menu_glass_style);

    let clamped_x = menu.pos.x.clamp(12.0, 1050.0);
    let clamped_y = menu.pos.y.clamp(38.0, 390.0);

    mouse_area(
        container(
            column![
                container("").height(Length::Fixed(clamped_y)),
                row![
                    container("").width(Length::Fixed(clamped_x)),
                    card,
                ],
            ],
        )
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .on_press(Message::CloseContextMenu)
    .on_right_press(Message::CloseContextMenu)
    .into()
}
