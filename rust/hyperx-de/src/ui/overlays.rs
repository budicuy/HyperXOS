use std::fs;
use std::path::PathBuf;
use std::process::Command;

use iced::border::Radius;
use iced::widget::{button, column, container, mouse_area, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    brand_sidebar_header, fluent_acrylic_modal_style, glass_action_button, glass_card_accent_style,
    glass_card_style, glass_input_style, glass_scrollable, glass_window_style, icons, logo_image,
    metric_card, popover_glass_style, primary_button_style, sidebar_container_style,
    sidebar_nav_item, sidebar_section_header, status_pill, svg_icon, terminal_container_style,
    window_header_bar, ACCENT_AMBER, ACCENT_CYAN, ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED,
    FLUENT_BORDER, FLUENT_PRIMARY, FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD,
    FONT_DISPLAY_BOLD, FONT_DISPLAY_EXTRABOLD, TEXT_MAIN, TEXT_MUTED, TEXT_SUB,
};

use crate::ui::window_manager::{window_meta, FloatingWindowManager, WindowSnapMode};
use crate::ui::{DesktopWindowId, Message};

#[derive(Debug, Clone)]
pub struct TerminalAppState {
    pub command_input: String,
    pub output_lines: Vec<String>,
    pub active_tab: usize, // 0 = Console Shell, 1 = System Monitor & Shortcuts
}

impl Default for TerminalAppState {
    fn default() -> Self {
        Self {
            command_input: String::new(),
            output_lines: vec![
                "HyperXOS Interactive Shell v2.0 (Smithay Wayland + WGPU)".to_string(),
                "Ketik perintah Linux (contoh: uname -a, ls -la, df -h, free -h) atau gunakan tombol cepat di atas.".to_string(),
                "------------------------------------------------------------------------".to_string(),
            ],
            active_tab: 0,
        }
    }
}

