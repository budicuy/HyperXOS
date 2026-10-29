use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Color, Element, Length};

use crate::core::snapshot_manager::SnapshotRecord;
use crate::ui::components::{glass_scrollable, status_pill, terminal_box};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    is_btrfs: bool,
    fs_type_name: &'a str,
    snapshot_input: &'a str,
    snapshots: &'a [SnapshotRecord],
    log_text: &'a str,
    on_input_change: impl Fn(String) -> Message + 'a,
    on_create: Message,
    on_restore: impl Fn(String) -> Message + 'a,
    on_delete: impl Fn(String) -> Message + 'a,
    on_clear_log: Message,
) -> Element<'a, Message> {
    let top_bar = row![
        column![
            text("Snapshot Btrfs & Pemulihan Sistem")
                .size(16)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text("Cadangkan kondisi sistem secara instan dengan teknologi Copy-on-Write (CoW).")
                .size(11)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                }),
        ]
        .spacing(2),
        container("").width(Length::Fill),
        status_pill(format!("Filesystem: {}", fs_type_name), FLUENT_PRIMARY, is_btrfs),
    ]
    .align_y(Alignment::Center);

    // Create Snapshot Input Card
    let input = text_input("Deskripsi snapshot (misal: sebelum update driver)", snapshot_input)
        .padding([8, 12])
        .size(12)
        .style(glass_input_style)
        .on_input(on_input_change);

    let create_btn = button(
        row![
            svg_icon(icons::SNAPSHOT, 13.0, Color::WHITE),
            text("Ambil Snapshot").size(11)
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding([8, 16])
    .style(primary_button_style(ACCENT_CYAN))
    .on_press(on_create);

    let create_card = container(
        column![
            text("Buat Snapshot Baru:")
                .size(13)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            row![input, create_btn].spacing(10).align_y(Alignment::Center),
        ]
        .spacing(10)
        .padding(14),
    )
    .width(Length::Fill)
    .style(glass_card_style);

    // Snapshots History List
    let history_title = text("Daftar Riwayat Snapshot Tersedia:")
        .size(13)
        .font(FONT_DISPLAY_BOLD)
        .style(|_| text::Style {
            color: Some(TEXT_MAIN),
        });

    let mut snap_list = column![].spacing(8);

    if snapshots.is_empty() {
        snap_list = snap_list.push(
            container(
                text("Belum ada snapshot yang tersimpan di /.snapshots.")
                    .size(11)
                    .style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
            )
            .padding(20)
            .width(Length::Fill)
            .style(glass_card_style),
        );
    } else {
        for s in snapshots {
            let snap_name = s.name.clone();

            let row_item = row![
                column![
                    row![
                        text(&s.name).size(12).font(FONT_DISPLAY_SEMIBOLD).style(|_| text::Style {
                            color: Some(TEXT_MAIN),
                        }),
                        text(&s.date).size(10).style(|_| text::Style {
                            color: Some(TEXT_MUTED),
                        }),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                    text(&s.description).size(10).style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
                ]
                .spacing(2),
                container("").width(Length::Fill),
                row![
                    button(
                        row![
                            svg_icon(icons::CHECK, 11.0, Color::WHITE),
                            text("Pulihkan").size(10)
                        ]
                        .spacing(5)
                        .align_y(Alignment::Center)
                    )
                    .padding([6, 12])
                    .style(primary_button_style(FLUENT_PRIMARY))
                    .on_press(on_restore(snap_name.clone())),
                    button(
                        row![
                            svg_icon(icons::TRASH, 11.0, Color::WHITE),
                            text("Hapus").size(10)
                        ]
                        .spacing(5)
                        .align_y(Alignment::Center)
                    )
                    .padding([6, 12])
                    .style(primary_button_style(ACCENT_RED))
                    .on_press(on_delete(snap_name)),
                ]
                .spacing(6),
            ]
            .align_y(Alignment::Center)
            .padding(12);

            let card = container(row_item).width(Length::Fill).style(glass_card_style);
            snap_list = snap_list.push(card);
        }
    }

    let terminal = terminal_box(log_text, on_clear_log);

    let content = column![top_bar, create_card, history_title, snap_list, terminal]
        .spacing(14)
        .padding(20);

    glass_scrollable(content).height(Length::Fill).into()
}
