use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};

use crate::core::kernel_manager::KernelEntry;
use crate::ui::components::{glass_scrollable, status_pill, terminal_box};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    running_name: &'a str,
    running_ver: &'a str,
    kernels: &'a [KernelEntry],
    log_text: &'a str,
    on_install: impl Fn(String) -> Message + 'a,
    on_remove: impl Fn(String) -> Message + 'a,
    on_rebuild: Message,
    on_clear_log: Message,
) -> Element<'a, Message> {
    // 1. Header with Rebuild Initramfs button
    let top_bar = row![
        column![
            text("CachyOS Kernel Manager")
                .size(16)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text("Kelola kernel performa tinggi CachyOS dengan patch BORE, SCX, dan compiler v3.")
                .size(11)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                }),
        ]
        .spacing(2),
        container("").width(Length::Fill),
        button(
            row![
                svg_icon(icons::SLIDERS, 13.0, ACCENT_PURPLE),
                text("Rebuild Initramfs").size(11)
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding([6, 12])
        .style(primary_button_style(ACCENT_PURPLE))
        .on_press(on_rebuild),
    ]
    .align_y(Alignment::Center);

    // 2. Active Running Kernel Banner
    let running_banner = container(
        row![
            svg_icon(icons::KERNEL, 22.0, ACCENT_GREEN),
            column![
                text(format!("Kernel Aktif: {}", running_name))
                    .size(13)
                    .style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                text(format!("Versi OS Release: {}", running_ver))
                    .size(10)
                    .style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
            ]
            .spacing(2),
            container("").width(Length::Fill),
            status_pill("Aktif Berjalan", ACCENT_GREEN, true),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .padding(14),
    )
    .width(Length::Fill)
    .style(glass_card_accent_style(ACCENT_GREEN));

    // 3. Catalog List
    let catalog_title = text("Katalog Kernel Performa & Komunitas:")
        .size(13)
        .style(|_| text::Style {
            color: Some(TEXT_MAIN),
        });

    let mut kernel_items = column![].spacing(10);

    for k in kernels {
        let pkg = k.package_name.clone();

        let badges = row![
            if k.is_running {
                status_pill("Running", ACCENT_GREEN, true)
            } else if k.installed {
                status_pill("Terpasang", ACCENT_CYAN, true)
            } else {
                status_pill("Tersedia", TEXT_MUTED, false)
            }
        ];

        let mut desc_col = column![
            row![
                text(&k.name).size(13).style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
                badges,
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(&k.description).size(11).style(|_| text::Style {
                color: Some(TEXT_SUB),
            }),
        ]
        .spacing(4);

        if k.installed && !k.installed_version.is_empty() {
            desc_col = desc_col.push(
                text(format!("Versi Terpasang: {}", k.installed_version))
                    .size(10)
                    .style(|_| text::Style {
                        color: Some(ACCENT_PURPLE),
                    }),
            );
        }

        let action_btn = if !k.installed {
            button(
                row![
                    svg_icon(icons::DOWNLOAD, 12.0, ACCENT_CYAN),
                    text("Pasang (+Headers)").size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([6, 12])
            .style(primary_button_style(ACCENT_CYAN))
            .on_press(on_install(pkg))
        } else if !k.is_running {
            button(
                row![
                    svg_icon(icons::TRASH, 12.0, ACCENT_RED),
                    text("Hapus").size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([6, 12])
            .style(primary_button_style(ACCENT_RED))
            .on_press(on_remove(pkg))
        } else {
            button(
                row![
                    svg_icon(icons::CHECK, 12.0, ACCENT_GREEN),
                    text("Kernel Aktif").size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([6, 12])
            .style(nav_button_style(false))
        };

        let card = container(
            row![desc_col, container("").width(Length::Fill), action_btn]
                .spacing(12)
                .align_y(Alignment::Center)
                .padding(14),
        )
        .width(Length::Fill)
        .style(glass_card_style);

        kernel_items = kernel_items.push(card);
    }

    // 4. Terminal Console
    let terminal = terminal_box(log_text, on_clear_log);

    let content = column![top_bar, running_banner, catalog_title, kernel_items, terminal]
        .spacing(16)
        .padding(20);

    glass_scrollable(content).height(Length::Fill).into()
}
