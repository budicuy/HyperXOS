use iced::border::Radius;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    active_toggle_button_style, glass_action_button, glass_card_accent_style, glass_card_style,
    glass_input_style, glass_scrollable, icons, popover_glass_style, primary_button_style,
    status_pill, svg_icon, topbar_glass_style, ACCENT_AMBER, ACCENT_CYAN, ACCENT_GREEN,
    ACCENT_PURPLE, ACCENT_RED, ACTIVE_DARK_FG, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD,
    FONT_DISPLAY_BOLD, TEXT_MAIN, TEXT_MUTED, TEXT_SUB,
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
    // 1. Center: Combined Clock + Notification Bell Pill
    let fg_col = if notification_drawer_open {
        ACTIVE_DARK_FG
    } else {
        TEXT_MAIN
    };
    let bell_col = if notification_drawer_open {
        ACTIVE_DARK_FG
    } else if unread_notifications > 0 {
        ACCENT_AMBER
    } else {
        ACCENT_PURPLE
    };

    let mut clock_row = row![
        text(clock_str)
            .size(12)
            .font(FONT_DISPLAY_BOLD)
            .style(move |_| text::Style {
                color: Some(fg_col),
            }),
        container("")
            .width(1)
            .height(12)
            .style(move |_| container::Style {
                background: Some(Background::Color(if notification_drawer_open {
                    Color::from_rgba(0.0, 0.0, 0.0, 0.25)
                } else {
                    Color::from_rgba(1.0, 1.0, 1.0, 0.22)
                })),
                ..container::Style::default()
            }),
        svg_icon(icons::BELL, 13.5, bell_col),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    if unread_notifications > 0 {
        clock_row = clock_row.push(
            container(
                text(format!("{}", unread_notifications))
                    .size(10)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACTIVE_DARK_FG),
                    }),
            )
            .padding([1, 6])
            .style(|_| container::Style {
                background: Some(Background::Color(ACCENT_AMBER)),
                border: Border::default().rounded(Radius::from(8.0)),
                ..container::Style::default()
            }),
        );
    }

    let center_clock_btn = button(clock_row)
        .padding([5, 14])
        .style(active_toggle_button_style(
            notification_drawer_open,
            ACCENT_PURPLE,
        ))
        .on_press(Message::ToggleNotificationDrawer);

    // 2. Right: Minimalist Icon-Only Wi-Fi Button + Volume Icon + Control Center Button
    let wifi_icon = if wifi_enabled {
        icons::WIFI
    } else {
        icons::WIFI_OFF
    };
    let wifi_btn = button(svg_icon(
        wifi_icon,
        15.0,
        if wifi_modal_open {
            ACTIVE_DARK_FG
        } else if wifi_enabled {
            ACCENT_CYAN
        } else {
            ACCENT_RED
        },
    ))
    .padding([5, 9])
    .style(active_toggle_button_style(wifi_modal_open, ACCENT_CYAN))
    .on_press(Message::ToggleWifiModal);

    let vol_icon = if muted || volume <= 0.5 {
        icons::VOLUME_MUTE
    } else {
        icons::VOLUME_HIGH
    };
    let vol_btn = button(svg_icon(
        vol_icon,
        15.0,
        if muted { ACCENT_RED } else { ACCENT_AMBER },
    ))
    .padding([5, 9])
    .style(glass_action_button)
    .on_press(Message::ToggleMute);

    let cc_btn = button(
        row![
            svg_icon(
                icons::SLIDERS,
                14.0,
                if control_center_open {
                    ACTIVE_DARK_FG
                } else {
                    ACCENT_CYAN
                }
            ),
            text("Control")
                .size(11)
                .font(FONT_DISPLAY_BOLD),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 11])
    .style(active_toggle_button_style(control_center_open, ACCENT_CYAN))
    .on_press(Message::ToggleControlCenter);

    let right_group = row![wifi_btn, vol_btn, cc_btn]
        .spacing(6)
        .align_y(Alignment::Center);

    let bar_row = row![
        container("").width(Length::Fill),
        center_clock_btn,
        container(right_group)
            .width(Length::Fill)
            .align_x(Alignment::End),
    ]
    .align_y(Alignment::Center)
    .padding([5, 14]);

    container(bar_row)
        .width(Length::Fill)
        .style(topbar_glass_style)
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
