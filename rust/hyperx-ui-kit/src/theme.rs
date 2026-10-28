use iced::border::Radius;
use iced::font::{Family, Weight};
use iced::gradient::Linear;
use iced::widget::scrollable::{self, Rail, Scroller};
use iced::widget::{button, container, progress_bar, slider, text_input};
use iced::{Background, Border, Color, Font, Radians, Shadow, Vector};

// --- Embedded Font Bytes ---
pub const FONT_PLUS_JAKARTA_BYTES: &[u8] = include_bytes!("../assets/fonts/PlusJakartaSans.ttf");
pub const FONT_INTER_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter.ttf");

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
pub const ACCENT_ORANGE: Color = Color::from_rgb(0.98, 0.70, 0.53); // #fab387

pub const TEXT_MAIN: Color = Color::WHITE;
pub const TEXT_SUB: Color = Color::from_rgb(0.85, 0.88, 0.95);
pub const TEXT_MUTED: Color = Color::from_rgb(0.65, 0.70, 0.82);

pub const CARD_BG_BLACK: Color = Color::from_rgb(0.05, 0.06, 0.09); // Solid Deep Black (#0d0f17)
pub const ACTIVE_DARK_FG: Color = Color::from_rgb(0.06, 0.07, 0.10);

// --- Window & Container Glass Styles (Frosted Gradient Translucency) ---
pub fn glass_window_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(0.785)) // 45 degrees
        .add_stop(0.0, Color::from_rgba(0.06, 0.08, 0.14, 0.94))
        .add_stop(0.5, Color::from_rgba(0.04, 0.05, 0.10, 0.92))
        .add_stop(1.0, Color::from_rgba(0.07, 0.05, 0.13, 0.94));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::from(16.0))
            .width(1.4)
            .color(Color::from_rgba(0.54, 0.71, 0.98, 0.32)),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.65),
            offset: Vector::new(0.0, 12.0),
            blur_radius: 32.0,
        },
        ..container::Style::default()
    }
}

pub fn context_menu_glass_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(0.785))
        .add_stop(0.0, Color::from_rgba(0.06, 0.07, 0.12, 0.96))
        .add_stop(1.0, Color::from_rgba(0.08, 0.07, 0.15, 0.96));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::from(12.0))
            .width(1.2)
            .color(Color::from_rgba(0.54, 0.71, 0.98, 0.45)),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.72),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..container::Style::default()
    }
}

pub fn context_menu_item_style(is_danger: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let hover_bg = if is_danger {
            Color::from_rgba(0.95, 0.35, 0.45, 0.85)
        } else {
            Color::from_rgba(0.54, 0.71, 0.98, 0.25)
        };
        let (bg, txt) = match status {
            button::Status::Hovered | button::Status::Pressed => (
                Some(Background::Color(hover_bg)),
                Color::WHITE,
            ),
            _ => (
                None,
                if is_danger { ACCENT_RED } else { TEXT_MAIN },
            ),
        };
        button::Style {
            background: bg,
            text_color: txt,
            border: Border::default().rounded(Radius::from(7.0)),
            ..button::Style::default()
        }
    }
}

pub fn topbar_glass_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(1.57))
        .add_stop(0.0, Color::from_rgba(0.05, 0.06, 0.10, 0.88))
        .add_stop(1.0, Color::from_rgba(0.07, 0.08, 0.14, 0.88));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .width(1.0)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.14)),
        ..container::Style::default()
    }
}

pub fn dock_glass_style(_theme: &iced::Theme) -> container::Style {
    let gradient = Linear::new(Radians(0.785))
        .add_stop(0.0, Color::from_rgba(0.06, 0.08, 0.14, 0.86))
        .add_stop(0.5, Color::from_rgba(0.04, 0.05, 0.10, 0.82))
        .add_stop(1.0, Color::from_rgba(0.08, 0.06, 0.15, 0.86));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default()
            .rounded(Radius::from(22.0))
            .width(1.5)
            .color(Color::from_rgba(0.54, 0.71, 0.98, 0.35)),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.60),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..container::Style::default()
    }
}

pub fn popover_glass_style(accent: Color) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {
        let gradient = Linear::new(Radians(0.785))
            .add_stop(0.0, Color::from_rgba(0.05, 0.06, 0.11, 0.95))
            .add_stop(1.0, Color::from_rgba(0.07, 0.06, 0.14, 0.95));

        container::Style {
            background: Some(Background::from(gradient)),
            border: Border::default()
                .rounded(Radius::from(16.0))
                .width(1.5)
                .color(Color { a: 0.55, ..accent }),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.65),
                offset: Vector::new(0.0, 12.0),
                blur_radius: 30.0,
            },
            ..container::Style::default()
        }
    }
}

