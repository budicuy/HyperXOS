use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};
use crate::core::disk::DiskInfo;
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    disks: &'a [DiskInfo],
    selected_disk_path: Option<&str>,
    on_select_disk: impl Fn(String) -> Message + 'a,
    on_refresh_disks: Message,
    on_back: Message,
    on_next: Message,
) -> Element<'a, Message> {
    let mut disk_list = column![].spacing(8);

    if disks.is_empty() {
        disk_list = disk_list.push(
            container(
                column![
                    svg_icon(icons::ALERT_TRIANGLE, 32.0, ACCENT_AMBER),
                    text("No storage drives detected")
                        .size(14)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(TEXT_MAIN),
                        }),
                    text("Please ensure a hard drive or SSD is attached and click Refresh.")
                        .size(12)
                        .font(FONT_BODY)
                        .style(|_| text::Style {
                            color: Some(TEXT_MUTED),
                        }),
                ]
                .spacing(8)
                .align_x(Alignment::Center),
            )
            .padding(30)
            .width(Length::Fill)
            .style(glass_card_style),
        );
    } else {
        for disk in disks {
            let is_sel = selected_disk_path == Some(disk.path.as_str());
            let path_clone = disk.path.clone();

            let card = button(
                row![
                    svg_icon(icons::HARD_DRIVE, 24.0, if is_sel { ACCENT_CYAN } else { TEXT_MUTED }),
                    column![
                        row![
                            text(&disk.model)
                                .size(14)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(move |_| text::Style {
                                    color: Some(if is_sel { Color::WHITE } else { TEXT_MAIN }),
                                }),
                            container(
                                text(&disk.transport)
                                    .size(9)
                                    .font(FONT_DISPLAY_BOLD)
                                    .style(move |_| text::Style {
                                        color: Some(ACCENT_AMBER),
                                    }),
                            )
                            .padding([2, 6])
                            .style(glass_card_accent_style(ACCENT_AMBER)),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                        row![
                            text(&disk.path).size(12).font(FONT_BODY).style(|_| text::Style {
                                color: Some(ACCENT_CYAN),
                            }),
                            text("•").size(12).style(|_| text::Style {
                                color: Some(TEXT_MUTED),
                            }),
                            text(format!("{:.1} GB", disk.size_gb)).size(12).font(FONT_BODY_SEMIBOLD).style(|_| text::Style {
                                color: Some(TEXT_SUB),
                            }),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    if is_sel {
                        svg_icon(icons::CHECK_CIRCLE, 18.0, ACCENT_CYAN)
                    } else {
                        svg_icon(icons::MINUS, 0.0, Color::TRANSPARENT)
                    },
                ]
                .spacing(14)
                .align_y(Alignment::Center),
            )
            .padding([12, 16])
            .width(Length::Fill)
            .style(move |_, _| {
                let style = selection_card_style(is_sel)(&iced::Theme::Dark);
                button::Style {
                    background: style.background,
                    border: style.border,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            })
            .on_press(on_select_disk(path_clone));

            disk_list = disk_list.push(card);
        }
    }

    let scheme_info = container(
        column![
            row![
                svg_icon(icons::SHIELD_CHECK, 16.0, ACCENT_GREEN),
                text("AUTOMATIC STORAGE ARCHITECTURE")
                    .size(11)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_GREEN),
                    }),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            column![
                text("• Boot Partition: 4096 MiB (4 GB) FAT32 mounted at /boot (Limine EFI binaries & kernels)").size(11).font(FONT_BODY).style(|_| text::Style { color: Some(TEXT_SUB) }),
                text("• Root Partition: Btrfs spanning remainder of drive with '@' (root) and '@home' (user files) subvolumes").size(11).font(FONT_BODY).style(|_| text::Style { color: Some(TEXT_SUB) }),
                text("• Bootloader: Limine modern bootloader registered to UEFI NVRAM").size(11).font(FONT_BODY).style(|_| text::Style { color: Some(TEXT_SUB) }),
            ]
            .spacing(3),
        ]
        .spacing(6),
    )
    .padding(14)
    .width(Length::Fill)
    .style(glass_card_style);

    let back_btn = button(text("Back").size(12).font(FONT_BODY_MEDIUM))
        .on_press(on_back)
        .style(secondary_button_style())
        .padding([8, 16]);

    let refresh_btn = button(
        row![
            svg_icon(icons::REFRESH, 12.0, Color::WHITE),
            text("Scan Drives").size(12).font(FONT_BODY_MEDIUM),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .on_press(on_refresh_disks)
    .style(secondary_button_style())
    .padding([8, 14]);

    let mut next_btn = button(
        row![
            text("Next: User Accounts")
                .size(13)
                .font(FONT_BODY_SEMIBOLD),
            svg_icon(icons::ROCKET, 13.0, Color::WHITE),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .style(primary_button_style(ACCENT_CYAN))
    .padding([9, 20]);

    if selected_disk_path.is_some() {
        next_btn = next_btn.on_press(on_next);
    }

    column![
        column![
            text("Target Storage Drive")
                .size(20)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN)
                }),
            text("Select the drive to format and install HyperXOS Linux")
                .size(12)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED)
                }),
        ]
        .spacing(4),
        column![
            row![
                text("DETECTED DRIVES")
                    .size(11)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_CYAN)
                    }),
                row![].width(Length::Fill),
                refresh_btn,
            ]
            .align_y(Alignment::Center),
            scrollable(disk_list)
                .height(Length::Fixed(240.0))
                .style(glass_scrollable_style),
        ]
        .spacing(8),
        scheme_info,
        column![].height(Length::Fill),
        row![
            back_btn,
            row![].width(Length::Fill),
            next_btn,
        ]
        .align_y(Alignment::Center),
    ]
    .spacing(14)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
