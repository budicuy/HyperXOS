use iced::border::Radius;
use iced::widget::{button, container, progress_bar, text_input};
use iced::{Background, Border, Color};

// --- Glassmorphism Color Palette ---
pub const BG_WINDOW: Color = Color::from_rgba(0.06, 0.07, 0.11, 0.85);
pub const BG_SIDEBAR: Color = Color::from_rgba(0.04, 0.05, 0.08, 0.92);
pub const BG_CARD: Color = Color::from_rgba(0.12, 0.14, 0.22, 0.55);
#[allow(dead_code)]
pub const BG_CARD_HOVER: Color = Color::from_rgba(0.18, 0.21, 0.32, 0.65);
pub const BG_INPUT: Color = Color::from_rgba(0.08, 0.09, 0.15, 0.70);
pub const BG_TERMINAL: Color = Color::from_rgba(0.05, 0.05, 0.09, 0.90);

pub const BORDER_GLASS: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.10);
pub const BORDER_LIGHT: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.16);

pub const ACCENT_CYAN: Color = Color::from_rgb(0.54, 0.71, 0.98); // #89b4fa
pub const ACCENT_PURPLE: Color = Color::from_rgb(0.80, 0.65, 0.97); // #cba6f7
pub const ACCENT_TEAL: Color = Color::from_rgb(0.58, 0.89, 0.84); // #94e2d5
pub const ACCENT_GREEN: Color = Color::from_rgb(0.65, 0.89, 0.63); // #a6e3a1
pub const ACCENT_RED: Color = Color::from_rgb(0.95, 0.55, 0.66); // #f38ba8
pub const ACCENT_AMBER: Color = Color::from_rgb(0.98, 0.89, 0.69); // #f9e2af

pub const TEXT_MAIN: Color = Color::from_rgb(0.95, 0.96, 0.99);
pub const TEXT_SUB: Color = Color::from_rgb(0.65, 0.68, 0.78);
pub const TEXT_MUTED: Color = Color::from_rgb(0.42, 0.44, 0.55);
#[allow(dead_code)]
pub const TEXT_DARK: Color = Color::from_rgb(0.07, 0.07, 0.11);

// --- Container Styles ---
pub fn glass_card_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_CARD)),
        border: Border::default()
            .rounded(Radius::from(12.0))
            .width(1.0)
            .color(BORDER_GLASS),
        text_color: Some(TEXT_MAIN),
        ..container::Style::default()
    }
}

pub fn glass_card_accent_style(accent_color: Color) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(BG_CARD)),
        border: Border::default()
            .rounded(Radius::from(12.0))
            .width(1.0)
            .color(Color {
                a: 0.35,
                ..accent_color
            }),
        text_color: Some(TEXT_MAIN),
        ..container::Style::default()
    }
}

pub fn terminal_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_TERMINAL)),
        border: Border::default()
            .rounded(Radius::from(10.0))
            .width(1.0)
            .color(BORDER_GLASS),
        text_color: Some(ACCENT_GREEN),
        ..container::Style::default()
    }
}

pub fn sidebar_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BG_SIDEBAR)),
        border: Border::default()
            .width(1.0)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.06)),
        ..container::Style::default()
    }
}

pub fn header_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.05, 0.06, 0.10, 0.70))),
        border: Border::default()
            .width(1.0)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.06)),
        ..container::Style::default()
    }
}

// --- Button Styles ---
pub fn nav_button_style(is_active: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border_color, text_color) = if is_active {
            (
                Color::from_rgba(0.54, 0.71, 0.98, 0.18),
                Color::from_rgba(0.54, 0.71, 0.98, 0.40),
                ACCENT_CYAN,
            )
        } else {
            match status {
                button::Status::Hovered => (
                    Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                    Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                    TEXT_MAIN,
                ),
                button::Status::Pressed => (
                    Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                    Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                    TEXT_MAIN,
                ),
                _ => (Color::TRANSPARENT, Color::TRANSPARENT, TEXT_SUB),
            }
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color,
            border: Border::default()
                .rounded(Radius::from(8.0))
                .width(1.0)
                .color(border_color),
            ..button::Style::default()
        }
    }
}

pub fn primary_button_style(accent: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (alpha, text_col) = match status {
            button::Status::Hovered => (0.35, TEXT_MAIN),
            button::Status::Pressed => (0.50, TEXT_MAIN),
            button::Status::Disabled => (0.08, TEXT_MUTED),
            _ => (0.20, accent),
        };

        button::Style {
            background: Some(Background::Color(Color {
                a: alpha,
                ..accent
            })),
            text_color: text_col,
            border: Border::default()
                .rounded(Radius::from(8.0))
                .width(1.0)
                .color(Color {
                    a: alpha + 0.2,
                    ..accent
                }),
            ..button::Style::default()
        }
    }
}

pub fn glass_action_button(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (bg, text) = match status {
        button::Status::Hovered => (Color::from_rgba(1.0, 1.0, 1.0, 0.12), TEXT_MAIN),
        button::Status::Pressed => (Color::from_rgba(1.0, 1.0, 1.0, 0.18), TEXT_MAIN),
        _ => (Color::from_rgba(1.0, 1.0, 1.0, 0.06), TEXT_SUB),
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: Border::default()
            .rounded(Radius::from(8.0))
            .width(1.0)
            .color(BORDER_GLASS),
        ..button::Style::default()
    }
}

// --- Progress Bar Style ---
pub fn glass_progress_bar_style(accent: Color) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    move |_theme| progress_bar::Style {
        background: Background::Color(Color::from_rgba(0.04, 0.05, 0.08, 0.70)),
        bar: Background::Color(accent),
        border: Border::default().rounded(Radius::from(4.0)),
    }
}

// --- Text Input Style ---
pub fn glass_input_style(_theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let border_color = match status {
        text_input::Status::Focused { .. } => ACCENT_CYAN,
        text_input::Status::Hovered => BORDER_LIGHT,
        _ => BORDER_GLASS,
    };

    text_input::Style {
        background: Background::Color(BG_INPUT),
        border: Border::default()
            .rounded(Radius::from(8.0))
            .width(1.0)
            .color(border_color),
        icon: TEXT_MUTED,
        placeholder: TEXT_MUTED,
        value: TEXT_MAIN,
        selection: Color::from_rgba(0.54, 0.71, 0.98, 0.35),
    }
}