pub fn sidebar_container_style(_theme: &iced::Theme) -> container::Style {
    // Signature HyperX Red-to-Orange Vertical Gradient (matching hyper-x-tools & hyper-x-installer)
    let gradient = Linear::new(Radians(0.0))
        .add_stop(0.0, Color::from_rgba(0.85, 0.12, 0.20, 0.84))
        .add_stop(0.5, Color::from_rgba(0.90, 0.26, 0.12, 0.82))
        .add_stop(1.0, Color::from_rgba(0.96, 0.44, 0.08, 0.82));

    container::Style {
        background: Some(Background::from(gradient)),
        border: Border::default().rounded(Radius::default().left(16.0)),
        ..container::Style::default()
    }
}

pub fn stepper_bar_style(theme: &iced::Theme) -> container::Style {
    sidebar_container_style(theme)
}

pub fn header_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style::default()
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
    move |_theme| container::Style {
        background: Some(Background::Color(CARD_BG_BLACK)),
        border: Border::default()
            .rounded(Radius::from(14.0))
            .width(1.5)
            .color(accent_color),
        text_color: Some(Color::WHITE),
        ..container::Style::default()
    }
}

pub fn selection_card_style(is_selected: bool) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {
        let (border_col, bg) = if is_selected {
            (ACCENT_CYAN, Color::from_rgba(0.54, 0.71, 0.98, 0.14))
        } else {
            (Color::from_rgba(1.0, 1.0, 1.0, 0.14), CARD_BG_BLACK)
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

// --- Button Styles (Signature Dark-to-White Hover/Active Inversion) ---
pub fn nav_button_style(is_active: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        if is_active {
            button::Style {
                background: Some(Background::Color(Color::WHITE)),
                text_color: ACTIVE_DARK_FG,
                border: Border::default().rounded(Radius::from(9.0)),
                ..button::Style::default()
            }
        } else {
            let bg = match status {
                button::Status::Hovered => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.20))),
                button::Status::Pressed => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.32))),
                _ => None,
            };

            button::Style {
                background: bg,
                text_color: Color::WHITE,
                border: Border::default().rounded(Radius::from(9.0)),
                ..button::Style::default()
            }
        }
    }
}

pub fn active_toggle_button_style(is_active: bool, accent_color: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        if is_active {
            button::Style {
                background: Some(Background::Color(Color::WHITE)),
                text_color: ACTIVE_DARK_FG,
                border: Border::default()
                    .rounded(Radius::from(9.0))
                    .width(1.4)
                    .color(Color::WHITE),
                ..button::Style::default()
            }
        } else {
            let (bg, border_col) = match status {
                button::Status::Hovered => (Color::from_rgba(1.0, 1.0, 1.0, 0.16), Color::WHITE),
                button::Status::Pressed => (Color::from_rgba(1.0, 1.0, 1.0, 0.24), Color::WHITE),
                _ => (Color::from_rgb(0.06, 0.07, 0.11), accent_color),
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
}

pub fn primary_button_style(accent: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border_col, text_col) = match status {
            button::Status::Hovered => (Color::WHITE, Color::WHITE, ACTIVE_DARK_FG),
            button::Status::Pressed => (Color::from_rgb(0.90, 0.90, 0.92), Color::WHITE, ACTIVE_DARK_FG),
            button::Status::Disabled => (
                Color::from_rgb(0.05, 0.06, 0.09),
                Color::from_rgba(1.0, 1.0, 1.0, 0.10),
                TEXT_MUTED,
            ),
            _ => (Color::from_rgb(0.06, 0.08, 0.12), accent, Color::WHITE),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_col,
            border: Border::default()
                .rounded(Radius::from(8.0))
                .width(1.3)
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
            _ => (
                Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                Color::from_rgba(1.0, 1.0, 1.0, 0.18),
            ),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: Color::WHITE,
            border: Border::default()
                .rounded(Radius::from(8.0))
                .width(1.2)
                .color(border_col),
            ..button::Style::default()
        }
    }
}

pub fn danger_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_theme, status| {
        let (bg, border_col, text_col) = match status {
            button::Status::Hovered => (Color::WHITE, Color::WHITE, ACTIVE_DARK_FG),
            button::Status::Pressed => (Color::from_rgb(0.90, 0.90, 0.92), Color::WHITE, ACTIVE_DARK_FG),
            _ => (
                Color::from_rgb(0.12, 0.03, 0.04),
                ACCENT_RED,
                Color::WHITE,
            ),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_col,
            border: Border::default()
                .rounded(Radius::from(8.0))
                .width(1.4)
                .color(border_col),
            ..button::Style::default()
        }
    }
}