impl TerminalAppState {
    pub fn run_command(&mut self, cmd_str: &str) {
        let trimmed = cmd_str.trim();
        if trimmed.is_empty() {
            return;
        }
        if trimmed == "clear" {
            self.output_lines.clear();
            return;
        }
        self.output_lines.push(format!("hyperxos@desktop:~$ {}", trimmed));
        match Command::new("sh").arg("-c").arg(trimmed).output() {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                for line in stdout.lines().take(35) {
                    self.output_lines.push(line.to_string());
                }
                for line in stderr.lines().take(15) {
                    self.output_lines.push(format!("[stderr] {}", line));
                }
                if stdout.trim().is_empty() && stderr.trim().is_empty() {
                    self.output_lines.push("(Perintah selesai tanpa output)".to_string());
                }
            }
            Err(e) => {
                self.output_lines
                    .push(format!("Gagal menjalankan perintah: {}", e));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotesAppState {
    pub current_file: PathBuf,
    pub title_input: String,
    pub content_lines: Vec<String>,
    pub new_line_input: String,
    pub status_msg: String,
}

impl Default for NotesAppState {
    fn default() -> Self {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"));
        let notes_path = home.join("Catatan-HyperXOS.txt");
        let default_lines = vec![
            "# Catatan & Panduan Cepat HyperXOS Desktop v2.0".to_string(),
            "• Alt + Tab        : Berpindah antar aplikasi aktif (App Switcher HUD)".to_string(),
            "• Super + Tab / F3 : Membuka Mission Control (Semua Jendela)".to_string(),
            "• Super + E        : Membuka HyperX Files (File Manager)".to_string(),
            "• Super + I        : Membuka HyperX Settings".to_string(),
            "• Super + T        : Membuka HyperX Terminal & Activity Monitor".to_string(),
            "• Super + Panah    : Aero-Snap Jendela (Kiri 50%, Kanan 50%, Maximize, Minimize)".to_string(),
            "• Spacebar         : Quick Look pratinjau berkas di HyperX Files".to_string(),
            "• Klik Kanan       : Menu Konteks Kaca macOS di Desktop, Berkas, & Dock".to_string(),
        ];
        Self {
            current_file: notes_path,
            title_input: "Catatan-HyperXOS.txt".to_string(),
            content_lines: default_lines,
            new_line_input: String::new(),
            status_msg: "Siap mengedit catatan atau berkas teks.".to_string(),
        }
    }
}

impl NotesAppState {
    pub fn load_file(&mut self, path: PathBuf) {
        self.title_input = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "berkas.txt".to_string());
        self.current_file = path.clone();
        match fs::read_to_string(&path) {
            Ok(raw) => {
                self.content_lines = raw.lines().take(120).map(|s| s.to_string()).collect();
                if self.content_lines.is_empty() {
                    self.content_lines.push("(Berkas kosong)".to_string());
                }
                self.status_msg = format!("Membuka '{}'", path.display());
            }
            Err(e) => {
                self.status_msg = format!("Info: Berkas biner atau tidak dapat dibaca teks ({})", e);
            }
        }
    }

    pub fn save_file(&mut self) -> Result<String, String> {
        let joined = self.content_lines.join("\n") + "\n";
        fs::write(&self.current_file, joined).map_err(|e| e.to_string())?;
        let msg = format!("Tersimpan ke {}", self.current_file.display());
        self.status_msg = msg.clone();
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct QuickLookData {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size_label: String,
    pub preview_text: String,
}

// 1. View Built-in Terminal & Activity Monitor Window
pub fn view_terminal_window<'a>(
    state: &'a TerminalAppState,
    cpu_brand: &'a str,
    cpu_usage: f32,
    ram_used_gb: f32,
    ram_total_gb: f32,
    wayland_socket: &'a str,
    w: Length,
    h: Length,
) -> Element<'a, Message> {
    let brand = brand_sidebar_header("HYPERX TERMINAL", "Konsol & Monitor OS");
    let sep = container("")
        .width(Length::Fill)
        .height(1)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.20))),
            ..container::Style::default()
        });

    let nav = column![
        sidebar_section_header("MODE UTILITAS"),
        sidebar_nav_item(
            "Konsol Shell Cepat",
            icons::TERMINAL,
            state.active_tab == 0,
            Message::TerminalSelectTab(0),
        ),
        sidebar_nav_item(
            "Monitor & Shortcut",
            icons::CPU,
            state.active_tab == 1,
            Message::TerminalSelectTab(1),
        ),
        sidebar_section_header("PERINTAH INSTAN"),
        sidebar_nav_item(
            "Info Kernel (uname)",
            icons::KERNEL,
            false,
            Message::TerminalRunQuick("uname -srmo && uptime -p".to_string()),
        ),
        sidebar_nav_item(
            "Kapasitas Disk (df -h)",
            icons::HARD_DRIVE,
            false,
            Message::TerminalRunQuick("df -h /".to_string()),
        ),
        sidebar_nav_item(
            "Memori RAM (free -h)",
            icons::RAM,
            false,
            Message::TerminalRunQuick("free -h".to_string()),
        ),
        sidebar_nav_item(
            "Buka Konsole Penuh",
            icons::ROCKET,
            false,
            Message::LaunchExternalApp("terminal".to_string()),
        ),
    ]
    .spacing(6);

    let sidebar = container(column![brand, sep, nav].spacing(12).padding(14))
        .width(Length::Fixed(220.0))
        .height(Length::Fill)
        .style(sidebar_container_style);

    let header = mouse_area(window_header_bar(
        "HyperX Terminal & Activity Monitor",
        "Eksekusi perintah shell cepat, pantau CPU/RAM, dan daftar shortcut sistem",
        icons::TERMINAL,
        ACCENT_GREEN,
        Message::MinimizeWindow(DesktopWindowId::Terminal),
        Message::ToggleMaximizeWindow(DesktopWindowId::Terminal),
        Message::CloseWindow(DesktopWindowId::Terminal),
    ))
    .on_press(Message::StartWindowDrag(DesktopWindowId::Terminal))
    .on_release(Message::EndWindowDrag);

    let body: Element<'a, Message> = if state.active_tab == 0 {
        let mut out_col = column![].spacing(4);
        for line in &state.output_lines {
            let is_cmd = line.starts_with("hyperxos@");
            out_col = out_col.push(
                text(line)
                    .size(11)
                    .font(if is_cmd {
                        FONT_DISPLAY_BOLD
                    } else {
                        FONT_BODY_MEDIUM
                    })
                    .style(move |_| text::Style {
                        color: Some(if is_cmd { ACCENT_CYAN } else { ACCENT_GREEN }),
                    }),
            );
        }

        let term_box = container(glass_scrollable(out_col.padding(12)).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(terminal_container_style);

        let cmd_row = row![
            text("hyperxos:~$")
                .size(12)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(ACCENT_GREEN),
                }),
            text_input("Ketik perintah shell lalu klik Jalankan...", &state.command_input)
                .on_input(Message::TerminalInputChanged)
                .on_submit(Message::TerminalExecuteInput)
                .padding([7, 12])
                .size(12)
                .width(Length::Fill)
                .style(glass_input_style),
            button(
                row![
                    svg_icon(icons::ROCKET, 13.0, ACCENT_GREEN),
                    text("Jalankan").size(11).font(FONT_DISPLAY_BOLD),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .padding([7, 14])
            .style(primary_button_style(ACCENT_GREEN))
            .on_press(Message::TerminalExecuteInput),
            button(text("Bersihkan").size(11).font(FONT_BODY_SEMIBOLD))
                .padding([7, 12])
                .style(glass_action_button)
                .on_press(Message::TerminalRunQuick("clear".to_string())),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        column![term_box, cmd_row]
            .spacing(10)
            .padding(14)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        let ram_ratio = (ram_used_gb / ram_total_gb.max(1.0)).clamp(0.0, 1.0);
        let metrics = row![
            metric_card(
                "Beban CPU Real-Time",
                icons::CPU,
                format!("{:.0}%", cpu_usage),
                (cpu_usage / 100.0).clamp(0.0, 1.0),
                cpu_brand,
                Some("Governor: Performance"),
                ACCENT_CYAN,
            ),
            metric_card(
                "Penggunaan RAM",
                icons::RAM,
                format!("{:.1} / {:.1} GB", ram_used_gb, ram_total_gb),
                ram_ratio,
                format!("Wayland Socket: {}", wayland_socket),
                Some("WGPU Direct"),
                ACCENT_GREEN,
            ),
        ]
        .spacing(12);

        let shortcuts_card = container(
            column![
                row![
                    svg_icon(icons::KEYBOARD, 18.0, ACCENT_PURPLE),
                    text("Daftar Shortcut Keyboard (Windows + macOS Hybrid)")
                        .size(13)
                        .font(FONT_DISPLAY_BOLD),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                text("• Alt + Tab / Ctrl + Tab  : Membuka App Switcher HUD & berpindah aplikasi")
                    .size(11),
                text("• Super + Tab / F3        : Membuka Mission Control (Overview semua jendela)")
                    .size(11),
                text("• Alt + F4 / Ctrl + W     : Menutup jendela yang sedang aktif")
                    .size(11),
                text("• Super + E / Super + I   : Membuka HyperX Files / HyperX Settings")
                    .size(11),
                text("• Super + Panah Kiri/Kanan: Aero-Snap Jendela 50% Kiri / 50% Kanan")
                    .size(11),
                text("• Super + Panah Atas/Bawah: Maximize Jendela / Minimize ke Dock")
                    .size(11),
                text("• Super + D / Super + L   : Show Desktop / Kunci Layar (macOS Lock Screen)")
                    .size(11),
                text("• Spacebar / F2 / Delete  : Quick Look Pratinjau / Rename / Hapus di Files")
                    .size(11),
            ]
            .spacing(6)
            .padding(14),
        )
        .width(Length::Fill)
        .style(glass_card_accent_style(ACCENT_PURPLE));

        glass_scrollable(column![metrics, shortcuts_card].spacing(12).padding(14))
            .height(Length::Fill)
            .into()
    };

    let right = column![header, body]
        .width(Length::Fill)
        .height(Length::Fill);

    container(row![sidebar, right].width(Length::Fill).height(Length::Fill))
        .width(w)
        .height(h)
        .style(glass_window_style)
        .into()
}

// 2. View Built-in HyperX Notes & Text Editor Window
pub fn view_notes_window<'a>(
    state: &'a NotesAppState,
    w: Length,
    h: Length,
) -> Element<'a, Message> {
    let header = mouse_area(window_header_bar(
        format!("HyperX Notes — {}", state.title_input),
        &state.status_msg,
        icons::EDIT_PEN,
        ACCENT_PURPLE,
        Message::MinimizeWindow(DesktopWindowId::Notes),
        Message::ToggleMaximizeWindow(DesktopWindowId::Notes),
        Message::CloseWindow(DesktopWindowId::Notes),
    ))
    .on_press(Message::StartWindowDrag(DesktopWindowId::Notes))
    .on_release(Message::EndWindowDrag);

    let mut lines_col = column![].spacing(5);
    for (idx, line) in state.content_lines.iter().enumerate() {
        lines_col = lines_col.push(
            row![
                text(format!("{:02}", idx + 1))
                    .size(10)
                    .font(FONT_BODY_SEMIBOLD)
                    .width(Length::Fixed(26.0))
                    .style(|_| text::Style {
                        color: Some(TEXT_MUTED),
                    }),
                text(line).size(12).font(FONT_BODY_MEDIUM),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        );
    }

    let editor_box = container(glass_scrollable(lines_col.padding(14)).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(glass_card_style);

    let input_bar = row![
        text_input("Tambahkan baris catatan baru...", &state.new_line_input)
            .on_input(Message::NotesLineInputChanged)
            .on_submit(Message::NotesAppendLine)
            .padding([7, 12])
            .size(12)
            .width(Length::Fill)
            .style(glass_input_style),
        button(
            row![
                svg_icon(icons::EDIT_PEN, 13.0, ACCENT_CYAN),
                text("Tambah Baris").size(11).font(FONT_DISPLAY_BOLD),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .padding([7, 12])
        .style(primary_button_style(ACCENT_CYAN))
        .on_press(Message::NotesAppendLine),
        button(
            row![
                svg_icon(icons::CHECK, 13.0, ACCENT_GREEN),
                text("Simpan Berkas").size(11).font(FONT_DISPLAY_BOLD),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .padding([7, 14])
        .style(primary_button_style(ACCENT_GREEN))
        .on_press(Message::NotesSaveFile),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    container(
        column![
            header,
            column![editor_box, input_bar]
                .spacing(10)
                .padding(14)
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(w)
    .height(h)
    .style(glass_window_style)
    .into()
}

// 3. App Switcher HUD Overlay (Alt + Tab / Ctrl + Tab)
pub fn view_app_switcher<'a>(
    wm: &'a FloatingWindowManager,
    selected_idx: usize,
) -> Element<'a, Message> {
    let running = wm.running_windows();
    let target_apps: Vec<DesktopWindowId> = if running.is_empty() {
        vec![DesktopWindowId::Files, DesktopWindowId::Settings]
    } else {
        running
    };

    let total = target_apps.len();
    let mut cards_row = row![].spacing(12).align_y(Alignment::Center);

    for (idx, &win_id) in target_apps.iter().enumerate() {
        let (title, sub, ic, accent) = window_meta(win_id);
        let st = wm.state(win_id);
        let is_selected = idx == (selected_idx % total);

        let item_btn = button(
            column![
                container(svg_icon(ic, 32.0, accent))
                    .width(Length::Fixed(56.0))
                    .height(Length::Fixed(56.0))
                    .center_x(Length::Fixed(56.0))
                    .center_y(Length::Fixed(56.0))
                    .style(move |_| container::Style {
                        background: Some(Background::Color(Color { a: 0.12, ..accent })),
                        border: Border::default()
                            .rounded(Radius::from(10.0))
                            .width(1.0)
                            .color(Color { a: 0.35, ..accent }),
                        ..container::Style::default()
                    }),
                text(title)
                    .size(12)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
                text(sub)
                    .size(10)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
                if st.minimized {
                    status_pill("Terkecilkan", ACCENT_AMBER, false)
                } else {
                    status_pill("Aktif", FLUENT_PRIMARY, true)
                },
            ]
            .spacing(5)
            .align_x(Alignment::Center)
            .padding(10),
        )
        .width(Length::Fixed(150.0))
        .height(Length::Fixed(148.0))
        .style(move |_theme, status| {
            let (bg, border_c, border_w) = if is_selected {
                (
                    Color::from_rgba(0.0, 0.47, 0.83, 0.08),
                    FLUENT_PRIMARY,
                    2.0,
                )
            } else {
                match status {
                    button::Status::Hovered => (
                        Color::from_rgba(0.0, 0.0, 0.0, 0.04),
                        Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                        1.0,
                    ),
                    _ => (Color::WHITE, FLUENT_BORDER, 1.0),
                }
            };
            button::Style {
                background: Some(Background::Color(bg)),
                border: Border::default()
                    .rounded(Radius::from(8.0))
                    .width(border_w)
                    .color(border_c),
                ..button::Style::default()
            }
        })
        .on_press(Message::SelectAppSwitcherWindow(win_id));

        cards_row = cards_row.push(item_btn);
    }

    let card_width = ((target_apps.len() * 162 + 40).max(440)) as f32;

    let hud_card = container(
        column![
            row![
                svg_icon(icons::LAYERS, 16.0, FLUENT_PRIMARY),
                text("Pengalih Aplikasi (Alt + Tab)")
                    .size(13)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
                container("").width(Length::Fill),
                text("Tab: Pilih  •  Enter / Klik: Fokus")
                    .size(11)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            cards_row,
        ]
        .spacing(14)
        .padding(18),
    )
    .width(Length::Fixed(card_width))
    .style(fluent_acrylic_modal_style);

    mouse_area(
        container(hud_card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.20))),
                ..container::Style::default()
            }),
    )
    .on_press(Message::CloseAppSwitcher)
    .into()
}

// 4. Mission Control / Stage Manager Overview (Super + Tab / F3)
pub fn view_mission_control<'a>(wm: &'a FloatingWindowManager) -> Element<'a, Message> {
    let all_apps = [
        DesktopWindowId::Files,
        DesktopWindowId::Settings,
        DesktopWindowId::Terminal,
        DesktopWindowId::Notes,
    ];

    let mut grid = row![].spacing(16);
    for win_id in all_apps {
        let (short_title, _sub, ic, accent) = match win_id {
            DesktopWindowId::Files => ("HyperX Files", "Manajer Berkas", icons::FOLDER, ACCENT_CYAN),
            DesktopWindowId::Settings => ("HyperX Settings", "Pengaturan OS", icons::SETTINGS_GEAR, ACCENT_AMBER),
            DesktopWindowId::Terminal => ("HyperX Terminal", "Konsol & Monitor", icons::TERMINAL, ACCENT_GREEN),
            DesktopWindowId::Notes => ("HyperX Notes", "Editor Catatan", icons::EDIT_PEN, ACCENT_PURPLE),
        };
        let st = wm.state(win_id);

        let card = container(
            column![
                row![
                    svg_icon(ic, 22.0, accent),
                    column![
                        text(short_title).size(13).font(FONT_DISPLAY_BOLD),
                        text(_sub)
                            .size(10)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(TEXT_SUB),
                            }),
                    ]
                    .spacing(2),
                    container("").width(Length::Fill),
                    status_pill(
                        if st.open { "Terbuka" } else { "Siap" },
                        if st.open { ACCENT_GREEN } else { TEXT_MUTED },
                        st.open,
                    ),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                row![
                    button(
                        text("Buka Mengambang")
                            .size(10)
                            .font(FONT_DISPLAY_BOLD),
                    )
                    .width(Length::Fill)
                    .padding([7, 10])
                    .style(primary_button_style(accent))
                    .on_press(Message::SnapWindow(win_id, WindowSnapMode::Floating)),
                    button(
                        text("Penuh (Max)")
                            .size(10)
                            .font(FONT_DISPLAY_BOLD),
                    )
                    .width(Length::Fill)
                    .padding([7, 10])
                    .style(primary_button_style(ACCENT_GREEN))
                    .on_press(Message::SnapWindow(win_id, WindowSnapMode::Maximized)),
                ]
                .spacing(8),
                row![
                    button(text("Snap Kiri 50%").size(10).font(FONT_BODY_SEMIBOLD))
                        .width(Length::Fill)
                        .padding([6, 8])
                        .style(glass_action_button)
                        .on_press(Message::SnapWindow(win_id, WindowSnapMode::LeftHalf)),
                    button(text("Snap Kanan 50%").size(10).font(FONT_BODY_SEMIBOLD))
                        .width(Length::Fill)
                        .padding([6, 8])
                        .style(glass_action_button)
                        .on_press(Message::SnapWindow(win_id, WindowSnapMode::RightHalf)),
                ]
                .spacing(8),
            ]
            .spacing(12)
            .padding(16),
        )
        .width(Length::Fixed(275.0))
        .style(glass_card_accent_style(accent));

        grid = grid.push(card);
    }

    let panel = container(
        column![
            row![
                svg_icon(icons::LAYERS, 20.0, ACCENT_PURPLE),
                column![
                    text("macOS Mission Control & Window Tiling")
                        .size(16)
                        .font(FONT_DISPLAY_BOLD),
                    text("Pilih aplikasi untuk dibuka secara mengambang, layar penuh, atau berdampingan (Split 50% Kiri/Kanan)")
                        .size(11)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(TEXT_SUB),
                        }),
                ]
                .spacing(2),
                container("").width(Length::Fill),
                button(text("Tutup (Esc)").size(11).font(FONT_DISPLAY_BOLD))
                    .padding([7, 14])
                    .style(primary_button_style(ACCENT_RED))
                    .on_press(Message::ToggleMissionControl),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            grid,
        ]
        .spacing(18)
        .padding(24),
    )
    .style(popover_glass_style(ACCENT_PURPLE));

    mouse_area(
        container(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.02, 0.03, 0.07, 0.78))),
                ..container::Style::default()
            }),
    )
    .on_press(Message::ToggleMissionControl)
    .into()
}

