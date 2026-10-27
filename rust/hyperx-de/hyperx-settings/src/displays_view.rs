use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, scrollable, slider, text, Space};
use iced::{Element, Length};

use crate::theme::{self, ACCENT_AMBER, ACCENT_BLUE, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub struct DisplaysSettings {
    pub resolution: String,
    pub refresh_rate: u32,
    pub scale: u32,
    pub night_light_enabled: bool,
    pub night_light_warmth: f32,
}

impl Default for DisplaysSettings {
    fn default() -> Self {
        Self {
            resolution: "1920 x 1080 (16:9)".to_string(),
            refresh_rate: 144,
            scale: 100,
            night_light_enabled: false,
            night_light_warmth: 45.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DisplaysMessage {
    SetResolution(String),
    SetRefreshRate(u32),
    SetScale(u32),
    ToggleNightLight,
    SetNightLightWarmth(f32),
}

pub fn view<'a, Message: 'static + Clone>(
    settings: &'a DisplaysSettings,
    map_fn: impl Fn(DisplaysMessage) -> Message + Copy + 'static,
) -> Element<'a, Message> {
    let title = text("Displays").size(22).color(TEXT_PRIMARY);

    // Display Preview Card
    let preview_box = container(
        column![
            text("Built-in Liquid Retina / eDP-1").size(14).color(TEXT_PRIMARY),
            text(format!("{} @ {}Hz ({}% scale)", settings.resolution, settings.refresh_rate, settings.scale))
                .size(11)
                .color(TEXT_SECONDARY),
        ]
        .spacing(4)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .padding(24)
    .center_x(Length::Fill)
    .style(theme::card_style);

    // Scaling options
    let make_scale_btn = |scale_val: u32| {
        let is_active = settings.scale == scale_val;
        button(
            container(text(format!("{}%", scale_val)).size(12))
                .padding([6, 12])
                .style(if is_active { theme::active_item_style } else { theme::card_style })
        )
        .padding(0)
        .style(button::text)
        .on_press(map_fn(DisplaysMessage::SetScale(scale_val)))
    };

    let scale_card = container(
        column![
            row![
                text("User Interface Scaling").size(13).color(TEXT_PRIMARY),
                Space::new().width(Length::Fill),
                text(format!("{}%", settings.scale)).size(12).color(ACCENT_BLUE),
            ],
            row![
                make_scale_btn(100),
                make_scale_btn(125),
                make_scale_btn(150),
                make_scale_btn(200),
            ]
            .spacing(8),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    // Refresh rate options
    let make_hz_btn = |hz: u32| {
        let is_active = settings.refresh_rate == hz;
        button(
            container(text(format!("{}Hz", hz)).size(12))
                .padding([6, 12])
                .style(if is_active { theme::active_item_style } else { theme::card_style })
        )
        .padding(0)
        .style(button::text)
        .on_press(map_fn(DisplaysMessage::SetRefreshRate(hz)))
    };

    let hz_card = container(
        column![
            row![
                text("Refresh Rate").size(13).color(TEXT_PRIMARY),
                Space::new().width(Length::Fill),
                text(format!("{}Hz ProMotion", settings.refresh_rate)).size(12).color(ACCENT_BLUE),
            ],
            row![
                make_hz_btn(60),
                make_hz_btn(120),
                make_hz_btn(144),
                make_hz_btn(165),
            ]
            .spacing(8),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    // Night Light
    let night_btn = button(
        row![
            text("🌙").size(18),
            column![
                text("Night Shift / Night Light").size(13).color(TEXT_PRIMARY),
                text("Shift display colors to the warmer end of the spectrum after dark").size(10).color(TEXT_SECONDARY),
            ]
            .spacing(2),
            Space::new().width(Length::Fill),
            text(if settings.night_light_enabled { "On" } else { "Off" })
                .size(12)
                .color(if settings.night_light_enabled { ACCENT_AMBER } else { TEXT_SECONDARY }),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .padding(0)
    .style(button::text)
    .on_press(map_fn(DisplaysMessage::ToggleNightLight));

    let warmth_slider = row![
        text("Color Temperature").size(11).color(TEXT_SECONDARY),
        slider(0.0..=100.0, settings.night_light_warmth, move |v| map_fn(DisplaysMessage::SetNightLightWarmth(v)))
            .width(Length::Fill),
        text(format!("{:.0}%", settings.night_light_warmth)).size(11).color(ACCENT_AMBER),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let night_card = container(
        column![night_btn, warmth_slider].spacing(10)
    )
    .padding(14)
    .style(theme::card_style);

    let content = column![title, preview_box, scale_card, hz_card, night_card].spacing(14);

    scrollable(content).height(Length::Fill).into()
}
