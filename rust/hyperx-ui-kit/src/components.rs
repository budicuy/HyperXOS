use iced::border::Radius;
use iced::mouse;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{button, column, container, mouse_area, progress_bar, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use crate::icons::{self, svg_icon};
use crate::theme::*;

pub fn brand_sidebar_header<'a, Message: Clone + 'a>(
    title: &'static str,
    subtitle: &'static str,
) -> Element<'a, Message> {
    let logo_img = icons::logo_image(34.0);

    let brand_inner = row![
        logo_img,
        column![
            text(title)
                .size(13)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(Color::WHITE),
                }),
            text(subtitle)
                .size(10)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.95, 0.75, 0.60)),
                }),
        ]
        .spacing(1)
        .align_x(Alignment::Start),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    container(brand_inner)
        .padding([10, 12])
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(CARD_BG_BLACK)),
            border: Border::default()
                .rounded(Radius::from(10.0))
                .width(1.0)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.14)),
            ..container::Style::default()
        })
        .into()
}

pub fn sidebar_nav_item<'a, Message: Clone + 'a>(
    label: &'static str,
    icon_svg: &'static str,
    is_active: bool,
    on_press: Message,
) -> Element<'a, Message> {
    let (icon_col, text_col, font_weight) = if is_active {
        (ACTIVE_DARK_FG, ACTIVE_DARK_FG, FONT_DISPLAY_BOLD)
    } else {
        (Color::WHITE, Color::WHITE, FONT_BODY_SEMIBOLD)
    };

    let content = row![
        svg_icon(icon_svg, 15.0, icon_col),
        text(label)
            .size(12)
            .font(font_weight)
            .style(move |_| text::Style {
                color: Some(text_col),
            }),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    button(content)
        .width(Length::Fill)
        .padding([9, 12])
        .style(nav_button_style(is_active))
        .on_press(on_press)
        .into()
}

pub fn sidebar_section_header<'a, Message: 'a>(name: &'static str) -> Element<'a, Message> {
    text(name)
        .size(9)
        .font(FONT_DISPLAY_BOLD)
        .style(|_| text::Style {
            color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.75)),
        })
        .into()
}

