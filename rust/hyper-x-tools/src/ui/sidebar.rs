use iced::widget::{column, container, mouse_area, text};
use iced::{Alignment, Element, Length};

use hyperx_ui_kit::{
    brand_sidebar_header, glass_card_style, icons, sidebar_container_style, sidebar_nav_item,
    sidebar_section_header, FONT_BODY_MEDIUM, FONT_DISPLAY_BOLD, TEXT_MAIN, TEXT_MUTED,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    KernelManager,
    GameTweaks,
    Snapshots,
    Maintenance,
    PackageStore,
}

pub fn sidebar<'a, Message: Clone + 'a>(
    current_page: Page,
    on_select: impl Fn(Page) -> Message + 'a,
    on_drag: Message,
) -> Element<'a, Message> {
    let brand = mouse_area(brand_sidebar_header("HYPERX TOOLS", "Pusat Kendali Sistem"))
        .on_press(on_drag);

    let separator = container("")
        .width(Length::Fill)
        .height(1)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.08))),
            ..container::Style::default()
        });

    let nav_links = column![
        sidebar_section_header("SISTEM"),
        sidebar_nav_item(
            "Dashboard Metrik",
            icons::DASHBOARD,
            current_page == Page::Dashboard,
            on_select(Page::Dashboard),
        ),
        sidebar_section_header("PERFORMA & KERNEL"),
        sidebar_nav_item(
            "Kernel Manager",
            icons::KERNEL,
            current_page == Page::KernelManager,
            on_select(Page::KernelManager),
        ),
        sidebar_nav_item(
            "Tweak Gaming",
            icons::GAMING,
            current_page == Page::GameTweaks,
            on_select(Page::GameTweaks),
        ),
        sidebar_section_header("CADANGAN & PERAWATAN"),
        sidebar_nav_item(
            "Snapshot & Restore",
            icons::SNAPSHOT,
            current_page == Page::Snapshots,
            on_select(Page::Snapshots),
        ),
        sidebar_nav_item(
            "Perawatan Sistem",
            icons::MAINTENANCE,
            current_page == Page::Maintenance,
            on_select(Page::Maintenance),
        ),
        sidebar_section_header("PERANGKAT LUNAK"),
        sidebar_nav_item(
            "Software Store",
            icons::STORE,
            current_page == Page::PackageStore,
            on_select(Page::PackageStore),
        ),
    ]
    .spacing(6);

    let footer = container(
        column![
            text("HyperXOS Linux 2026.10")
                .size(11)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text("Rust + Iced Glass v2.2")
                .size(10)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED),
                }),
        ]
        .spacing(4)
        .align_x(Alignment::Center),
    )
    .center_x(Length::Fill)
    .padding([10, 12])
    .width(Length::Fill)
    .style(glass_card_style);

    let content = column![
        brand,
        separator,
        nav_links,
        container("").height(Length::Fill),
        footer,
    ]
    .spacing(14)
    .padding(16);

    container(content)
        .width(230)
        .height(Length::Fill)
        .style(sidebar_container_style)
        .into()
}