// 5. macOS Quick Look & Get Info Modal (Spacebar in Files)
pub fn view_quick_look_modal<'a>(ql: &'a QuickLookData) -> Element<'a, Message> {
    let icon = if ql.is_dir {
        icons::FOLDER
    } else {
        icons::FILE_TEXT
    };

    let modal = container(
        column![
            row![
                svg_icon(icon, 22.0, ACCENT_CYAN),
                column![
                    text(format!("Quick Look — {}", ql.name))
                        .size(14)
                        .font(FONT_DISPLAY_BOLD),
                    text(format!("{}  •  {}", ql.size_label, ql.path))
                        .size(10)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(TEXT_MUTED),
                        }),
                ]
                .spacing(2),
                container("").width(Length::Fill),
                button(
                    row![
                        svg_icon(icons::EDIT_PEN, 12.0, ACCENT_PURPLE),
                        text("Buka di Notes").size(11).font(FONT_DISPLAY_BOLD),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                )
                .padding([6, 12])
                .style(primary_button_style(ACCENT_PURPLE))
                .on_press(Message::QuickLookOpenInNotes),
                button(text("Tutup (Space / Esc)").size(11).font(FONT_DISPLAY_BOLD))
                    .padding([6, 12])
                    .style(glass_action_button)
                    .on_press(Message::CloseQuickLook),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            container(
                glass_scrollable(
                    container(
                        text(&ql.preview_text)
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(TEXT_MAIN),
                            }),
                    )
                    .padding(12),
                )
                .height(Length::Fixed(320.0)),
            )
            .width(Length::Fill)
            .style(glass_card_style),
        ]
        .spacing(12)
        .padding(18),
    )
    .width(Length::Fixed(680.0))
    .style(popover_glass_style(ACCENT_CYAN));

    mouse_area(
        container(modal)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.02, 0.03, 0.06, 0.65))),
                ..container::Style::default()
            }),
    )
    .on_press(Message::CloseQuickLook)
    .into()
}