pub fn window_header_bar<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    subtitle: impl Into<String>,
    icon_svg: &'static str,
    accent: Color,
    on_minimize: Message,
    on_maximize: Message,
    on_close: Message,
) -> Element<'a, Message> {
    let title_str = title.into();
    let sub_str = subtitle.into();

    // macOS-style Traffic Lights (Red = Close, Yellow = Minimize, Green = Maximize/Zoom)
    let tl_close = button(
        container(svg_icon(icons::CLOSE, 8.0, Color::from_rgb(0.18, 0.02, 0.04)))
            .width(Length::Fixed(14.0))
            .height(Length::Fixed(14.0))
            .center_x(Length::Fixed(14.0))
            .center_y(Length::Fixed(14.0)),
    )
    .width(Length::Fixed(14.0))
    .height(Length::Fixed(14.0))
    .padding(0)
    .style(|_theme, status| {
        let col = match status {
            button::Status::Hovered | button::Status::Pressed => Color::from_rgb(1.0, 0.38, 0.34),
            _ => Color::from_rgb(0.95, 0.37, 0.34),
        };
        button::Style {
            background: Some(iced::Background::Color(col)),
            border: Border::default()
                .rounded(Radius::from(7.0))
                .width(1.0)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.30)),
            ..button::Style::default()
        }
    })
    .on_press(on_close.clone());

    let tl_min = button(
        container(svg_icon(icons::MINUS, 8.0, Color::from_rgb(0.22, 0.14, 0.0)))
            .width(Length::Fixed(14.0))
            .height(Length::Fixed(14.0))
            .center_x(Length::Fixed(14.0))
            .center_y(Length::Fixed(14.0)),
    )
    .width(Length::Fixed(14.0))
    .height(Length::Fixed(14.0))
    .padding(0)
    .style(|_theme, status| {
        let col = match status {
            button::Status::Hovered | button::Status::Pressed => Color::from_rgb(1.0, 0.78, 0.18),
            _ => Color::from_rgb(0.98, 0.74, 0.18),
        };
        button::Style {
            background: Some(iced::Background::Color(col)),
            border: Border::default()
                .rounded(Radius::from(7.0))
                .width(1.0)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.30)),
            ..button::Style::default()
        }
    })
    .on_press(on_minimize.clone());

    let tl_max = button(
        container(svg_icon(
            icons::MAXIMIZE_ARROWS,
            8.0,
            Color::from_rgb(0.02, 0.20, 0.05),
        ))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
        .center_x(Length::Fixed(14.0))
        .center_y(Length::Fixed(14.0)),
    )
    .width(Length::Fixed(14.0))
    .height(Length::Fixed(14.0))
    .padding(0)
    .style(|_theme, status| {
        let col = match status {
            button::Status::Hovered | button::Status::Pressed => Color::from_rgb(0.22, 0.85, 0.32),
            _ => Color::from_rgb(0.16, 0.78, 0.25),
        };
        button::Style {
            background: Some(iced::Background::Color(col)),
            border: Border::default()
                .rounded(Radius::from(7.0))
                .width(1.0)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.30)),
            ..button::Style::default()
        }
    })
    .on_press(on_maximize.clone());

    let traffic_lights = row![tl_close, tl_min, tl_max]
        .spacing(7)
        .align_y(Alignment::Center);

    let left = row![
        traffic_lights,
        container("").width(Length::Fixed(6.0)),
        container(svg_icon(icon_svg, 15.0, accent))
            .padding(6)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(Color { a: 0.15, ..accent })),
                border: Border::default()
                    .rounded(Radius::from(8.0))
                    .width(1.0)
                    .color(Color { a: 0.40, ..accent }),
                ..container::Style::default()
            }),
        column![
            text(title_str)
                .size(13)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text(sub_str)
                .size(10)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED),
                }),
        ]
        .spacing(1),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let win_min = button(svg_icon(icons::MINUS, 11.0, ACCENT_AMBER))
        .padding([4, 7])
        .style(window_control_btn(ACCENT_AMBER, false))
        .on_press(on_minimize);

    let win_max = button(svg_icon(icons::SQUARE, 10.0, ACCENT_GREEN))
        .padding([4, 7])
        .style(window_control_btn(ACCENT_GREEN, false))
        .on_press(on_maximize);

    let win_close = button(svg_icon(icons::CLOSE, 11.0, ACCENT_RED))
        .padding([4, 7])
        .style(window_control_btn(ACCENT_RED, true))
        .on_press(on_close);

    let controls = row![win_min, win_max, win_close]
        .spacing(5)
        .align_y(Alignment::Center);

    container(
        row![left, container("").width(Length::Fill), controls]
            .align_y(Alignment::Center)
            .padding([9, 14]),
    )
    .width(Length::Fill)
    .style(|_| container::Style {
        border: Border {
            width: 1.0,
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            radius: Radius::default(),
        },
        ..container::Style::default()
    })
    .into()
}

pub fn metric_card<'a, Message: 'a>(
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
                .size(13)
                .font(FONT_DISPLAY_BOLD)
                .style(move |_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        container("").width(Length::Fill),
        text(val_str)
            .size(22)
            .font(FONT_DISPLAY_EXTRABOLD)
            .style(move |_| text::Style {
                color: Some(accent_color),
            }),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let bar = progress_bar(0.0..=1.0, progress_val)
        .length(Length::Fill)
        .girth(6.0)
        .style(glass_progress_bar_style(accent_color));

    let mut footer = row![text(sub_str).size(11).font(FONT_BODY_MEDIUM).style(|_| text::Style {
        color: Some(TEXT_SUB),
    })]
    .spacing(8);

    if let Some(extra) = extratext {
        footer = footer.push(text(extra.into()).size(11).font(FONT_BODY_SEMIBOLD).style(move |_| text::Style {
            color: Some(ACCENT_PURPLE),
        }));
    }

    let card_content = column![header, bar, footer].spacing(10).padding(14);

    container(card_content)
        .width(Length::Fill)
        .style(glass_card_accent_style(accent_color))
        .into()
}

pub fn status_pill<'a, Message: 'a>(
    label: impl Into<String>,
    accent: Color,
    is_active: bool,
) -> Element<'a, Message> {
    let lbl = label.into();

    let (bg, border_col, txt) = if is_active {
        (
            Color { a: 0.18, ..accent },
            Color { a: 0.45, ..accent },
            accent,
        )
    } else {
        (
            Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            Color::from_rgba(1.0, 1.0, 1.0, 0.12),
            TEXT_MUTED,
        )
    };

    let content = text(lbl)
        .size(11)
        .font(FONT_BODY_SEMIBOLD)
        .style(move |_| text::Style { color: Some(txt) });

    container(content)
        .padding([4, 10])
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(bg)),
            border: Border::default()
                .rounded(Radius::from(6.0))
                .width(1.0)
                .color(border_col),
            ..container::Style::default()
        })
        .into()
}

