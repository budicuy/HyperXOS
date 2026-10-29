use iced::border::Radius;
use iced::font::{Family, Weight};
use iced::widget::scrollable::{self, Rail, Scroller};
use iced::widget::{button, container, progress_bar, slider, text_input};
use iced::{Background, Border, Color, Font, Shadow, Vector};

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

// --- Windows 11 Fluent Design System Tokens ---
pub const FLUENT_PRIMARY: Color = Color::from_rgb(0.0, 0.47, 0.83); // #0078D4
pub const FLUENT_PRIMARY_HOVER: Color = Color::from_rgb(0.06, 0.43, 0.75); // #106EBE
pub const FLUENT_PRIMARY_ACTIVE: Color = Color::from_rgb(0.0, 0.35, 0.62); // #005A9E
pub const FLUENT_TEXT_DARK: Color = Color::from_rgb(0.12, 0.12, 0.12); // #1E1E1E
pub const FLUENT_TEXT_MUTED: Color = Color::from_rgb(0.38, 0.38, 0.38); // #605E5C
pub const FLUENT_TEXT_DISABLED: Color = Color::from_rgb(0.63, 0.62, 0.61); // #A19F9D

pub const FLUENT_CANVAS: Color = Color::from_rgb(0.95, 0.95, 0.95); // #F3F3F3
pub const FLUENT_CARD: Color = Color::WHITE; // #FFFFFF
pub const FLUENT_MICA_BG: Color = Color::from_rgba(0.97, 0.98, 0.99, 0.96);
pub const FLUENT_ACRYLIC_BG: Color = Color::from_rgba(0.98, 0.98, 0.98, 0.92);
pub const FLUENT_DOCK_BG: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.82);
pub const FLUENT_PILL_BG: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.88);
pub const FLUENT_BORDER: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.08);
pub const FLUENT_BORDER_LIGHT: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.80);
pub const FLUENT_BORDER_DARK: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.10);

// --- Unified 2-3 Color System ---
// 1. Primary Brand Accent: HyperX Royal Blue (Used for all buttons, active states, progress, highlights)
// 2. Semantic Alerts: Red (Danger / Warning / Destructive Actions) & Green (Passed / Success)
// 3. Monochrome Base: White cards, light neutral canvas, dark high-contrast typography
pub const ACCENT_RED: Color = Color::from_rgb(0.85, 0.15, 0.15); // Crimson Red (Danger/Destructive)
pub const ACCENT_GREEN: Color = Color::from_rgb(0.08, 0.62, 0.36); // Emerald Green (Success/Passed)

// Consolidated Primary Accents (Replaced rainbow palette with unified brand blue)
pub const ACCENT_CYAN: Color = FLUENT_PRIMARY;
pub const ACCENT_PURPLE: Color = FLUENT_PRIMARY;
pub const ACCENT_TEAL: Color = FLUENT_PRIMARY;
pub const ACCENT_AMBER: Color = FLUENT_PRIMARY;
pub const ACCENT_ORANGE: Color = FLUENT_PRIMARY;

// Compatibility Tokens (Clean Light equivalents)
pub const TEXT_MAIN: Color = Color::from_rgb(0.12, 0.12, 0.12);
pub const TEXT_SUB: Color = Color::from_rgb(0.32, 0.32, 0.32);
pub const TEXT_MUTED: Color = Color::from_rgb(0.30, 0.33, 0.38);
pub const CARD_BG_BLACK: Color = Color::WHITE;
pub const ACTIVE_DARK_FG: Color = Color::from_rgb(0.12, 0.12, 0.12);

// --- 1. Windows 11 Fluent Window Style (Clean Rectilinear) ---
pub fn fluent_window_style(_theme: &iced::Theme) -> container::Style {
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

// --- 2. Windows 11 Fluent Card Style (Clean Rectilinear White Card) ---
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

pub fn fluent_card_accent_style(accent: Color) -> impl Fn(&iced::Theme) -> container::Style {
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

// --- 3. Windows 11 Acrylic Modal Style (Flyouts, Start Menu, Quick Settings) ---
pub fn fluent_acrylic_modal_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(FLUENT_ACRYLIC_BG)),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(FLUENT_BORDER_DARK),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.16),
            offset: Vector::new(0.0, 12.0),
            blur_radius: 32.0,
        },
        ..container::Style::default()
    }
}

