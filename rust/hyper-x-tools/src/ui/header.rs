use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};

use crate::ui::icons::{self, svg_icon};
use crate::ui::sidebar::Page;
use crate::ui::theme::*;

pub fn header<'a, Message: Clone + 'a>(
    current_page: Page,
    on_refresh: Message,
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
            .size(15)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let refresh_btn = button(
        row![
            svg_icon(icons::REFRESH, 13.0, TEXT_MAIN),
            text("Segarkan")
                .size(11)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([5, 12])
    .style(glass_action_button)
    .on_press(on_refresh);

    let bar = row![
        title_breadcrumb,
        container("").width(Length::Fill),
        refresh_btn,
    ]
    .align_y(Alignment::Center)
    .padding([12, 20]);

    container(bar)
        .width(Length::Fill)
        .style(header_container_style)
        .into()
}