pub fn glass_scrollable<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Scrollable<'a, Message> {
    let scrollbar = Scrollbar::new()
        .width(5.0)
        .scroller_width(5.0)
        .margin(6.0);

    iced::widget::scrollable(content)
        .direction(Direction::Vertical(scrollbar))
        .style(glass_scrollable_style)
}

pub fn terminal_box<'a, Message: Clone + 'a>(
    log_text: impl Into<String>,
    on_clear: Message,
) -> Element<'a, Message> {
    let log_str = log_text.into();

    let title_bar = row![
        row![
            container("").width(9).height(9).style(|_| container::Style {
                background: Some(Background::Color(ACCENT_RED)),
                border: Border::default().rounded(Radius::from(4.5)),
                ..container::Style::default()
            }),
            container("").width(9).height(9).style(|_| container::Style {
                background: Some(Background::Color(ACCENT_AMBER)),
                border: Border::default().rounded(Radius::from(4.5)),
                ..container::Style::default()
            }),
            container("").width(9).height(9).style(|_| container::Style {
                background: Some(Background::Color(ACCENT_GREEN)),
                border: Border::default().rounded(Radius::from(4.5)),
                ..container::Style::default()
            }),
            text("  Terminal Eksekusi (Polkit / Bash)")
                .size(11)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
        button(text("Bersihkan Log").size(10).font(FONT_BODY_SEMIBOLD))
            .padding([3, 10])
            .style(glass_action_button)
            .on_press(on_clear),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let console = glass_scrollable(
        container(
            text(log_str)
                .size(11)
                .font(iced::Font::MONOSPACE)
                .style(|_| text::Style {
                    color: Some(ACCENT_GREEN),
                }),
        )
        .padding(10),
    )
    .height(110);

    let content = column![title_bar, console].spacing(8).padding(12);

    container(content)
        .width(Length::Fill)
        .style(terminal_container_style)
        .into()
}

fn header_link_pill<'a, Message: Clone + 'a>(
    id: &'static str,
    label: &'static str,
    icon_svg: &'static str,
    accent_color: Color,
    url: &'static str,
    hovered_btn: Option<&'static str>,
    on_hover: &impl Fn(Option<&'static str>) -> Message,
    on_open_url: &impl Fn(&'static str) -> Message,
) -> Element<'a, Message> {
    let is_h = hovered_btn == Some(id);
    let (bg, border, fg) = if is_h {
        (Color::WHITE, Color::WHITE, ACTIVE_DARK_FG)
    } else {
        (
            Color::from_rgb(0.06, 0.08, 0.12),
            Color::from_rgba(1.0, 1.0, 1.0, 0.22),
            Color::WHITE,
        )
    };
    let icon_c = if is_h { ACTIVE_DARK_FG } else { accent_color };

    let content = container(
        row![
            svg_icon(icon_svg, 12.5, icon_c),
            text(label)
                .size(11)
                .font(FONT_BODY_SEMIBOLD)
                .style(move |_| text::Style { color: Some(fg) }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 10])
    .style(move |_| container::Style {
        background: Some(Background::Color(bg)),
        border: Border::default()
            .rounded(Radius::from(8.0))
            .width(1.2)
            .color(border),
        ..container::Style::default()
    });

    mouse_area(content)
        .interaction(mouse::Interaction::Pointer)
        .on_enter(on_hover(Some(id)))
        .on_exit(on_hover(None))
        .on_press(on_open_url(url))
        .into()
}

pub fn tools_header<'a, Message: Clone + 'a>(
    hovered_btn: Option<&'static str>,
    on_hover: impl Fn(Option<&'static str>) -> Message + 'a,
    on_open_url: impl Fn(&'static str) -> Message + 'a,
    on_drag: Message,
    on_minimize: Message,
    on_maximize: Message,
    on_close: Message,
) -> Element<'a, Message> {
    let links_group = row![
        header_link_pill(
            "github",
            "GitHub",
            icons::GITHUB,
            Color::WHITE,
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
        header_link_pill(
            "issues",
            "Issues",
            icons::BUG,
            ACCENT_AMBER,
            "https://github.com/budicuy/HyperXOS/issues",
            hovered_btn,
            &on_hover,
            &on_open_url,
        ),
        header_link_pill(
            "docs",
            "Docs",
            icons::DOCS,
            ACCENT_GREEN,
            "https://github.com/budicuy/HyperXOS#readme",
            hovered_btn,
            &on_hover,
            &on_open_url,
        ),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let drag_area = mouse_area(container("").width(Length::Fill).height(28)).on_press(on_drag);

    let win_min = button(svg_icon(icons::MINUS, 12.0, ACCENT_AMBER))
        .padding([5, 8])
        .style(window_control_btn(ACCENT_AMBER, false))
        .on_press(on_minimize);

    let win_max = button(svg_icon(icons::SQUARE, 11.0, ACCENT_GREEN))
        .padding([5, 8])
        .style(window_control_btn(ACCENT_GREEN, false))
        .on_press(on_maximize);

    let win_close = button(svg_icon(icons::CLOSE, 12.0, ACCENT_RED))
        .padding([5, 8])
        .style(window_control_btn(ACCENT_RED, true))
        .on_press(on_close);

    let window_controls = row![win_min, win_max, win_close]
        .spacing(6)
        .align_y(Alignment::Center);

    let bar = row![links_group, drag_area, window_controls]
        .spacing(12)
        .align_y(Alignment::Center)
        .padding([10, 16]);

    container(bar)
        .width(Length::Fill)
        .style(header_container_style)
        .into()
}

pub fn installer_header<'a, Message: Clone + 'a>(
    hovered_btn: Option<&'static str>,
    on_hover: impl Fn(Option<&'static str>) -> Message + 'a,
    on_open_url: impl Fn(&'static str) -> Message + 'a,
    on_minimize: Message,
    on_close: Message,
) -> Element<'a, Message> {
    let brand = row![
        icons::logo_image(28.0),
        text("HyperXOS Installer")
            .size(14)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN),
            }),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let links = row![
        header_link_pill(
            "github",
            "GitHub",
            icons::GITHUB,
            Color::WHITE,
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
        header_link_pill(
            "issues",
            "Issues",
            icons::BUG,
            ACCENT_AMBER,
            "https://github.com/budicuy/HyperXOS/issues",
            hovered_btn,
            &on_hover,
            &on_open_url,
        ),
        header_link_pill(
            "docs",
            "Docs",
            icons::DOCS,
            ACCENT_TEAL,
            "https://github.com/budicuy/HyperXOS/wiki",
            hovered_btn,
            &on_hover,
            &on_open_url,
        ),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let win_controls = row![
        button(svg_icon(icons::MINUS, 12.0, TEXT_MUTED))
            .on_press(on_minimize)
            .style(secondary_button_style())
            .padding([5, 8]),
        button(svg_icon(icons::CLOSE, 12.0, ACCENT_RED))
            .on_press(on_close)
            .style(secondary_button_style())
            .padding([5, 8]),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    container(
        row![
            brand,
            row![].width(Length::Fill),
            links,
            row![].width(Length::Fixed(16.0)),
            win_controls,
        ]
        .align_y(Alignment::Center),
    )
    .padding([12, 20])
    .into()
}
