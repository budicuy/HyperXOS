use iced::border::Radius;
use iced::widget::{button, column, container, mouse_area, row};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    app_icon, fluent_dock_item_button_style, fluent_dock_style, icons, svg_icon, ACCENT_AMBER,
    ACCENT_RED, FLUENT_PRIMARY, FLUENT_TEXT_MUTED,
};

use crate::sys::apps::DesktopAppEntry;
use crate::ui::window_manager::FloatingWindowManager;
use crate::ui::{DesktopWindowId, DockAppKey, Message};

pub fn view_dock<'a>(
    wm: &'a FloatingWindowManager,
    launcher_open: bool,
    _mission_control_open: bool,
    trash_count: usize,
    dock_icon_size: f32,
    pinned_dock_apps: &'a [DesktopAppEntry],
    dock_app_order: &'a [DockAppKey],
    dragging_dock_idx: Option<usize>,
) -> Element<'a, Message> {
    let focused_win = wm.focused_window();

    // Constant-height dock item without text labels (zero layout shift)
    let make_dock_item = |icon_el: Element<'a, Message>,
                          is_running: bool,
                          is_minimized: bool,
                          is_focused: bool,
                          on_press: Message,
                          right_click_msg: Option<Message>|
     -> Element<'a, Message> {
        let (dot_width, dot_color) = if is_focused {
            (14.0, FLUENT_PRIMARY)
        } else if is_minimized {
            (4.0, ACCENT_AMBER)
        } else if is_running {
            (4.0, FLUENT_PRIMARY)
        } else {
            (4.0, Color::TRANSPARENT)
        };

        // Indicator slot has strict fixed dimensions (44x4px) so item height never changes
        let indicator_dot = container(
            container("")
                .width(Length::Fixed(dot_width))
                .height(Length::Fixed(3.0))
                .style(move |_| container::Style {
                    background: Some(Background::Color(dot_color)),
                    border: Border::default().rounded(Radius::from(2.0)),
                    ..container::Style::default()
                }),
        )
        .width(Length::Fixed(44.0))
        .height(Length::Fixed(4.0))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        let btn = button(
            container(icon_el)
                .width(Length::Fixed(dock_icon_size))
                .height(Length::Fixed(dock_icon_size))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .width(Length::Fixed(44.0))
        .height(Length::Fixed(44.0))
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

    // 1. Windows 11 Fluent Start Button
    let item_start = make_dock_item(
        svg_icon(icons::FLUENT_START, dock_icon_size - 4.0, FLUENT_PRIMARY).into(),
        launcher_open,
        false,
        launcher_open,
        Message::ToggleLauncher,
        None,
    );

    // 2. Search Button
    let item_search = make_dock_item(
        svg_icon(icons::SEARCH, dock_icon_size - 4.0, Color::from_rgb8(30, 41, 59)).into(),
        false,
        false,
        false,
        Message::ToggleLauncher,
        None,
    );

    // 3. Separator
    let make_sep = || {
        container("")
            .width(Length::Fixed(1.0))
            .height(Length::Fixed(24.0))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.15))),
                ..container::Style::default()
            })
    };

    let mut dock_row = row![item_start, item_search, make_sep()]
        .spacing(4)
        .align_y(Alignment::Center)
        .padding([4, 8]);

    // 4. Reorderable App Icons (Draggable Dock Items)
    for (order_idx, key) in dock_app_order.iter().enumerate() {
        let is_being_dragged = dragging_dock_idx == Some(order_idx);

        let item_el: Element<'a, Message> = match key {
            DockAppKey::Installer => make_dock_item(
                app_icon("hyper-x-installer", dock_icon_size),
                false,
                false,
                false,
                Message::LaunchExternalApp("launch-installer".to_string()),
                None,
            ),
            DockAppKey::Files => {
                let files_st = wm.state(DesktopWindowId::Files);
                make_dock_item(
                    svg_icon(icons::FOLDER, dock_icon_size, Color::from_rgb8(245, 158, 11)).into(),
                    files_st.open,
                    files_st.minimized,
                    focused_win == Some(DesktopWindowId::Files),
                    Message::DockItemClicked(DesktopWindowId::Files),
                    Some(Message::OpenDockContextMenu(DesktopWindowId::Files)),
                )
            }
            DockAppKey::Browser => make_dock_item(
                app_icon("firefox", dock_icon_size),
                false,
                false,
                false,
                Message::LaunchExternalApp("firefox".to_string()),
                None,
            ),
            DockAppKey::Terminal => {
                let term_st = wm.state(DesktopWindowId::Terminal);
                make_dock_item(
                    svg_icon(icons::TERMINAL, dock_icon_size, Color::from_rgb8(16, 185, 129))
                        .into(),
                    term_st.open,
                    term_st.minimized,
                    focused_win == Some(DesktopWindowId::Terminal),
                    Message::DockItemClicked(DesktopWindowId::Terminal),
                    Some(Message::OpenDockContextMenu(DesktopWindowId::Terminal)),
                )
            }
            DockAppKey::Notes => {
                let notes_st = wm.state(DesktopWindowId::Notes);
                make_dock_item(
                    svg_icon(icons::EDIT_PEN, dock_icon_size, Color::from_rgb8(139, 92, 246))
                        .into(),
                    notes_st.open,
                    notes_st.minimized,
                    focused_win == Some(DesktopWindowId::Notes),
                    Message::DockItemClicked(DesktopWindowId::Notes),
                    Some(Message::OpenDockContextMenu(DesktopWindowId::Notes)),
                )
            }
            DockAppKey::Tools => make_dock_item(
                app_icon("hyper-x-tools", dock_icon_size),
                false,
                false,
                false,
                Message::LaunchExternalApp("hyper-x-tools".to_string()),
                None,
            ),
            DockAppKey::Settings => {
                let settings_st = wm.state(DesktopWindowId::Settings);
                make_dock_item(
                    svg_icon(icons::SETTINGS_GEAR, dock_icon_size, FLUENT_TEXT_MUTED).into(),
                    settings_st.open,
                    settings_st.minimized,
                    focused_win == Some(DesktopWindowId::Settings),
                    Message::DockItemClicked(DesktopWindowId::Settings),
                    Some(Message::OpenDockContextMenu(DesktopWindowId::Settings)),
                )
            }
            DockAppKey::Pinned(pinned_idx) => {
                if let Some(pinned) = pinned_dock_apps.get(*pinned_idx) {
                    make_dock_item(
                        app_icon(&pinned.icon_key, dock_icon_size),
                        false,
                        false,
                        false,
                        Message::LaunchExternalApp(pinned.exec.clone()),
                        Some(Message::OpenPinnedDockContextMenu(*pinned_idx)),
                    )
                } else {
                    container("").width(Length::Fixed(0.0)).height(Length::Fixed(0.0)).into()
                }
            }
        };

        let styled_item = if is_being_dragged {
            container(item_el).style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.18))),
                border: Border::default()
                    .rounded(Radius::from(8.0))
                    .width(1.0)
                    .color(Color::from_rgba(0.0, 0.47, 0.83, 0.50)),
                ..container::Style::default()
            })
        } else {
            container(item_el)
        };

        let draggable = mouse_area(styled_item)
            .on_press(Message::StartDockDrag(order_idx))
            .on_enter(Message::DockDragOver(order_idx))
            .on_release(Message::EndDockDrag);

        dock_row = dock_row.push(draggable);
    }

    // 5. Trash Bin (Keranjang Sampah)
    let item_trash = make_dock_item(
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
        Message::OpenTrashInFiles,
        Some(Message::OpenTrashDockContextMenu),
    );

    dock_row = dock_row.push(make_sep()).push(item_trash);

    container(dock_row)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(fluent_dock_style)
        .into()
}
