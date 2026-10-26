use iced::widget::{button, container, mouse_area, row, text};
use iced::{Alignment, Color, Element, Length};

use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn header<'a, Message: Clone + 'a>(
    on_open_url: impl Fn(&'static str) -> Message + 'a,
    on_drag: Message,
    on_minimize: Message,
    on_maximize: Message,
    on_close: Message,
) -> Element<'a, Message> {
    // 1. External Link Buttons (GitHub, Releases, Issues, Docs)
    let github_btn = button(
        row![
            svg_icon(icons::GITHUB, 13.0, Color::WHITE),
            text("GitHub")
                .size(11)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 10])
    .style(glass_action_button)
    .on_press(on_open_url("https://github.com/budicuy/HyperXOS"));

    let releases_btn = button(
        row![
            svg_icon(icons::TAG, 12.0, ACCENT_CYAN),
            text("Releases")
                .size(11)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.85, 0.92, 1.0)),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 10])
    .style(glass_action_button)
    .on_press(on_open_url("https://github.com/budicuy/HyperXOS/releases"));

    let issues_btn = button(
        row![
            svg_icon(icons::BUG, 12.0, ACCENT_AMBER),
            text("Issues")
                .size(11)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.98, 0.90, 0.75)),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 10])
    .style(glass_action_button)
    .on_press(on_open_url("https://github.com/budicuy/HyperXOS/issues"));

    let docs_btn = button(
        row![
            svg_icon(icons::DOCS, 12.0, ACCENT_GREEN),
            text("Docs")
                .size(11)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.80, 0.95, 0.85)),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 10])
    .style(glass_action_button)
    .on_press(on_open_url("https://github.com/budicuy/HyperXOS#readme"));

    let links_group = row![github_btn, releases_btn, issues_btn, docs_btn]
        .spacing(8)
        .align_y(Alignment::Center);

    // 2. Draggable area across the header
    let drag_area = mouse_area(
        container("")
            .width(Length::Fill)
            .height(28),
    )
    .on_press(on_drag);

    // 3. Custom Window Controls (Minimize, Maximize, Close)
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

    let bar = row![
        links_group,
        drag_area,
        window_controls,
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .padding([10, 16]);

    container(bar)
        .width(Length::Fill)
        .style(header_container_style)
        .into()
}
