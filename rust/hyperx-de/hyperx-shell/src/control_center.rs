use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, slider, text, Space};
use iced::{Element, Length};

use crate::theme::{self, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub enum ControlCenterAction {
    ToggleWifi,
    ToggleBluetooth,
    ToggleAirdrop,
    ToggleDnd,
    SetBrightness(f32),
    SetVolume(f32),
    SetPowerProfile(PowerProfile),
    TogglePlayPause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

pub struct ControlCenter {
    pub wifi_enabled: bool,
    pub wifi_ssid: String,
    pub bluetooth_enabled: bool,
    pub airdrop_enabled: bool,
    pub dnd_enabled: bool,
    pub brightness: f32,
    pub volume: f32,
    pub power_profile: PowerProfile,
    pub is_playing: bool,
}

impl Default for ControlCenter {
    fn default() -> Self {
        Self {
            wifi_enabled: true,
            wifi_ssid: "HyperX-FastNet".to_string(),
            bluetooth_enabled: true,
            airdrop_enabled: true,
            dnd_enabled: false,
            brightness: 85.0,
            volume: 75.0,
            power_profile: PowerProfile::Balanced,
            is_playing: true,
        }
    }
}

impl ControlCenter {
    pub fn update(&mut self, action: ControlCenterAction) {
        match action {
            ControlCenterAction::ToggleWifi => self.wifi_enabled = !self.wifi_enabled,
            ControlCenterAction::ToggleBluetooth => self.bluetooth_enabled = !self.bluetooth_enabled,
            ControlCenterAction::ToggleAirdrop => self.airdrop_enabled = !self.airdrop_enabled,
            ControlCenterAction::ToggleDnd => self.dnd_enabled = !self.dnd_enabled,
            ControlCenterAction::SetBrightness(val) => self.brightness = val,
            ControlCenterAction::SetVolume(val) => self.volume = val,
            ControlCenterAction::SetPowerProfile(prof) => self.power_profile = prof,
            ControlCenterAction::TogglePlayPause => self.is_playing = !self.is_playing,
        }
    }

    pub fn view<Message: 'static + Clone>(
        &self,
        map_fn: impl Fn(ControlCenterAction) -> Message + Copy + 'static,
    ) -> Element<'_, Message> {
        // Connectivity block (Wi-Fi, Bluetooth, AirDrop, Hotspot)
        let wifi_btn = button(
            container(
                row![
                    text(if self.wifi_enabled { "📶" } else { "📴" }).size(18),
                    column![
                        text("Wi-Fi").size(12).color(TEXT_PRIMARY),
                        text(if self.wifi_enabled { &self.wifi_ssid } else { "Off" })
                            .size(10)
                            .color(TEXT_SECONDARY),
                    ]
                    .spacing(1),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding([8, 12])
            .style(if self.wifi_enabled {
                theme::card_active_style
            } else {
                theme::card_style
            }),
        )
        .padding(0)
        .style(button::text)
        .width(Length::FillPortion(1))
        .on_press(map_fn(ControlCenterAction::ToggleWifi));

        let bt_btn = button(
            container(
                row![
                    text("ᛒ").size(18),
                    column![
                        text("Bluetooth").size(12).color(TEXT_PRIMARY),
                        text(if self.bluetooth_enabled { "On" } else { "Off" })
                            .size(10)
                            .color(TEXT_SECONDARY),
                    ]
                    .spacing(1),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding([8, 12])
            .style(if self.bluetooth_enabled {
                theme::card_active_style
            } else {
                theme::card_style
            }),
        )
        .padding(0)
        .style(button::text)
        .width(Length::FillPortion(1))
        .on_press(map_fn(ControlCenterAction::ToggleBluetooth));

        let airdrop_btn = button(
            container(
                row![
                    text("📡").size(18),
                    column![
                        text("HyperX Share").size(12).color(TEXT_PRIMARY),
                        text(if self.airdrop_enabled { "Everyone" } else { "Off" })
                            .size(10)
                            .color(TEXT_SECONDARY),
                    ]
                    .spacing(1),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding([8, 12])
            .style(if self.airdrop_enabled {
                theme::card_active_style
            } else {
                theme::card_style
            }),
        )
        .padding(0)
        .style(button::text)
        .width(Length::FillPortion(1))
        .on_press(map_fn(ControlCenterAction::ToggleAirdrop));

        let dnd_btn = button(
            container(
                row![
                    text("🌙").size(18),
                    column![
                        text("Focus").size(12).color(TEXT_PRIMARY),
                        text(if self.dnd_enabled { "Do Not Disturb" } else { "Off" })
                            .size(10)
                            .color(TEXT_SECONDARY),
                    ]
                    .spacing(1),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding([8, 12])
            .style(if self.dnd_enabled {
                theme::card_active_style
            } else {
                theme::card_style
            }),
        )
        .padding(0)
        .style(button::text)
        .width(Length::FillPortion(1))
        .on_press(map_fn(ControlCenterAction::ToggleDnd));

        let connectivity_grid = column![
            row![wifi_btn, bt_btn].spacing(8),
            row![airdrop_btn, dnd_btn].spacing(8),
        ]
        .spacing(8);

        // Display Brightness Slider Card
        let brightness_card = container(
            column![
                row![
                    text("Display Brightness").size(11).color(TEXT_SECONDARY),
                    Space::new().width(Length::Fill),
                    text(format!("{:.0}%", self.brightness)).size(11).color(TEXT_PRIMARY),
                ],
                row![
                    text("☀️").size(16),
                    slider(
                        0.0..=100.0,
                        self.brightness,
                        move |v| map_fn(ControlCenterAction::SetBrightness(v))
                    )
                    .width(Length::Fill),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(4),
        )
        .padding([8, 12])
        .style(theme::card_style);

        // Sound Volume Slider Card
        let sound_card = container(
            column![
                row![
                    text("Sound Volume").size(11).color(TEXT_SECONDARY),
                    Space::new().width(Length::Fill),
                    text(format!("{:.0}%", self.volume)).size(11).color(TEXT_PRIMARY),
                ],
                row![
                    text(if self.volume > 0.0 { "🔊" } else { "🔇" }).size(16),
                    slider(
                        0.0..=100.0,
                        self.volume,
                        move |v| map_fn(ControlCenterAction::SetVolume(v))
                    )
                    .width(Length::Fill),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(4),
        )
        .padding([8, 12])
        .style(theme::card_style);

        // Media Player Card
        let media_card = container(
            row![
                text("🎧").size(24),
                column![
                    text("Neon Horizon").size(12).color(TEXT_PRIMARY),
                    text("HyperX Synthwave").size(10).color(TEXT_SECONDARY),
                ]
                .spacing(1),
                Space::new().width(Length::Fill),
                button(text(if self.is_playing { "⏸" } else { "▶" }).size(16).color(TEXT_PRIMARY))
                    .padding([4, 8])
                    .style(button::text)
                    .on_press(map_fn(ControlCenterAction::TogglePlayPause)),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding([8, 12])
        .style(theme::card_style);

        // Power profile card
        let make_prof_btn = |label: &'static str, prof: PowerProfile| {
            let active = self.power_profile == prof;
            button(
                container(text(label).size(10))
                    .padding([4, 8])
                    .style(if active {
                        theme::card_active_style
                    } else {
                        theme::card_style
                    }),
            )
            .padding(0)
            .style(button::text)
            .on_press(map_fn(ControlCenterAction::SetPowerProfile(prof)))
        };

        let perf_btn = make_prof_btn("⚡ Perf", PowerProfile::Performance);
        let bal_btn = make_prof_btn("⚖ Balanced", PowerProfile::Balanced);
        let eco_btn = make_prof_btn("🍃 Eco", PowerProfile::PowerSaver);

        let power_card = container(
            row![
                text("Power Mode:").size(11).color(TEXT_SECONDARY),
                Space::new().width(Length::Fill),
                perf_btn,
                bal_btn,
                eco_btn,
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding([6, 12])
        .style(theme::card_style);

        let content = column![
            connectivity_grid,
            brightness_card,
            sound_card,
            media_card,
            power_card,
        ]
        .spacing(10);

        container(content)
            .width(320)
            .padding(14)
            .style(theme::control_center_style)
            .into()
    }
}
