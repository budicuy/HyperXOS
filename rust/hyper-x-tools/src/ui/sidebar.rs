use iced::widget::{button, column, container, image, row, text};
use iced::{Alignment, Element, Length};

use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

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
) -> Element<'a, Message> {
    // 1. Brand Logo Header
    let logo_data: &'static [u8] = include_bytes!("../../assets/logo.png");
    let logo_img = image(image::Handle::from_bytes(logo_data))
        .width(36)
        .height(36);

    let brand = row![
        logo_img,
        column![
            text("HYPERX TOOLS")
                .size(13)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text("Pusat Kendali Sistem")
                .size(10)
                .style(|_| text::Style {
                    color: Some(ACCENT_CYAN),
                }),
        ]
        .spacing(1)
        .align_x(Alignment::Start),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let separator = container("")
        .width(Length::Fill)
        .height(1)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.08))),
            ..container::Style::default()
        });

    // Helper to build a styled navigation button with open-source SVG icons
    let nav_item = |page: Page, label: &'static str, icon_svg: &'static str| -> Element<'a, Message> {
        let is_active = current_page == page;
        let icon_col = if is_active { ACCENT_CYAN } else { TEXT_SUB };
        let content = row![
            svg_icon(icon_svg, 15.0, icon_col),
            text(label).size(12).style(move |_| text::Style {
                color: Some(if is_active { ACCENT_CYAN } else { TEXT_MAIN }),
            }),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        button(content)
            .width(Length::Fill)
            .padding([8, 12])
            .style(nav_button_style(is_active))
            .on_press(on_select(page))
            .into()
    };

    let section_header = |name: &'static str| -> Element<'a, Message> {
        text(name)
            .size(10)
            .style(|_| text::Style {
                color: Some(TEXT_MUTED),
            })
            .into()
    };

    // Navigation Groups
    let nav_links = column![
        // Group: SISTEM
        section_header("SISTEM"),
        nav_item(Page::Dashboard, "Dashboard Metrik", icons::DASHBOARD),
        // Group: PERFORMA & KERNEL
        section_header("PERFORMA & KERNEL"),
        nav_item(Page::KernelManager, "Kernel Manager", icons::KERNEL),
        nav_item(Page::GameTweaks, "Tweak Gaming", icons::GAMING),
        // Group: CADANGAN & PERAWATAN
        section_header("CADANGAN & PERAWATAN"),
        nav_item(Page::Snapshots, "Snapshot & Restore", icons::SNAPSHOT),
        nav_item(Page::Maintenance, "Perawatan Sistem", icons::MAINTENANCE),
        // Group: PERANGKAT LUNAK
        section_header("PERANGKAT LUNAK"),
        nav_item(Page::PackageStore, "Software Store", icons::STORE),
    ]
    .spacing(6);

    // Sidebar Footer
    let footer = container(
        column![
            text("HyperXOS Linux 2026.10")
                .size(10)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                }),
            text("Rust + Iced Glass v2.2")
                .size(9)
                .style(|_| text::Style {
                    color: Some(ACCENT_PURPLE),
                }),
        ]
        .spacing(2)
        .align_x(Alignment::Center),
    )
    .padding(10)
    .width(Length::Fill)
    .style(glass_card_style);

    let content = column![
        brand,
        separator,
        nav_links,
        container("").height(Length::Fill), // Push footer down
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
