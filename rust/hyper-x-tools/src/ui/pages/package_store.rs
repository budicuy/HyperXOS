use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::core::package_manager::StorePackage;
use crate::ui::components::{glass_scrollable, status_pill, terminal_box};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

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
            iced::Color::from_rgb(0.06, 0.07, 0.10)
        } else {
            iced::Color::WHITE
        };
        button(
            text(cat)
                .size(11)
                .font(if is_act { FONT_DISPLAY_BOLD } else { FONT_BODY_SEMIBOLD })
                .style(move |_| text::Style {
                    color: Some(fg),
                }),
        )
        .padding([5, 14])
        .style(active_toggle_button_style(is_act, ACCENT_CYAN))
        .on_press(on_select_cat(cat))
    };

    let categories = row![
        cat_btn("Semua"),
        cat_btn("Browsers"),
        cat_btn("Gaming"),
        cat_btn("Development"),
        cat_btn("Multimedia"),
        cat_btn("Utilities"),
    ]
    .spacing(8);

    // Package List
    let mut pkg_col = column![].spacing(8);

    for p in packages {
        let pkg_name = p.package_name.clone();

        let badges = status_pill(
            if p.installed {
                "Terpasang"
            } else {
                "Belum Terpasang"
            },
            if p.installed {
                ACCENT_GREEN
            } else {
                TEXT_MUTED
            },
            p.installed,
        );

        let action_btn = if !p.installed {
            button(
                row![
                    svg_icon(icons::DOWNLOAD, 12.0, ACCENT_CYAN),
                    text("Pasang").size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center)
            )
            .padding([5, 12])
            .style(primary_button_style(ACCENT_CYAN))
            .on_press(on_install(pkg_name))
        } else {
            button(
                row![
                    svg_icon(icons::TRASH, 12.0, ACCENT_RED),
                    text("Copot").size(11)
                ]
                .spacing(6)
                .align_y(Alignment::Center)
            )
            .padding([5, 12])
            .style(primary_button_style(ACCENT_RED))
            .on_press(on_remove(pkg_name))
        };

        let icon_view = icons::app_icon(&p.package_name, 38.0);

        let card = container(
            row![
                icon_view,
                column![
                    row![
                        text(&p.display_name).size(13).font(FONT_DISPLAY_BOLD).style(|_| text::Style {
                            color: Some(TEXT_MAIN),
                        }),
                        badges,
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                    text(&p.description).size(11).style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
                ]
                .spacing(3),
                container("").width(Length::Fill),
                action_btn,
            ]
            .spacing(14)
            .align_y(Alignment::Center)
            .padding(12),
        )
        .width(Length::Fill)
        .style(glass_card_style);

        pkg_col = pkg_col.push(card);
    }

    let terminal = terminal_box(log_text, on_clear_log);

    let content = column![top_bar, search, categories, pkg_col, terminal]
        .spacing(14)
        .padding(20);

    glass_scrollable(content).height(Length::Fill).into()
}
