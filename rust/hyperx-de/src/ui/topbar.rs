use iced::border::Radius;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    active_toggle_button_style, fluent_status_pill_style, glass_action_button,
    glass_card_accent_style, glass_card_style, glass_input_style, glass_scrollable, icons,
    popover_glass_style, primary_button_style, status_pill, svg_icon, ACCENT_AMBER, ACCENT_CYAN,
    ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED, ACTIVE_DARK_FG, FLUENT_PRIMARY, FLUENT_TEXT_DARK,
    FLUENT_TEXT_MUTED, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD, TEXT_MAIN,
    TEXT_MUTED, TEXT_SUB,
};

use crate::sys::network::NetworkState;
use crate::ui::Message;

pub fn view_topbar<'a>(
    clock_str: &'a str,
    wifi_enabled: bool,
    volume: f32,
    muted: bool,
    unread_notifications: usize,
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

    // 2. Date & Time Pill with Notification Badge
    let clock_content = row![
        text(clock_str)
            .size(12)
            .font(FONT_BODY_SEMIBOLD)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_DARK),
            }),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let mut clock_group = row![clock_content].align_y(Alignment::Center);
    if unread_notifications > 0 {
        clock_group = clock_group.push(
            container(
                text(format!("{}", unread_notifications))
                    .size(9)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(Color::WHITE),
                    }),
            )
            .padding([1, 5])
            .style(|_| container::Style {
                background: Some(Background::Color(FLUENT_PRIMARY)),
                border: Border::default().rounded(Radius::from(8.0)),
                ..container::Style::default()
            }),
        );
    }

    let clock_btn = button(clock_group)
        .padding([3, 8])
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

    // 3. System Status Icons: Wi-Fi, Volume, Control / Battery
    let wifi_icon = if wifi_enabled {
        icons::WIFI
    } else {
        icons::WIFI_OFF
    };
    let wifi_btn = button(svg_icon(
        wifi_icon,
        14.0,
        if wifi_modal_open {
            FLUENT_PRIMARY
        } else if wifi_enabled {
            FLUENT_PRIMARY
        } else {
            ACCENT_RED
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

    let vol_icon = if muted || volume <= 0.5 {
        icons::VOLUME_MUTE
    } else {
        icons::VOLUME_HIGH
    };
    let vol_btn = button(svg_icon(
        vol_icon,
        14.0,
        if muted { ACCENT_RED } else { FLUENT_TEXT_DARK },
    ))
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
    .on_press(Message::ToggleMute);

    let cc_btn = button(svg_icon(
        icons::SLIDERS,
        14.0,
        if control_center_open { FLUENT_PRIMARY } else { FLUENT_TEXT_DARK },
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

    let status_island = container(
        row![
            search_pill,
            make_divider(),
            clock_btn,
            make_divider(),
            row![wifi_btn, vol_btn, cc_btn].spacing(1).align_y(Alignment::Center),
        ]
        .spacing(7)
        .align_y(Alignment::Center),
    )
    .style(fluent_status_pill_style)
    .padding([3, 9]);

    let bar_row = row![
        container("").width(Length::Fill),
        status_island,
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
            18.0,
            if net.wifi_enabled {
                ACCENT_CYAN
            } else {
                ACCENT_RED
            }
        ),
        column![
            text("Jaringan Wi-Fi")
                .size(14)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text(if net.wifi_enabled {
                format!("Terhubung: {} ({})", net.active_ssid, net.ip_address)
            } else {
                "Radio Wi-Fi Nonaktif".to_string()
            })
            .size(10)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(TEXT_MUTED),
            }),
        ]
        .spacing(1),
        container("").width(Length::Fill),
        button(svg_icon(icons::REFRESH, 13.0, ACCENT_CYAN))
            .padding([5, 8])
            .style(glass_action_button)
            .on_press(Message::RefreshNetwork),
        button(
            text(if net.wifi_enabled { "ON" } else { "OFF" })
                .size(10)
                .font(FONT_DISPLAY_BOLD),
        )
        .padding([5, 10])
        .style(active_toggle_button_style(net.wifi_enabled, ACCENT_GREEN))
        .on_press(Message::ToggleWifiRadio),
        button(svg_icon(icons::CLOSE, 13.0, ACCENT_RED))
            .padding([5, 8])
            .style(glass_action_button)
            .on_press(Message::ToggleWifiModal),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let mut ssid_list = column![].spacing(6);
    for item in &net.networks {
        let is_sel = selected_ssid == Some(item.ssid.as_str());
        let badge = status_pill(
            format!("{}% • {}", item.signal, item.security),
            if item.is_connected {
                ACCENT_GREEN
            } else {
                ACCENT_CYAN
            },
            item.is_connected,
        );

        let item_btn = button(
            row![
                svg_icon(
                    icons::WIFI,
                    15.0,
                    if is_sel {
                        ACTIVE_DARK_FG
                    } else if item.is_connected {
                        ACCENT_GREEN
                    } else {
                        ACCENT_CYAN
                    }
                ),
                column![
                    text(&item.ssid).size(12).font(FONT_DISPLAY_BOLD),
                    text(if item.is_connected {
                        "Terhubung"
                    } else {
                        "Klik untuk menyambungkan"
                    })
                    .size(9)
                    .font(FONT_BODY_MEDIUM)
                    .style(move |_| text::Style {
                        color: Some(if is_sel { ACTIVE_DARK_FG } else { TEXT_MUTED }),
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
        .style(active_toggle_button_style(is_sel, ACCENT_CYAN))
        .on_press(Message::SelectWifiSsid(item.ssid.clone()));

        ssid_list = ssid_list.push(item_btn);
    }

    let connect_box: Element<'a, Message> = if let Some(ssid) = selected_ssid {
        container(
            column![
                text(format!("Sandi untuk '{}':", ssid))
                    .size(11)
                    .font(FONT_DISPLAY_BOLD),
                row![
                    text_input("Kata sandi WPA2/WPA3...", wifi_password_input)
                        .on_input(Message::WifiPasswordChanged)
                        .secure(true)
                        .padding([6, 10])
                        .size(11)
                        .width(Length::Fill)
                        .style(glass_input_style),
                    button(
                        row![
                            svg_icon(icons::CHECK, 12.0, ACCENT_GREEN),
                            text("Sambung").size(11).font(FONT_DISPLAY_BOLD),
                        ]
                        .spacing(5)
                        .align_y(Alignment::Center),
                    )
                    .padding([6, 12])
                    .style(primary_button_style(ACCENT_GREEN))
                    .on_press(Message::ConnectSelectedWifi),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            ]
            .spacing(6)
            .padding(10),
        )
        .width(Length::Fill)
        .style(glass_card_accent_style(ACCENT_AMBER))
        .into()
    } else {
        container("").into()
    };

    let footer = row![
        text(status_msg)
            .size(10)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(TEXT_SUB),
            }),
        container("").width(Length::Fill),
        button(
            row![
                svg_icon(icons::SETTINGS_GEAR, 12.0, ACCENT_PURPLE),
                text("Pengaturan Lengkap").size(10).font(FONT_BODY_SEMIBOLD),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .padding([5, 10])
        .style(glass_action_button)
        .on_press(Message::OpenSettingsPage(
            crate::ui::settings::SettingsTab::Network,
        )),
    ]
    .align_y(Alignment::Center);

    let scroll = glass_scrollable(ssid_list).height(Length::Fixed(185.0));

    let body = column![
        header,
        container(scroll).padding(8).style(glass_card_style),
        connect_box,
        footer,
    ]
    .spacing(10)
    .padding(14);

    container(body)
        .width(Length::Fixed(350.0))
        .style(popover_glass_style(ACCENT_CYAN))
        .into()
}
