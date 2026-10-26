#![allow(dead_code)]

use iced::border::Radius;
use iced::font::{Family, Weight};
use iced::gradient::Linear;
use iced::widget::scrollable::{self, Rail, Scroller};
use iced::widget::{button, container, progress_bar, text_input};
use iced::{Background, Border, Color, Font, Radians};

// --- Typography: Plus Jakarta Sans (Headings & Brand) + Inter (Body & UI) ---
pub const FONT_DISPLAY_EXTRABOLD: Font = Font {
    family: Family::Name("Plus Jakarta Sans"),
    weight: Weight::ExtraBold,
    ..Font::DEFAULT
};

pub const FONT_DISPLAY_BOLD: Font = Font {
    family: Family::Name("Plus Jakarta Sans"),
    weight: Weight::Bold,
    ..Font::DEFAULT
};

pub const FONT_DISPLAY_SEMIBOLD: Font = Font {
    family: Family::Name("Plus Jakarta Sans"),
    weight: Weight::Semibold,
    ..Font::DEFAULT
};

pub const FONT_BODY_EXTRABOLD: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::ExtraBold,
    ..Font::DEFAULT
};

pub const FONT_BODY_BOLD: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::Bold,
    ..Font::DEFAULT
};

pub const FONT_BODY_SEMIBOLD: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::Semibold,
    ..Font::DEFAULT
};

pub const FONT_BODY_MEDIUM: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::Medium,
    ..Font::DEFAULT
};

pub const FONT_BODY: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::Medium,
    ..Font::DEFAULT
};

// --- Glassmorphism Accent & Text Colors ---
pub const ACCENT_CYAN: Color = Color::from_rgb(0.54, 0.71, 0.98); // #89b4fa
pub const ACCENT_PURPLE: Color = Color::from_rgb(0.80, 0.65, 0.97); // #cba6f7
pub const ACCENT_TEAL: Color = Color::from_rgb(0.58, 0.89, 0.84); // #94e2d5
pub const ACCENT_GREEN: Color = Color::from_rgb(0.65, 0.89, 0.63); // #a6e3a1
pub const ACCENT_RED: Color = Color::from_rgb(0.95, 0.55, 0.66); // #f38ba8
pub const ACCENT_AMBER: Color = Color::from_rgb(0.98, 0.89, 0.69); // #f9e2af

pub const TEXT_MAIN: Color = Color::WHITE;
pub const TEXT_SUB: Color = Color::from_rgb(0.85, 0.88, 0.95);
pub const TEXT_MUTED: Color = Color::from_rgb(0.65, 0.70, 0.82);

pub const CARD_BG_BLACK: Color = Color::from_rgb(0.05, 0.06, 0.09); // Solid Deep Black

// --- Window & Container Glass Styles (Frosted Gradient Translucency) ---
pub fn glass_window_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(0.785)) // 45 degrees
        .add_stop(0.0, Color::from_rgba(0.06, 0.08, 0.14, 0.76))
        .add_stop(0.5, Color::from_rgba(0.04, 0.05, 0.10, 0.72))
        .add_stop(1.0, Color::from_rgba(0.07, 0.05, 0.13, 0.76));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::from(16.0))
            .width(1.2)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.16)),
        ..container::Style::default()
    }
}

pub fn glass_card_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(CARD_BG_BLACK)),
        border: Border::default()
            .rounded(Radius::from(14.0))
            .width(1.5)
            .color(Color::from_rgba(0.35, 0.45, 0.65, 0.40)),
        text_color: Some(Color::WHITE),
        ..container::Style::default()
    }
}

pub fn glass_card_accent_style(accent_color: Color) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {
        container::Style {
            background: Some(Background::Color(CARD_BG_BLACK)),
            border: Border::default()
                .rounded(Radius::from(14.0))
                .width(1.5)
                .color(accent_color),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub fn terminal_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.02, 0.03, 0.05))),
        border: Border::default()
            .rounded(Radius::from(10.0))
            .width(1.2)
            .color(Color::from_rgba(0.65, 0.89, 0.63, 0.40)),
        text_color: Some(ACCENT_GREEN),
        ..container::Style::default()
    }
}

pub fn stepper_bar_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(0.0))
        .add_stop(0.0, Color::from_rgba(0.85, 0.12, 0.20, 0.82))
        .add_stop(0.5, Color::from_rgba(0.90, 0.26, 0.12, 0.80))
        .add_stop(1.0, Color::from_rgba(0.96, 0.44, 0.08, 0.80));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::default().left(16.0)),
        ..container::Style::default()
    }
}

