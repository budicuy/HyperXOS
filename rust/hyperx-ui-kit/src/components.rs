use iced::border::Radius;
use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow};

use crate::icons::{self, svg_icon};
use crate::theme::*;

// --- 1. Windows 11 Fluent Window Header Bar (Tabs) ---
pub fn fluent_window_header_bar<'a, Message: Clone + 'a>(
    _title: impl Into<String>,
    tab_name: impl Into<String>,
    icon_svg: &'static str,
    accent: Color,
    _on_minimize: Message,
    _on_maximize: Message,
    _on_close: Message,
) -> Element<'a, Message> {
    let tab_str = tab_name.into();

    // Active Fluent Tab
    let active_tab = container(
        row![
            svg_icon(icon_svg, 14.0, accent),
            text(tab_str)
                .size(12)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
        ]
        .spacing(7)
        .align_y(Alignment::Center),
    )
    .padding([6, 12])
    .style(|_| container::Style {
        background: Some(Background::Color(Color::WHITE)),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(Color::from_rgba(0.0, 0.0, 0.0, 0.10)),
        shadow: Shadow::default(),
        ..container::Style::default()
    });

    let add_tab_btn = button(svg_icon(icons::PLUS, 13.0, FLUENT_TEXT_DARK))
        .padding([6, 8])
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)))
                }
                _ => None,
            };
            button::Style {
                background: bg,
                border: Border::default().rounded(Radius::from(0.0)),
                ..button::Style::default()
            }
        });

    let left_tabs = row![active_tab, add_tab_btn]
        .spacing(4)
        .align_y(Alignment::Center);

    container(
        row![left_tabs, container("").width(Length::Fill)]
            .align_y(Alignment::Center)
            .padding([4, 8]),
    )
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.015))),
        border: Border::default()
            .width(1.0)
            .color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)),
        ..container::Style::default()
    })
    .into()
}

// --- 2. Windows 11 Fluent Sidebar Nav Item ---
pub fn fluent_sidebar_nav_item<'a, Message: Clone + 'a>(
    label: &'static str,
    icon_svg: &'static str,
    is_active: bool,
    icon_color: Color,
    on_press: Message,
) -> Element<'a, Message> {
    let (text_col, indicator_col) = if is_active {
        (FLUENT_PRIMARY, Some(FLUENT_PRIMARY))
    } else {
        (FLUENT_TEXT_DARK, None)
    };

    let indicator: Element<'a, Message> = if let Some(col) = indicator_col {
        container("")
            .width(Length::Fixed(3.0))
            .height(Length::Fixed(16.0))
            .style(move |_| container::Style {
                background: Some(Background::Color(col)),
                border: Border::default().rounded(Radius::from(0.0)),
                ..container::Style::default()
            })
            .into()
    } else {
        container("").width(Length::Fixed(3.0)).into()
    };

    let content = row![
        indicator,
        svg_icon(
            icon_svg,
            16.0,
            if is_active {
                FLUENT_PRIMARY
            } else {
                icon_color
            }
        ),
        text(label)
            .size(12)
            .font(if is_active {
                FONT_BODY_SEMIBOLD
            } else {
                FONT_BODY_MEDIUM
            })
            .style(move |_| text::Style {
                color: Some(text_col),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    button(content)
        .width(Length::Fill)
        .padding([7, 8])
        .style(move |_theme, status| {
            let bg = if is_active {
                Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.08)))
            } else {
                match status {
                    button::Status::Hovered => {
                        Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.04)))
                    }
                    button::Status::Pressed => {
                        Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.08)))
                    }
                    _ => None,
                }
            };
            button::Style {
                background: bg,
                border: Border::default().rounded(Radius::from(0.0)),
                ..button::Style::default()
            }
        })
        .on_press(on_press)
        .into()
}

// --- 3. Windows 11 Fluent Toggle Switch (Pill Track + Slider Thumb) ---
pub fn fluent_toggle_switch<'a, Message: Clone + 'a>(
    is_on: bool,
    on_toggle: Message,
) -> Element<'a, Message> {
    let (bg, border_col, thumb_align_right) = if is_on {
        (FLUENT_PRIMARY, FLUENT_PRIMARY, true)
    } else {
        (
            Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            Color::from_rgba(0.0, 0.0, 0.0, 0.32),
            false,
        )
    };

    let thumb = container("")
        .width(Length::Fixed(12.0))
        .height(Length::Fixed(12.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(if is_on {
                Color::WHITE
            } else {
                Color::from_rgb(0.35, 0.35, 0.35)
            })),
            border: Border::default().rounded(Radius::from(0.0)),
            ..container::Style::default()
        });

    let track_content = if thumb_align_right {
        row![container("").width(Length::Fill), thumb].align_y(Alignment::Center)
    } else {
        row![thumb, container("").width(Length::Fill)].align_y(Alignment::Center)
    };

    let track = container(track_content)
        .width(Length::Fixed(40.0))
        .height(Length::Fixed(20.0))
        .padding([2, 3])
        .align_y(Alignment::Center)
        .style(move |_| container::Style {
            background: Some(Background::Color(bg)),
            border: Border::default().rounded(Radius::from(0.0)).width(1.0).color(border_col),
            ..container::Style::default()
        });

    button(track)
        .padding(0)
        .style(|_theme, _status| button::Style {
            background: None,
            border: Border::default(),
            ..button::Style::default()
        })
        .on_press(on_toggle)
        .into()
}

