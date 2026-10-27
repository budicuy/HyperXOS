use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Color, Element, Length};

use crate::network::NetworkManager;
use crate::theme::{self, ACCENT_BLUE, ACCENT_GREEN, ACCENT_RED, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub enum NetworkMessage {
    ToggleNetworking,
    ToggleWifi,
    RefreshNetworks,
    SelectWifi(String),
    PasswordChanged(String),
    ConnectWifi,
    CancelConnect,
    DisconnectDevice(String),
}

pub fn view<'a, Message: 'static + Clone>(
    manager: &'a NetworkManager,
    map_fn: impl Fn(NetworkMessage) -> Message + Copy + 'static,
) -> Element<'a, Message> {
    // Header
    let title = row![
        text("Network & Wi-Fi").size(22).color(TEXT_PRIMARY),
        Space::new().width(Length::Fill),
        button(text("↻ Refresh").size(12).color(ACCENT_BLUE))
            .padding([4, 10])
            .style(button::text)
            .on_press(map_fn(NetworkMessage::RefreshNetworks)),
    ]
    .align_y(Alignment::Center);

    // Switches row (Networking & Wi-Fi)
    let net_switch = button(
        container(
            row![
                text(if manager.is_networking_enabled { "🌐" } else { "📴" }).size(18),
                text("Network Adapter").size(13).color(TEXT_PRIMARY),
                Space::new().width(Length::Fill),
                text(if manager.is_networking_enabled { "Enabled" } else { "Disabled" })
                    .size(12)
                    .color(if manager.is_networking_enabled { ACCENT_GREEN } else { ACCENT_RED }),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding([10, 14])
        .style(theme::card_style),
    )
    .padding(0)
    .style(button::text)
    .width(Length::FillPortion(1))
    .on_press(map_fn(NetworkMessage::ToggleNetworking));

    let wifi_switch = button(
        container(
            row![
                text(if manager.is_wifi_enabled { "📶" } else { "📴" }).size(18),
                text("Wi-Fi Radio").size(13).color(TEXT_PRIMARY),
                Space::new().width(Length::Fill),
                text(if manager.is_wifi_enabled { "On" } else { "Off" })
                    .size(12)
                    .color(if manager.is_wifi_enabled { ACCENT_GREEN } else { TEXT_SECONDARY }),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding([10, 14])
        .style(theme::card_style),
    )
    .padding(0)
    .style(button::text)
    .width(Length::FillPortion(1))
    .on_press(map_fn(NetworkMessage::ToggleWifi));

    let switches_row = row![net_switch, wifi_switch].spacing(12);

    // Active Devices Card
    let mut devices_col = column![
        text("Active Network Interfaces").size(14).color(TEXT_PRIMARY)
    ]
    .spacing(8);

    for dev in &manager.devices {
        let is_connected = dev.state.to_lowercase().contains("connect") || dev.state.to_lowercase().contains("terhubung");
        let dev_row = row![
            text(if dev.dev_type.contains("wifi") { "📶" } else { "🔌" }).size(18),
            column![
                text(format!("{} ({})", dev.name, dev.dev_type)).size(13).color(TEXT_PRIMARY),
                text(format!("IP: {}  •  {}", dev.ip_address, dev.connection)).size(11).color(TEXT_SECONDARY),
            ]
            .spacing(2),
            Space::new().width(Length::Fill),
            text(if is_connected { "Connected" } else { &dev.state })
                .size(11)
                .color(if is_connected { ACCENT_GREEN } else { TEXT_SECONDARY }),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        devices_col = devices_col.push(
            container(dev_row)
                .padding([8, 12])
                .style(theme::card_style)
        );
    }

    // Available Wi-Fi Networks Section
    let mut wifi_col = column![
        text("Available Wi-Fi Networks").size(14).color(TEXT_PRIMARY)
    ]
    .spacing(8);

    for net in &manager.wifi_networks {
        let signal_bars = if net.signal > 75 {
            "▂▄▆█"
        } else if net.signal > 50 {
            "▂▄▆_"
        } else if net.signal > 25 {
            "▂▄__"
        } else {
            "▂___"
        };

        let ssid_clone = net.ssid.clone();
        let net_row = row![
            text(signal_bars).size(14).color(ACCENT_BLUE),
            column![
                text(&net.ssid).size(13).color(TEXT_PRIMARY),
                text(format!("Signal: {}%  •  {}", net.signal, net.security))
                    .size(10)
                    .color(TEXT_SECONDARY),
            ]
            .spacing(2),
            Space::new().width(Length::Fill),
            if net.in_use {
                button(text("Connected ✓").size(11).color(ACCENT_GREEN))
                    .padding([4, 10])
                    .style(button::text)
            } else {
                button(text("Connect").size(11).color(Color::WHITE))
                    .padding([4, 12])
                    .style(button::primary)
                    .on_press(map_fn(NetworkMessage::SelectWifi(ssid_clone)))
            },
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        wifi_col = wifi_col.push(
            container(net_row)
                .padding([8, 12])
                .style(theme::card_style)
        );
    }

    // Password Connect Dialog Modal if an SSID is selected
    let mut content = column![title, switches_row, devices_col].spacing(16);

    if let Some(ref target_ssid) = manager.connecting_ssid {
        let modal = container(
            column![
                text(format!("Connect to '{}'", target_ssid)).size(14).color(TEXT_PRIMARY),
                text_input("Enter Wi-Fi Password...", &manager.password_input)
                    .on_input(move |p| map_fn(NetworkMessage::PasswordChanged(p)))
                    .padding([8, 12])
                    .secure(true),
                row![
                    Space::new().width(Length::Fill),
                    button(text("Cancel").size(12))
                        .padding([6, 12])
                        .style(button::secondary)
                        .on_press(map_fn(NetworkMessage::CancelConnect)),
                    button(text("Join Network").size(12))
                        .padding([6, 14])
                        .style(button::primary)
                        .on_press(map_fn(NetworkMessage::ConnectWifi)),
                ]
                .spacing(10),
            ]
            .spacing(12),
        )
        .padding(16)
        .style(theme::card_style);

        content = content.push(modal);
    }

    content = content.push(wifi_col);

    scrollable(content)
        .height(Length::Fill)
        .into()
}
