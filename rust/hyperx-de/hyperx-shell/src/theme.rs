use iced::border::Radius;
use iced::widget::container;
use iced::{color, Border, Color, Shadow, Theme, Vector};

// HyperX Glassmorphism Palette (macOS Sonoma / Sequoia Dark Glass)
pub const BG_TOPBAR: Color = Color {
    r: 0.08,
    g: 0.09,
    b: 0.12,
    a: 0.88,
};

pub const BG_DOCK: Color = Color {
    r: 0.08,
    g: 0.09,
    b: 0.13,
    a: 0.82,
};

pub const BG_ISLAND: Color = Color {
    r: 0.02,
    g: 0.02,
    b: 0.03,
    a: 0.98,
};

pub const BG_PANEL: Color = Color {
    r: 0.10,
    g: 0.11,
    b: 0.16,
    a: 0.92,
};

pub const BG_CARD: Color = Color {
    r: 0.14,
    g: 0.15,
    b: 0.22,
    a: 0.85,
};

pub const BG_CARD_ACTIVE: Color = Color {
    r: 0.22,
    g: 0.45,
    b: 0.95,
    a: 0.90,
};

pub const BORDER_GLASS: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.14,
};

pub const BORDER_ISLAND: Color = Color {
    r: 0.25,
    g: 0.25,
    b: 0.32,
    a: 0.50,
};

pub const TEXT_PRIMARY: Color = Color {
    r: 0.95,
    g: 0.96,
    b: 0.98,
    a: 1.0,
};

pub const TEXT_SECONDARY: Color = Color {
    r: 0.65,
    g: 0.68,
    b: 0.75,
    a: 1.0,
};

pub const ACCENT_BLUE: Color = color!(0x38, 0xbd, 0xf8);
pub const ACCENT_PURPLE: Color = color!(0xa8, 0x55, 0xf7);
pub const ACCENT_GREEN: Color = color!(0x10, 0xb9, 0x81);
pub const ACCENT_AMBER: Color = color!(0xf5, 0x9e, 0x0b);

pub fn top_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_TOPBAR.into()),
        border: Border {
            color: BORDER_GLASS,
            width: 1.0,
            radius: Radius::default(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

pub fn dock_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_DOCK.into()),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.18),
            width: 1.0,
            radius: 26.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.55),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

pub fn dynamic_island_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_ISLAND.into()),
        border: Border {
            color: BORDER_ISLAND,
            width: 1.0,
            radius: 20.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.70),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 14.0,
        },
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

pub fn control_center_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_PANEL.into()),
        border: Border {
            color: BORDER_GLASS,
            width: 1.0,
            radius: 18.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.65),
            offset: Vector::new(0.0, 10.0),
            blur_radius: 28.0,
        },
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
        shadow: Shadow::default(),
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}

pub fn card_active_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_CARD_ACTIVE.into()),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.25),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.2, 0.45, 0.95, 0.4),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 6.0,
        },
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn menu_popup_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_PANEL.into()),
        border: Border {
            color: BORDER_GLASS,
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.6),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 18.0,
        },
        text_color: Some(TEXT_PRIMARY),
        ..Default::default()
    }
}
