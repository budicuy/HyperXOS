use iced::border::Radius;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    app_icon, fluent_acrylic_modal_style, fluent_input_style, glass_scrollable, icons, svg_icon,
    ACCENT_AMBER, FLUENT_PRIMARY, FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED,
    FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD,
};

use crate::sys::apps::DesktopAppEntry;
use crate::ui::{DesktopWindowId, Message};

pub fn view_launcher<'a>(
    search_query: &'a str,
    apps: &'a [DesktopAppEntry],
    _pinned_dock_apps: &'a [DesktopAppEntry],
) -> Element<'a, Message> {
    // 1. Top Search Bar with Search & Mic Icons
    let search_input = text_input("Ketik di sini untuk mencari...", search_query)
        .on_input(Message::LauncherSearchChanged)
        .padding([8, 12])
        .size(13)
        .style(fluent_input_style);

    let mic_btn = button(svg_icon(icons::MIC, 15.0, FLUENT_TEXT_MUTED))
        .padding([4, 6])
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                }
                _ => None,
            };
            button::Style {
                background: bg,
                border: Border::default().rounded(Radius::from(6.0)),
                ..button::Style::default()
            }
        });

    let search_bar = container(
        row![
            svg_icon(icons::SEARCH, 16.0, FLUENT_TEXT_MUTED),
            search_input,
            mic_btn,
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([2, 10])
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.95))),
        border: Border::default()
            .rounded(Radius::from(10.0))
            .width(1.0)
            .color(Color::from_rgba(0.0, 0.0, 0.0, 0.08)),
        ..container::Style::default()
    });

    let q_lower = search_query.trim().to_lowercase();
    let is_searching = !q_lower.is_empty();

    let content_area: Element<'a, Message> = if is_searching {
        // Search Results View
        let filtered: Vec<&DesktopAppEntry> = apps
            .iter()
            .filter(|a| {
                a.name.to_lowercase().contains(&q_lower)
                    || a.comment.to_lowercase().contains(&q_lower)
                    || a.id.to_lowercase().contains(&q_lower)
            })
            .collect();

        let mut app_rows = column![].spacing(6);
        for chunk in filtered.chunks(4) {
            let mut r = row![].spacing(8);
            for app in chunk {
                let icon_widget: Element<'a, Message> = if app.icon_key == "terminal" {
                    svg_icon(icons::TERMINAL, 28.0, Color::from_rgb8(16, 185, 129)).into()
                } else {
                    app_icon(&app.icon_key, 28.0)
                };

                let card_btn = button(
                    column![
                        container(icon_widget)
                            .width(Length::Fixed(40.0))
                            .height(Length::Fixed(40.0))
                            .center_x(Length::Fixed(40.0))
                            .center_y(Length::Fixed(40.0)),
                        text(&app.name)
                            .size(11)
                            .font(FONT_BODY_SEMIBOLD)
                            .style(|_| text::Style {
                                color: Some(FLUENT_TEXT_DARK),
                            }),
                    ]
                    .spacing(4)
                    .align_x(Alignment::Center),
                )
                .width(Length::Fixed(120.0))
                .padding([8, 6])
                .style(|_theme, status| {
                    let bg = match status {
                        button::Status::Hovered | button::Status::Pressed => {
                            Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                        }
                        _ => None,
                    };
                    button::Style {
                        background: bg,
                        border: Border::default().rounded(Radius::from(8.0)),
                        ..button::Style::default()
                    }
                })
                .on_press(Message::LaunchExternalApp(app.exec.clone()));

                r = r.push(card_btn);
            }
            app_rows = app_rows.push(r);
        }

        glass_scrollable(app_rows).height(Length::Fixed(360.0)).into()
    } else {
        // Standard Windows 11 Start Menu: Pinned (Disematkan) + Recommended (Direkomendasikan)
        let pinned_header = row![
            text("Disematkan")
                .size(13)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
            container("").width(Length::Fill),
            button(
                row![
                    text("Semua aplikasi").size(11).font(FONT_BODY_SEMIBOLD),
                    svg_icon(icons::ARROW_LEFT, 11.0, FLUENT_TEXT_MUTED),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            )
            .padding([3, 8])
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                    }
                    _ => Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.03))),
                };
                button::Style {
                    background: bg,
                    text_color: FLUENT_TEXT_DARK,
                    border: Border::default().rounded(Radius::from(6.0)),
                    ..button::Style::default()
                }
            })
            .on_press(Message::RefreshDesktopApps),
        ]
        .align_y(Alignment::Center);

        // 6-Column Pinned App Grid
        let make_app_tile = |name: &'static str,
                             icon_el: Element<'a, Message>,
                             msg: Message|
         -> Element<'a, Message> {
            button(
                column![
                    container(icon_el)
                        .width(Length::Fixed(38.0))
                        .height(Length::Fixed(38.0))
                        .center_x(Length::Fixed(38.0))
                        .center_y(Length::Fixed(38.0))
                        .style(|_| container::Style {
                            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.90))),
                            border: Border::default()
                                .rounded(Radius::from(10.0))
                                .width(1.0)
                                .color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)),
                            ..container::Style::default()
                        }),
                    text(name)
                        .size(11)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(FLUENT_TEXT_DARK),
                        }),
                ]
                .spacing(4)
                .align_x(Alignment::Center),
            )
            .width(Length::Fixed(82.0))
            .padding([6, 4])
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                    }
                    _ => None,
                };
                button::Style {
                    background: bg,
                    border: Border::default().rounded(Radius::from(8.0)),
                    ..button::Style::default()
                }
            })
            .on_press(msg)
            .into()
        };

        let row1 = row![
            make_app_tile(
                "Browser",
                app_icon("firefox", 24.0),
                Message::LaunchExternalApp("firefox".to_string()),
            ),
            make_app_tile(
                "Files",
                svg_icon(icons::FOLDER, 22.0, Color::from_rgb8(245, 158, 11)).into(),
                Message::OpenWindow(DesktopWindowId::Files),
            ),
            make_app_tile(
                "Terminal",
                svg_icon(icons::TERMINAL, 22.0, Color::from_rgb8(16, 185, 129)).into(),
                Message::OpenWindow(DesktopWindowId::Terminal),
            ),
            make_app_tile(
                "Catatan",
                svg_icon(icons::EDIT_PEN, 22.0, Color::from_rgb8(139, 92, 246)).into(),
                Message::OpenWindow(DesktopWindowId::Notes),
            ),
            make_app_tile(
                "Tools",
                app_icon("hyper-x-tools", 24.0),
                Message::LaunchExternalApp("hyper-x-tools".to_string()),
            ),
            make_app_tile(
                "Pengaturan",
                svg_icon(icons::SETTINGS_GEAR, 22.0, FLUENT_TEXT_MUTED).into(),
                Message::OpenWindow(DesktopWindowId::Settings),
            ),
        ]
        .spacing(4);

        let row2 = row![
            make_app_tile(
                "Info",
                svg_icon(icons::INFO, 22.0, FLUENT_PRIMARY).into(),
                Message::OpenSettingsPage(crate::ui::settings::SettingsTab::About),
            ),
            make_app_tile(
                "Foto",
                svg_icon(icons::IMAGE_ICON, 22.0, Color::from_rgb8(14, 165, 233)).into(),
                Message::OpenWindow(DesktopWindowId::Files),
            ),
            make_app_tile(
                "Jaringan",
                svg_icon(icons::WIFI, 22.0, Color::from_rgb8(16, 185, 129)).into(),
                Message::OpenSettingsPage(crate::ui::settings::SettingsTab::Network),
            ),
            make_app_tile(
                "Display",
                svg_icon(icons::SUN_BRIGHTNESS, 22.0, ACCENT_AMBER).into(),
                Message::OpenSettingsPage(crate::ui::settings::SettingsTab::Display),
            ),
            make_app_tile(
                "Audio",
                svg_icon(icons::VOLUME_HIGH, 22.0, Color::from_rgb8(236, 72, 153)).into(),
                Message::OpenSettingsPage(crate::ui::settings::SettingsTab::Audio),
            ),
            make_app_tile(
                "Installer",
                app_icon("hyper-x-installer", 24.0),
                Message::LaunchExternalApp("launch-installer".to_string()),
            ),
        ]
        .spacing(4);

        let pinned_grid = column![row1, row2].spacing(4);

        // Section 2: Recommended (Direkomendasikan)
        let recommended_header = row![
            text("Direkomendasikan")
                .size(13)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
            container("").width(Length::Fill),
            button(text("Lainnya").size(11).font(FONT_BODY_SEMIBOLD))
                .padding([3, 8])
                .style(|_theme, status| {
                    let bg = match status {
                        button::Status::Hovered | button::Status::Pressed => {
                            Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                        }
                        _ => None,
                    };
                    button::Style {
                        background: bg,
                        text_color: FLUENT_TEXT_MUTED,
                        border: Border::default().rounded(Radius::from(6.0)),
                        ..button::Style::default()
                    }
                })
                .on_press(Message::OpenWindow(DesktopWindowId::Files)),
        ]
        .align_y(Alignment::Center);

        let make_rec_item = |name: &'static str,
                             subtitle: &'static str,
                             icon_svg: &'static str,
                             accent: Color|
         -> Element<'a, Message> {
            button(
                row![
                    container(svg_icon(icon_svg, 18.0, accent))
                        .width(Length::Fixed(32.0))
                        .height(Length::Fixed(32.0))
                        .center_x(Length::Fixed(32.0))
                        .center_y(Length::Fixed(32.0))
                        .style(move |_| container::Style {
                            background: Some(Background::Color(Color { a: 0.12, ..accent })),
                            border: Border::default().rounded(Radius::from(8.0)),
                            ..container::Style::default()
                        }),
                    column![
                        text(name)
                            .size(11)
                            .font(FONT_BODY_SEMIBOLD)
                            .style(|_| text::Style {
                                color: Some(FLUENT_TEXT_DARK),
                            }),
                        text(subtitle)
                            .size(9)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(FLUENT_TEXT_MUTED),
                            }),
                    ]
                    .spacing(1),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([6, 8])
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                    }
                    _ => None,
                };
                button::Style {
                    background: bg,
                    border: Border::default().rounded(Radius::from(8.0)),
                    ..button::Style::default()
                }
            })
            .on_press(Message::OpenWindow(DesktopWindowId::Files))
            .into()
        };

        let rec_row1 = row![
            make_rec_item(
                "Laporan_Tahunan.docx",
                "12 mnt lalu • BS",
                icons::DOCS,
                FLUENT_PRIMARY,
            ),
            make_rec_item(
                "Desain_Fluent_UI.rs",
                "2 jam lalu • BS",
                icons::FILE_CODE,
                Color::from_rgb8(139, 92, 246),
            ),
        ]
        .spacing(8);

        let rec_row2 = row![
            make_rec_item(
                "Presentasi_Projek.pdf",
                "Kemarin pada 15:40 • BS",
                icons::FILE_TEXT,
                Color::from_rgb8(249, 115, 22),
            ),
            make_rec_item(
                "HyperXOS_Spec.json",
                "2 hari lalu • BS",
                icons::FILE_CODE,
                Color::from_rgb8(16, 185, 129),
            ),
        ]
        .spacing(8);

        let rec_grid = column![rec_row1, rec_row2].spacing(4);

        column![
            pinned_header,
            pinned_grid,
            container("").height(Length::Fixed(6.0)),
            recommended_header,
            rec_grid,
        ]
        .spacing(8)
        .into()
    };

    // 4. User Profile & Power Controls Footer
    let user_avatar = container(
        text("BS")
            .size(11)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(Color::WHITE),
            }),
    )
    .width(Length::Fixed(30.0))
    .height(Length::Fixed(30.0))
    .center_x(Length::Fixed(30.0))
    .center_y(Length::Fixed(30.0))
    .style(|_| container::Style {
        background: Some(Background::Color(FLUENT_PRIMARY)),
        border: Border::default().rounded(Radius::from(9999.0)),
        ..container::Style::default()
    });

    let user_info = button(
        row![
            user_avatar,
            column![
                text("Budi Santoso")
                    .size(12)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
                text("Administrator")
                    .size(10)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
            ]
            .spacing(1),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([4, 6])
    .style(|_theme, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => {
                Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
            }
            _ => None,
        };
        button::Style {
            background: bg,
            border: Border::default().rounded(Radius::from(8.0)),
            ..button::Style::default()
        }
    })
    .on_press(Message::OpenWindow(DesktopWindowId::Settings));

    let power_btn = button(svg_icon(icons::POWER, 18.0, FLUENT_TEXT_DARK))
        .padding([6, 8])
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.91, 0.07, 0.14, 0.15)))
                }
                _ => None,
            };
            button::Style {
                background: bg,
                border: Border::default().rounded(Radius::from(8.0)),
                ..button::Style::default()
            }
        })
        .on_press(Message::ToggleLauncher);

    let footer = container(
        row![
            user_info,
            container("").width(Length::Fill),
            power_btn,
        ]
        .align_y(Alignment::Center),
    )
    .padding([8, 16])
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.03))),
        border: Border::default()
            .width(1.0)
            .color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)),
        ..container::Style::default()
    });

    let body = column![
        container(search_bar).padding([12, 16]),
        container(content_area).padding([4, 16]),
        footer,
    ]
    .spacing(0);

    container(body)
        .width(Length::Fixed(560.0))
        .style(fluent_acrylic_modal_style)
        .into()
}