// 6. macOS Sonoma/Sequoia Lock Screen Overlay (Super + L)
pub fn view_lock_screen<'a>(clock_str: &'a str, date_str: &'a str) -> Element<'a, Message> {
    let time_only = clock_str
        .split_whitespace()
        .last()
        .unwrap_or("09:41");

    let card = column![
        text(date_str)
            .size(16)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(ACCENT_CYAN),
            }),
        text(time_only)
            .size(72)
            .font(FONT_DISPLAY_EXTRABOLD)
            .style(|_| text::Style {
                color: Some(Color::WHITE),
            }),
        container("").height(Length::Fixed(80.0)),
        container(logo_image(64.0))
            .padding(12)
            .style(popover_glass_style(ACCENT_CYAN)),
        text("HyperXOS Desktop User")
            .size(16)
            .font(FONT_DISPLAY_BOLD),
        text("Sesi Terkunci — Klik tombol di bawah atau tekan Enter / Esc untuk membuka")
            .size(11)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(TEXT_SUB),
            }),
        button(
            row![
                svg_icon(icons::UNLOCK, 15.0, ACCENT_GREEN),
                text("Buka Kunci Layar (Unlock)").size(12).font(FONT_DISPLAY_BOLD),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding([10, 24])
        .style(primary_button_style(ACCENT_GREEN))
        .on_press(Message::UnlockScreen),
    ]
    .spacing(12)
    .align_x(Alignment::Center);

    container(card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.02, 0.03, 0.07, 0.94))),
            ..container::Style::default()
        })
        .into()
}

