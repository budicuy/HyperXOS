use iced::alignment::Alignment;
use iced::widget::{button, container, row, text, Space};
use iced::{Element, Length};

use crate::dynamic_island::{DynamicIsland, IslandAction};
use crate::theme::{self, ACCENT_BLUE, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub enum TopBarAction {
    ToggleAppMenu,
    ToggleControlCenter,
    ToggleNotifications,
    Island(IslandAction),
    MenuAction(&'static str),
}

pub struct TopBar {
    pub active_app: String,
    pub battery_level: u8,
    pub time_str: String,
}

impl Default for TopBar {
    fn default() -> Self {
        Self {
            active_app: "Finder".to_string(),
            battery_level: 95,
            time_str: "Sel, 27 Okt 23:45".to_string(),
        }
    }
}

impl TopBar {
    pub fn update_time(&mut self) {
        let now = chrono::Local::now();
        self.time_str = now.format("%a %d %b %H:%M").to_string();
    }

    pub fn view<'a, Message: 'static + Clone>(
        &'a self,
        island: &'a DynamicIsland,
        map_fn: impl Fn(TopBarAction) -> Message + Copy + 'static,
    ) -> Element<'a, Message> {
        // Left side: HyperX Logo, Active App name, App Menus
        let logo_btn = button(
            text("⚡").size(15).color(ACCENT_BLUE)
        )
        .padding([2, 6])
        .style(button::text)
        .on_press(map_fn(TopBarAction::ToggleAppMenu));

        let app_name = text(&self.active_app)
            .size(13)
            .color(TEXT_PRIMARY);

        let menu_item = move |name: &'static str| {
            button(text(name).size(12).color(TEXT_SECONDARY))
                .padding([2, 6])
                .style(button::text)
                .on_press(map_fn(TopBarAction::MenuAction(name)))
        };

        let left_section = row![
            logo_btn,
            app_name,
            menu_item("File"),
            menu_item("Edit"),
            menu_item("View"),
            menu_item("Go"),
            menu_item("Window"),
            menu_item("Help"),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        // Center section: Dynamic Island capsule
        let island_element = island.view(move |act| map_fn(TopBarAction::Island(act)));

        // Right side: System Status & Trays
        let wifi_icon = text("📶").size(13);
        let battery_str = format!("🔋 {}%", self.battery_level);
        let battery_elem = text(battery_str).size(12).color(TEXT_SECONDARY);

        let control_center_btn = button(
            text("🎛").size(14).color(TEXT_PRIMARY)
        )
        .padding([2, 6])
        .style(button::text)
        .on_press(map_fn(TopBarAction::ToggleControlCenter));

        let clock_elem = button(
            text(&self.time_str).size(12).color(TEXT_PRIMARY)
        )
        .padding([2, 6])
        .style(button::text)
        .on_press(map_fn(TopBarAction::ToggleNotifications));

        let notif_bell = button(
            text("🔔").size(13).color(TEXT_PRIMARY)
        )
        .padding([2, 6])
        .style(button::text)
        .on_press(map_fn(TopBarAction::ToggleNotifications));

        let right_section = row![
            wifi_icon,
            battery_elem,
            control_center_btn,
            clock_elem,
            notif_bell,
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let bar_content = row![
            left_section,
            Space::new().width(Length::Fill),
            island_element,
            Space::new().width(Length::Fill),
            right_section,
        ]
        .padding([2, 12])
        .align_y(Alignment::Center);

        container(bar_content)
            .width(Length::Fill)
            .height(34)
            .style(theme::top_bar_style)
            .into()
    }
}
