use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element, Length};

use hyperx_ui_kit::{
    app_icon, glass_action_button, glass_card_style, glass_input_style, glass_scrollable,
    glass_window_style, icons, primary_button_style, svg_icon, ACCENT_AMBER, ACCENT_CYAN,
    ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD,
    FONT_DISPLAY_BOLD, TEXT_MAIN, TEXT_MUTED,
};

use crate::sys::apps::DesktopAppEntry;
use crate::ui::{DesktopWindowId, Message};

pub fn view_launcher<'a>(
    search_query: &'a str,
    apps: &'a [DesktopAppEntry],
    pinned_dock_apps: &'a [DesktopAppEntry],
) -> Element<'a, Message> {
    let top_header = row![
        svg_icon(icons::LAUNCHPAD_GRID, 20.0, ACCENT_PURPLE),
        column![
            text("HyperX Launchpad & Spotlight")
                .size(16)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text(format!(
                "{} Aplikasi Terdeteksi Otomatis (/usr/share/applications & Pacman)",
                apps.len()
            ))
            .size(11)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(TEXT_MUTED),
            }),
        ]
        .spacing(1),
        container("").width(Length::Fill),
        button(
            row![
                svg_icon(icons::REFRESH, 12.0, ACCENT_CYAN),
                text("Segarkan").size(11).font(FONT_BODY_SEMIBOLD),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding([6, 12])
        .style(glass_action_button)
        .on_press(Message::RefreshDesktopApps),
        button(svg_icon(icons::CLOSE, 13.0, ACCENT_RED))
            .padding([6, 9])
            .style(glass_action_button)
            .on_press(Message::ToggleLauncher),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let search_bar = text_input("Ketik nama aplikasi atau pengaturan...", search_query)
        .on_input(Message::LauncherSearchChanged)
        .padding([10, 14])
        .size(13)
        .style(glass_input_style);

    // Built-in Seamless DE Shortcut Cards
    let quick_internal = row![
        button(
            row![
                svg_icon(icons::FOLDER, 18.0, ACCENT_CYAN),
                column![
                    text("HyperX Files").size(12).font(FONT_DISPLAY_BOLD),
                    text("Manajer Berkas").size(10).font(FONT_BODY_MEDIUM),
                ]
                .spacing(1),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([10, 14])
        .style(primary_button_style(ACCENT_CYAN))
        .on_press(Message::OpenWindow(DesktopWindowId::Files)),
        button(
            row![
                svg_icon(icons::SETTINGS_GEAR, 18.0, ACCENT_AMBER),
                column![
                    text("HyperX Settings").size(12).font(FONT_DISPLAY_BOLD),
                    text("Pengaturan Sistem").size(10).font(FONT_BODY_MEDIUM),
                ]
                .spacing(1),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([10, 14])
        .style(primary_button_style(ACCENT_AMBER))
        .on_press(Message::OpenWindow(DesktopWindowId::Settings)),
        button(
            row![
                svg_icon(icons::WIFI, 18.0, ACCENT_GREEN),
                column![
                    text("Jaringan Wi-Fi").size(12).font(FONT_DISPLAY_BOLD),
                    text("NetworkManager").size(10).font(FONT_BODY_MEDIUM),
                ]
                .spacing(1),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([10, 14])
        .style(primary_button_style(ACCENT_GREEN))
        .on_press(Message::OpenSettingsPage(crate::ui::settings::SettingsTab::Network)),
    ]
    .spacing(10);

    let q_lower = search_query.trim().to_lowercase();
    let filtered: Vec<&DesktopAppEntry> = apps
        .iter()
        .filter(|a| {
            q_lower.is_empty()
                || a.name.to_lowercase().contains(&q_lower)
                || a.comment.to_lowercase().contains(&q_lower)
                || a.id.to_lowercase().contains(&q_lower)
        })
        .collect();

    let mut rows = column![].spacing(10);
    for chunk in filtered.chunks(3) {
        let mut r = row![].spacing(10);
        for app in chunk {
            let icon_widget: Element<'a, Message> = if app.icon_key == "terminal" {
                svg_icon(icons::TERMINAL, 32.0, ACCENT_GREEN).into()
            } else {
                app_icon(&app.icon_key, 32.0)
            };

            let is_pinned_in_dock = pinned_dock_apps.iter().any(|p| p.id == app.id)
                || matches!(
                    app.id.as_str(),
                    "hyper-x-installer" | "firefox" | "hyper-x-tools" | "terminal"
                );

            let pin_dock_btn = button(
                row![
                    svg_icon(
                        if is_pinned_in_dock {
                            icons::CHECK
                        } else {
                            icons::PLUS
                        },
                        10.0,
                        if is_pinned_in_dock {
                            ACCENT_GREEN
                        } else {
                            ACCENT_PURPLE
                        },
                    ),
                    text(if is_pinned_in_dock { "Dock" } else { "Pin Dock" })
                        .size(10)
                        .font(FONT_BODY_SEMIBOLD),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([5, 6])
            .style(glass_action_button)
            .on_press(Message::TogglePinAppToDock((*app).clone()));

            let add_desktop_btn = button(
                row![
                    svg_icon(icons::PLUS, 10.0, ACCENT_AMBER),
                    text("Desktop").size(10).font(FONT_BODY_SEMIBOLD),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([5, 6])
            .style(glass_action_button)
            .on_press(Message::AddAppToDesktop((*app).clone()));

            let open_btn = button(
                row![
                    svg_icon(icons::ROCKET, 11.0, ACCENT_CYAN),
                    text("Buka").size(10).font(FONT_BODY_SEMIBOLD),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([5, 6])
            .style(primary_button_style(ACCENT_CYAN))
            .on_press(Message::LaunchExternalApp(app.exec.clone()));

            let card = container(
                column![
                    row![
                        icon_widget,
                        column![
                            text(&app.name)
                                .size(13)
                                .font(FONT_DISPLAY_BOLD)
                                .style(|_| text::Style {
                                    color: Some(TEXT_MAIN),
                                }),
                            text(&app.category)
                                .size(10)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(|_| text::Style {
                                    color: Some(ACCENT_CYAN),
                                }),
                        ]
                        .spacing(2),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                    text(&app.comment)
                        .size(11)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(TEXT_MUTED),
                        }),
                    row![open_btn, add_desktop_btn, pin_dock_btn]
                        .spacing(5)
                        .align_y(Alignment::Center),
                ]
                .spacing(9)
                .padding(11),
            )
            .width(Length::Fill)
            .style(glass_card_style);

            r = r.push(card);
        }
        rows = rows.push(r);
    }

    let scroll = glass_scrollable(rows).height(Length::Fixed(310.0));

    let body = column![top_header, search_bar, quick_internal, scroll]
        .spacing(14)
        .padding(20);

    container(body)
        .width(Length::Fixed(760.0))
        .style(glass_window_style)
        .into()
}