// --- Button Styles ---
pub fn primary_button_style(accent: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border_col, text_col) = match status {
            button::Status::Hovered => (
                Color::WHITE,
                Color::WHITE,
                Color::from_rgb(0.06, 0.07, 0.10),
            ),
            button::Status::Pressed => (
                Color::from_rgb(0.90, 0.90, 0.92),
                Color::WHITE,
                Color::from_rgb(0.06, 0.07, 0.10),
            ),
            button::Status::Disabled => (
                Color::from_rgb(0.05, 0.06, 0.09),
                Color::from_rgba(1.0, 1.0, 1.0, 0.10),
                TEXT_MUTED,
            ),
            _ => (
                Color::from_rgb(0.06, 0.07, 0.11),
                accent,
                Color::WHITE,
            ),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_col,
            border: Border::default()
                .rounded(Radius::from(9.0))
                .width(1.5)
                .color(border_col),
            ..button::Style::default()
        }
    }
}

pub fn secondary_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_theme, status| {
        let (bg, border_col) = match status {
            button::Status::Hovered => (
                Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                Color::from_rgba(1.0, 1.0, 1.0, 0.40),
            ),
            button::Status::Pressed => (
                Color::from_rgba(1.0, 1.0, 1.0, 0.28),
                Color::from_rgba(1.0, 1.0, 1.0, 0.60),
            ),
            button::Status::Disabled => (
                Color::from_rgba(1.0, 1.0, 1.0, 0.03),
                Color::from_rgba(1.0, 1.0, 1.0, 0.06),
            ),
            _ => (
                Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                Color::from_rgba(1.0, 1.0, 1.0, 0.18),
            ),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: Color::WHITE,
            border: Border::default()
                .rounded(Radius::from(9.0))
                .width(1.2)
                .color(border_col),
            ..button::Style::default()
        }
    }
}

pub fn danger_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_theme, status| {
        let (bg, border_col, text_col) = match status {
            button::Status::Hovered => (
                Color::WHITE,
                Color::WHITE,
                Color::from_rgb(0.06, 0.07, 0.10),
            ),
            button::Status::Pressed => (
                Color::from_rgb(0.90, 0.90, 0.92),
                Color::WHITE,
                Color::from_rgb(0.06, 0.07, 0.10),
            ),
            _ => (
                Color::from_rgb(0.12, 0.03, 0.04),
                Color::from_rgb(0.95, 0.30, 0.35),
                Color::WHITE,
            ),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_col,
            border: Border::default()
                .rounded(Radius::from(9.0))
                .width(1.5)
                .color(border_col),
            ..button::Style::default()
        }
    }
}

pub fn selection_card_style(is_selected: bool) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {
        let (border_col, bg) = if is_selected {
            (
                ACCENT_CYAN,
                Color::from_rgba(0.54, 0.71, 0.98, 0.12),
            )
        } else {
            (
                Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                CARD_BG_BLACK,
            )
        };

        container::Style {
            background: Some(Background::Color(bg)),
            border: Border::default()
                .rounded(Radius::from(12.0))
                .width(if is_selected { 1.8 } else { 1.2 })
                .color(border_col),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub fn custom_text_input_style() -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    |_theme, status| {
        let border_col = match status {
            text_input::Status::Focused { .. } => ACCENT_CYAN,
            text_input::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.35),
            _ => Color::from_rgba(1.0, 1.0, 1.0, 0.18),
        };

        text_input::Style {
            background: Background::Color(Color::from_rgba(0.04, 0.05, 0.09, 0.90)),
            border: Border::default()
                .rounded(Radius::from(8.0))
                .width(1.2)
                .color(border_col),
            icon: TEXT_MUTED,
            placeholder: TEXT_MUTED,
            value: Color::WHITE,
            selection: Color::from_rgba(0.54, 0.71, 0.98, 0.35),
        }
    }
}

pub fn custom_progress_bar_style(accent_color: Color) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    move |_theme| {
        progress_bar::Style {
            background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.08)),
            bar: Background::Color(accent_color),
            border: Border::default().rounded(Radius::from(6.0)),
        }
    }
}

pub fn glass_scrollable_style(_theme: &iced::Theme, status: scrollable::Status) -> scrollable::Style {
    let scroller_color = match status {
        scrollable::Status::Hovered { .. } => Color::from_rgba(0.54, 0.71, 0.98, 0.45),
        scrollable::Status::Dragged { .. } => Color::from_rgba(0.54, 0.71, 0.98, 0.70),
        _ => Color::from_rgba(1.0, 1.0, 1.0, 0.20),
    };

    scrollable::Style {
        container: container::Style::default(),
        vertical_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                background: Background::Color(scroller_color),
                border: Border::default().rounded(Radius::from(3.0)),
            },
        },
        horizontal_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                background: Background::Color(scroller_color),
                border: Border::default().rounded(Radius::from(3.0)),
            },
        },
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
            shadow: iced::Shadow::default(),
            icon: Color::TRANSPARENT,
        },
    }
}