// 7. Clipboard Manager HUD Overlay (Super + V)
pub fn view_clipboard_hud<'a>(
    history: &'a [String],
    file_clip_label: Option<String>,
) -> Element<'a, Message> {
    let mut items_col = column![].spacing(8);

    if let Some(fc) = file_clip_label {
        items_col = items_col.push(
            container(
                row![
                    svg_icon(icons::PASTE, 16.0, ACCENT_GREEN),
                    column![
                        text("Buffer Berkas Aktif (Siap Ditempel dengan Ctrl+V)")
                            .size(11)
                            .font(FONT_DISPLAY_BOLD),
                        text(fc)
                            .size(10)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(ACCENT_GREEN),
                            }),
                    ]
                    .spacing(2),
                ]
                .spacing(10)
                .align_y(Alignment::Center)
                .padding(10),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_GREEN)),
        );
    }

    for (idx, entry) in history.iter().enumerate() {
        let row_card = container(
            row![
                svg_icon(icons::CLIPBOARD, 15.0, ACCENT_CYAN),
                text(entry)
                    .size(11)
                    .font(FONT_BODY_SEMIBOLD)
                    .width(Length::Fill),
                button(
                    row![
                        svg_icon(icons::EDIT_PEN, 11.0, ACCENT_PURPLE),
                        text("Tempel ke Notes").size(10).font(FONT_DISPLAY_BOLD),
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center),
                )
                .padding([5, 10])
                .style(primary_button_style(ACCENT_PURPLE))
                .on_press(Message::ClipboardPasteToNotes(idx)),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .padding(10),
        )
        .width(Length::Fill)
        .style(glass_card_style);

        items_col = items_col.push(row_card);
    }

    let modal = container(
        column![
            row![
                svg_icon(icons::CLIPBOARD, 20.0, ACCENT_CYAN),
                column![
                    text("HyperX Clipboard Manager (Super + V)")
                        .size(14)
                        .font(FONT_DISPLAY_BOLD),
                    text("Riwayat teks, path berkas (Ctrl+C / Ctrl+X), dan tangkapan layar")
                        .size(10)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(TEXT_SUB),
                        }),
                ]
                .spacing(2),
                container("").width(Length::Fill),
                button(text("Bersihkan").size(11).font(FONT_BODY_SEMIBOLD))
                    .padding([6, 12])
                    .style(glass_action_button)
                    .on_press(Message::ClipboardClearHistory),
                button(text("Tutup (Esc)").size(11).font(FONT_DISPLAY_BOLD))
                    .padding([6, 12])
                    .style(primary_button_style(ACCENT_RED))
                    .on_press(Message::ToggleClipboardHud),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            glass_scrollable(items_col.padding(4)).height(Length::Fixed(270.0)),
        ]
        .spacing(14)
        .padding(18),
    )
    .width(Length::Fixed(640.0))
    .style(popover_glass_style(ACCENT_CYAN));

    mouse_area(
        container(modal)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.02, 0.03, 0.06, 0.65))),
                ..container::Style::default()
            }),
    )
    .on_press(Message::ToggleClipboardHud)
    .into()
}

