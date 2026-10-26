use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};
use crate::core::configurator::UserConfig;
use crate::core::disk::DiskInfo;
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    selected_disk: Option<&DiskInfo>,
    cfg: &UserConfig,
    on_back: Message,
    on_start_install: Message,
) -> Element<'a, Message> {
    let disk_summary = if let Some(d) = selected_disk {
        format!("{} ({}, {:.1} GB, {})", d.path, d.model, d.size_gb, d.transport)
    } else {
        "No disk selected".to_string()
    };

    let summary_item = |label: &'static str, value: String, accent: iced::Color| {
        row![
            container(text(label).size(12).font(FONT_BODY_SEMIBOLD).style(|_| text::Style {
                color: Some(TEXT_MUTED),
            }))
            .width(Length::Fixed(180.0)),
            text(value).size(12).font(FONT_BODY_SEMIBOLD).style(move |_| text::Style {
                color: Some(accent),
            }),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
    };

    let summary_card = container(
        column![
            summary_item("TARGET STORAGE", disk_summary, ACCENT_CYAN),
            summary_item("BOOT PARTITION", "4096 MiB (4 GB) FAT32 mounted at /boot".to_string(), ACCENT_GREEN),
            summary_item("ROOT PARTITION", "Btrfs with @ (root) and @home (home) subvolumes".to_string(), ACCENT_GREEN),
            summary_item("BOOTLOADER", "Limine (UEFI NVRAM Registered)".to_string(), ACCENT_AMBER),
            summary_item("DESKTOP ENVIRONMENT", "KDE Plasma 6 (Wayland Native) + SDDM".to_string(), ACCENT_PURPLE),
            summary_item("REGION & KEYBOARD", format!("{}, {} (Keymap: {})", cfg.timezone, cfg.locale, cfg.keymap), TEXT_MAIN),
            summary_item("USER & HOSTNAME", format!("{} on {}", cfg.username, cfg.hostname), TEXT_MAIN),
        ]
        .spacing(12),
    )
    .padding(18)
    .width(Length::Fill)
    .style(glass_card_style);

    let warning_box = container(
        row![
            svg_icon(icons::ALERT_TRIANGLE, 30.0, ACCENT_RED),
            column![
                text("CRITICAL DISK WARNING")
                    .size(12)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_RED),
                    }),
                text("All existing partitions and data on the target storage drive will be completely formatted and destroyed. Ensure you have backed up any critical files before proceeding.")
                    .size(11)
                    .font(FONT_BODY)
                    .style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
            ]
            .spacing(4)
            .width(Length::Fill),
        ]
        .spacing(14)
        .align_y(Alignment::Center),
    )
    .padding(14)
    .width(Length::Fill)
    .style(glass_card_accent_style(ACCENT_RED));

    let back_btn = button(text("Back").size(12).font(FONT_BODY_MEDIUM))
        .on_press(on_back)
        .style(secondary_button_style())
        .padding([8, 16]);

    let install_btn = button(
        row![
            svg_icon(icons::ROCKET, 14.0, Color::WHITE),
            text("Install HyperXOS Now")
                .size(13)
                .font(FONT_BODY_BOLD),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .on_press(on_start_install)
    .style(danger_button_style())
    .padding([10, 24]);

    column![
        column![
            text("Installation Summary")
                .size(20)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN)
                }),
            text("Review your installation parameters carefully before writing changes to disk")
                .size(12)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED)
                }),
        ]
        .spacing(4),
        summary_card,
        warning_box,
        column![].height(Length::Fill),
        row![
            back_btn,
            row![].width(Length::Fill),
            install_btn,
        ]
        .align_y(Alignment::Center),
    ]
    .spacing(14)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
