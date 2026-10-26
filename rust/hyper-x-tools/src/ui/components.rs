use iced::border::Radius;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Alignment, Border, Color, Element, Length};

use crate::ui::icons::svg_icon;
use crate::ui::theme::*;

pub fn metric_card<'a, Message: 'a>(
    title: &'static str,
    icon_svg: &'static str,
    value_text: impl Into<String>,
    progress_val: f32, // 0.0 to 1.0
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
                .font(FONT_DISPLAY_SEMIBOLD)
                .style(move |_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        text(val_str)
            .size(22)
            .font(FONT_DISPLAY_BOLD)
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

    let mut footer = row![text(sub_str).size(11).style(|_| text::Style {
        color: Some(TEXT_SUB),
    })]
    .spacing(8);

    if let Some(extra) = extratext {
        footer = footer.push(text(extra.into()).size(11).style(move |_| text::Style {
            color: Some(ACCENT_PURPLE),
        }));
    }

    let card_content = column![header, bar, footer]
        .spacing(12)
        .padding(16);

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
            Color {
                a: 0.18,
                ..accent
            },
            Color {
                a: 0.40,
                ..accent
            },
            accent,
        )
    } else {
        (
            Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            Color::from_rgba(1.0, 1.0, 1.0, 0.12),
            TEXT_MUTED,
        )
    };

    let content = row![
        container("")
            .width(6)
            .height(6)
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(txt)),
                border: Border::default().rounded(Radius::from(3.0)),
                ..container::Style::default()
            }),
        text(lbl).size(11).font(FONT_BODY_MEDIUM).style(move |_| text::Style {
            color: Some(txt),
        }),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

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

pub fn terminal_box<'a, Message: Clone + 'a>(
    log_text: impl Into<String>,
    on_clear: Message,
) -> Element<'a, Message> {
    let log_str = log_text.into();

    let title_bar = row![
        row![
            // Mac/Linux style window dots
            container("").width(9).height(9).style(|_| container::Style {
                background: Some(iced::Background::Color(ACCENT_RED)),
                border: Border::default().rounded(Radius::from(4.5)),
                ..container::Style::default()
            }),
            container("").width(9).height(9).style(|_| container::Style {
                background: Some(iced::Background::Color(ACCENT_AMBER)),
                border: Border::default().rounded(Radius::from(4.5)),
                ..container::Style::default()
            }),
            container("").width(9).height(9).style(|_| container::Style {
                background: Some(iced::Background::Color(ACCENT_GREEN)),
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
        button(
            text("Bersihkan Log")
                .size(10)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                })
        )
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
                })
        )
        .padding(10)
    )
    .height(110);

    let content = column![title_bar, console].spacing(8).padding(12);

    container(content)
        .width(Length::Fill)
        .style(terminal_container_style)
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
