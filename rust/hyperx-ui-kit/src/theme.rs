use iced::border::Radius;
use iced::font::{Family, Weight};
use iced::widget::scrollable::{self, Rail, Scroller};
use iced::widget::{button, container, progress_bar, text_input};
use iced::{Background, Border, Color, Font, Shadow};

// --- Embedded Font Bytes ---
pub const FONT_PLUS_JAKARTA_BYTES: &[u8] = include_bytes!("../assets/fonts/PlusJakartaSans.ttf");
pub const FONT_INTER_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter.ttf");

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

// --- Windows 11 Fluent Design System Tokens ---
pub const FLUENT_PRIMARY: Color = Color::from_rgb(0.0, 0.47, 0.83); // #0078D4
const FLUENT_PRIMARY_HOVER: Color = Color::from_rgb(0.06, 0.43, 0.75); // #106EBE
const FLUENT_PRIMARY_ACTIVE: Color = Color::from_rgb(0.0, 0.35, 0.62); // #005A9E
pub const FLUENT_TEXT_DARK: Color = Color::from_rgb(0.12, 0.12, 0.12); // #1E1E1E
pub const FLUENT_TEXT_MUTED: Color = Color::from_rgb(0.38, 0.38, 0.38); // #605E5C

const FLUENT_CARD: Color = Color::WHITE;
const FLUENT_MICA_BG: Color = Color::from_rgba(0.97, 0.98, 0.99, 0.96);
const FLUENT_BORDER: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.08);
const FLUENT_BORDER_DARK: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.10);

// --- Unified Semantic & Accent Tokens ---
pub const ACCENT_RED: Color = Color::from_rgb(0.85, 0.15, 0.15);
pub const ACCENT_GREEN: Color = Color::from_rgb(0.08, 0.62, 0.36);
pub const ACCENT_CYAN: Color = FLUENT_PRIMARY;
pub const ACCENT_AMBER: Color = FLUENT_PRIMARY;

pub const TEXT_MAIN: Color = Color::from_rgb(0.12, 0.12, 0.12);
pub const TEXT_SUB: Color = Color::from_rgb(0.32, 0.32, 0.32);
pub const TEXT_MUTED: Color = Color::from_rgb(0.30, 0.33, 0.38);

// --- Window, Card & Container Styles ---
pub fn glass_window_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(FLUENT_MICA_BG)),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(FLUENT_BORDER_DARK),
        shadow: Shadow::default(),
        ..container::Style::default()
    }
}

pub fn fluent_card_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(FLUENT_CARD)),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(FLUENT_BORDER),
        shadow: Shadow::default(),
        ..container::Style::default()
    }
}

pub fn glass_card_style(theme: &iced::Theme) -> container::Style {
    fluent_card_style(theme)
}

pub fn glass_card_accent_style(accent: Color) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(FLUENT_CARD)),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.5)
            .color(accent),
        shadow: Shadow::default(),
        ..container::Style::default()
    }
}

pub fn sidebar_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.02))),
        border: Border::default()
            .width(1.0)
            .color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)),
        ..container::Style::default()
    }
}

pub fn terminal_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb8(24, 28, 36))),
        border: Border::default().rounded(Radius::from(0.0)),
        ..container::Style::default()
    }
}

pub fn stepper_bar_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::WHITE)),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(FLUENT_BORDER),
        ..container::Style::default()
    }
}

pub fn selection_card_style(is_selected: bool) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| {
        let (border_col, bg_col) = if is_selected {
            (FLUENT_PRIMARY, Color::from_rgba(0.0, 0.47, 0.83, 0.10))
        } else {
            (FLUENT_BORDER, Color::WHITE)
        };
        container::Style {
            background: Some(Background::Color(bg_col)),
            border: Border::default()
                .rounded(Radius::from(0.0))
                .width(if is_selected { 1.5 } else { 1.0 })
                .color(border_col),
            ..container::Style::default()
        }
    }
}

// --- Input, Progress & Scrollable Styles ---
pub fn glass_input_style(_theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let (border_col, border_w) = match status {
        text_input::Status::Focused { .. } => (FLUENT_PRIMARY, 2.0),
        text_input::Status::Hovered => (Color::from_rgba(0.0, 0.0, 0.0, 0.20), 1.0),
        _ => (Color::from_rgba(0.0, 0.0, 0.0, 0.12), 1.0),
    };
    text_input::Style {
        background: Background::Color(Color::WHITE),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(border_w)
            .color(border_col),
        icon: FLUENT_TEXT_MUTED,
        placeholder: Color::from_rgb(0.55, 0.55, 0.55),
        value: FLUENT_TEXT_DARK,
        selection: Color::from_rgba(0.0, 0.47, 0.83, 0.25),
    }
}

