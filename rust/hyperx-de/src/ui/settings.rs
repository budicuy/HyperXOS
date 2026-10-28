use iced::widget::{button, column, container, mouse_area, row, slider, text, text_input};
use iced::{Alignment, Background, Color, Element, Length};

use hyperx_ui_kit::{
    active_toggle_button_style, fluent_sidebar_container_style, fluent_sidebar_nav_item,
    fluent_window_header_bar, fluent_window_style, glass_action_button, glass_card_accent_style,
    glass_card_style, glass_input_style, glass_scrollable, glass_slider_style, icons, logo_image,
    metric_card, primary_button_style, status_pill, svg_icon, ACCENT_AMBER, ACCENT_CYAN,
    ACCENT_GREEN, ACCENT_PURPLE, ACCENT_TEAL, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD,
    FONT_DISPLAY_BOLD, TEXT_MUTED, TEXT_SUB,
};

use crate::sys::network::NetworkState;
use crate::ui::{DesktopWindowId, Message, WallpaperTheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    Network,
    Bluetooth,
    Audio,
    Display,
    Appearance,
    About,
}

#[derive(Debug, Clone)]
pub struct SettingsViewState {
    pub active_tab: SettingsTab,
    pub selected_ssid: Option<String>,
    pub wifi_password_input: String,
    pub network_status_msg: String,
    pub custom_wallpaper_input: String,
}

impl Default for SettingsViewState {
    fn default() -> Self {
        Self {
            active_tab: SettingsTab::Network,
            selected_ssid: None,
            wifi_password_input: String::new(),
            network_status_msg: "NetworkManager aktif dan siap.".to_string(),
            custom_wallpaper_input: String::new(),
        }
    }
}

