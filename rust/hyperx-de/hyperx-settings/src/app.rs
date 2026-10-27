use std::process::Command;

use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Color, Element, Length, Task, Theme};

use crate::about_view::{self, AboutMessage, SystemSpecs};
use crate::appearance_view::{self, AppearanceMessage, AppearanceSettings};
use crate::bluetooth_view::{self, BluetoothMessage, BluetoothSettings};
use crate::displays_view::{self, DisplaysMessage, DisplaysSettings};
use crate::keyboard_view;
use crate::network::NetworkManager;
use crate::network_view::{self, NetworkMessage};
use crate::power_view::{self, PowerMessage, PowerSettings};
use crate::sound_view::{self, SoundMessage, SoundSettings};
use crate::theme::{self, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Network,
    Bluetooth,
    Displays,
    Appearance,
    Sound,
    Power,
    Keyboard,
    About,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectPage(Page),
    SearchChanged(String),
    Network(NetworkMessage),
    Bluetooth(BluetoothMessage),
    Displays(DisplaysMessage),
    Appearance(AppearanceMessage),
    Sound(SoundMessage),
    Power(PowerMessage),
    About(AboutMessage),
}

#[derive(Clone)]
pub struct SettingsApp {
    pub current_page: Page,
    pub search_query: String,
    pub network: NetworkManager,
    pub bluetooth: BluetoothSettings,
    pub displays: DisplaysSettings,
    pub appearance: AppearanceSettings,
    pub sound: SoundSettings,
    pub power: PowerSettings,
    pub specs: SystemSpecs,
}

impl Default for SettingsApp {
    fn default() -> Self {
        Self {
            current_page: Page::Network,
            search_query: String::new(),
            network: NetworkManager::default(),
            bluetooth: BluetoothSettings::default(),
            displays: DisplaysSettings::default(),
            appearance: AppearanceSettings::default(),
            sound: SoundSettings::default(),
            power: PowerSettings::default(),
            specs: SystemSpecs::default(),
        }
    }
}