// --- 4. Floating Taskbar Dock Style ---
pub fn fluent_dock_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(FLUENT_DOCK_BG)),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(FLUENT_BORDER_LIGHT),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 20.0,
        },
        ..container::Style::default()
    }
}

// --- 5. Status Island Pill Style ---
pub fn fluent_status_pill_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(FLUENT_PILL_BG)),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(FLUENT_BORDER_LIGHT),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 12.0,
        },
        ..container::Style::default()
    }
}

// --- 6. Desktop Icon Tile Style ---
pub fn fluent_desktop_icon_tile_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.88))),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.65)),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.14),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        ..container::Style::default()
    }
}

// --- 7. Fluent Input Style ---
pub fn fluent_input_style(_theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
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

// --- 8. Fluent Ribbon / Toolbar Button Style ---
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

pub fn fluent_button_style(
    accent: Color,
    filled: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border_color) = if filled {
            match status {
                button::Status::Hovered => (
                    Some(Background::Color(Color { a: 0.90, ..accent })),
                    accent,
                ),
                button::Status::Pressed => (
                    Some(Background::Color(Color { a: 0.80, ..accent })),
                    accent,
                ),
                _ => (Some(Background::Color(accent)), accent),
            }
        } else {
            match status {
                button::Status::Hovered => (
                    Some(Background::Color(Color { a: 0.08, ..accent })),
                    accent,
                ),
                button::Status::Pressed => (
                    Some(Background::Color(Color { a: 0.16, ..accent })),
                    accent,
                ),
                _ => (None, Color::from_rgba(0.0, 0.0, 0.0, 0.12)),
            }
        };
        button::Style {
            background: bg,
            border: Border::default()
                .rounded(Radius::from(0.0))
                .width(1.0)
                .color(border_color),
            ..button::Style::default()
        }
    }
}

// --- 9. Fluent Address Bar Style ---
pub fn fluent_address_bar_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.85))),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(Color::from_rgba(0.0, 0.0, 0.0, 0.10)),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.02),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
        ..container::Style::default()
    }
}

// --- 10. Fluent File Item Button Style ---
pub fn fluent_file_item_style(
    is_selected: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, border_col, border_w) = if is_selected {
            (
                Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.12))),
                Color::from_rgba(0.0, 0.47, 0.83, 0.45),
                1.0,
            )
        } else {
            match status {
                button::Status::Hovered => (
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.04))),
                    Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    1.0,
                ),
                button::Status::Pressed => (
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.08))),
                    Color::from_rgba(0.0, 0.0, 0.0, 0.14),
                    1.0,
                ),
                _ => (None, Color::TRANSPARENT, 0.0),
            }
        };
        button::Style {
            background: bg,
            border: Border::default()
                .rounded(Radius::from(0.0))
                .width(border_w)
                .color(border_col),
            ..button::Style::default()
        }
    }
}

// --- 11. Fluent Sidebar Container Style ---
pub fn fluent_sidebar_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.02))),
        border: Border::default()
            .width(1.0)
            .color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)),
        ..container::Style::default()
    }
}

// --- 12. Quick Toggle Pill Style (Windows 11 Action Center) ---
pub fn fluent_quick_toggle_style(
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

// --- 13. Fluent Slider Style ---
pub fn fluent_slider_style(_theme: &iced::Theme, status: slider::Status) -> slider::Style {
    let handle_col = match status {
        slider::Status::Hovered | slider::Status::Dragged => Color::from_rgb8(43, 136, 216),
        _ => FLUENT_PRIMARY,
    };
    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Background::Color(FLUENT_PRIMARY),
                Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.12)),
            ),
            width: 4.0,
            border: Border::default().rounded(Radius::from(0.0)),
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Background::Color(Color::WHITE),
            border_width: 2.0,
            border_color: handle_col,
        },
    }
}

// --- 14. Primary Solid Button Style ---
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

// --- 15. Subtle Secondary Button Style ---
pub fn subtle_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
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