// 8. macOS Screenshot & Capture HUD Bar (PrintScreen / Super + Shift + S)
pub fn view_screenshot_hud<'a>(last_saved: Option<&'a str>) -> Element<'a, Message> {
    let status_lbl = last_saved.unwrap_or("Disimpan otomatis ke ~/Pictures/Screenshots");

    let hud = container(
        column![
            row![
                svg_icon(icons::SNAPSHOT, 20.0, ACCENT_PURPLE),
                column![
                    text("macOS Screenshot & Screen Capture (Super + Shift + S)")
                        .size(14)
                        .font(FONT_DISPLAY_BOLD),
                    text(status_lbl)
                        .size(10)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(ACCENT_GREEN),
                        }),
                ]
                .spacing(2),
                container("").width(Length::Fill),
                button(text("Tutup (Esc)").size(11).font(FONT_DISPLAY_BOLD))
                    .padding([6, 12])
                    .style(glass_action_button)
                    .on_press(Message::ToggleScreenshotHud),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                button(
                    row![
                        svg_icon(icons::SNAPSHOT, 15.0, ACCENT_CYAN),
                        text("Tangkap Layar Penuh Sekarang").size(11).font(FONT_DISPLAY_BOLD),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([10, 14])
                .style(primary_button_style(ACCENT_CYAN))
                .on_press(Message::CaptureScreenshotNow),
                button(
                    row![
                        svg_icon(icons::SPARKLES, 15.0, ACCENT_PURPLE),
                        text("Tangkap & Jadikan Wallpaper").size(11).font(FONT_DISPLAY_BOLD),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([10, 14])
                .style(primary_button_style(ACCENT_PURPLE))
                .on_press(Message::CaptureScreenshotAndSetWallpaper),
                button(
                    row![
                        svg_icon(icons::FOLDER, 15.0, ACCENT_AMBER),
                        text("Buka Folder Screenshots").size(11).font(FONT_DISPLAY_BOLD),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([10, 14])
                .style(primary_button_style(ACCENT_AMBER))
                .on_press(Message::OpenScreenshotsFolder),
            ]
            .spacing(10),
        ]
        .spacing(14)
        .padding(20),
    )
    .width(Length::Fixed(720.0))
    .style(popover_glass_style(ACCENT_PURPLE));

    mouse_area(
        container(hud)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.02, 0.03, 0.06, 0.62))),
                ..container::Style::default()
            }),
    )
    .on_press(Message::ToggleScreenshotHud)
    .into()
}
