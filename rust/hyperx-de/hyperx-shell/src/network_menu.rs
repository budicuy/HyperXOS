use std::process::Command;

use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, text, Space};
use iced::{Color, Element, Length};

use crate::theme::{self, ACCENT_BLUE, ACCENT_GREEN, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub struct WifiEntry {
    pub ssid: String,
    pub signal: u8,
    pub in_use: bool,
}

#[derive(Debug, Clone)]
pub enum NetworkMenuAction {
    ToggleWifi,
    ConnectWifi(String),
    OpenNetworkSettings,
}

pub struct NetworkMenu {
    pub wifi_enabled: bool,
    pub active_conn: String,
    pub networks: Vec<WifiEntry>,
}

impl Default for NetworkMenu {
    fn default() -> Self {
        let mut menu = Self {
            wifi_enabled: true,
            active_conn: "Wired Connection (enp3s0)".to_string(),
            networks: Vec::new(),
        };
        menu.scan();
        menu
    }
}

impl NetworkMenu {
    pub fn scan(&mut self) {
        // Query active device via nmcli
        if let Ok(out) = Command::new("nmcli").args(["-t", "-f", "NAME,DEVICE,TYPE", "c", "show", "--active"]).output() {
            let s = String::from_utf8_lossy(&out.stdout);
            if let Some(first_line) = s.lines().next() {
                let parts: Vec<&str> = first_line.split(':').collect();
                if let Some(name) = parts.first() {
                    self.active_conn = name.to_string();
                }
            }
        }

        // Scan available Wi-Fi
        let mut list = Vec::new();
        if let Ok(out) = Command::new("nmcli").args(["-t", "-f", "SSID,SIGNAL,IN-USE", "dev", "wifi", "list"]).output() {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let ssid = parts[0].trim().to_string();
                    if ssid.is_empty() {
                        continue;
                    }
                    let signal = parts[1].parse::<u8>().unwrap_or(50);
                    let in_use = parts.get(2).map(|v| v.trim() == "*").unwrap_or(false);
                    if !list.iter().any(|e: &WifiEntry| e.ssid == ssid) {
                        list.push(WifiEntry { ssid, signal, in_use });
                    }
                }
            }
        }

        if list.is_empty() {
            list = vec![
                WifiEntry {
                    ssid: "HyperX-FastNet-5G".to_string(),
                    signal: 95,
                    in_use: true,
                },
                WifiEntry {
                    ssid: "HyperX-Mesh-Office".to_string(),
                    signal: 80,
                    in_use: false,
                },
                WifiEntry {
                    ssid: "Guest-Network".to_string(),
                    signal: 60,
                    in_use: false,
                },
            ];
        }

        self.networks = list;
    }

    pub fn view<Message: 'static + Clone>(
        &self,
        map_fn: impl Fn(NetworkMenuAction) -> Message + Copy + 'static,
    ) -> Element<'_, Message> {
        let header = row![
            text("Wi-Fi").size(13).color(TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            button(
                text(if self.wifi_enabled { "On" } else { "Off" })
                    .size(11)
                    .color(if self.wifi_enabled { ACCENT_GREEN } else { TEXT_SECONDARY }),
            )
            .padding([2, 6])
            .style(button::text)
            .on_press(map_fn(NetworkMenuAction::ToggleWifi)),
        ]
        .align_y(Alignment::Center);

        let active_section = container(
            row![
                text("●").size(8).color(ACCENT_GREEN),
                column![
                    text(&self.active_conn).size(12).color(TEXT_PRIMARY),
                    text("Connected").size(10).color(TEXT_SECONDARY),
                ]
                .spacing(1),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding([6, 10])
        .style(theme::card_style);

        let mut known_col = column![
            text("Known Networks").size(11).color(TEXT_SECONDARY)
        ]
        .spacing(4);

        for net in &self.networks {
            let ssid = net.ssid.clone();
            let bars = if net.signal > 75 {
                "▂▄▆█"
            } else if net.signal > 50 {
                "▂▄▆_"
            } else if net.signal > 25 {
                "▂▄__"
            } else {
                "▂___"
            };

            let net_btn = button(
                container(
                    row![
                        text(bars).size(12).color(ACCENT_BLUE),
                        text(&net.ssid).size(12).color(TEXT_PRIMARY),
                        Space::new().width(Length::Fill),
                        if net.in_use {
                            text("✓").size(12).color(ACCENT_GREEN)
                        } else {
                            text("").size(12).color(Color::TRANSPARENT)
                        },
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([4, 8])
                .style(|_| container::Style::default()),
            )
            .padding(0)
            .style(button::text)
            .on_press(map_fn(NetworkMenuAction::ConnectWifi(ssid)));

            known_col = known_col.push(net_btn);
        }

        let divider = container(Space::new().width(Length::Fill).height(1))
            .style(|_| container::Style {
                background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()),
                ..Default::default()
            });

        let settings_btn = button(
            container(
                row![
                    text("⚙").size(14),
                    text("Network Settings...").size(12).color(TEXT_PRIMARY),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([4, 8]),
        )
        .padding(0)
        .style(button::text)
        .on_press(map_fn(NetworkMenuAction::OpenNetworkSettings));

        let content = column![header, active_section, known_col, divider, settings_btn].spacing(8);

        container(content)
            .width(280)
            .padding(12)
            .style(theme::menu_popup_style)
            .into()
    }
}