// --- 16. Scrollable Style ---
pub fn glass_scrollable<'a, Message: 'a>(
    content: impl Into<iced::Element<'a, Message>>,
) -> iced::widget::Scrollable<'a, Message> {
    let scroller_color = Color::from_rgba(0.0, 0.0, 0.0, 0.22);
    iced::widget::scrollable(content).style(move |_theme, status| {
        let col = match status {
            scrollable::Status::Hovered { .. } => Color::from_rgba(0.0, 0.47, 0.83, 0.45),
            scrollable::Status::Dragged { .. } => Color::from_rgba(0.0, 0.47, 0.83, 0.70),
            _ => scroller_color,
        };
        scrollable::Style {
            container: container::Style::default(),
            vertical_rail: Rail {
                background: None,
                border: Border::default(),
                scroller: Scroller {
                    background: Background::Color(col),
                    border: Border::default().rounded(Radius::from(0.0)),
                },
            },
            horizontal_rail: Rail {
                background: None,
                border: Border::default(),
                scroller: Scroller {
                    background: Background::Color(col),
                    border: Border::default().rounded(Radius::from(0.0)),
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
    })
}

// --- 17. Dock Item Button Style ---
pub fn fluent_dock_item_button_style(
    is_focused: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let bg = if is_focused {
            Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.09)))
        } else {
            match status {
                button::Status::Hovered => {
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.05)))
                }
                button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.09)))
                }
                _ => None,
            }
        };
        button::Style {
            background: bg,
            border: Border::default().rounded(Radius::from(0.0)),
            ..button::Style::default()
        }
    }
}

// --- Compatibility Aliases (Map legacy dark names to modern light Fluent styles) ---
pub fn glass_window_style(theme: &iced::Theme) -> container::Style {
    fluent_window_style(theme)
}

pub fn glass_card_style(theme: &iced::Theme) -> container::Style {
    fluent_card_style(theme)
}

pub fn glass_card_accent_style(accent: Color) -> impl Fn(&iced::Theme) -> container::Style {
    fluent_card_accent_style(accent)
}

pub fn popover_glass_style(_accent: Color) -> impl Fn(&iced::Theme) -> container::Style {
    |_theme| fluent_acrylic_modal_style(_theme)
}

pub fn glass_input_style(theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    fluent_input_style(theme, status)
}

pub fn glass_action_button(theme: &iced::Theme, status: button::Status) -> button::Style {
    fluent_ribbon_button_style(theme, status)
}

pub fn glass_slider_style(accent: Color) -> impl Fn(&iced::Theme, slider::Status) -> slider::Style {
    move |_theme, status| {
        let handle_col = match status {
            slider::Status::Hovered | slider::Status::Dragged => Color::from_rgb8(43, 136, 216),
            _ => accent,
        };
        slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(accent),
                    Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.12)),
                ),
                width: 4.0,
                border: Border::default().rounded(Radius::from(0.0)),
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Rectangle { width: 8, border_radius: Radius::from(0.0) },
                background: Background::Color(Color::WHITE),
                border_width: 2.0,
                border_color: handle_col,
            },
        }
    }
}

pub fn context_menu_glass_style(_theme: &iced::Theme) -> container::Style {
    fluent_acrylic_modal_style(_theme)
}

pub fn context_menu_item_style(
    is_danger: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let (bg, text_col) = if is_danger {
            match status {
                button::Status::Hovered | button::Status::Pressed => (
                    Some(Background::Color(Color::from_rgba(0.91, 0.07, 0.14, 0.12))),
                    ACCENT_RED,
                ),
                _ => (None, ACCENT_RED),
            }
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => (
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06))),
                    FLUENT_TEXT_DARK,
                ),
                _ => (None, FLUENT_TEXT_DARK),
            }
        };
        button::Style {
            background: bg,
            text_color: text_col,
            border: Border::default().rounded(Radius::from(0.0)),
            ..button::Style::default()
        }
    }
}

pub fn sidebar_container_style(theme: &iced::Theme) -> container::Style {
    fluent_sidebar_container_style(theme)
}

pub fn active_toggle_button_style(
    is_active: bool,
    _accent: Color,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    fluent_quick_toggle_style(is_active)
}

pub fn glass_progress_bar_style(accent: Color) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    move |_theme| progress_bar::Style {
        background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.08)),
        bar: Background::Color(accent),
        border: Border::default().rounded(Radius::from(0.0)),
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
        border: Border::default().rounded(Radius::from(0.0)).width(1.0).color(FLUENT_BORDER),
        ..container::Style::default()
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

pub fn nav_button_style(is_active: bool) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    fluent_quick_toggle_style(is_active)
}

pub fn secondary_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    subtle_button_style
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

pub fn custom_text_input_style() -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    fluent_input_style
}

pub fn custom_progress_bar_style(accent: Color) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    glass_progress_bar_style(accent)
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
