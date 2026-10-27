use std::process::Command;
use std::time::Duration;

use iced::widget::{button, column, container, row, stack, Space};
use iced::{Element, Length, Padding, Subscription, Task, Theme};

use crate::app_menu::{AppMenu, AppMenuAction};
use crate::control_center::{ControlCenter, ControlCenterAction};
use crate::dock::{Dock, DockAction};
use crate::dynamic_island::{DynamicIsland, IslandAction};
use crate::notifications::{NotifAction, NotificationCenter};
use crate::top_bar::{TopBar, TopBarAction};

#[derive(Debug, Clone)]
pub enum Message {
    TopBar(TopBarAction),
    Island(IslandAction),
    ControlCenter(ControlCenterAction),
    AppMenu(AppMenuAction),
    Notifications(NotifAction),
    Dock(DockAction),
    Tick,
    DismissPopups,
}

pub struct ShellApp {
    pub top_bar: TopBar,
    pub island: DynamicIsland,
    pub control_center: ControlCenter,
    pub notifications: NotificationCenter,
    pub dock: Dock,
    pub is_control_center_open: bool,
    pub is_app_menu_open: bool,
    pub is_notifications_open: bool,
}

impl Default for ShellApp {
    fn default() -> Self {
        Self {
            top_bar: TopBar::default(),
            island: DynamicIsland::default(),
            control_center: ControlCenter::default(),
            notifications: NotificationCenter::default(),
            dock: Dock::default(),
            is_control_center_open: false,
            is_app_menu_open: false,
            is_notifications_open: false,
        }
    }
}

impl ShellApp {
    pub fn title(&self) -> String {
        "HyperX Desktop Shell".to_string()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.top_bar.update_time();
                self.island.tick();
            }
            Message::TopBar(action) => match action {
                TopBarAction::ToggleAppMenu => {
                    self.is_app_menu_open = !self.is_app_menu_open;
                    self.is_control_center_open = false;
                    self.is_notifications_open = false;
                }
                TopBarAction::ToggleControlCenter => {
                    self.is_control_center_open = !self.is_control_center_open;
                    self.is_app_menu_open = false;
                    self.is_notifications_open = false;
                }
                TopBarAction::ToggleNotifications => {
                    self.is_notifications_open = !self.is_notifications_open;
                    self.is_app_menu_open = false;
                    self.is_control_center_open = false;
                }
                TopBarAction::Island(act) => {
                    self.island.update(act);
                }
                TopBarAction::MenuAction(_name) => {
                    // Menu actions placeholder
                }
            },
            Message::Island(action) => {
                self.island.update(action);
            }
            Message::ControlCenter(action) => {
                // Intercept volume/brightness changes to reflect on dynamic island
                match &action {
                    ControlCenterAction::SetBrightness(val) => {
                        self.island.show_brightness_toast(*val as u8);
                    }
                    ControlCenterAction::SetVolume(val) => {
                        self.island.show_volume_toast(*val as u8);
                    }
                    _ => {}
                }
                self.control_center.update(action);
            }
            Message::AppMenu(action) => {
                self.is_app_menu_open = false;
                match action {
                    AppMenuAction::AboutHyperX => {
                        self.island.show_notification(
                            "HyperXOS",
                            "HyperXOS Desktop 2026",
                            "Pure Rust Wayland Desktop Environment powered by Smithay & Iced.",
                        );
                    }
                    AppMenuAction::SystemSettings => {
                        self.is_control_center_open = true;
                    }
                    AppMenuAction::PackageStore => {
                        let _ = Command::new("hyper-x-tools").spawn();
                    }
                    AppMenuAction::Sleep => {
                        let _ = Command::new("systemctl").arg("suspend").spawn();
                    }
                    AppMenuAction::Restart => {
                        let _ = Command::new("systemctl").arg("reboot").spawn();
                    }
                    AppMenuAction::Shutdown => {
                        let _ = Command::new("systemctl").arg("poweroff").spawn();
                    }
                    AppMenuAction::LockScreen => {
                        let _ = Command::new("loginctl").arg("lock-session").spawn();
                    }
                    AppMenuAction::Logout => {
                        let _ = Command::new("loginctl").arg("terminate-user").arg("").spawn();
                    }
                    AppMenuAction::Dismiss => {}
                }
            }
            Message::Notifications(action) => {
                self.notifications.update(action);
            }
            Message::Dock(action) => match action {
                DockAction::LaunchApp(cmd) => {
                    if cmd == "hyperx-launcher" {
                        let _ = Command::new("hyperx-launcher").spawn();
                    } else if cmd == "hyperx-settings" {
                        self.is_control_center_open = !self.is_control_center_open;
                    } else {
                        let parts: Vec<&str> = cmd.split_whitespace().collect();
                        if let Some((bin, args)) = parts.split_first() {
                            let _ = Command::new(bin).args(args).spawn();
                        }
                    }
                }
            },
            Message::DismissPopups => {
                self.is_app_menu_open = false;
                self.is_control_center_open = false;
                self.is_notifications_open = false;
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Base Desktop Layout
        let topbar_elem = self
            .top_bar
            .view(&self.island, Message::TopBar);

        let dock_elem = self.dock.view(Message::Dock);

        // Center space can be clicked to dismiss any open popups
        let center_space = button(
            container(Space::new().width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .padding(0)
        .style(button::text)
        .on_press(Message::DismissPopups);

        let desktop_column = column![
            topbar_elem,
            center_space,
            container(dock_elem)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .padding(Padding {
                    top: 0.0,
                    right: 0.0,
                    bottom: 16.0,
                    left: 0.0,
                }),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        // Overlay popups (Control Center, App Menu, Notification Center)
        let mut layers = vec![desktop_column.into()];

        if self.is_app_menu_open {
            let menu = AppMenu::view(Message::AppMenu);
            let menu_overlay = column![
                Space::new().height(36),
                row![Space::new().width(8), menu],
            ];
            layers.push(menu_overlay.into());
        }

        if self.is_control_center_open {
            let cc = self
                .control_center
                .view(Message::ControlCenter);
            let cc_overlay = column![
                Space::new().height(38),
                row![
                    Space::new().width(Length::Fill),
                    cc,
                    Space::new().width(12),
                ],
            ];
            layers.push(cc_overlay.into());
        }

        if self.is_notifications_open {
            let notif = self
                .notifications
                .view(Message::Notifications);
            let notif_overlay = column![
                Space::new().height(38),
                row![
                    Space::new().width(Length::Fill),
                    notif,
                    Space::new().width(12),
                ],
            ];
            layers.push(notif_overlay.into());
        }

        stack(layers)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
