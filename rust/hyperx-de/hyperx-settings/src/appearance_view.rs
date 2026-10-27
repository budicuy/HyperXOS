use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, scrollable, slider, text, Space};
use iced::{Color, Element, Length};

use crate::theme::{self, ACCENT_AMBER, ACCENT_BLUE, ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccentColor {
    Blue,
    Purple,
    Green,
    Amber,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockPosition {
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct AppearanceSettings {
    pub theme_mode: ThemeMode,
    pub accent: AccentColor,
    pub dock_position: DockPosition,
    pub dock_size: f32,
    pub glass_blur: f32,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            accent: AccentColor::Blue,
            dock_position: DockPosition::Bottom,
            dock_size: 48.0,
            glass_blur: 85.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppearanceMessage {
    SetTheme(ThemeMode),
    SetAccent(AccentColor),
    SetDockPosition(DockPosition),
    SetDockSize(f32),
    SetGlassBlur(f32),
}

pub fn view<'a, Message: 'static + Clone>(
    settings: &'a AppearanceSettings,
    map_fn: impl Fn(AppearanceMessage) -> Message + Copy + 'static,
) -> Element<'a, Message> {
    let title = text("Appearance").size(22).color(TEXT_PRIMARY);

    // Theme Mode Selector
    let make_theme_btn = |mode: ThemeMode, label: &'static str, icon: &'static str| {
        let is_active = settings.theme_mode == mode;
        button(
            container(
                column![
                    text(icon).size(24),
                    text(label).size(12),
                ]
                .spacing(4)
                .align_x(Alignment::Center),
            )
            .width(Length::FillPortion(1))
            .padding([12, 16])
            .center_x(Length::Fill)
            .style(if is_active { theme::active_item_style } else { theme::card_style })
        )
        .padding(0)
        .style(button::text)
        .on_press(map_fn(AppearanceMessage::SetTheme(mode)))
    };

    let theme_card = container(
        column![
            text("Appearance Style").size(13).color(TEXT_PRIMARY),
            row![
                make_theme_btn(ThemeMode::Dark, "Dark Mode", "🌙"),
                make_theme_btn(ThemeMode::Light, "Light Mode", "☀️"),
                make_theme_btn(ThemeMode::Auto, "Automatic", "🌗"),
            ]
            .spacing(10),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    // Accent Color Selector
    let make_color_btn = |acc: AccentColor, col: Color, name: &'static str| {
        let is_active = settings.accent == acc;
        button(
            container(
                row![
                    text("●").size(16).color(col),
                    text(name).size(11).color(if is_active { Color::WHITE } else { TEXT_PRIMARY }),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([6, 12])
            .style(if is_active { theme::active_item_style } else { theme::card_style })
        )
        .padding(0)
        .style(button::text)
        .on_press(map_fn(AppearanceMessage::SetAccent(acc)))
    };

    let accent_card = container(
        column![
            text("Accent Color").size(13).color(TEXT_PRIMARY),
            row![
                make_color_btn(AccentColor::Blue, ACCENT_BLUE, "Ocean Blue"),
                make_color_btn(AccentColor::Purple, ACCENT_PURPLE, "Cyber Purple"),
                make_color_btn(AccentColor::Green, ACCENT_GREEN, "Emerald"),
                make_color_btn(AccentColor::Amber, ACCENT_AMBER, "Sunset Amber"),
                make_color_btn(AccentColor::Red, ACCENT_RED, "Crimson"),
            ]
            .spacing(8),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    // Dock Configuration
    let make_dock_pos_btn = |pos: DockPosition, label: &'static str| {
        let is_active = settings.dock_position == pos;
        button(
            container(text(label).size(11))
                .padding([6, 12])
                .style(if is_active { theme::active_item_style } else { theme::card_style })
        )
        .padding(0)
        .style(button::text)
        .on_press(map_fn(AppearanceMessage::SetDockPosition(pos)))
    };

    let dock_size_row = row![
        text("Dock Icon Size").size(11).color(TEXT_SECONDARY),
        slider(32.0..=64.0, settings.dock_size, move |v| map_fn(AppearanceMessage::SetDockSize(v)))
            .width(Length::Fill),
        text(format!("{:.0}px", settings.dock_size)).size(11).color(ACCENT_BLUE),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let dock_card = container(
        column![
            text("macOS Floating Dock").size(13).color(TEXT_PRIMARY),
            row![
                text("Screen Position:").size(11).color(TEXT_SECONDARY),
                Space::new().width(Length::Fill),
                make_dock_pos_btn(DockPosition::Bottom, "Bottom"),
                make_dock_pos_btn(DockPosition::Left, "Left"),
                make_dock_pos_btn(DockPosition::Right, "Right"),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            dock_size_row,
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    // Glassmorphism Blur Slider
    let blur_row = row![
        text("Glass Transparency").size(11).color(TEXT_SECONDARY),
        slider(40.0..=100.0, settings.glass_blur, move |v| map_fn(AppearanceMessage::SetGlassBlur(v)))
            .width(Length::Fill),
        text(format!("{:.0}%", settings.glass_blur)).size(11).color(ACCENT_BLUE),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let glass_card = container(
        column![
            text("Frosted Glass Aero Effects").size(13).color(TEXT_PRIMARY),
            blur_row,
        ]
        .spacing(10),
    )
    .padding(14)
    .style(theme::card_style);

    let content = column![title, theme_card, accent_card, dock_card, glass_card].spacing(14);

    scrollable(content).height(Length::Fill).into()
}