pub fn glass_action_button(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (bg, text, border) = match status {
        button::Status::Hovered => (Color::WHITE, ACTIVE_DARK_FG, Color::WHITE),
        button::Status::Pressed => (Color::from_rgb(0.90, 0.90, 0.92), ACTIVE_DARK_FG, Color::WHITE),
        _ => (
            Color::from_rgb(0.06, 0.08, 0.12),
            Color::WHITE,
            Color::from_rgba(1.0, 1.0, 1.0, 0.22),
        ),
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: Border::default()
            .rounded(Radius::from(8.0))
            .width(1.2)
            .color(border),
        ..button::Style::default()
    }
}

pub fn dock_item_button_style(is_focused: bool, accent: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border_col) = match status {
            button::Status::Hovered => (
                Color::from_rgba(1.0, 1.0, 1.0, 0.22),
                Color::WHITE,
            ),
            button::Status::Pressed => (
                Color::from_rgba(1.0, 1.0, 1.0, 0.34),
                Color::WHITE,
            ),
            _ if is_focused => (
                Color::from_rgba(0.54, 0.71, 0.98, 0.18),
                accent,
            ),
            _ => (
                Color::from_rgba(0.05, 0.06, 0.09, 0.75),
                Color::from_rgba(1.0, 1.0, 1.0, 0.14),
            ),
        };

        button::Style {
            background: Some(Background::Color(bg)),
            text_color: Color::WHITE,
            border: Border::default()
                .rounded(Radius::from(14.0))
                .width(if is_focused { 1.5 } else { 1.0 })
                .color(border_col),
            ..button::Style::default()
        }
    }
}

pub fn desktop_icon_button_style(is_dragging: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border_col) = match status {
            button::Status::Hovered | button::Status::Pressed => (
                Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.14))),
                Color::from_rgba(1.0, 1.0, 1.0, 0.28),
            ),
            _ if is_dragging => (
                Some(Background::Color(Color::from_rgba(0.54, 0.71, 0.98, 0.22))),
                ACCENT_CYAN,
            ),
            _ => (None, Color::TRANSPARENT),
        };

        button::Style {
            background: bg,
            text_color: Color::WHITE,
            border: Border::default()
                .rounded(Radius::from(12.0))
                .width(1.0)
                .color(border_col),
            ..button::Style::default()
        }
    }
}

pub fn window_control_btn(color: Color, is_close: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border, icon_color) = match status {
            button::Status::Hovered => {
                if is_close {
                    (
                        Some(Background::Color(Color::from_rgba(0.95, 0.35, 0.45, 0.45))),
                        Color::from_rgba(0.95, 0.55, 0.66, 0.75),
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
                Color::from_rgba(1.0, 1.0, 1.0, 0.14),
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

// --- Progress Bar & Slider Styles ---
pub fn glass_progress_bar_style(accent: Color) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    move |_theme| {
        let bar_gradient = Linear::new(Radians(0.0))
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
            background: Background::Color(Color::from_rgba(0.04, 0.05, 0.10, 0.60)),
            bar: Background::from(bar_gradient),
            border: Border::default().rounded(Radius::from(6.0)),
        }
    }
}

pub fn glass_slider_style(accent: Color) -> impl Fn(&iced::Theme, slider::Status) -> slider::Style {
    move |_theme, status| {
        let handle_col = match status {
            slider::Status::Hovered | slider::Status::Dragged => Color::WHITE,
            _ => accent,
        };
        slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(accent),
                    Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.14)),
                ),
                width: 6.0,
                border: Border::default().rounded(Radius::from(3.0)),
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 8.0 },
                background: Background::Color(handle_col),
                border_width: 1.5,
                border_color: Color::WHITE,
            },
        }
    }
}

// --- Text Input Style ---
pub fn glass_input_style(_theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let border_color = match status {
        text_input::Status::Focused { .. } => ACCENT_CYAN,
        text_input::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.30),
        _ => Color::from_rgba(1.0, 1.0, 1.0, 0.16),
    };

    text_input::Style {
        background: Background::Color(Color::from_rgba(0.04, 0.05, 0.09, 0.85)),
        border: Border::default()
            .rounded(Radius::from(8.0))
            .width(1.2)
            .color(border_color),
        icon: TEXT_MUTED,
        placeholder: TEXT_MUTED,
        value: TEXT_MAIN,
        selection: Color::from_rgba(0.54, 0.71, 0.98, 0.35),
    }
}

pub fn custom_text_input_style() -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    glass_input_style
}

pub fn custom_progress_bar_style(accent_color: Color) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    glass_progress_bar_style(accent_color)
}

// --- Glassmorphic Scrollbar Style ---
pub fn glass_scrollable_style(_theme: &iced::Theme, status: scrollable::Status) -> scrollable::Style {
    let scroller_color = match status {
        scrollable::Status::Hovered { .. } => Color::from_rgba(0.54, 0.71, 0.98, 0.55),
        scrollable::Status::Dragged { .. } => Color::from_rgba(0.54, 0.71, 0.98, 0.80),
        _ => Color::from_rgba(1.0, 1.0, 1.0, 0.22),
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
