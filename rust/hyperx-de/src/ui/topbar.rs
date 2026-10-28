use iced::border::Radius;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    fluent_acrylic_modal_style, fluent_card_style, fluent_input_style, fluent_quick_toggle_style,
    fluent_ribbon_button_style, fluent_status_pill_style, fluent_toggle_switch, glass_scrollable,
    icons, primary_button_style, status_pill, svg_icon, FLUENT_PRIMARY,
    FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD,
};

use crate::sys::network::NetworkState;
use crate::ui::Message;

pub fn view_topbar<'a>(
    clock_str: &'a str,
    wifi_enabled: bool,
    volume: f32,
    muted: bool,
    unread_notifications: usize,
    calendar_flyout_open: bool,
    wifi_modal_open: bool,
    control_center_open: bool,
    notification_drawer_open: bool,
) -> Element<'a, Message> {
    // 1. Quick Search + Mic Pill
    let search_pill = button(
        row![
            svg_icon(icons::SEARCH, 15.0, FLUENT_TEXT_DARK),
            svg_icon(icons::MIC, 14.0, FLUENT_TEXT_MUTED),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
    )
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
            border: Border::default().rounded(Radius::from(9999.0)),
            ..button::Style::default()
        }
    })
    .on_press(Message::ToggleLauncher);

    // Subtle 1px vertical divider
    let make_divider = || {
        container("")
            .width(Length::Fixed(1.0))
            .height(Length::Fixed(14.0))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.15))),
                ..container::Style::default()
            })
    };

    // 2. Date & Time Pill (Click to open Windows 11 Calendar Flyout)
    let clock_content = text(clock_str)
        .size(12)
        .font(FONT_BODY_SEMIBOLD)
        .style(|_| text::Style {
            color: Some(FLUENT_TEXT_DARK),
        });

    let clock_btn = button(clock_content)
        .padding([3, 8])
        .style(move |_theme, status| {
            let bg = if calendar_flyout_open {
                Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.15)))
            } else {
                match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                    }
                    _ => None,
                }
            };
            button::Style {
                background: bg,
                border: Border::default().rounded(Radius::from(9999.0)),
                ..button::Style::default()
            }
        })
        .on_press(Message::ToggleCalendarFlyout);

    // 3. Separate Notification Bell Button with Unread Badge
    let bell_icon = svg_icon(
        icons::BELL,
        15.0,
        if notification_drawer_open {
            FLUENT_PRIMARY
        } else {
            FLUENT_TEXT_DARK
        },
    );

    let bell_content: Element<'a, Message> = if unread_notifications > 0 {
        let badge_dot = container("")
            .width(Length::Fixed(6.0))
            .height(Length::Fixed(6.0))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(232, 17, 35))),
                border: Border::default().rounded(Radius::from(999.0)),
                ..container::Style::default()
            });

        row![bell_icon, badge_dot]
            .spacing(3)
            .align_y(Alignment::Center)
            .into()
    } else {
        bell_icon.into()
    };

    let bell_btn = button(bell_content)
        .padding([3, 7])
        .style(move |_theme, status| {
            let bg = if notification_drawer_open {
                Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.15)))
            } else {
                match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                    }
                    _ => None,
                }
            };
            button::Style {
                background: bg,
                border: Border::default().rounded(Radius::from(9999.0)),
                ..button::Style::default()
            }
        })
        .on_press(Message::ToggleNotificationDrawer);

    // 4. System Status Icons: Wi-Fi, Volume, Quick Settings
    let wifi_icon = if wifi_enabled {
        icons::WIFI
    } else {
        icons::WIFI_OFF
    };
    let wifi_btn = button(svg_icon(
        wifi_icon,
        14.0,
        if wifi_modal_open || wifi_enabled {
            FLUENT_PRIMARY
        } else {
            Color::from_rgb8(232, 17, 35)
        },
    ))
    .padding([3, 6])
    .style(move |_theme, status| {
        let bg = if wifi_modal_open {
            Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.15)))
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                }
                _ => None,
            }
        };
        button::Style {
            background: bg,
            border: Border::default().rounded(Radius::from(9999.0)),
            ..button::Style::default()
        }
    })
    .on_press(Message::ToggleWifiModal);

    let vol_icon = if muted || volume <= 0.0 {
        icons::VOLUME_MUTE
    } else {
        icons::VOLUME_HIGH
    };
    let vol_btn = button(svg_icon(vol_icon, 14.0, FLUENT_TEXT_DARK))
        .padding([3, 6])
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                }
                _ => None,
            };
            button::Style {
                background: bg,
                border: Border::default().rounded(Radius::from(9999.0)),
                ..button::Style::default()
            }
        })
        .on_press(Message::ToggleControlCenter);

    let cc_btn = button(svg_icon(
        icons::SLIDERS,
        14.0,
        if control_center_open {
            FLUENT_PRIMARY
        } else {
            FLUENT_TEXT_DARK
        },
    ))
    .padding([3, 6])
    .style(move |_theme, status| {
        let bg = if control_center_open {
            Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.15)))
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                }
                _ => None,
            }
        };
        button::Style {
            background: bg,
            border: Border::default().rounded(Radius::from(9999.0)),
            ..button::Style::default()
        }
    })
    .on_press(Message::ToggleControlCenter);

    let quick_settings_group = row![wifi_btn, vol_btn, cc_btn]
        .spacing(2)
        .align_y(Alignment::Center);

    let island = container(
        row![
            search_pill,
            make_divider(),
            clock_btn,
            make_divider(),
            bell_btn,
            make_divider(),
            quick_settings_group,
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([4, 10])
    .style(fluent_status_pill_style);

    let bar_row = row![
        container("").width(Length::Fill),
        island,
    ]
    .align_y(Alignment::Center)
    .padding([8, 18]);

    container(bar_row)
        .width(Length::Fill)
        .style(|_| container::Style::default())
        .into()
}

pub fn view_wifi_modal<'a>(
    net: &'a NetworkState,
    selected_ssid: Option<&'a str>,
    wifi_password_input: &'a str,
    status_msg: &'a str,
) -> Element<'a, Message> {
    let header = row![
        svg_icon(
            if net.wifi_enabled {
                icons::WIFI
            } else {
                icons::WIFI_OFF
            },
            16.0,
            if net.wifi_enabled {
                FLUENT_PRIMARY
            } else {
                Color::from_rgb8(232, 17, 35)
            }
        ),
        column![
            text("Jaringan Wi-Fi")
                .size(13)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
            text(if net.wifi_enabled {
                if net.active_ssid.is_empty() {
                    "Radio Wi-Fi Aktif".to_string()
                } else {
                    format!("Terhubung: {}", net.active_ssid)
                }
            } else {
                "Radio Wi-Fi Nonaktif".to_string()
            })
            .size(10)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_MUTED),
            }),
        ]
        .spacing(1),
        container("").width(Length::Fill),
        button(svg_icon(icons::REFRESH, 13.0, FLUENT_TEXT_DARK))
            .padding([4, 6])
            .style(fluent_ribbon_button_style)
            .on_press(Message::RefreshNetwork),
        fluent_toggle_switch(net.wifi_enabled, Message::ToggleWifiRadio),
        button(svg_icon(icons::CLOSE, 12.0, FLUENT_TEXT_MUTED))
            .padding([4, 6])
            .style(fluent_ribbon_button_style)
            .on_press(Message::ToggleWifiModal),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let mut ssid_list = column![].spacing(6);
    for item in &net.networks {
        let is_sel = selected_ssid == Some(item.ssid.as_str());
        let badge = status_pill(
            format!("{}%", item.signal),
            if item.is_connected {
                Color::from_rgb8(16, 185, 129)
            } else {
                FLUENT_PRIMARY
            },
            item.is_connected,
        );

        let item_btn = button(
            row![
                svg_icon(
                    icons::WIFI,
                    15.0,
                    if item.is_connected {
                        Color::from_rgb8(16, 185, 129)
                    } else {
                        FLUENT_PRIMARY
                    }
                ),
                column![
                    text(&item.ssid)
                        .size(12)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(FLUENT_TEXT_DARK),
                        }),
                    text(if item.is_connected {
                        "Terhubung"
                    } else {
                        "Klik untuk menghubungkan"
                    })
                    .size(9)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
                ]
                .spacing(1),
                container("").width(Length::Fill),
                badge,
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([8, 10])
        .style(fluent_quick_toggle_style(is_sel))
        .on_press(Message::SelectWifiSsid(item.ssid.clone()));

        ssid_list = ssid_list.push(item_btn);
    }

    let connect_box: Element<'a, Message> = if let Some(ssid) = selected_ssid {
        container(
            column![
                text(format!("Kata sandi untuk '{}':", ssid))
                    .size(11)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
                row![
                    text_input("Kata sandi WPA2/WPA3...", wifi_password_input)
                        .on_input(Message::WifiPasswordChanged)
                        .secure(true)
                        .padding([6, 10])
                        .size(11)
                        .width(Length::Fill)
                        .style(fluent_input_style),
                    button(
                        row![
                            svg_icon(icons::CHECK, 12.0, Color::WHITE),
                            text("Sambung").size(11).font(FONT_BODY_SEMIBOLD),
                        ]
                        .spacing(5)
                        .align_y(Alignment::Center),
                    )
                    .padding([6, 12])
                    .style(primary_button_style(FLUENT_PRIMARY))
                    .on_press(Message::ConnectSelectedWifi),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            ]
            .spacing(6)
            .padding(10),
        )
        .width(Length::Fill)
        .style(fluent_card_style)
        .into()
    } else {
        container("").into()
    };

    let footer = row![
        text(status_msg)
            .size(10)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_MUTED),
            }),
        container("").width(Length::Fill),
        button(
            row![
                svg_icon(icons::SETTINGS_GEAR, 12.0, FLUENT_TEXT_DARK),
                text("Pengaturan Lengkap").size(10).font(FONT_BODY_SEMIBOLD).style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .padding([4, 8])
        .style(fluent_ribbon_button_style)
        .on_press(Message::OpenSettingsPage(
            crate::ui::settings::SettingsTab::Network,
        )),
    ]
    .align_y(Alignment::Center);

    let scroll = glass_scrollable(ssid_list).height(Length::Fixed(185.0));

    let body = column![
        header,
        container(scroll).padding(8).style(fluent_card_style),
        connect_box,
        footer,
    ]
    .spacing(10)
    .padding(14);

    container(body)
        .width(Length::Fixed(350.0))
        .style(fluent_acrylic_modal_style)
        .into()
}
