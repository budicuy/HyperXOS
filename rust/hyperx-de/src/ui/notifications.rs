use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};

use hyperx_ui_kit::{
    fluent_acrylic_modal_style, fluent_card_style, fluent_ribbon_button_style, glass_scrollable,
    icons, svg_icon, FLUENT_PRIMARY, FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED, FONT_BODY_MEDIUM,
    FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD,
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
                    .font(FONT_BODY_SEMIBOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
                container("").width(Length::Fill),
                text(&notif.time_label)
                    .size(10)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
            ]
            .align_y(Alignment::Center),
            text(&notif.body)
                .size(11)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_MUTED),
                }),
        ]
        .spacing(2)
        .width(Length::Fill),
        button(svg_icon(icons::CLOSE, 11.0, FLUENT_TEXT_MUTED))
            .padding([3, 5])
            .style(fluent_ribbon_button_style)
            .on_press(Message::DismissToast),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .padding(12);

    container(content)
        .width(Length::Fixed(330.0))
        .style(fluent_acrylic_modal_style)
        .into()
}

pub fn view_notification_drawer<'a>(
    notifications: &'a [DesktopNotification],
) -> Element<'a, Message> {
    let header = row![
        svg_icon(icons::BELL, 16.0, FLUENT_PRIMARY),
        text("Pusat Notifikasi")
            .size(13)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_DARK),
            }),
        container("").width(Length::Fill),
        button(
            text("Hapus Semua")
                .size(11)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(FLUENT_PRIMARY),
                }),
        )
        .padding([2, 6])
        .style(fluent_ribbon_button_style)
        .on_press(Message::ClearAllNotifications),
        button(svg_icon(icons::CLOSE, 12.0, FLUENT_TEXT_MUTED))
            .padding([2, 5])
            .style(fluent_ribbon_button_style)
            .on_press(Message::ToggleNotificationDrawer),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let notif_list: Element<'a, Message> = if notifications.is_empty() {
        container(
            column![
                svg_icon(icons::BELL, 28.0, Color::from_rgb(0.70, 0.72, 0.78)),
                text("Tidak ada notifikasi baru")
                    .size(12)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
                    }),
            ]
            .spacing(6)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fixed(120.0))
        .center_x(Length::Fill)
        .center_y(Length::Fixed(120.0))
        .into()
    } else {
        let mut list_col = column![].spacing(6);
        for notif in notifications.iter().rev() {
            let card = container(
                row![
                    svg_icon(icons::INFO, 16.0, notif.accent),
                    column![
                        row![
                            text(&notif.title)
                                .size(12)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(|_| text::Style {
                                    color: Some(FLUENT_TEXT_DARK),
                                }),
                            container("").width(Length::Fill),
                            text(&notif.time_label)
                                .size(10)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style {
                                    color: Some(FLUENT_TEXT_MUTED),
                                }),
                        ]
                        .align_y(Alignment::Center),
                        text(&notif.body)
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(FLUENT_TEXT_MUTED),
                            }),
                    ]
                    .spacing(2)
                    .width(Length::Fill),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding(10)
            .width(Length::Fill)
            .style(fluent_card_style);

            list_col = list_col.push(card);
        }
        glass_scrollable(list_col).height(Length::Fixed(240.0)).into()
    };

    let content = column![header, notif_list].spacing(10).padding(14);

    container(content)
        .width(Length::Fixed(330.0))
        .style(fluent_acrylic_modal_style)
        .into()
}