impl SettingsApp {
    pub fn title(&self) -> String {
        "System Settings".to_string()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectPage(page) => {
                self.current_page = page;
            }
            Message::SearchChanged(q) => {
                self.search_query = q;
            }
            Message::Network(net_msg) => match net_msg {
                NetworkMessage::ToggleNetworking => {
                    self.network.toggle_networking();
                }
                NetworkMessage::ToggleWifi => {
                    self.network.toggle_wifi();
                }
                NetworkMessage::RefreshNetworks => {
                    self.network.refresh();
                }
                NetworkMessage::SelectWifi(ssid) => {
                    self.network.connecting_ssid = Some(ssid);
                    self.network.password_input.clear();
                }
                NetworkMessage::PasswordChanged(p) => {
                    self.network.password_input = p;
                }
                NetworkMessage::ConnectWifi => {
                    if let Some(ssid) = self.network.connecting_ssid.clone() {
                        let password = self.network.password_input.clone();
                        let _ = self.network.connect_to_wifi(&ssid, &password);
                        self.network.connecting_ssid = None;
                    }
                }
                NetworkMessage::CancelConnect => {
                    self.network.connecting_ssid = None;
                }
                NetworkMessage::DisconnectDevice(dev) => {
                    self.network.disconnect_device(&dev);
                }
            },
            Message::Bluetooth(bt_msg) => match bt_msg {
                BluetoothMessage::ToggleBluetooth => {
                    self.bluetooth.enabled = !self.bluetooth.enabled;
                }
                BluetoothMessage::ToggleDevice(name) => {
                    if let Some(dev) = self.bluetooth.devices.iter_mut().find(|d| d.name == name) {
                        dev.connected = !dev.connected;
                    }
                }
            },
            Message::Displays(d_msg) => match d_msg {
                DisplaysMessage::SetResolution(res) => self.displays.resolution = res,
                DisplaysMessage::SetRefreshRate(hz) => self.displays.refresh_rate = hz,
                DisplaysMessage::SetScale(scale) => self.displays.scale = scale,
                DisplaysMessage::ToggleNightLight => self.displays.night_light_enabled = !self.displays.night_light_enabled,
                DisplaysMessage::SetNightLightWarmth(w) => self.displays.night_light_warmth = w,
            },
            Message::Appearance(a_msg) => match a_msg {
                AppearanceMessage::SetTheme(mode) => self.appearance.theme_mode = mode,
                AppearanceMessage::SetAccent(acc) => self.appearance.accent = acc,
                AppearanceMessage::SetDockPosition(pos) => self.appearance.dock_position = pos,
                AppearanceMessage::SetDockSize(size) => self.appearance.dock_size = size,
                AppearanceMessage::SetGlassBlur(blur) => self.appearance.glass_blur = blur,
            },
            Message::Sound(s_msg) => match s_msg {
                SoundMessage::SetVolume(val) => {
                    self.sound.volume = val;
                    self.sound.is_muted = false;
                }
                SoundMessage::ToggleMute => self.sound.is_muted = !self.sound.is_muted,
                SoundMessage::SetMicVolume(val) => self.sound.mic_volume = val,
                SoundMessage::SelectOutput(dev) => self.sound.output_device = dev,
            },
            Message::Power(p_msg) => match p_msg {
                PowerMessage::SetProfile(prof) => self.power.profile = prof,
                PowerMessage::SetSleepTimeout(to) => self.power.sleep_minutes = to,
                PowerMessage::ToggleOptimizeBattery => self.power.optimize_battery = !self.power.optimize_battery,
            },
            Message::About(AboutMessage::CheckForUpdates) => {
                let _ = Command::new("hyper-x-tools").spawn();
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let user_profile = container(
            row![
                text("👤").size(24),
                column![
                    text("HyperX User").size(13).color(TEXT_PRIMARY),
                    text("Local Account").size(10).color(TEXT_SECONDARY),
                ]
                .spacing(2),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .padding([8, 12])
        .style(theme::card_style);

        let search_bar = text_input("Search Settings...", &self.search_query)
            .on_input(Message::SearchChanged)
            .padding([6, 10])
            .size(12);

        // Sidebar Navigation Items
        let nav_item = |page: Page, icon: &'static str, label: &'static str| {
            let is_active = self.current_page == page;
            button(
                container(
                    row![
                        text(icon).size(16),
                        text(label).size(12).color(if is_active { Color::WHITE } else { TEXT_PRIMARY }),
                        Space::new().width(Length::Fill),
                        text("›").size(14).color(TEXT_SECONDARY),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([8, 10])
                .style(if is_active { theme::active_item_style } else { theme::inactive_item_style }),
            )
            .padding(0)
            .style(button::text)
            .on_press(Message::SelectPage(page))
        };

        let nav_col = column![
            nav_item(Page::Network, "📶", "Wi-Fi & Network"),
            nav_item(Page::Bluetooth, "ᛒ", "Bluetooth"),
            nav_item(Page::Displays, "🖥", "Displays"),
            nav_item(Page::Appearance, "🎨", "Appearance"),
            nav_item(Page::Sound, "🔊", "Sound"),
            nav_item(Page::Power, "🔋", "Battery & Power"),
            nav_item(Page::Keyboard, "⌨", "Keyboard & Shortcuts"),
            nav_item(Page::About, "⚡", "About HyperXOS"),
        ]
        .spacing(4);

        let sidebar = container(column![user_profile, search_bar, nav_col].spacing(12))
            .width(260)
            .height(Length::Fill)
            .padding(14)
            .style(theme::sidebar_style);

        // Content View Router
        let content_view: Element<'_, Message> = match self.current_page {
            Page::Network => network_view::view(&self.network, Message::Network),
            Page::Bluetooth => bluetooth_view::view(&self.bluetooth, Message::Bluetooth),
            Page::Displays => displays_view::view(&self.displays, Message::Displays),
            Page::Appearance => appearance_view::view(&self.appearance, Message::Appearance),
            Page::Sound => sound_view::view(&self.sound, Message::Sound),
            Page::Power => power_view::view(&self.power, Message::Power),
            Page::Keyboard => keyboard_view::view(),
            Page::About => about_view::view(&self.specs, Message::About),
        };

        let content_area = container(content_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(24)
            .style(theme::content_style);

        row![sidebar, content_area].width(Length::Fill).height(Length::Fill).into()
    }
}
