use iced::widget::{button, container, mouse_area, row, text};
use iced::{Alignment, Element, Length};

use crate::ui::icons::{self, svg_icon};
use crate::ui::sidebar::Page;
use crate::ui::theme::*;

pub fn header<'a, Message: Clone + 'a>(
    current_page: Page,
    on_refresh: Message,
    on_drag: Message,
    on_minimize: Message,
    on_maximize: Message,
    on_close: Message,
) -> Element<'a, Message> {
    let page_title = match current_page {
        Page::Dashboard => "Dashboard & Monitoring Sistem",
        Page::KernelManager => "CachyOS Kernel Manager",
        Page::GameTweaks => "Tweak Gaming & Latensi",
        Page::Snapshots => "Snapshot & Pemulihan Btrfs",
        Page::Maintenance => "Pemeliharaan & Perawatan Sistem",
        Page::PackageStore => "HyperX Software Store",
    };

    let title_breadcrumb = row![
        text(page_title)
            .size(14)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    // Draggable area across the header
    let drag_area = mouse_area(
        container("")
            .width(Length::Fill)
            .height(28)
    )
    .on_press(on_drag);

    let refresh_btn = button(
        row![
            svg_icon(icons::REFRESH, 12.0, TEXT_MAIN),
            text("Segarkan")
                .size(11)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 11])
    .style(glass_action_button)
    .on_press(on_refresh);

    let divider = container("")
        .width(1)
        .height(16)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.12))),
            ..container::Style::default()
        });

    // Custom Window Controls (Minimize, Maximize, Close)
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
        title_breadcrumb,
        drag_area,
        refresh_btn,
        divider,
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