// --- 4. Windows 11 Fluent Metric Card ---
pub fn fluent_metric_card<'a, Message: 'a>(
    title: &'static str,
    icon_svg: &'static str,
    value_text: impl Into<String>,
    progress_val: f32,
    subtext: impl Into<String>,
    extratext: Option<impl Into<String>>,
    accent_color: Color,
) -> Element<'a, Message> {
    let val_str = value_text.into();
    let sub_str = subtext.into();

    let header = row![
        row![
            svg_icon(icon_svg, 18.0, accent_color),
            text(title)
                .size(12)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        container("").width(Length::Fill),
        text(val_str)
            .size(20)
            .font(FONT_DISPLAY_BOLD)
            .style(move |_| text::Style {
                color: Some(FLUENT_PRIMARY),
            }),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let bar = progress_bar(0.0..=1.0, progress_val)
        .length(Length::Fill)
        .girth(5.0)
        .style(move |_| progress_bar::Style {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.08)),
            bar: Background::Color(accent_color),
            border: Border::default().rounded(Radius::from(0.0)),
        });

    let mut footer = row![text(sub_str).size(11).font(FONT_BODY_MEDIUM).style(|_| text::Style {
        color: Some(FLUENT_TEXT_MUTED),
    })]
    .spacing(8);

    if let Some(extra) = extratext {
        let extra_str = extra.into();
        footer = footer.push(container("").width(Length::Fill)).push(
            text(extra_str)
                .size(10)
                .font(FONT_BODY_SEMIBOLD)
                .style(move |_| text::Style {
                    color: Some(accent_color),
                }),
        );
    }

    container(column![header, bar, footer].spacing(10))
        .padding(14)
        .width(Length::Fill)
        .style(fluent_card_style)
        .into()
}

// --- 5. Sidebar Section Header & Brand Header ---
pub fn sidebar_section_header<'a, Message: 'a>(label: &'static str) -> Element<'a, Message> {
    container(
        text(label)
            .size(10)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(Color::from_rgb(0.55, 0.58, 0.64)),
            }),
    )
    .padding([6, 10])
    .into()
}

pub fn brand_sidebar_header<'a, Message: Clone + 'a>(
    title: &'static str,
    subtitle: &'static str,
) -> Element<'a, Message> {
    let logo_img = icons::logo_image(28.0);
    container(
        row![
            logo_img,
            column![
                text(title)
                    .size(13)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
                text(subtitle)
                    .size(10)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
            ]
            .spacing(1),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([4, 6])
    .into()
}

// --- 6. Status Pill ---
pub fn status_pill<'a, Message: 'a>(
    label: impl Into<String>,
    accent_color: Color,
    _pulse: bool,
) -> Element<'a, Message> {
    let dot = container("")
        .width(Length::Fixed(6.0))
        .height(Length::Fixed(6.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(accent_color)),
            border: Border::default().rounded(Radius::from(0.0)),
            ..container::Style::default()
        });

    container(
        row![
            dot,
            text(label.into())
                .size(10)
                .font(FONT_BODY_SEMIBOLD)
                .style(move |_| text::Style {
                    color: Some(accent_color),
                }),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
    )
    .padding([3, 8])
    .style(move |_| container::Style {
        background: Some(Background::Color(Color::from_rgba(
            accent_color.r,
            accent_color.g,
            accent_color.b,
            0.12,
        ))),
        border: Border::default()
            .rounded(Radius::from(0.0))
            .width(1.0)
            .color(Color::from_rgba(accent_color.r, accent_color.g, accent_color.b, 0.25)),
        ..container::Style::default()
    })
    .into()
}

pub use crate::theme::glass_scrollable;

// --- 8. Tools & Installer Fluent Headers ---
pub fn header_link_pill<'a, Message: Clone + 'a>(
    id: &'static str,
    label: &'static str,
    icon_svg: &'static str,
    accent: Color,
    _url: &'static str,
    hovered_btn: Option<&'static str>,
    _on_hover: &impl Fn(Option<&'static str>) -> Message,
    on_open_url: &impl Fn(&'static str) -> Message,
) -> Element<'a, Message> {
    let is_hovered = hovered_btn == Some(id);
    let bg = if is_hovered {
        Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.12)))
    } else {
        Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.04)))
    };

    button(
        row![
            svg_icon(icon_svg, 13.0, accent),
            text(label).size(11).font(FONT_BODY_SEMIBOLD).style(|_| text::Style {
                color: Some(FLUENT_TEXT_DARK),
            }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 10])
    .style(move |_theme, _status| button::Style {
        background: bg,
        border: Border::default().rounded(Radius::from(0.0)),
        ..button::Style::default()
    })
    .on_press(on_open_url(_url))
    .into()
}

