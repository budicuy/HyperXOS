use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Color, Element, Length};

use crate::theme::{self, ACCENT_BLUE, ACCENT_GREEN, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub name: String,
    pub dev_type: String, // "Headphones", "Mouse", "Keyboard", "Controller"
    pub connected: bool,
    pub battery: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct BluetoothSettings {
    pub enabled: bool,
    pub devices: Vec<BluetoothDevice>,
}

impl Default for BluetoothSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            devices: vec![
                BluetoothDevice {
                    name: "AirPods Pro (2nd gen)".to_string(),
                    dev_type: "Headphones".to_string(),
                    connected: true,
                    battery: Some(88),
                },
                BluetoothDevice {
                    name: "Logitech MX Master 3S".to_string(),
                    dev_type: "Mouse".to_string(),
                    connected: true,
                    battery: Some(95),
                },
                BluetoothDevice {
                    name: "Keychron K2 Mechanical Keyboard".to_string(),
                    dev_type: "Keyboard".to_string(),
                    connected: false,
                    battery: Some(70),
                },
                BluetoothDevice {
                    name: "Xbox Wireless Controller".to_string(),
                    dev_type: "Controller".to_string(),
                    connected: false,
                    battery: None,
                },
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub enum BluetoothMessage {
    ToggleBluetooth,
    ToggleDevice(String),
}

pub fn view<'a, Message: 'static + Clone>(
    settings: &'a BluetoothSettings,
    map_fn: impl Fn(BluetoothMessage) -> Message + Copy + 'static,
) -> Element<'a, Message> {
    let title = text("Bluetooth").size(22).color(TEXT_PRIMARY);

    let switch = button(
        container(
            row![
                text("ᛒ").size(20).color(ACCENT_BLUE),
                column![
                    text("Bluetooth Radio").size(13).color(TEXT_PRIMARY),
                    text(if settings.enabled { "Discoverable as 'HyperXOS-Host'" } else { "Turned off" })
                        .size(10)
                        .color(TEXT_SECONDARY),
                ]
                .spacing(2),
                Space::new().width(Length::Fill),
                text(if settings.enabled { "On" } else { "Off" })
                    .size(12)
                    .color(if settings.enabled { ACCENT_GREEN } else { TEXT_SECONDARY }),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .padding(14)
        .style(theme::card_style),
    )
    .padding(0)
    .style(button::text)
    .on_press(map_fn(BluetoothMessage::ToggleBluetooth));

    let mut dev_col = column![
        text("My Devices").size(13).color(TEXT_PRIMARY)
    ]
    .spacing(8);

    for dev in &settings.devices {
        let dev_name = dev.name.clone();
        let icon = match dev.dev_type.as_str() {
            "Headphones" => "🎧",
            "Mouse" => "🖱",
            "Keyboard" => "⌨",
            "Controller" => "🎮",
            _ => "📱",
        };

        let row_content = row![
            text(icon).size(20),
            column![
                text(&dev.name).size(13).color(TEXT_PRIMARY),
                text(if let Some(batt) = dev.battery {
                    format!("{}  •  🔋 {}%", dev.dev_type, batt)
                } else {
                    dev.dev_type.clone()
                })
                .size(10)
                .color(TEXT_SECONDARY),
            ]
            .spacing(2),
            Space::new().width(Length::Fill),
            button(
                text(if dev.connected { "Connected ✓" } else { "Connect" })
                    .size(11)
                    .color(if dev.connected { ACCENT_GREEN } else { Color::WHITE }),
            )
            .padding([4, 10])
            .style(if dev.connected { button::text } else { button::primary })
            .on_press(map_fn(BluetoothMessage::ToggleDevice(dev_name))),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        dev_col = dev_col.push(container(row_content).padding([8, 12]).style(theme::card_style));
    }

    let content = column![title, switch, dev_col].spacing(14);

    scrollable(content).height(Length::Fill).into()
}
