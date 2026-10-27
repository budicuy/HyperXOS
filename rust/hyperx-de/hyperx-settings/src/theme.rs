use iced::border::Radius;
use iced::widget::container;
use iced::{color, Border, Color, Shadow, Theme, Vector};

pub const BG_WINDOW: Color = Color {
    r: 0.08,
    g: 0.09,
    b: 0.12,
    a: 0.98,
};

pub const BG_SIDEBAR: Color = Color {
    r: 0.06,
    g: 0.07,
    b: 0.10,
    a: 0.95,
};

pub const BG_CONTENT: Color = Color {
    r: 0.09,
    g: 0.10,
    b: 0.14,
    a: 0.98,
};

pub const BG_CARD: Color = Color {
    r: 0.13,
    g: 0.14,
    b: 0.20,
    a: 0.85,
};

pub const BG_ACTIVE: Color = Color {
    r: 0.15,
    g: 0.38,
    b: 0.92,
    a: 0.90,
};

pub const TEXT_PRIMARY: Color = Color {
    r: 0.95,
    g: 0.96,
    b: 0.98,
    a: 1.0,
};

pub const TEXT_SECONDARY: Color = Color {
    r: 0.60,
    g: 0.65,
    b: 0.75,
    a: 1.0,
};

pub const ACCENT_BLUE: Color = color!(0x38, 0xbd, 0xf8);
pub const ACCENT_PURPLE: Color = color!(0xa8, 0x55, 0xf7);
pub const ACCENT_GREEN: Color = color!(0x10, 0xb9, 0x81);
pub const ACCENT_AMBER: Color = color!(0xf5, 0x9e, 0x0b);
pub const ACCENT_RED: Color = color!(0xef, 0x44, 0x44);

pub fn sidebar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_SIDEBAR.into()),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            width: 1.0,
            radius: Radius::default(),
        },
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

pub fn content_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_CONTENT.into()),
        border: Border::default(),
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

pub fn card_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_CARD.into()),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 6.0,
        },
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

pub fn active_item_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_ACTIVE.into()),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn inactive_item_style(_theme: &Theme) -> container::Style {
    container::Style::default()
}
