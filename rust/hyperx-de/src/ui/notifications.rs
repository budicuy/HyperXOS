use iced::border::Radius;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    glass_action_button, glass_card_accent_style, glass_card_style, glass_scrollable, icons,
    popover_glass_style, svg_icon, ACCENT_AMBER, ACCENT_CYAN, ACCENT_GREEN, ACCENT_PURPLE,
    ACCENT_RED, ACTIVE_DARK_FG, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD,
    TEXT_MAIN, TEXT_MUTED, TEXT_SUB,
};

use crate::ui::Message;

#[derive(Debug, Clone)]
pub struct DesktopNotification {
    pub _id: usize,
    pub title: String,
    pub body: String,
    pub time_label: String,
    pub accent: Color,
}

pub fn view_toast_banner<'a>(notif: &'a DesktopNotification) -> Element<'a, Message> {
    let content = row![
        svg_icon(icons::BELL, 18.0, notif.accent),
        column![
            row![
                text(&notif.title)
                    .size(12)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                container("").width(Length::Fill),
                text(&notif.time_label)
                    .size(10)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(TEXT_MUTED),
                    }),
            ]
            .align_y(Alignment::Center),
            text(&notif.body)
                .size(11)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                }),
        ]
        .spacing(2)
        .width(Length::Fill),
        button(svg_icon(icons::CLOSE, 12.0, TEXT_MUTED))
            .padding([3, 6])
            .style(glass_action_button)
            .on_press(Message::DismissToast),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .padding(12);

    container(content)
        .width(Length::Fixed(340.0))
        .style(popover_glass_style(notif.accent))
        .into()
}

pub fn view_notification_drawer<'a>(
    date_header: &'a str,
    current_day: u32,
    notifications: &'a [DesktopNotification],
) -> Element<'a, Message> {
    let header = row![
        svg_icon(icons::CALENDAR, 18.0, ACCENT_PURPLE),
        column![
            text("Kalender & Pusat Notifikasi")
                .size(14)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text(date_header)
                .size(11)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(ACCENT_CYAN),
                }),
        ]
        .spacing(1),
        container("").width(Length::Fill),
        button(svg_icon(icons::CLOSE, 13.0, ACCENT_RED))
            .padding([4, 8])
            .style(glass_action_button)
            .on_press(Message::ToggleNotificationDrawer),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    // Mini Monthly Calendar Card
    let days_header = row![
        text("Sen").size(10).font(FONT_DISPLAY_BOLD),
        text("Sel").size(10).font(FONT_DISPLAY_BOLD),
        text("Rab").size(10).font(FONT_DISPLAY_BOLD),
        text("Kam").size(10).font(FONT_DISPLAY_BOLD),
        text("Jum").size(10).font(FONT_DISPLAY_BOLD),
        text("Sab").size(10).font(FONT_DISPLAY_BOLD),
        text("Min").size(10).font(FONT_DISPLAY_BOLD),
    ]
    .spacing(18);

    let mut cal_rows = column![days_header].spacing(6).align_x(Alignment::Center);
    let mut day_counter = 1u32;
    for _week in 0..5 {
        let mut r = row![].spacing(8);
        for _dow in 0..7 {
            if day_counter <= 31 {
                let d = day_counter;
                let is_today = d == current_day;
                let cell = container(
                    text(format!("{:2}", d))
                        .size(11)
                        .font(if is_today {
                            FONT_DISPLAY_BOLD
                        } else {
                            FONT_BODY_MEDIUM
                        })
                        .style(move |_| text::Style {
                            color: Some(if is_today {
                                ACTIVE_DARK_FG
                            } else {
                                TEXT_MAIN
                            }),
                        }),
                )
                .width(Length::Fixed(32.0))
                .height(Length::Fixed(24.0))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(if is_today {
                        Color::WHITE
                    } else {
                        Color::from_rgba(1.0, 1.0, 1.0, 0.04)
                    })),
                    border: Border::default().rounded(Radius::from(6.0)),
                    ..container::Style::default()
                });
                r = r.push(cell);
                day_counter += 1;
            }
        }
        cal_rows = cal_rows.push(r);
    }

    let calendar_card = container(cal_rows.padding(12))
        .width(Length::Fill)
        .style(glass_card_accent_style(ACCENT_PURPLE));

    // Notification List Header
    let notif_top = row![
        svg_icon(icons::BELL, 15.0, ACCENT_AMBER),
        text(format!("Riwayat Notifikasi ({})", notifications.len()))
            .size(12)
            .font(FONT_DISPLAY_BOLD),
        container("").width(Length::Fill),
        button(
            row![
                svg_icon(icons::TRASH, 12.0, ACCENT_RED),
                text("Bersihkan").size(10).font(FONT_BODY_SEMIBOLD),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .padding([4, 10])
        .style(glass_action_button)
        .on_press(Message::ClearNotifications),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let mut notif_list = column![].spacing(8);
    if notifications.is_empty() {
        notif_list = notif_list.push(
            container(
                row![
                    svg_icon(icons::CHECK_CIRCLE, 16.0, ACCENT_GREEN),
                    text("Tidak ada notifikasi baru saat ini.")
                        .size(11)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(TEXT_MUTED),
                        }),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
                .padding(14),
            )
            .width(Length::Fill)
            .style(glass_card_style),
        );
    } else {
        for item in notifications.iter().rev() {
            let accent = item.accent;
            let card = container(
                column![
                    row![
                        svg_icon(icons::SPARKLES, 14.0, accent),
                        text(&item.title)
                            .size(12)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style {
                                color: Some(TEXT_MAIN),
                            }),
                        container("").width(Length::Fill),
                        text(&item.time_label)
                            .size(10)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(TEXT_MUTED),
                            }),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                    text(&item.body)
                        .size(11)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(TEXT_SUB),
                        }),
                ]
                .spacing(4)
                .padding(10),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(accent));
            notif_list = notif_list.push(card);
        }
    }

    let scroll = glass_scrollable(notif_list).height(Length::Fixed(210.0));

    let content = column![header, calendar_card, notif_top, scroll]
        .spacing(10)
        .padding(14);

    container(content)
        .width(Length::Fixed(350.0))
        .style(popover_glass_style(ACCENT_PURPLE))
        .into()
}
