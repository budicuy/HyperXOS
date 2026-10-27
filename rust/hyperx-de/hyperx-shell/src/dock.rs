use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, text};
use iced::{Color, Element};

use crate::theme::{self, ACCENT_BLUE};

#[derive(Debug, Clone)]
pub struct DockItem {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub is_running: bool,
    pub command: &'static str,
}

#[derive(Debug, Clone)]
pub enum DockAction {
    LaunchApp(&'static str),
}

pub struct Dock {
    pub items: Vec<DockItem>,
}

impl Default for Dock {
    fn default() -> Self {
        Self {
            items: vec![
                DockItem {
                    id: "finder",
                    name: "Finder",
                    icon: "📁",
                    is_running: true,
                    command: "hyperx-file-manager",
                },
                DockItem {
                    id: "terminal",
                    name: "Terminal",
                    icon: "💻",
                    is_running: true,
                    command: "alacritty",
                },
                DockItem {
                    id: "tools",
                    name: "HyperX Tools",
                    icon: "🛠",
                    is_running: false,
                    command: "hyper-x-tools",
                },
                DockItem {
                    id: "installer",
                    name: "HyperX Installer",
                    icon: "💿",
                    is_running: false,
                    command: "hyper-x-installer",
                },
                DockItem {
                    id: "browser",
                    name: "Web Browser",
                    icon: "🌐",
                    is_running: true,
                    command: "firefox",
                },
                DockItem {
                    id: "editor",
                    name: "Code Editor",
                    icon: "📝",
                    is_running: false,
                    command: "code",
                },
                DockItem {
                    id: "launcher",
                    name: "Spotlight",
                    icon: "🔍",
                    is_running: false,
                    command: "hyperx-launcher",
                },
                DockItem {
                    id: "settings",
                    name: "System Settings",
                    icon: "⚙",
                    is_running: false,
                    command: "hyperx-settings",
                },
                DockItem {
                    id: "trash",
                    name: "Trash",
                    icon: "🗑",
                    is_running: false,
                    command: "hyperx-file-manager --trash",
                },
            ],
        }
    }
}

impl Dock {
    pub fn view<Message: 'static + Clone>(
        &self,
        map_fn: impl Fn(DockAction) -> Message + Copy + 'static,
    ) -> Element<'_, Message> {
        let mut dock_icons = row![].spacing(6).align_y(Alignment::Center);

        for item in &self.items {
            let indicator = if item.is_running {
                text("●").size(6).color(ACCENT_BLUE)
            } else {
                text(" ").size(6).color(Color::TRANSPARENT)
            };

            let icon_col = column![
                text(item.icon).size(26),
                indicator,
            ]
            .spacing(2)
            .align_x(Alignment::Center);

            let btn = button(
                container(icon_col)
                    .padding([6, 8])
            )
            .padding(0)
            .style(button::text)
            .on_press(map_fn(DockAction::LaunchApp(item.command)));

            dock_icons = dock_icons.push(btn);
        }

        container(dock_icons)
            .padding([4, 12])
            .style(theme::dock_style)
            .into()
    }
}
