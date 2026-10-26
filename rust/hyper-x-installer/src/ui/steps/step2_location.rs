use iced::widget::{button, column, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub const TIMEZONES: &[(&str, &str)] = &[
    ("Asia/Jakarta", "Jakarta, Indonesia (WIB - UTC+7)"),
    ("Asia/Makassar", "Makassar, Indonesia (WITA - UTC+8)"),
    ("Asia/Jayapura", "Jayapura, Indonesia (WIT - UTC+9)"),
    ("Asia/Singapore", "Singapore (SGT - UTC+8)"),
    ("Asia/Tokyo", "Tokyo, Japan (JST - UTC+9)"),
    ("UTC", "UTC (Coordinated Universal Time)"),
    ("Europe/London", "London, United Kingdom (GMT/BST)"),
    ("Europe/Berlin", "Berlin, Germany (CET/CEST)"),
    ("America/New_York", "New York, USA (EST/EDT)"),
    ("America/Los_Angeles", "Los Angeles, USA (PST/PDT)"),
    ("Australia/Sydney", "Sydney, Australia (AEST)"),
];

pub const LOCALES: &[(&str, &str)] = &[
    ("en_US.UTF-8", "English (United States) - en_US.UTF-8"),
    ("id_ID.UTF-8", "Bahasa Indonesia - id_ID.UTF-8"),
    ("ja_JP.UTF-8", "Japanese (日本語) - ja_JP.UTF-8"),
    ("de_DE.UTF-8", "German (Deutsch) - de_DE.UTF-8"),
];

pub fn view<'a, Message: Clone + 'a>(
    selected_tz: &str,
    selected_locale: &str,
    on_select_tz: impl Fn(String) -> Message + 'a,
    on_select_locale: impl Fn(String) -> Message + 'a,
    on_back: Message,
    on_next: Message,
) -> Element<'a, Message> {
    let mut tz_list = column![].spacing(6);
    for (tz_code, tz_desc) in TIMEZONES {
        let is_sel = *tz_code == selected_tz;
        let code_str = tz_code.to_string();

        let btn = button(
            row![
                svg_icon(icons::GLOBE, 14.0, if is_sel { ACCENT_CYAN } else { TEXT_MUTED }),
                column![
                    text(*tz_code).size(12).font(FONT_BODY_SEMIBOLD).style(move |_| text::Style {
                        color: Some(if is_sel { Color::WHITE } else { TEXT_MAIN }),
                    }),
                    text(*tz_desc).size(10).font(FONT_BODY).style(move |_| text::Style {
                        color: Some(TEXT_MUTED),
                    }),
                ]
                .spacing(2)
                .width(Length::Fill),
                if is_sel {
                    svg_icon(icons::CHECK_CIRCLE, 14.0, ACCENT_CYAN)
                } else {
                    svg_icon(icons::MINUS, 0.0, Color::TRANSPARENT)
                },
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .padding(10)
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
        .on_press(on_select_tz(code_str));

        tz_list = tz_list.push(btn);
    }

    let mut locale_list = column![].spacing(6);
    for (loc_code, loc_desc) in LOCALES {
        let is_sel = *loc_code == selected_locale;
        let code_str = loc_code.to_string();

        let btn = button(
            row![
                text(*loc_desc).size(12).font(FONT_BODY_SEMIBOLD).style(move |_| text::Style {
                    color: Some(if is_sel { Color::WHITE } else { TEXT_MAIN }),
                }),
                row![].width(Length::Fill),
                if is_sel {
                    svg_icon(icons::CHECK_CIRCLE, 14.0, ACCENT_GREEN)
                } else {
                    svg_icon(icons::MINUS, 0.0, Color::TRANSPARENT)
                },
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding(10)
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
        .on_press(on_select_locale(code_str));

        locale_list = locale_list.push(btn);
    }

    let back_btn = button(text("Back").size(12).font(FONT_BODY_MEDIUM))
        .on_press(on_back)
        .style(secondary_button_style())
        .padding([8, 16]);

    let next_btn = button(
        row![
            text("Next: Keyboard")
                .size(13)
                .font(FONT_BODY_SEMIBOLD),
            svg_icon(icons::ROCKET, 13.0, Color::WHITE),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .on_press(on_next)
    .style(primary_button_style(ACCENT_CYAN))
    .padding([9, 20]);

    column![
        column![
            text("Location & Regional Settings")
                .size(20)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN)
                }),
            text("Select your local timezone and system display language")
                .size(12)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED)
                }),
        ]
        .spacing(4),
        row![
            column![
                text("TIMEZONE")
                    .size(11)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_CYAN)
                    }),
                scrollable(tz_list)
                    .height(Length::Fixed(380.0))
                    .style(glass_scrollable_style),
            ]
            .spacing(8)
            .width(Length::FillPortion(3)),
            column![
                text("SYSTEM LOCALE")
                    .size(11)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_GREEN)
                    }),
                scrollable(locale_list)
                    .height(Length::Fixed(380.0))
                    .style(glass_scrollable_style),
            ]
            .spacing(8)
            .width(Length::FillPortion(2)),
        ]
        .spacing(20),
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
