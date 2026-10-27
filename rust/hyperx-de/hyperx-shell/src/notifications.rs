use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length};

use crate::theme::{self, ACCENT_BLUE, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub struct NotificationItem {
    pub id: usize,
    pub app: String,
    pub title: String,
    pub body: String,
    pub time: String,
}

#[derive(Debug, Clone)]
pub enum NotifAction {
    Dismiss(usize),
    ClearAll,
}

pub struct NotificationCenter {
    pub notifications: Vec<NotificationItem>,
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self {
            notifications: vec![
                NotificationItem {
                    id: 1,
                    app: "HyperX Package Manager".to_string(),
                    title: "System Updates Ready".to_string(),
                    body: "Linux 6.17-hyperx and 12 packages are ready to update.".to_string(),
                    time: "10m ago".to_string(),
                },
                NotificationItem {
                    id: 2,
                    app: "HyperX File Manager".to_string(),
                    title: "Download Finished".to_string(),
                    body: "hyperxos-installer-x86_64.iso saved to ~/Downloads".to_string(),
                    time: "25m ago".to_string(),
                },
            ],
        }
    }
}

impl NotificationCenter {
    pub fn update(&mut self, action: NotifAction) {
        match action {
            NotifAction::Dismiss(id) => {
                self.notifications.retain(|n| n.id != id);
            }
            NotifAction::ClearAll => {
                self.notifications.clear();
            }
        }
    }

    pub fn view<Message: 'static + Clone>(
        &self,
        map_fn: impl Fn(NotifAction) -> Message + Copy + 'static,
    ) -> Element<'_, Message> {
        let header = row![
            text("Notification Center").size(13).color(TEXT_PRIMARY),
            Space::new().width(Length::Fill),
            button(text("Clear All").size(11).color(ACCENT_BLUE))
                .padding([2, 6])
                .style(button::text)
                .on_press(map_fn(NotifAction::ClearAll)),
        ]
        .align_y(Alignment::Center);

        let mut list_col = column![].spacing(8);

        if self.notifications.is_empty() {
            list_col = list_col.push(
                container(text("No New Notifications").size(12).color(TEXT_SECONDARY))
                    .padding(20)
                    .center_x(Length::Fill),
            );
        } else {
            for item in &self.notifications {
                let id = item.id;
                let notif_card = container(
                    column![
                        row![
                            text(&item.app).size(11).color(ACCENT_BLUE),
                            Space::new().width(Length::Fill),
                            text(&item.time).size(10).color(TEXT_SECONDARY),
                            button(text("✕").size(10).color(TEXT_SECONDARY))
                                .padding([1, 4])
                                .style(button::text)
                                .on_press(map_fn(NotifAction::Dismiss(id))),
                        ]
                        .align_y(Alignment::Center),
                        text(&item.title).size(12).color(TEXT_PRIMARY),
                        text(&item.body).size(11).color(TEXT_SECONDARY),
                    ]
                    .spacing(3),
                )
                .padding([8, 12])
                .style(theme::card_style);

                list_col = list_col.push(notif_card);
            }
        }

        let content = column![header, scrollable(list_col).height(Length::Shrink)].spacing(10);

        container(content)
            .width(320)
            .padding(14)
            .style(theme::control_center_style)
            .into()
    }
}