pub fn view_settings<'a>(
    state: &'a SettingsViewState,
    net: &'a NetworkState,
    bluetooth_enabled: bool,
    volume: f32,
    muted: bool,
    output_device: &'a str,
    brightness: f32,
    resolution: &'a str,
    refresh_rate: &'a str,
    night_light: bool,
    wallpaper_theme: WallpaperTheme,
    dock_icon_size: f32,
    cpu_brand: &'a str,
    cpu_usage: f32,
    ram_used_gb: f32,
    ram_total_gb: f32,
    wayland_socket: &'a str,
    w: Length,
    h: Length,
) -> Element<'a, Message> {
    // 1. Modern Fluent Sidebar
    let brand = container(
        row![
            logo_image(22.0),
            column![
                text("Pengaturan")
                    .size(13)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(Color::from_rgb(0.12, 0.14, 0.17)),
                    }),
                text("HyperX Fluent Control")
                    .size(10)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(Color::from_rgb(0.45, 0.50, 0.58)),
                    }),
            ]
            .spacing(1),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([4, 6]);

    let sep = container("")
        .width(Length::Fill)
        .height(1)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.08))),
            ..container::Style::default()
        });

    let section_label = |name: &'static str| {
        text(name)
            .size(10)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(Color::from_rgb(0.55, 0.58, 0.64)),
            })
    };

    let nav_links = column![
        section_label("KONEKTIVITAS"),
        fluent_sidebar_nav_item(
            "Wi-Fi & Jaringan",
            icons::WIFI,
            state.active_tab == SettingsTab::Network,
            Color::from_rgb(0.0, 0.47, 0.83),
            Message::SelectSettingsTab(SettingsTab::Network),
        ),
        fluent_sidebar_nav_item(
            "Bluetooth",
            icons::BLUETOOTH,
            state.active_tab == SettingsTab::Bluetooth,
            Color::from_rgb(0.55, 0.36, 0.96),
            Message::SelectSettingsTab(SettingsTab::Bluetooth),
        ),
        section_label("PERANGKAT & MULTIMEDIA"),
        fluent_sidebar_nav_item(
            "Suara & Audio",
            icons::VOLUME_HIGH,
            state.active_tab == SettingsTab::Audio,
            Color::from_rgb(0.96, 0.62, 0.04),
            Message::SelectSettingsTab(SettingsTab::Audio),
        ),
        fluent_sidebar_nav_item(
            "Tampilan & Layar",
            icons::GPU,
            state.active_tab == SettingsTab::Display,
            Color::from_rgb(0.06, 0.65, 0.38),
            Message::SelectSettingsTab(SettingsTab::Display),
        ),
        section_label("DESKTOP & SISTEM"),
        fluent_sidebar_nav_item(
            "Wallpaper & Dock",
            icons::SPARKLES,
            state.active_tab == SettingsTab::Appearance,
            Color::from_rgb(0.55, 0.36, 0.96),
            Message::SelectSettingsTab(SettingsTab::Appearance),
        ),
        fluent_sidebar_nav_item(
            "Tentang Sistem",
            icons::INFO,
            state.active_tab == SettingsTab::About,
            Color::from_rgb(0.0, 0.47, 0.83),
            Message::SelectSettingsTab(SettingsTab::About),
        ),
    ]
    .spacing(4);

    let sidebar_footer = container(
        column![
            text("HyperXOS 2026.10")
                .size(11)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.12, 0.14, 0.17)),
                }),
            text("Smithay + Iced 0.14 WGPU")
                .size(9)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.45, 0.50, 0.58)),
                }),
        ]
        .spacing(3),
    )
    .padding([10, 12])
    .width(Length::Fill)
    .style(glass_card_style);

    let sidebar = container(
        column![
            brand,
            sep,
            nav_links,
            container("").height(Length::Fill),
            sidebar_footer,
        ]
        .spacing(12)
        .padding(14),
    )
    .width(Length::Fixed(225.0))
    .height(Length::Fill)
    .style(fluent_sidebar_container_style);

    // 2. Right Panel Header
    let (tab_title, _tab_sub, tab_icon, tab_accent) = match state.active_tab {
        SettingsTab::Network => (
            "Wi-Fi & Jaringan (NetworkManager)",
            "Kelola koneksi nirkabel, alamat IPv4, dan pemindaian SSID",
            icons::WIFI,
            ACCENT_CYAN,
        ),
        SettingsTab::Bluetooth => (
            "Perangkat Bluetooth (BlueZ)",
            "Pasangkan audio nirkabel, kontroler game, dan periferal",
            icons::BLUETOOTH,
            ACCENT_PURPLE,
        ),
        SettingsTab::Audio => (
            "Suara & Audio (PipeWire)",
            "Atur volume master, perangkat output, dan uji kanal audio",
            icons::VOLUME_HIGH,
            ACCENT_AMBER,
        ),
        SettingsTab::Display => (
            "Tampilan, Resolusi & Kecerahan",
            "Konfigurasi resolusi monitor, refresh rate, dan cahaya malam",
            icons::GPU,
            ACCENT_GREEN,
        ),
        SettingsTab::Appearance => (
            "Personalisasi Wallpaper & macOS Dock",
            "Ganti tema latar belakang desktop dan ukuran taskbar secara langsung",
            icons::SPARKLES,
            ACCENT_PURPLE,
        ),
        SettingsTab::About => (
            "Tentang HyperXOS Linux",
            "Informasi perangkat keras, Wayland compositor, dan utilitas sistem",
            icons::INFO,
            ACCENT_CYAN,
        ),
    };

    let top_header = mouse_area(fluent_window_header_bar(
        "HyperX Settings",
        tab_title,
        tab_icon,
        tab_accent,
        Message::MinimizeWindow(DesktopWindowId::Settings),
        Message::ToggleMaximizeWindow(DesktopWindowId::Settings),
        Message::CloseWindow(DesktopWindowId::Settings),
    ))
    .on_press(Message::StartWindowDrag(DesktopWindowId::Settings))
    .on_release(Message::EndWindowDrag);

    // 3. Page Content
    let page_body: Element<'a, Message> = match state.active_tab {
        SettingsTab::Network => {
            let status_card = container(
                row![
                    svg_icon(
                        if net.wifi_enabled {
                            icons::WIFI
                        } else {
                            icons::WIFI_OFF
                        },
                        24.0,
                        if net.wifi_enabled {
                            ACCENT_GREEN
                        } else {
                            ACCENT_AMBER
                        },
                    ),
                    column![
                        text(if net.wifi_enabled {
                            format!("Terhubung: {}", net.active_ssid)
                        } else {
                            "Radio Wi-Fi Dinonaktifkan".to_string()
                        })
                        .size(13)
                        .font(FONT_DISPLAY_BOLD),
                        text(format!(
                            "IPv4: {}   •   Gateway: {}   •   {}",
                            net.ip_address, net.gateway, state.network_status_msg
                        ))
                        .size(11)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style {
                            color: Some(TEXT_SUB),
                        }),
                    ]
                    .spacing(3),
                    container("").width(Length::Fill),
                    button(
                        row![
                            svg_icon(icons::REFRESH, 13.0, ACCENT_CYAN),
                            text("Pindai Ulang").size(11).font(FONT_BODY_SEMIBOLD),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center),
                    )
                    .padding([7, 12])
                    .style(glass_action_button)
                    .on_press(Message::RefreshNetwork),
                    button(
                        text(if net.wifi_enabled {
                            "Matikan Wi-Fi"
                        } else {
                            "Aktifkan Wi-Fi"
                        })
                        .size(11)
                        .font(FONT_DISPLAY_BOLD),
                    )
                    .padding([7, 14])
                    .style(active_toggle_button_style(net.wifi_enabled, ACCENT_GREEN))
                    .on_press(Message::ToggleWifiRadio),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
                .padding(14),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_CYAN));

            let mut ssid_list = column![].spacing(8);
            for item in &net.networks {
                let is_sel = state.selected_ssid.as_deref() == Some(item.ssid.as_str());
                let badge = status_pill(
                    format!("{}%  •  {}", item.signal, item.security),
                    if item.is_connected {
                        ACCENT_GREEN
                    } else {
                        ACCENT_CYAN
                    },
                    item.is_connected,
                );

                let row_card = container(
                    row![
                        svg_icon(
                            icons::WIFI,
                            18.0,
                            if item.is_connected {
                                ACCENT_GREEN
                            } else {
                                ACCENT_CYAN
                            }
                        ),
                        column![
                            text(&item.ssid).size(13).font(FONT_DISPLAY_BOLD),
                            text(if item.is_connected {
                                "Jaringan Aktif Saat Ini"
                            } else {
                                "Tersedia untuk dihubungkan"
                            })
                            .size(10)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(TEXT_MUTED),
                            }),
                        ]
                        .spacing(2),
                        container("").width(Length::Fill),
                        badge,
                        button(
                            text(if item.is_connected {
                                "Terhubung"
                            } else if is_sel {
                                "Dipilih"
                            } else {
                                "Pilih SSID"
                            })
                            .size(11)
                            .font(FONT_BODY_SEMIBOLD),
                        )
                        .padding([6, 12])
                        .style(active_toggle_button_style(
                            is_sel || item.is_connected,
                            ACCENT_CYAN,
                        ))
                        .on_press(Message::SelectWifiSsid(item.ssid.clone())),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center)
                    .padding(12),
                )
                .width(Length::Fill)
                .style(glass_card_style);

                ssid_list = ssid_list.push(row_card);
            }

            let connect_box: Element<'a, Message> = if let Some(ssid) = &state.selected_ssid {
                container(
                    row![
                        svg_icon(icons::LOCK, 16.0, ACCENT_AMBER),
                        text(format!("Hubungkan ke '{}':", ssid))
                            .size(12)
                            .font(FONT_DISPLAY_BOLD),
                        text_input("Masukkan kata sandi WPA2/WPA3 (kosongkan jika Open)...", &state.wifi_password_input)
                            .on_input(Message::WifiPasswordChanged)
                            .secure(true)
                            .padding([7, 12])
                            .size(12)
                            .width(Length::Fill)
                            .style(glass_input_style),
                        button(
                            row![
                                svg_icon(icons::CHECK, 13.0, ACCENT_GREEN),
                                text("Sambungkan").size(11).font(FONT_DISPLAY_BOLD),
                            ]
                            .spacing(6)
                            .align_y(Alignment::Center),
                        )
                        .padding([7, 14])
                        .style(primary_button_style(ACCENT_GREEN))
                        .on_press(Message::ConnectSelectedWifi),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center)
                    .padding(12),
                )
                .width(Length::Fill)
                .style(glass_card_accent_style(ACCENT_AMBER))
                .into()
            } else {
                container("").into()
            };

            column![status_card, connect_box, ssid_list]
                .spacing(12)
                .into()
        }

        SettingsTab::Bluetooth => {
            let bt_card = container(
                row![
                    svg_icon(icons::BLUETOOTH, 24.0, ACCENT_PURPLE),
                    column![
                        text(if bluetooth_enabled {
                            "Adaptor Bluetooth Aktif (BlueZ)"
                        } else {
                            "Adaptor Bluetooth Nonaktif"
                        })
                        .size(13)
                        .font(FONT_DISPLAY_BOLD),
                        text("Dapat ditemukan sebagai 'HyperXOS-Desktop'")
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(TEXT_SUB),
                            }),
                    ]
                    .spacing(3),
                    container("").width(Length::Fill),
                    button(
                        text(if bluetooth_enabled {
                            "Matikan Bluetooth"
                        } else {
                            "Aktifkan Bluetooth"
                        })
                        .size(11)
                        .font(FONT_DISPLAY_BOLD),
                    )
                    .padding([8, 16])
                    .style(active_toggle_button_style(bluetooth_enabled, ACCENT_PURPLE))
                    .on_press(Message::ToggleBluetooth),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
                .padding(16),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_PURPLE));

            column![bt_card].spacing(12).into()
        }

        SettingsTab::Audio => {
            let audio_card = container(
                column![
                    row![
                        svg_icon(
                            if muted {
                                icons::VOLUME_MUTE
                            } else {
                                icons::VOLUME_HIGH
                            },
                            22.0,
                            ACCENT_CYAN
                        ),
                        column![
                            text("Perangkat Output: ").size(11).font(FONT_BODY_MEDIUM),
                            text(output_device).size(13).font(FONT_DISPLAY_BOLD),
                        ]
                        .spacing(2),
                        container("").width(Length::Fill),
                        button(
                            text(if muted {
                                "Suara Dibisukan (Unmute)"
                            } else {
                                "Bisukan (Mute)"
                            })
                            .size(11)
                            .font(FONT_DISPLAY_BOLD),
                        )
                        .padding([7, 14])
                        .style(active_toggle_button_style(muted, ACCENT_AMBER))
                        .on_press(Message::ToggleMute),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                    row![
                        text("Volume Master:")
                            .size(12)
                            .font(FONT_BODY_SEMIBOLD)
                            .width(Length::Fixed(120.0)),
                        slider(0.0..=100.0, volume, Message::VolumeChanged)
                            .style(glass_slider_style(ACCENT_CYAN)),
                        text(format!("{:.0}%", volume))
                            .size(13)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style {
                                color: Some(ACCENT_CYAN),
                            }),
                    ]
                    .spacing(12)
                    .align_y(Alignment::Center),
                ]
                .spacing(14)
                .padding(16),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_CYAN));

            column![audio_card].spacing(12).into()
        }

        SettingsTab::Display => {
            let disp_card = container(
                column![
                    row![
                        svg_icon(icons::GPU, 22.0, ACCENT_GREEN),
                        column![
                            text(format!("Resolusi Aktif: {}", resolution))
                                .size(13)
                                .font(FONT_DISPLAY_BOLD),
                            text(format!("Refresh Rate: {}  •  Akselerasi WGPU Direct-GPU", refresh_rate))
                                .size(11)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style {
                                    color: Some(TEXT_SUB),
                                }),
                        ]
                        .spacing(2),
                        container("").width(Length::Fill),
                        button(
                            text(if night_light {
                                "Night Light: Aktif"
                            } else {
                                "Night Light: Nonaktif"
                            })
                            .size(11)
                            .font(FONT_DISPLAY_BOLD),
                        )
                        .padding([7, 14])
                        .style(active_toggle_button_style(night_light, ACCENT_AMBER))
                        .on_press(Message::ToggleNightLight),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                    row![
                        svg_icon(icons::SUN_BRIGHTNESS, 16.0, ACCENT_AMBER),
                        text("Kecerahan Panel:")
                            .size(12)
                            .font(FONT_BODY_SEMIBOLD)
                            .width(Length::Fixed(130.0)),
                        slider(10.0..=100.0, brightness, Message::BrightnessChanged)
                            .style(glass_slider_style(ACCENT_AMBER)),
                        text(format!("{:.0}%", brightness))
                            .size(13)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style {
                                color: Some(ACCENT_AMBER),
                            }),
                    ]
                    .spacing(12)
                    .align_y(Alignment::Center),
                ]
                .spacing(14)
                .padding(16),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_GREEN));

            column![disp_card].spacing(12).into()
        }

        SettingsTab::Appearance => {
            let wp_card = container(
                column![
                    row![
                        svg_icon(icons::SPARKLES, 18.0, ACCENT_PURPLE),
                        text("Tema Gradasi & Custom Wallpaper Desktop")
                            .size(13)
                            .font(FONT_DISPLAY_BOLD),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                    row![
                        button(
                            text("HyperX Cyber Crimson")
                                .size(11)
                                .font(FONT_DISPLAY_BOLD),
                        )
                        .width(Length::Fill)
                        .padding([9, 10])
                        .style(active_toggle_button_style(
                            wallpaper_theme == WallpaperTheme::CyberCrimson,
                            ACCENT_AMBER,
                        ))
                        .on_press(Message::SelectWallpaper(WallpaperTheme::CyberCrimson)),
                        button(
                            text("macOS Deep Nebula")
                                .size(11)
                                .font(FONT_DISPLAY_BOLD),
                        )
                        .width(Length::Fill)
                        .padding([9, 10])
                        .style(active_toggle_button_style(
                            wallpaper_theme == WallpaperTheme::DeepNebula,
                            ACCENT_CYAN,
                        ))
                        .on_press(Message::SelectWallpaper(WallpaperTheme::DeepNebula)),
                        button(
                            text("Emerald Aurora")
                                .size(11)
                                .font(FONT_DISPLAY_BOLD),
                        )
                        .width(Length::Fill)
                        .padding([9, 10])
                        .style(active_toggle_button_style(
                            wallpaper_theme == WallpaperTheme::EmeraldAurora,
                            ACCENT_GREEN,
                        ))
                        .on_press(Message::SelectWallpaper(WallpaperTheme::EmeraldAurora)),
                        button(
                            text("Custom Image Wallpaper")
                                .size(11)
                                .font(FONT_DISPLAY_BOLD),
                        )
                        .width(Length::Fill)
                        .padding([9, 10])
                        .style(active_toggle_button_style(
                            wallpaper_theme == WallpaperTheme::CustomImage,
                            ACCENT_PURPLE,
                        ))
                        .on_press(Message::GenerateAndSetSampleCustomWallpaper),
                    ]
                    .spacing(8),
                ]
                .spacing(12)
                .padding(14),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_PURPLE));

            let custom_img_card = container(
                column![
                    row![
                        svg_icon(icons::IMAGE_ICON, 18.0, ACCENT_CYAN),
                        column![
                            text("Pasang Gambar Kustom sebagai Wallpaper (.PNG / .JPG / .WebP)")
                                .size(12)
                                .font(FONT_DISPLAY_BOLD),
                            text("Anda juga dapat mengklik kanan gambar apa pun di HyperX Files -> 'Jadikan Wallpaper Desktop'")
                                .size(10)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style {
                                    color: Some(TEXT_SUB),
                                }),
                        ]
                        .spacing(2),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                    row![
                        text_input(
                            "Masukkan path gambar (/home/hyperxos/Pictures/wallpaper.png)...",
                            &state.custom_wallpaper_input,
                        )
                        .on_input(Message::CustomWallpaperInputChanged)
                        .on_submit(Message::ApplyCustomWallpaperInput)
                        .padding([7, 12])
                        .size(11)
                        .width(Length::Fill)
                        .style(glass_input_style),
                        button(
                            row![
                                svg_icon(icons::CHECK, 13.0, ACCENT_GREEN),
                                text("Terapkan Gambar").size(11).font(FONT_DISPLAY_BOLD),
                            ]
                            .spacing(5)
                            .align_y(Alignment::Center),
                        )
                        .padding([7, 12])
                        .style(primary_button_style(ACCENT_GREEN))
                        .on_press(Message::ApplyCustomWallpaperInput),
                        button(
                            row![
                                svg_icon(icons::SPARKLES, 13.0, ACCENT_PURPLE),
                                text("Buat Wallpaper Neon 4K").size(11).font(FONT_DISPLAY_BOLD),
                            ]
                            .spacing(5)
                            .align_y(Alignment::Center),
                        )
                        .padding([7, 12])
                        .style(primary_button_style(ACCENT_PURPLE))
                        .on_press(Message::GenerateAndSetSampleCustomWallpaper),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
                .spacing(10)
                .padding(14),
            )
            .width(Length::Fill)
            .style(glass_card_style);

            let dock_card = container(
                row![
                    svg_icon(icons::SLIDERS, 18.0, ACCENT_CYAN),
                    text("Ukuran Ikon macOS Dock:")
                        .size(12)
                        .font(FONT_DISPLAY_BOLD)
                        .width(Length::Fixed(180.0)),
                    slider(26.0..=48.0, dock_icon_size, Message::DockSizeChanged)
                        .style(glass_slider_style(ACCENT_CYAN)),
                    text(format!("{:.0} px", dock_icon_size))
                        .size(12)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style {
                            color: Some(ACCENT_CYAN),
                        }),
                ]
                .spacing(12)
                .align_y(Alignment::Center)
                .padding(14),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_CYAN));

            column![wp_card, custom_img_card, dock_card]
                .spacing(10)
                .into()
        }

        SettingsTab::About => {
            let hero = container(
                row![
                    logo_image(54.0),
                    column![
                        text("HyperXOS Linux — Desktop Environment (hyperx-de)")
                            .size(16)
                            .font(FONT_DISPLAY_BOLD),
                        text(format!(
                            "Smithay Wayland Compositor ({})  •  Iced 0.14 WGPU Glassmorphism",
                            wayland_socket
                        ))
                        .size(11)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(ACCENT_CYAN),
                        }),
                        text(format!("Prosesor: {}", cpu_brand))
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style {
                                color: Some(TEXT_SUB),
                            }),
                    ]
                    .spacing(4),
                    container("").width(Length::Fill),
                    button(
                        row![
                            svg_icon(icons::ROCKET, 14.0, ACCENT_PURPLE),
                            text("Buka HyperXTools").size(11).font(FONT_DISPLAY_BOLD),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center),
                    )
                    .padding([8, 14])
                    .style(primary_button_style(ACCENT_PURPLE))
                    .on_press(Message::LaunchExternalApp("hyper-x-tools".to_string())),
                ]
                .spacing(14)
                .align_y(Alignment::Center)
                .padding(16),
            )
            .width(Length::Fill)
            .style(glass_card_accent_style(ACCENT_CYAN));

            let ram_ratio = if ram_total_gb > 0.0 {
                (ram_used_gb / ram_total_gb).clamp(0.0, 1.0)
            } else {
                0.2
            };

            let metrics_row = row![
                metric_card(
                    "Penggunaan CPU",
                    icons::CPU,
                    format!("{:.0}%", cpu_usage),
                    (cpu_usage / 100.0).clamp(0.0, 1.0),
                    cpu_brand,
                    Some("64-bit x86_64"),
                    ACCENT_CYAN,
                ),
                metric_card(
                    " Penggunaan Memori RAM",
                    icons::RAM,
                    format!("{:.1} GB", ram_used_gb),
                    ram_ratio,
                    format!("Total Kapasitas: {:.1} GB", ram_total_gb),
                    Some("ZRAM Aktif"),
                    ACCENT_TEAL,
                ),
            ]
            .spacing(12);

            column![hero, metrics_row].spacing(12).into()
        }
    };

    let scroll_page = glass_scrollable(page_body).height(Length::Fill);

    let right_panel = column![
        top_header,
        container(scroll_page)
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill),
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    container(row![sidebar, right_panel].width(Length::Fill).height(Length::Fill))
        .width(w)
        .height(h)
        .style(fluent_window_style)
        .into()
}
