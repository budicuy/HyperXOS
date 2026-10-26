use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};

use crate::ui::components::{status_pill, terminal_box};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    current_governor: &'a str,
    max_map_count_val: &'a str,
    is_max_map_opt: bool,
    is_split_lock_disabled: bool,
    is_gamemode_active: bool,
    log_text: &'a str,
    on_set_governor: impl Fn(&'static str) -> Message + 'a,
    on_toggle_max_map: Message,
    on_toggle_split_lock: Message,
    on_toggle_gamemode: Message,
    on_install_runtimes: Message,
    on_clear_log: Message,
) -> Element<'a, Message> {
    let top_bar = column![
        text("Tweak Gaming & Optimasi Latensi")
            .size(16)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN),
            }),
        text("Tingkatkan performa framerate dan kurangi stuttering pada game modern di Linux.")
            .size(11)
            .style(|_| text::Style {
                color: Some(TEXT_SUB),
            }),
    ]
    .spacing(2);

    // 1. CPU Governor Card
    let gov_btn = |gov: &'static str, label: &'static str, is_active: bool| {
        button(text(label).size(11))
            .padding([6, 12])
            .style(primary_button_style(if is_active {
                ACCENT_CYAN
            } else {
                Color::from_rgba(1.0, 1.0, 1.0, 0.12)
            }))
            .on_press(on_set_governor(gov))
    };

    let gov_card = container(
        column![
            row![
                column![
                    text("Profil CPU Governor")
                        .size(13)
                        .style(|_| text::Style {
                            color: Some(TEXT_MAIN),
                        }),
                    text("Mengontrol frekuensi clock CPU saat beban game berat.")
                        .size(11)
                        .style(|_| text::Style {
                            color: Some(TEXT_SUB),
                        }),
                ]
                .spacing(2),
                container("").width(Length::Fill),
                status_pill(
                    format!("Aktif: {}", current_governor),
                    ACCENT_CYAN,
                    true
                ),
            ]
            .align_y(Alignment::Center),
            row![
                gov_btn("performance", "Performance", current_governor == "performance"),
                gov_btn("schedutil", "Schedutil", current_governor == "schedutil"),
                gov_btn("powersave", "Powersave", current_governor == "powersave"),
            ]
            .spacing(10),
        ]
        .spacing(12)
        .padding(14),
    )
    .width(Length::Fill)
    .style(glass_card_style);

    // 2. vm.max_map_count Card
    let map_card = container(
        row![
            column![
                text("Memori Game (vm.max_map_count)")
                    .size(13)
                    .style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                text(format!(
                    "Mencegah crash game Unreal Engine & Proton (Nilai: {})",
                    max_map_count_val
                ))
                .size(11)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                }),
            ]
            .spacing(2),
            container("").width(Length::Fill),
            button(
                row![
                    svg_icon(
                        if is_max_map_opt { icons::CHECK } else { icons::ZAP },
                        12.0,
                        if is_max_map_opt { ACCENT_GREEN } else { ACCENT_CYAN }
                    ),
                    text(if is_max_map_opt {
                        "Sudah Optimal (2.1M)"
                    } else {
                        "Terapkan Tweak (2147483642)"
                    })
                    .size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center)
            )
            .padding([6, 12])
            .style(primary_button_style(if is_max_map_opt {
                ACCENT_GREEN
            } else {
                ACCENT_CYAN
            }))
            .on_press(on_toggle_max_map),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .padding(14),
    )
    .width(Length::Fill)
    .style(glass_card_style);

    // 3. Split Lock Mitigation
    let split_card = container(
        row![
            column![
                text("Mitigasi Split Lock (Anti-Stuttering)")
                    .size(13)
                    .style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                text("Nonaktifkan penalti split-lock kernel untuk kestabilan frametime game.")
                    .size(11)
                    .style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
            ]
            .spacing(2),
            container("").width(Length::Fill),
            button(
                row![
                    svg_icon(
                        if is_split_lock_disabled { icons::CHECK } else { icons::SHIELD },
                        12.0,
                        if is_split_lock_disabled { ACCENT_GREEN } else { TEXT_SUB }
                    ),
                    text(if is_split_lock_disabled {
                        "Nonaktif (Optimal)"
                    } else {
                        "Nonaktifkan Mitigasi"
                    })
                    .size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center)
            )
            .padding([6, 12])
            .style(primary_button_style(if is_split_lock_disabled {
                ACCENT_GREEN
            } else {
                Color::from_rgba(1.0, 1.0, 1.0, 0.12)
            }))
            .on_press(on_toggle_split_lock),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .padding(14),
    )
    .width(Length::Fill)
    .style(glass_card_style);

    // 4. GameMode Daemon
    let gamemode_card = container(
        row![
            column![
                text("Feral GameMode Daemon")
                    .size(13)
                    .style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                text("Otomatis mengoptimalkan prioritas CPU/GPU saat game dijalankan.")
                    .size(11)
                    .style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
            ]
            .spacing(2),
            container("").width(Length::Fill),
            button(
                row![
                    svg_icon(
                        if is_gamemode_active { icons::CHECK } else { icons::GAMING },
                        12.0,
                        if is_gamemode_active { ACCENT_GREEN } else { TEXT_SUB }
                    ),
                    text(if is_gamemode_active {
                        "Aktif (Running)"
                    } else {
                        "Aktifkan Service"
                    })
                    .size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center)
            )
            .padding([6, 12])
            .style(primary_button_style(if is_gamemode_active {
                ACCENT_GREEN
            } else {
                Color::from_rgba(1.0, 1.0, 1.0, 0.12)
            }))
            .on_press(on_toggle_gamemode),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .padding(14),
    )
    .width(Length::Fill)
    .style(glass_card_style);

    // 5. Gaming Toolchain
    let toolchain_card = container(
        row![
            row![
                svg_icon(icons::STORE, 18.0, ACCENT_PURPLE),
                column![
                    text("Pasang Ekosistem Gaming Lengkap")
                        .size(13)
                        .style(|_| text::Style {
                            color: Some(TEXT_MAIN),
                        }),
                    text("Pasang Wine, Proton-GE, MangoHud, GOverlay, Gamemode, dan Vulkan Tools.")
                        .size(11)
                        .style(|_| text::Style {
                            color: Some(TEXT_SUB),
                        }),
                ]
                .spacing(2),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            container("").width(Length::Fill),
            button(
                row![
                    svg_icon(icons::DOWNLOAD, 12.0, ACCENT_PURPLE),
                    text("Pasang Toolchain Gaming").size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center)
            )
            .padding([6, 12])
            .style(primary_button_style(ACCENT_PURPLE))
            .on_press(on_install_runtimes),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .padding(14),
    )
    .width(Length::Fill)
    .style(glass_card_style);

    let terminal = terminal_box(log_text, on_clear_log);

    let content = column![
        top_bar,
        gov_card,
        map_card,
        split_card,
        gamemode_card,
        toolchain_card,
        terminal,
    ]
    .spacing(14)
    .padding(20);

    scrollable(content).height(Length::Fill).into()
}