pub fn custom_text_input_style() -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    glass_input_style
}

pub fn custom_progress_bar_style(accent: Color) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    move |_theme| progress_bar::Style {
        background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.08)),
        bar: Background::Color(accent),
        border: Border::default().rounded(Radius::from(0.0)),
    }
}

pub fn glass_scrollable_style(_theme: &iced::Theme, status: scrollable::Status) -> scrollable::Style {
    let scroller_color = match status {
        scrollable::Status::Hovered { .. } => Color::from_rgba(0.0, 0.47, 0.83, 0.45),
        scrollable::Status::Dragged { .. } => Color::from_rgba(0.0, 0.47, 0.83, 0.70),
        _ => Color::from_rgba(0.0, 0.0, 0.0, 0.22),
    };
    scrollable::Style {
        container: container::Style::default(),
        vertical_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                background: Background::Color(scroller_color),
                border: Border::default().rounded(Radius::from(0.0)),
            },
        },
        horizontal_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                background: Background::Color(scroller_color),
                border: Border::default().rounded(Radius::from(0.0)),
            },
        },
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
            shadow: Shadow::default(),
            icon: Color::TRANSPARENT,
        },
    }
}

pub fn glass_scrollable<'a, Message: 'a>(
    content: impl Into<iced::Element<'a, Message>>,
) -> iced::widget::Scrollable<'a, Message> {
    iced::widget::scrollable(content).style(glass_scrollable_style)
}

// --- Button Styles ---
pub fn fluent_ribbon_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.05))),
        button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.10))),
        _ => None,
    };
    button::Style {
        background: bg,
        border: Border::default().rounded(Radius::from(0.0)),
        ..button::Style::default()
    }
}

pub fn glass_action_button(theme: &iced::Theme, status: button::Status) -> button::Style {
    fluent_ribbon_button_style(theme, status)
}

fn fluent_quick_toggle_style(
    is_active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, text_color, border_col) = if is_active {
            match status {
                button::Status::Hovered => (
                    Background::Color(FLUENT_PRIMARY_HOVER),
                    Color::WHITE,
                    FLUENT_PRIMARY_HOVER,
                ),
                button::Status::Pressed => (
                    Background::Color(FLUENT_PRIMARY_ACTIVE),
                    Color::WHITE,
                    FLUENT_PRIMARY_ACTIVE,
                ),
                _ => (Background::Color(FLUENT_PRIMARY), Color::WHITE, FLUENT_PRIMARY),
            }
        } else {
            match status {
                button::Status::Hovered => (
                    Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.08)),
                    FLUENT_TEXT_DARK,
                    Color::from_rgba(0.0, 0.0, 0.0, 0.18),
                ),
                button::Status::Pressed => (
                    Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.12)),
                    FLUENT_TEXT_DARK,
                    Color::from_rgba(0.0, 0.0, 0.0, 0.24),
                ),
                _ => (
                    Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.04)),
                    FLUENT_TEXT_DARK,
                    Color::from_rgba(0.0, 0.0, 0.0, 0.14),
                ),
            }
        };
        button::Style {
            background: Some(bg),
            text_color,
            border: Border::default()
                .rounded(Radius::from(0.0))
                .width(1.0)
                .color(border_col),
            ..button::Style::default()
        }
    }
}

pub fn active_toggle_button_style(
    is_active: bool,
    _accent: Color,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    fluent_quick_toggle_style(is_active)
}

pub fn nav_button_style(is_active: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    fluent_quick_toggle_style(is_active)
}

pub fn primary_button_style(accent: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, text_col) = match status {
            button::Status::Hovered => (FLUENT_PRIMARY_HOVER, Color::WHITE),
            button::Status::Pressed => (FLUENT_PRIMARY_ACTIVE, Color::WHITE),
            _ => (accent, Color::WHITE),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: text_col,
            border: Border::default().rounded(Radius::from(0.0)),
            shadow: Shadow::default(),
            ..button::Style::default()
        }
    }
}

pub fn secondary_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    |_theme, status| {
        let bg = match status {
            button::Status::Hovered => Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06))),
            button::Status::Pressed => Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.10))),
            _ => Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.03))),
        };
        button::Style {
            background: bg,
            text_color: FLUENT_TEXT_DARK,
            border: Border::default()
                .rounded(Radius::from(0.0))
                .width(1.0)
                .color(FLUENT_BORDER),
            ..button::Style::default()
        }
    }
}

pub fn danger_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => Color::from_rgb8(196, 43, 28),
            _ => Color::from_rgb8(232, 17, 35),
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: Color::WHITE,
            border: Border::default().rounded(Radius::from(0.0)),
            ..button::Style::default()
        }
    }
}
