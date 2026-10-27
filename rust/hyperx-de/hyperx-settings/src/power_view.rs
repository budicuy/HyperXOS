use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length};

use crate::theme::{self, ACCENT_BLUE, ACCENT_GREEN, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

#[derive(Debug, Clone)]
pub struct PowerSettings {
    pub profile: PowerProfile,
    pub sleep_minutes: u32,
    pub battery_health: u8,
    pub battery_level: u8,
    pub is_charging: bool,
    pub optimize_battery: bool,
}

impl Default for PowerSettings {
    fn default() -> Self {
        Self {
            profile: PowerProfile::Balanced,
            sleep_minutes: 15,
            battery_health: 98,
            battery_level: 94,
            is_charging: true,
            optimize_battery: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PowerMessage {
    SetProfile(PowerProfile),
    SetSleepTimeout(u32),
    ToggleOptimizeBattery,
}

pub fn view<'a, Message: 'static + Clone>(
    settings: &'a PowerSettings,
    map_fn: impl Fn(PowerMessage) -> Message + Copy + 'static,
) -> Element<'a, Message> {
    let title = text("Battery & Power").size(22).color(TEXT_PRIMARY);

    // Battery Status Card
    let batt_row = row![
        text(if settings.is_charging { "⚡" } else { "🔋" }).size(28).color(ACCENT_GREEN),
        column![
            text(format!("{}% Charged", settings.battery_level)).size(15).color(TEXT_PRIMARY),
            text(if settings.is_charging { "Connected to Power Adapter" } else { "Running on Battery" })
                .size(11)
                .color(TEXT_SECONDARY),
        ]
        .spacing(2),
        Space::new().width(Length::Fill),
        column![
            text(format!("Health: {}%", settings.battery_health)).size(12).color(TEXT_PRIMARY),
            text("Normal Condition").size(10).color(ACCENT_GREEN),
        ]
        .spacing(2),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let batt_card = container(batt_row).padding(16).style(theme::card_style);

    // Power Mode Buttons
    let make_prof_btn = |prof: PowerProfile, label: &'static str, desc: &'static str| {
        let is_active = settings.profile == prof;
        button(
            container(
                column![
                    text(label).size(13).color(if is_active { iced::Color::WHITE } else { TEXT_PRIMARY }),
                    text(desc).size(10).color(if is_active { iced::Color::WHITE } else { TEXT_SECONDARY }),
                ]
                .spacing(2)
                .align_x(Alignment::Center),
            )
            .width(Length::FillPortion(1))
            .padding([10, 14])
            .center_x(Length::Fill)
            .style(if is_active { theme::active_item_style } else { theme::card_style })
        )
        .padding(0)
        .style(button::text)
        .on_press(map_fn(PowerMessage::SetProfile(prof)))
    };

    let mode_card = container(
        column![
            text("Energy Mode").size(13).color(TEXT_PRIMARY),
            row![
                make_prof_btn(PowerProfile::Performance, "⚡ Performance", "Max CPU boost & FPS"),
                make_prof_btn(PowerProfile::Balanced, "⚖ Balanced", "Optimal power & thermals"),
                make_prof_btn(PowerProfile::PowerSaver, "🍃 Low Power", "Extend battery life"),
            ]
            .spacing(10),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    // Turn display off timeout
    let make_time_btn = |min: u32, label: &'static str| {
        let is_active = settings.sleep_minutes == min;
        button(
            container(text(label).size(11))
                .padding([6, 12])
                .style(if is_active { theme::active_item_style } else { theme::card_style })
        )
        .padding(0)
        .style(button::text)
        .on_press(map_fn(PowerMessage::SetSleepTimeout(min)))
    };

    let sleep_card = container(
        column![
            row![
                text("Turn Display Off When Inactive").size(13).color(TEXT_PRIMARY),
                Space::new().width(Length::Fill),
                text(if settings.sleep_minutes == 0 { "Never".to_string() } else { format!("{} min", settings.sleep_minutes) })
                    .size(12)
                    .color(ACCENT_BLUE),
            ],
            row![
                make_time_btn(5, "5 min"),
                make_time_btn(10, "10 min"),
                make_time_btn(15, "15 min"),
                make_time_btn(30, "30 min"),
                make_time_btn(0, "Never"),
            ]
            .spacing(8),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    let content = column![title, batt_card, mode_card, sleep_card].spacing(14);

    scrollable(content).height(Length::Fill).into()
}