pub fn tools_header<'a, Message: Clone + 'a>(
    hovered_btn: Option<&'static str>,
    on_hover: impl Fn(Option<&'static str>) -> Message + 'a,
    on_open_url: impl Fn(&'static str) -> Message + 'a,
    on_drag: Message,
) -> Element<'a, Message> {
    let brand = row![
        icons::logo_image(24.0),
        text("HyperX Tools")
            .size(13)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_DARK),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let links = row![
        header_link_pill(
            "github",
            "GitHub",
            icons::GITHUB,
            FLUENT_PRIMARY,
            "https://github.com/budicuy/HyperXOS",
            hovered_btn,
            &on_hover,
            &on_open_url,
        ),
        header_link_pill(
            "releases",
            "Releases",
            icons::TAG,
            ACCENT_CYAN,
            "https://github.com/budicuy/HyperXOS/releases",
            hovered_btn,
            &on_hover,
            &on_open_url,
        ),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let content = row![
        brand,
        container("").width(Length::Fixed(16.0)),
        links,
        iced::widget::mouse_area(container("").width(Length::Fill).height(32))
            .on_press(on_drag),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .padding([4, 8]);

    container(content)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.02))),
            border: Border::default().width(1.0).color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)),
            ..container::Style::default()
        })
        .into()
}

pub fn installer_header<'a, Message: Clone + 'a>(
    hovered_btn: Option<&'static str>,
    on_hover: impl Fn(Option<&'static str>) -> Message + 'a,
    on_open_url: impl Fn(&'static str) -> Message + 'a,
) -> Element<'a, Message> {
    let brand = row![
        icons::logo_image(24.0),
        text("HyperXOS Installer")
            .size(13)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_DARK),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let links = row![
        header_link_pill(
            "github",
            "GitHub",
            icons::GITHUB,
            FLUENT_PRIMARY,
            "https://github.com/budicuy/HyperXOS",
            hovered_btn,
            &on_hover,
            &on_open_url,
        ),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let content = row![
        brand,
        container("").width(Length::Fill),
        links,
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .padding([4, 8]);

    container(content)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.02))),
            border: Border::default().width(1.0).color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)),
            ..container::Style::default()
        })
        .into()
}

pub fn terminal_box<'a, Message: Clone + 'a>(
    log_text: impl Into<String>,
    on_clear: Message,
) -> Element<'a, Message> {
    let log_str = log_text.into();

    let title_bar = row![
        text("Terminal Eksekusi (Polkit / Bash)")
            .size(11)
            .font(FONT_BODY_SEMIBOLD)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_MUTED),
            }),
        container("").width(Length::Fill),
        button(text("Bersihkan").size(10).font(FONT_BODY_MEDIUM))
            .padding([2, 6])
            .style(fluent_ribbon_button_style)
            .on_press(on_clear),
    ]
    .align_y(Alignment::Center)
    .padding([6, 10]);

    let log_scroll = glass_scrollable(
        text(log_str)
            .size(11)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(Color::from_rgb8(16, 185, 129)),
            }),
    )
    .height(Length::Fill);

    let content = column![
        title_bar,
        container(log_scroll)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(20, 24, 33))),
                border: Border::default().rounded(Radius::from(0.0)),
                ..container::Style::default()
            }),
    ]
    .spacing(4);

    container(content)
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(fluent_card_style)
        .into()
}

// --- Backward Compatibility Redirects ---
pub fn metric_card<'a, Message: 'a>(
    title: &'static str,
    icon_svg: &'static str,
    value_text: impl Into<String>,
    progress_val: f32,
    subtext: impl Into<String>,
    extratext: Option<impl Into<String>>,
    accent_color: Color,
) -> Element<'a, Message> {
    fluent_metric_card(
        title,
        icon_svg,
        value_text,
        progress_val,
        subtext,
        extratext,
        accent_color,
    )
}

pub fn window_header_bar<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    _subtext: impl Into<String>,
    icon_svg: &'static str,
    accent: Color,
    on_minimize: Message,
    on_maximize: Message,
    on_close: Message,
) -> Element<'a, Message> {
    fluent_window_header_bar(
        "HyperX",
        title,
        icon_svg,
        accent,
        on_minimize,
        on_maximize,
        on_close,
    )
}

pub fn sidebar_nav_item<'a, Message: Clone + 'a>(
    label: &'static str,
    icon_svg: &'static str,
    is_active: bool,
    on_press: Message,
) -> Element<'a, Message> {
    fluent_sidebar_nav_item(label, icon_svg, is_active, FLUENT_PRIMARY, on_press)
}
