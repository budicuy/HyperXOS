use iced::border::Radius;
use iced::font::{Family, Weight};
use iced::gradient::Linear;
use iced::widget::scrollable::{self, Rail, Scroller};
use iced::widget::{button, container, progress_bar, text_input};
use iced::{Background, Border, Color, Font, Radians};

// --- Typography: Plus Jakarta Sans (Headings & Brand) + Inter (Body & UI) ---
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

pub const FONT_BODY_MEDIUM: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::Medium,
    ..Font::DEFAULT
};

pub const FONT_BODY: Font = Font {
    family: Family::Name("Inter"),
    weight: Weight::Normal,
    ..Font::DEFAULT
};

// --- Glassmorphism Accent & Text Colors ---
pub const ACCENT_CYAN: Color = Color::from_rgb(0.54, 0.71, 0.98); // #89b4fa
pub const ACCENT_PURPLE: Color = Color::from_rgb(0.80, 0.65, 0.97); // #cba6f7
pub const ACCENT_TEAL: Color = Color::from_rgb(0.58, 0.89, 0.84); // #94e2d5
pub const ACCENT_GREEN: Color = Color::from_rgb(0.65, 0.89, 0.63); // #a6e3a1
pub const ACCENT_RED: Color = Color::from_rgb(0.95, 0.55, 0.66); // #f38ba8
pub const ACCENT_AMBER: Color = Color::from_rgb(0.98, 0.89, 0.69); // #f9e2af

pub const TEXT_MAIN: Color = Color::from_rgb(0.95, 0.96, 0.99);
pub const TEXT_SUB: Color = Color::from_rgb(0.68, 0.72, 0.84);
pub const TEXT_MUTED: Color = Color::from_rgb(0.44, 0.48, 0.60);

// --- Window & Container Glass Styles (Frosted Gradient Translucency) ---
pub fn glass_window_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(0.785)) // 45 degrees
        .add_stop(0.0, Color::from_rgba(0.06, 0.09, 0.18, 0.38))
        .add_stop(0.5, Color::from_rgba(0.04, 0.06, 0.12, 0.28))
        .add_stop(1.0, Color::from_rgba(0.02, 0.03, 0.07, 0.42));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::from(16.0))
            .width(1.2)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.16)),
        ..container::Style::default()
    }
}

pub fn sidebar_container_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(1.571)) // 90 degrees (top to bottom)
        .add_stop(0.0, Color::from_rgba(0.05, 0.07, 0.15, 0.42))
        .add_stop(1.0, Color::from_rgba(0.02, 0.03, 0.08, 0.55));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::default().left(16.0)),
        ..container::Style::default()
    }
}

pub fn header_container_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(0.0)) // left to right
        .add_stop(0.0, Color::from_rgba(0.06, 0.10, 0.20, 0.30))
        .add_stop(1.0, Color::from_rgba(0.03, 0.05, 0.10, 0.15));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::default().top_right(16.0)),
        ..container::Style::default()
    }
}

pub fn glass_card_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(0.6))
        .add_stop(0.0, Color::from_rgba(0.14, 0.19, 0.34, 0.28))
        .add_stop(1.0, Color::from_rgba(0.07, 0.10, 0.18, 0.20));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::from(14.0))
            .width(1.2)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.15)),
        text_color: Some(TEXT_MAIN),
        ..container::Style::default()
    }
}

pub fn glass_card_accent_style(accent_color: Color) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {
        let gradient = Linear::new(Radians(0.6))
            .add_stop(
                0.0,
                Color {
                    r: (accent_color.r * 0.3 + 0.10).min(1.0),
                    g: (accent_color.g * 0.3 + 0.12).min(1.0),
                    b: (accent_color.b * 0.3 + 0.24).min(1.0),
                    a: 0.26,
                },
            )
            .add_stop(1.0, Color::from_rgba(0.06, 0.08, 0.16, 0.22));

        container::Style {
            background: Some(Background::from(gradient)),
            border: Border::default()
                .rounded(Radius::from(14.0))
                .width(1.2)
                .color(Color {
                    a: 0.40,
                    ..accent_color
                }),
            text_color: Some(TEXT_MAIN),
            ..container::Style::default()
        }
    }
}

pub fn terminal_container_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(1.571))
        .add_stop(0.0, Color::from_rgba(0.03, 0.04, 0.08, 0.60))
        .add_stop(1.0, Color::from_rgba(0.01, 0.02, 0.05, 0.75));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::from(10.0))
            .width(1.0)
            .color(Color::from_rgba(0.65, 0.89, 0.63, 0.25)),
        text_color: Some(ACCENT_GREEN),
        ..container::Style::default()
    }
}

