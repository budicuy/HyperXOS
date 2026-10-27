use iced::widget::{button, column, container, text, Space};
use iced::{Color, Element, Length};

use crate::theme::{self, TEXT_PRIMARY};

#[derive(Debug, Clone)]
pub enum AppMenuAction {
    AboutHyperX,
    SystemSettings,
    PackageStore,
    Sleep,
    Restart,
    Shutdown,
    LockScreen,
    Logout,
    Dismiss,
}

pub struct AppMenu;

impl AppMenu {
    pub fn view<Message: 'static + Clone>(
        map_fn: impl Fn(AppMenuAction) -> Message + Copy + 'static,
    ) -> Element<'static, Message> {
        let item = move |label: &'static str, action: AppMenuAction| {
            button(
                container(text(label).size(12).color(TEXT_PRIMARY))
                    .padding([4, 12])
                    .width(Length::Fill),
            )
            .padding(0)
            .style(button::text)
            .on_press(map_fn(action))
        };

        let divider = || {
            container(Space::new().width(Length::Fill).height(1))
                .style(|_| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()),
                    ..Default::default()
                })
        };

        let menu_items = column![
            item("About HyperXOS", AppMenuAction::AboutHyperX),
            divider(),
            item("System Settings...", AppMenuAction::SystemSettings),
            item("App Store & Repositories...", AppMenuAction::PackageStore),
            divider(),
            item("Sleep", AppMenuAction::Sleep),
            item("Restart...", AppMenuAction::Restart),
            item("Shut Down...", AppMenuAction::Shutdown),
            divider(),
            item("Lock Screen       ⌃⌘Q", AppMenuAction::LockScreen),
            item("Log Out...         ⇧⌘Q", AppMenuAction::Logout),
        ]
        .spacing(3);

        container(menu_items)
            .width(220)
            .padding(6)
            .style(theme::menu_popup_style)
            .into()
    }
}
