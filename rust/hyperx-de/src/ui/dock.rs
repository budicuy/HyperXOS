use iced::border::Radius;
use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    app_icon, dock_glass_style, dock_item_button_style, icons, svg_icon, ACCENT_AMBER,
    ACCENT_CYAN, ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED, FONT_BODY_SEMIBOLD, TEXT_MAIN,
};

use crate::sys::apps::DesktopAppEntry;
use crate::ui::window_manager::FloatingWindowManager;
use crate::ui::{DesktopWindowId, Message};

pub fn view_dock<'a>(
    wm: &'a FloatingWindowManager,
    launcher_open: bool,
    mission_control_open: bool,
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
                          accent: Color,
                          on_press: Message,
                          right_click_msg: Option<Message>|
     -> Element<'a, Message> {
        let dot_color = if is_focused {
            accent
        } else if is_minimized {
            ACCENT_AMBER
        } else if is_running {
            Color::WHITE
        } else {
            Color::TRANSPARENT
        };

        let indicator_dot = container("")
            .width(if is_focused { 16 } else { 6 })
            .height(4)
            .style(move |_| container::Style {
                background: Some(Background::Color(dot_color)),
                border: Border::default().rounded(Radius::from(2.0)),
                ..container::Style::default()
            });

        let btn = button(
            column![
                container(icon_el)
                    .width(Length::Fixed(dock_icon_size + 6.0))
                    .height(Length::Fixed(dock_icon_size + 6.0))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
                text(label)
                    .size(10)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
            ]
            .spacing(3)
            .align_x(Alignment::Center),
        )
        .width(Length::Fixed(70.0))
        .height(Length::Fixed(66.0))
        .padding([6, 6])
        .style(dock_item_button_style(is_focused, accent))
        .on_press(on_press);

        let wrapped_btn: Element<'a, Message> = if let Some(rc_msg) = right_click_msg {
            mouse_area(btn).on_right_press(rc_msg).into()
        } else {
            btn.into()
        };

        column![wrapped_btn, indicator_dot]
            .spacing(4)
            .align_x(Alignment::Center)
            .into()
    };

    // 1. Launchpad (App Grid)
    let item_launchpad = make_dock_item(
        "Launchpad",
        svg_icon(icons::LAUNCHPAD_GRID, dock_icon_size, ACCENT_PURPLE).into(),
        launcher_open,
        false,
        launcher_open,
        ACCENT_PURPLE,
        Message::ToggleLauncher,
        None,
    );

    // 2. Mission Control (Spaces / Super+Tab)
    let item_mission = make_dock_item(
        "Spaces",
        svg_icon(icons::LAYERS, dock_icon_size, ACCENT_CYAN).into(),
        mission_control_open || !wm.running_windows().is_empty(),
        false,
        mission_control_open,
        ACCENT_CYAN,
        Message::ToggleMissionControl,
        None,
    );

    // 3. HyperXOS Native Installer (Pinned Official App)
    let item_installer = make_dock_item(
        "Installer",
        app_icon("hyper-x-installer", dock_icon_size),
        false,
        false,
        false,
        ACCENT_CYAN,
        Message::LaunchExternalApp("launch-installer".to_string()),
        None,
    );

    // 4. Firefox Web Browser (Pinned Official App)
    let item_firefox = make_dock_item(
        "Firefox",
        app_icon("firefox", dock_icon_size),
        false,
        false,
        false,
        ACCENT_AMBER,
        Message::LaunchExternalApp("firefox".to_string()),
        None,
    );

    // 5. HyperX Files (Built-in Seamless File Manager)
    let files_st = wm.state(DesktopWindowId::Files);
    let item_files = make_dock_item(
        "Files",
        svg_icon(icons::FOLDER, dock_icon_size, ACCENT_CYAN).into(),
        files_st.open,
        files_st.minimized,
        focused_win == Some(DesktopWindowId::Files),
        ACCENT_CYAN,
        Message::DockItemClicked(DesktopWindowId::Files),
        Some(Message::OpenDockContextMenu(DesktopWindowId::Files)),
    );

    // 6. HyperX Terminal & Activity Monitor
    let term_st = wm.state(DesktopWindowId::Terminal);
    let item_terminal = make_dock_item(
        "Terminal",
        svg_icon(icons::TERMINAL, dock_icon_size, ACCENT_GREEN).into(),
        term_st.open,
        term_st.minimized,
        focused_win == Some(DesktopWindowId::Terminal),
        ACCENT_GREEN,
        Message::DockItemClicked(DesktopWindowId::Terminal),
        Some(Message::OpenDockContextMenu(DesktopWindowId::Terminal)),
    );

    // 7. HyperX Notes & Text Editor
    let notes_st = wm.state(DesktopWindowId::Notes);
    let item_notes = make_dock_item(
        "Notes",
        svg_icon(icons::EDIT_PEN, dock_icon_size, ACCENT_PURPLE).into(),
        notes_st.open,
        notes_st.minimized,
        focused_win == Some(DesktopWindowId::Notes),
        ACCENT_PURPLE,
        Message::DockItemClicked(DesktopWindowId::Notes),
        Some(Message::OpenDockContextMenu(DesktopWindowId::Notes)),
    );

    // 8. HyperXTools
    let item_tools = make_dock_item(
        "HyperXTools",
        app_icon("hyper-x-tools", dock_icon_size),
        false,
        false,
        false,
        ACCENT_PURPLE,
        Message::LaunchExternalApp("hyper-x-tools".to_string()),
        None,
    );

    // 9. System Settings
    let settings_st = wm.state(DesktopWindowId::Settings);
    let item_settings = make_dock_item(
        "Settings",
        svg_icon(icons::SETTINGS_GEAR, dock_icon_size, ACCENT_AMBER).into(),
        settings_st.open,
        settings_st.minimized,
        focused_win == Some(DesktopWindowId::Settings),
        ACCENT_AMBER,
        Message::DockItemClicked(DesktopWindowId::Settings),
        Some(Message::OpenDockContextMenu(DesktopWindowId::Settings)),
    );

    // 10. macOS Trash Bin at far right of Dock
    let item_trash = make_dock_item(
        if trash_count > 0 { "Trash •" } else { "Trash" },
        svg_icon(
            icons::TRASH,
            dock_icon_size,
            if trash_count > 0 {
                ACCENT_RED
            } else {
                ACCENT_CYAN
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

    let sep1 = container("")
        .width(1)
        .height(42)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.18))),
            ..container::Style::default()
        });

    let sep2 = container("")
        .width(1)
        .height(42)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.18))),
            ..container::Style::default()
        });

    let mut dock_row = row![
        item_launchpad,
        item_mission,
        sep1,
        item_installer,
        item_firefox,
        item_files,
        item_terminal,
        item_notes,
        item_tools,
        item_settings,
    ]
    .spacing(7)
    .align_y(Alignment::Center)
    .padding([8, 14]);

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
            ACCENT_PURPLE,
            Message::LaunchExternalApp(pinned.exec.clone()),
            Some(Message::OpenPinnedDockContextMenu(idx)),
        );
        dock_row = dock_row.push(pinned_item);
    }

    dock_row = dock_row.push(sep2).push(item_trash);

    container(dock_row)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(dock_glass_style)
        .into()
}
