use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, scrollable, slider, text};
use iced::{Element, Length};

use crate::theme::{self, ACCENT_BLUE, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub struct SoundSettings {
    pub volume: f32,
    pub is_muted: bool,
    pub mic_volume: f32,
    pub output_device: String,
    pub input_device: String,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            volume: 75.0,
            is_muted: false,
            mic_volume: 80.0,
            output_device: "Speakers (Realtek ALC High Definition Audio)".to_string(),
            input_device: "Internal Microphone Array".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SoundMessage {
    SetVolume(f32),
    ToggleMute,
    SetMicVolume(f32),
    SelectOutput(String),
}

pub fn view<'a, Message: 'static + Clone>(
    settings: &'a SoundSettings,
    map_fn: impl Fn(SoundMessage) -> Message + Copy + 'static,
) -> Element<'a, Message> {
    let title = text("Sound").size(22).color(TEXT_PRIMARY);

    // Master Volume Card
    let vol_slider = row![
        button(text(if settings.is_muted || settings.volume == 0.0 { "🔇" } else { "🔊" }).size(18))
            .padding([2, 6])
            .style(button::text)
            .on_press(map_fn(SoundMessage::ToggleMute)),
        slider(0.0..=100.0, if settings.is_muted { 0.0 } else { settings.volume }, move |v| map_fn(SoundMessage::SetVolume(v)))
            .width(Length::Fill),
        text(format!("{:.0}%", if settings.is_muted { 0.0 } else { settings.volume }))
            .size(12)
            .color(ACCENT_BLUE),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let output_card = container(
        column![
            text("Output Volume").size(13).color(TEXT_PRIMARY),
            vol_slider,
            text(format!("Output Device: {}", settings.output_device)).size(11).color(TEXT_SECONDARY),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    // Microphone Input Card
    let mic_slider = row![
        text("🎙").size(18),
        slider(0.0..=100.0, settings.mic_volume, move |v| map_fn(SoundMessage::SetMicVolume(v)))
            .width(Length::Fill),
        text(format!("{:.0}%", settings.mic_volume))
            .size(12)
            .color(ACCENT_BLUE),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let input_card = container(
        column![
            text("Input Volume (Microphone)").size(13).color(TEXT_PRIMARY),
            mic_slider,
            text(format!("Input Device: {}", settings.input_device)).size(11).color(TEXT_SECONDARY),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    let content = column![title, output_card, input_card].spacing(14);

    scrollable(content).height(Length::Fill).into()
}