// --- Button Styles ---
pub fn nav_button_style(is_active: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        if is_active {
            let gradient = Linear::new(Radians(0.0)) // left to right
                .add_stop(0.0, Color::from_rgba(0.54, 0.71, 0.98, 0.35))
                .add_stop(1.0, Color::from_rgba(0.80, 0.65, 0.97, 0.15));

            button::Style {
                background: Some(Background::from(gradient)),
                text_color: Color::WHITE,
                border: Border::default()
                    .rounded(Radius::from(10.0))
                    .width(1.2)
                    .color(Color::from_rgba(0.54, 0.71, 0.98, 0.50)),
                ..button::Style::default()
            }
        } else {
            let (bg, border_color, text_color) = match status {
                button::Status::Hovered => (
                    Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.08))),
                    Color::from_rgba(1.0, 1.0, 1.0, 0.14),
                    TEXT_MAIN,
                ),
                button::Status::Pressed => (
                    Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.14))),
                    Color::from_rgba(1.0, 1.0, 1.0, 0.20),
                    TEXT_MAIN,
                ),
                _ => (None, Color::TRANSPARENT, TEXT_SUB),
            };

            button::Style {
                background: bg,
                text_color,
                border: Border::default()
                    .rounded(Radius::from(10.0))
                    .width(1.0)
                    .color(border_color),
                ..button::Style::default()
            }
        }
    }
}

pub fn primary_button_style(accent: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (alpha, border_alpha, text_col) = match status {
            button::Status::Hovered => (0.40, 0.65, TEXT_MAIN),
            button::Status::Pressed => (0.55, 0.80, TEXT_MAIN),
            button::Status::Disabled => (0.08, 0.12, TEXT_MUTED),
            _ => (0.24, 0.45, accent),
        };

        let gradient = Linear::new(Radians(0.785))
            .add_stop(0.0, Color { a: alpha, ..accent })
            .add_stop(1.0, Color { a: alpha * 0.6, ..accent });

        button::Style {
            background: Some(Background::from(gradient)),
            text_color: text_col,
            border: Border::default()
                .rounded(Radius::from(10.0))
                .width(1.0)
                .color(Color {
                    a: border_alpha,
                    ..accent
                }),
            ..button::Style::default()
        }
    }
}

pub fn glass_action_button(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (gradient, text, border) = match status {
        button::Status::Hovered => {
            let g = Linear::new(Radians(0.785))
                .add_stop(0.0, Color::from_rgba(1.0, 1.0, 1.0, 0.16))
                .add_stop(1.0, Color::from_rgba(1.0, 1.0, 1.0, 0.08));
            (g, TEXT_MAIN, Color::from_rgba(1.0, 1.0, 1.0, 0.22))
        }
        button::Status::Pressed => {
            let g = Linear::new(Radians(0.785))
                .add_stop(0.0, Color::from_rgba(1.0, 1.0, 1.0, 0.22))
                .add_stop(1.0, Color::from_rgba(1.0, 1.0, 1.0, 0.12));
            (g, TEXT_MAIN, Color::from_rgba(1.0, 1.0, 1.0, 0.30))
        }
        _ => {
            let g = Linear::new(Radians(0.785))
                .add_stop(0.0, Color::from_rgba(1.0, 1.0, 1.0, 0.09))
                .add_stop(1.0, Color::from_rgba(1.0, 1.0, 1.0, 0.03));
            (g, TEXT_SUB, Color::from_rgba(1.0, 1.0, 1.0, 0.12))
        }
    };

    button::Style {
        background: Some(Background::from(gradient)),
        text_color: text,
        border: Border::default()
            .rounded(Radius::from(10.0))
            .width(1.0)
            .color(border),
        ..button::Style::default()
    }
}

pub fn window_control_btn(color: Color, is_close: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border, icon_color) = match status {
            button::Status::Hovered => {
                if is_close {
                    (
                        Some(Background::Color(Color::from_rgba(0.95, 0.35, 0.45, 0.40))),
                        Color::from_rgba(0.95, 0.55, 0.66, 0.65),
                        Color::WHITE,
                    )
                } else {
                    (
                        Some(Background::Color(Color { a: 0.35, ..color })),
                        Color { a: 0.65, ..color },
                        Color::WHITE,
                    )
                }
            }
            button::Status::Pressed => (
                Some(Background::Color(Color { a: 0.50, ..color })),
                Color { a: 0.80, ..color },
                Color::WHITE,
            ),
            _ => (
                Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))),
                Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                TEXT_SUB,
            ),
        };

        button::Style {
            background: bg,
            text_color: icon_color,
            border: Border::default()
                .rounded(Radius::from(6.0))
                .width(1.0)
                .color(border),
            ..button::Style::default()
        }
    }
}

// --- Progress Bar Style (Gradient Glow Bar) ---
pub fn glass_progress_bar_style(accent: Color) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    move |_theme| {
        let bar_gradient = Linear::new(Radians(0.0)) // left to right glow
            .add_stop(0.0, accent)
            .add_stop(
                1.0,
                Color {
                    r: (accent.r * 0.7 + 0.3).min(1.0),
                    g: (accent.g * 0.7 + 0.3).min(1.0),
                    b: (accent.b * 0.7 + 0.3).min(1.0),
                    a: 1.0,
                },
            );

        progress_bar::Style {
            background: Background::Color(Color::from_rgba(0.04, 0.05, 0.10, 0.45)),
            bar: Background::from(bar_gradient),
            border: Border::default().rounded(Radius::from(6.0)),
        }
    }
}

// --- Text Input Style ---
pub fn glass_input_style(_theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let border_color = match status {
        text_input::Status::Focused { .. } => ACCENT_CYAN,
        text_input::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.25),
        _ => Color::from_rgba(1.0, 1.0, 1.0, 0.12),
    };

    text_input::Style {
        background: Background::Color(Color::from_rgba(0.06, 0.08, 0.14, 0.40)),
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

// --- Glassmorphic Scrollbar Style (Floating Frosted Capsule) ---
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
