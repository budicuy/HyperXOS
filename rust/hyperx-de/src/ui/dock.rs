use iced::border::Radius;
use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    app_icon, fluent_dock_item_button_style, fluent_dock_style, icons, svg_icon, ACCENT_AMBER,
    ACCENT_RED, FLUENT_PRIMARY, FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED, FONT_BODY_SEMIBOLD,
};

use crate::sys::apps::DesktopAppEntry;
use crate::ui::window_manager::FloatingWindowManager;
use crate::ui::{DesktopWindowId, Message};

pub fn view_dock<'a>(
    wm: &'a FloatingWindowManager,
    launcher_open: bool,
    _mission_control_open: bool,
    trash_count: usize,
    dock_icon_size: f32,
    pinned_dock_apps: &'a [DesktopAppEntry],
) -> Element<'a, Message> {
    let focused_win = wm.focused_window();

    let make_dock_item = |label: &'a str,
                          icon_el: Element<'a, Message>,
                          is_running: bool,
                          is_minimized: bool,
                          is_focused: bool,
                          _accent: Color,
                          on_press: Message,
                          right_click_msg: Option<Message>|
     -> Element<'a, Message> {
        let (dot_width, dot_color) = if is_focused {
            (14, FLUENT_PRIMARY)
        } else if is_minimized {
            (4, ACCENT_AMBER)
        } else if is_running {
            (4, FLUENT_PRIMARY)
        } else {
            (0, Color::TRANSPARENT)
        };

        let indicator_dot = container("")
            .width(dot_width)
            .height(3)
            .style(move |_| container::Style {
                background: Some(Background::Color(dot_color)),
                border: Border::default().rounded(Radius::from(2.0)),
                ..container::Style::default()
            });

        let btn = button(
            column![
                container(icon_el)
                    .width(Length::Fixed(dock_icon_size + 4.0))
                    .height(Length::Fixed(dock_icon_size + 4.0))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
                text(label)
                    .size(10)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
            ]
            .spacing(2)
            .align_x(Alignment::Center),
        )
        .width(Length::Fixed(66.0))
        .height(Length::Fixed(60.0))
        .padding([4, 4])
        .style(fluent_dock_item_button_style(is_focused))
        .on_press(on_press);

        let wrapped_btn: Element<'a, Message> = if let Some(rc_msg) = right_click_msg {
            mouse_area(btn).on_right_press(rc_msg).into()
        } else {
            btn.into()
        };

        column![wrapped_btn, indicator_dot]
            .spacing(2)
            .align_x(Alignment::Center)
            .into()
    };

    // 1. Windows 11 Fluent Start Button (4-square logo)
    let item_start = make_dock_item(
        "Mulai",
        svg_icon(icons::FLUENT_START, dock_icon_size - 4.0, FLUENT_PRIMARY).into(),
        launcher_open,
        false,
        launcher_open,
        FLUENT_PRIMARY,
        Message::ToggleLauncher,
        None,
    );

    // 2. Search Button
    let item_search = make_dock_item(
        "Cari",
        svg_icon(icons::SEARCH, dock_icon_size - 4.0, FLUENT_TEXT_DARK).into(),
        false,
        false,
        false,
        FLUENT_PRIMARY,
        Message::ToggleLauncher,
        None,
    );

    // 3. HyperXOS Native Installer (Pinned Official App)
    let item_installer = make_dock_item(
        "Installer",
        app_icon("hyper-x-installer", dock_icon_size),
        false,
        false,
        false,
        FLUENT_PRIMARY,
        Message::LaunchExternalApp("launch-installer".to_string()),
        None,
    );

    // 4. File Explorer (Dokumen)
    let files_st = wm.state(DesktopWindowId::Files);
    let item_files = make_dock_item(
        "Files",
        svg_icon(icons::FOLDER, dock_icon_size, Color::from_rgb8(245, 158, 11)).into(),
        files_st.open,
        files_st.minimized,
        focused_win == Some(DesktopWindowId::Files),
        FLUENT_PRIMARY,
        Message::DockItemClicked(DesktopWindowId::Files),
        Some(Message::OpenDockContextMenu(DesktopWindowId::Files)),
    );

    // 5. Firefox / Web Browser
    let item_firefox = make_dock_item(
        "Browser",
        app_icon("firefox", dock_icon_size),
        false,
        false,
        false,
        FLUENT_PRIMARY,
        Message::LaunchExternalApp("firefox".to_string()),
        None,
    );

    // 6. HyperX Terminal
    let term_st = wm.state(DesktopWindowId::Terminal);
    let item_terminal = make_dock_item(
        "Terminal",
        svg_icon(icons::TERMINAL, dock_icon_size, Color::from_rgb8(16, 185, 129)).into(),
        term_st.open,
        term_st.minimized,
        focused_win == Some(DesktopWindowId::Terminal),
        FLUENT_PRIMARY,
        Message::DockItemClicked(DesktopWindowId::Terminal),
        Some(Message::OpenDockContextMenu(DesktopWindowId::Terminal)),
    );

    // 7. HyperX Notes
    let notes_st = wm.state(DesktopWindowId::Notes);
    let item_notes = make_dock_item(
        "Catatan",
        svg_icon(icons::EDIT_PEN, dock_icon_size, Color::from_rgb8(139, 92, 246)).into(),
        notes_st.open,
        notes_st.minimized,
        focused_win == Some(DesktopWindowId::Notes),
        FLUENT_PRIMARY,
        Message::DockItemClicked(DesktopWindowId::Notes),
        Some(Message::OpenDockContextMenu(DesktopWindowId::Notes)),
    );

    // 8. HyperXTools
    let item_tools = make_dock_item(
        "Tools",
        app_icon("hyper-x-tools", dock_icon_size),
        false,
        false,
        false,
        FLUENT_PRIMARY,
        Message::LaunchExternalApp("hyper-x-tools".to_string()),
        None,
    );

    // 9. System Settings
    let settings_st = wm.state(DesktopWindowId::Settings);
    let item_settings = make_dock_item(
        "Pengaturan",
        svg_icon(icons::SETTINGS_GEAR, dock_icon_size, FLUENT_TEXT_MUTED).into(),
        settings_st.open,
        settings_st.minimized,
        focused_win == Some(DesktopWindowId::Settings),
        FLUENT_PRIMARY,
        Message::DockItemClicked(DesktopWindowId::Settings),
        Some(Message::OpenDockContextMenu(DesktopWindowId::Settings)),
    );

    // 10. Trash Bin (Keranjang Sampah)
    let item_trash = make_dock_item(
        if trash_count > 0 { "Sampah •" } else { "Sampah" },
        svg_icon(
            icons::TRASH,
            dock_icon_size,
            if trash_count > 0 {
                ACCENT_RED
            } else {
                FLUENT_TEXT_MUTED
            },
        )
        .into(),
        trash_count > 0,
        false,
        false,
        ACCENT_RED,
        Message::OpenTrashInFiles,
        Some(Message::OpenTrashDockContextMenu),
    );

    let make_sep = || {
        container("")
            .width(Length::Fixed(1.0))
            .height(Length::Fixed(24.0))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.15))),
                ..container::Style::default()
            })
    };

    let has_installer = std::path::Path::new("/usr/bin/launch-installer").exists()
        || std::path::Path::new("/usr/bin/hyper-x-installer").exists()
        || std::path::Path::new("/usr/local/bin/launch-installer").exists();

    let mut dock_row = row![
        item_start,
        item_search,
        make_sep(),
    ];

    if has_installer {
        dock_row = dock_row.push(item_installer);
    }

    dock_row = dock_row
        .push(item_files)
        .push(item_firefox)
        .push(item_terminal)
        .push(item_notes)
        .push(item_tools)
        .push(item_settings)
        .spacing(4)
        .align_y(Alignment::Center)
        .padding([4, 10]);

    for (idx, pinned) in pinned_dock_apps.iter().enumerate() {
        let short_label = if pinned.name.chars().count() > 9 {
            pinned.name.split_whitespace().next().unwrap_or(&pinned.name)
        } else {
            &pinned.name
        };
        let pinned_item = make_dock_item(
            short_label,
            app_icon(&pinned.icon_key, dock_icon_size),
            false,
            false,
            false,
            FLUENT_PRIMARY,
            Message::LaunchExternalApp(pinned.exec.clone()),
            Some(Message::OpenPinnedDockContextMenu(idx)),
        );
        dock_row = dock_row.push(pinned_item);
    }

    dock_row = dock_row.push(make_sep()).push(item_trash);

    container(dock_row)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(fluent_dock_style)
        .into()
}
