use chrono::{Datelike, Local};
use iced::border::Radius;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    fluent_acrylic_modal_style, fluent_ribbon_button_style, icons, svg_icon, FLUENT_PRIMARY,
    FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD,
};

use crate::ui::Message;

pub fn view_calendar_flyout<'a>(date_header: &'a str, current_day: u32) -> Element<'a, Message> {
    let now = Local::now();
    let month_name = match now.month() {
        1 => "Januari",
        2 => "Februari",
        3 => "Maret",
        4 => "April",
        5 => "Mei",
        6 => "Juni",
        7 => "Juli",
        8 => "Agustus",
        9 => "September",
        10 => "Oktober",
        11 => "November",
        _ => "Desember",
    };
    let year = now.year();

    // 1. Month & Navigation Header
    let header = row![
        text(format!("{} {}", month_name, year))
            .size(14)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_DARK),
            }),
        container("").width(Length::Fill),
        button(svg_icon(icons::CHEVRON_LEFT, 13.0, FLUENT_TEXT_DARK))
            .padding([4, 6])
            .style(fluent_ribbon_button_style),
        button(svg_icon(icons::CHEVRON_RIGHT, 13.0, FLUENT_TEXT_DARK))
            .padding([4, 6])
            .style(fluent_ribbon_button_style),
        button(svg_icon(icons::CLOSE, 12.0, FLUENT_TEXT_MUTED))
            .padding([4, 6])
            .style(fluent_ribbon_button_style)
            .on_press(Message::ToggleCalendarFlyout),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    // 2. Day of Week Labels (Sen, Sel, Rab, Kam, Jum, Sab, Min)
    let day_labels = ["Sen", "Sel", "Rab", "Kam", "Jum", "Sab", "Min"];
    let mut days_header_row = row![].spacing(6).align_y(Alignment::Center);
    for label in day_labels {
        days_header_row = days_header_row.push(
            container(
                text(label)
                    .size(11)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
            )
            .width(Length::Fixed(34.0))
            .align_x(Alignment::Center),
        );
    }

    // 3. 31-day Monthly Grid
    let mut calendar_rows = column![].spacing(4);
    let mut current_row = row![].spacing(6).align_y(Alignment::Center);

    // October 2026 starts on Thursday (index 3)
    let start_offset = 3;
    for _ in 0..start_offset {
        current_row = current_row.push(container("").width(Length::Fixed(34.0)));
    }

    for day in 1..=31 {
        let is_today = day == current_day;
        let day_cell = container(
            text(format!("{}", day))
                .size(11)
                .font(if is_today {
                    FONT_DISPLAY_BOLD
                } else {
                    FONT_BODY_MEDIUM
                })
                .style(move |_| text::Style {
                    color: Some(if is_today {
                        Color::WHITE
                    } else {
                        FLUENT_TEXT_DARK
                    }),
                }),
        )
        .width(Length::Fixed(34.0))
        .height(Length::Fixed(30.0))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_| {
            if is_today {
                container::Style {
                    background: Some(Background::Color(FLUENT_PRIMARY)),
                    border: Border::default().rounded(Radius::from(999.0)),
                    ..container::Style::default()
                }
            } else {
                container::Style::default()
            }
        });

        current_row = current_row.push(day_cell);

        if (day + start_offset) % 7 == 0 || day == 31 {
            calendar_rows = calendar_rows.push(current_row);
            current_row = row![].spacing(6).align_y(Alignment::Center);
        }
    }

    // 4. Footer Bar
    let footer = container(
        row![
            text(date_header)
                .size(11)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_MUTED),
                }),
            container("").width(Length::Fill),
            button(
                text("Pengaturan Tanggal")
                    .size(11)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_PRIMARY),
                    }),
            )
            .padding([3, 6])
            .style(fluent_ribbon_button_style)
            .on_press(Message::OpenSettingsPage(crate::ui::settings::SettingsTab::About)),
        ]
        .align_y(Alignment::Center),
    )
    .padding([8, 0])
    .style(|_| container::Style {
        border: Border::default()
            .width(1.0)
            .color(Color::from_rgba(0.0, 0.0, 0.0, 0.06)),
        ..container::Style::default()
    });

    let content = column![
        header,
        container("").height(Length::Fixed(4.0)),
        days_header_row,
        calendar_rows,
        container("").height(Length::Fixed(6.0)),
        footer,
    ]
    .spacing(6)
    .padding(16);

    container(content)
        .width(Length::Fixed(310.0))
        .style(fluent_acrylic_modal_style)
        .into()
}
