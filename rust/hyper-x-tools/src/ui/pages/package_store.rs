use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Color, Element, Length};

use crate::core::package_manager::StorePackage;
use crate::ui::components::{glass_scrollable, status_pill, terminal_box};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

const CATEGORIES: [(&str, &str, &str); 5] = [
    ("Browsers", "Peramban Web", "Aplikasi penjelajah web cepat, aman, dan privasi tinggi"),
    ("Gaming", "Gaming & Hiburan", "Platform distribusi game digital dan obrolan komunitas"),
    ("Development", "Pengembangan & Koding", "Editor kode modern, kontrol versi, dan alat kontainerisasi"),
    ("Multimedia", "Multimedia & Kreatif", "Pemutar audio-video serbaguna dan editor grafis"),
    ("Utilities", "Utilitas & Pemeliharaan", "Pemantau sistem interaktif dan pemulihan snapshot data"),
];

pub fn view<'a, Message: Clone + 'a>(
    active_cat: &'a str,
    search_query: &'a str,
    packages: &'a [StorePackage],
    log_text: &'a str,
    on_search: impl Fn(String) -> Message + 'a,
    on_select_cat: impl Fn(&'static str) -> Message + 'a,
    on_install: impl Fn(String) -> Message + 'a,
    on_remove: impl Fn(String) -> Message + 'a,
    on_clear_log: Message,
) -> Element<'a, Message> {
    let top_bar = column![
        text("HyperX Software Store")
            .size(16)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN),
            }),
        text("Pemasangan aplikasi kurasi HyperXOS untuk kebutuhan harian, produktivitas, dan gaming.")
            .size(11)
            .style(|_| text::Style {
                color: Some(TEXT_SUB),
            }),
    ]
    .spacing(2);

    // Search bar
    let search = text_input("Cari aplikasi di store...", search_query)
        .padding([8, 12])
        .size(12)
        .style(glass_input_style)
        .on_input(on_search);

    // Category pills
    let cat_btn = |cat: &'static str| {
        let is_act = active_cat == cat;
        let fg = if is_act {
            Color::WHITE
        } else {
            TEXT_MAIN
        };
        button(
            text(cat)
                .size(11)
                .font(if is_act { FONT_DISPLAY_BOLD } else { FONT_BODY_SEMIBOLD })
                .style(move |_| text::Style {
                    color: Some(fg),
                }),
        )
        .padding([6, 14])
        .style(active_toggle_button_style(is_act, ACCENT_CYAN))
        .on_press(on_select_cat(cat))
    };

    let categories_bar = row![
        cat_btn("Semua"),
        cat_btn("Browsers"),
        cat_btn("Gaming"),
        cat_btn("Development"),
        cat_btn("Multimedia"),
        cat_btn("Utilities"),
    ]
    .spacing(8);

    // Single card renderer for 2-column grid
    let render_card = |p: &'a StorePackage| -> Element<'a, Message> {
        let pkg_name = p.package_name.clone();

        let badges = status_pill(
            if p.installed {
                "Terpasang"
            } else {
                "Belum Terpasang"
            },
            if p.installed {
                FLUENT_PRIMARY
            } else {
                TEXT_MUTED
            },
            p.installed,
        );

        let action_btn = if !p.installed {
            button(
                row![
                    svg_icon(icons::DOWNLOAD, 12.0, Color::WHITE),
                    text("Pasang").size(11).font(FONT_BODY_SEMIBOLD)
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([6, 14])
            .style(primary_button_style(FLUENT_PRIMARY))
            .on_press(on_install(pkg_name))
        } else {
            button(
                row![
                    svg_icon(icons::TRASH, 12.0, Color::WHITE),
                    text("Copot").size(11).font(FONT_BODY_SEMIBOLD)
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([6, 14])
            .style(primary_button_style(ACCENT_RED))
            .on_press(on_remove(pkg_name))
        };

        let icon_view = icons::app_icon(&p.package_name, 42.0);

        let card_content = row![
            icon_view,
            column![
                text(&p.display_name)
                    .size(13)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                text(&p.description)
                    .size(11)
                    .style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
                row![
                    badges,
                    container("").width(Length::Fill),
                    action_btn,
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(6)
            .width(Length::Fill),
        ]
        .spacing(14)
        .align_y(Alignment::Center)
        .padding(14);

        container(card_content)
            .width(Length::Fill)
            .style(glass_card_style)
            .into()
    };

    // Build category sections (2-Column Grid per category)
    let mut store_sections = column![].spacing(22);
    let mut total_rendered = 0;

    for (cat_key, cat_title, cat_sub) in CATEGORIES {
        let cat_pkgs: Vec<&StorePackage> = packages
            .iter()
            .filter(|p| p.category.eq_ignore_ascii_case(cat_key))
            .collect();

        if cat_pkgs.is_empty() {
            continue;
        }

        total_rendered += cat_pkgs.len();

        let header = column![
            row![
                text(cat_title)
                    .size(15)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                container(
                    text(format!("{} Aplikasi", cat_pkgs.len()))
                        .size(10)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(ACCENT_CYAN),
                        }),
                )
                .padding([2, 8])
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(0.54, 0.71, 0.98, 0.12))),
                    border: iced::Border::default()
                        .rounded(iced::border::Radius::from(0.0))
                        .width(1.0)
                        .color(Color::from_rgba(0.54, 0.71, 0.98, 0.30)),
                    ..container::Style::default()
                }),
                container("").width(Length::Fill),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            text(cat_sub)
                .size(11)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED),
                }),
        ]
        .spacing(3);

        // 2-Column Grid
        let mut grid = column![].spacing(10);
        let chunks: Vec<&[&StorePackage]> = cat_pkgs.chunks(2).collect();

        for chunk in chunks {
            if chunk.len() == 2 {
                let card_left = render_card(chunk[0]);
                let card_right = render_card(chunk[1]);
                grid = grid.push(
                    row![
                        container(card_left).width(Length::FillPortion(1)),
                        container(card_right).width(Length::FillPortion(1)),
                    ]
                    .spacing(12),
                );
            } else {
                let card_left = render_card(chunk[0]);
                grid = grid.push(
                    row![
                        container(card_left).width(Length::FillPortion(1)),
                        container("").width(Length::FillPortion(1)),
                    ]
                    .spacing(12),
                );
            }
        }

        let section = column![header, grid].spacing(10);
        store_sections = store_sections.push(section);
    }

    if total_rendered == 0 {
        let empty_box = container(
            column![
                svg_icon(icons::STORE, 32.0, TEXT_MUTED),
                text("Tidak ada aplikasi yang cocok dengan pencarian.")
                    .size(12)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(TEXT_MUTED),
                    }),
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .padding(36)
        .center_x(Length::Fill)
        .style(glass_card_style);

        store_sections = store_sections.push(empty_box);
    }

    let terminal = terminal_box(log_text, on_clear_log);

    let content = column![top_bar, search, categories_bar, store_sections, terminal]
        .spacing(18)
        .padding(20);

    glass_scrollable(content).height(Length::Fill).into()
}
