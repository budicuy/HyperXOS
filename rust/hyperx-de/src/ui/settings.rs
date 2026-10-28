use iced::border::Radius;
use iced::widget::{button, column, container, mouse_area, row, slider, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

use hyperx_ui_kit::{
    fluent_button_style, fluent_card_style, fluent_input_style, fluent_ribbon_button_style,
    fluent_sidebar_container_style, fluent_sidebar_nav_item, fluent_slider_style,
    fluent_toggle_switch, fluent_window_header_bar, fluent_window_style, glass_scrollable, icons,
    logo_image, status_pill, svg_icon, ACCENT_AMBER, ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED,
    FLUENT_BORDER, FLUENT_PRIMARY, FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED, FONT_BODY_MEDIUM,
    FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD,
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
            network_status_msg: "NetworkManager aktif dan siap digunakan.".to_string(),
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
    // 1. Fluent Sidebar Header & Navigation
    let brand = container(
        row![
            logo_image(24.0),
            column![
                text("Pengaturan")
                    .size(14)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_DARK),
                    }),
                text("Windows Fluent Design")
                    .size(10)
                    .font(FONT_BODY_MEDIUM)
                    .style(|_| text::Style {
                        color: Some(FLUENT_TEXT_MUTED),
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
                color: Some(FLUENT_TEXT_MUTED),
            })
    };

    let nav_links = column![
        section_label("JARINGAN & KONEKSI"),
        fluent_sidebar_nav_item(
            "Wi-Fi & Jaringan",
            icons::WIFI,
            state.active_tab == SettingsTab::Network,
            FLUENT_PRIMARY,
            Message::SelectSettingsTab(SettingsTab::Network),
        ),
        fluent_sidebar_nav_item(
            "Bluetooth",
            icons::BLUETOOTH,
            state.active_tab == SettingsTab::Bluetooth,
            Color::from_rgb(0.35, 0.45, 0.95),
            Message::SelectSettingsTab(SettingsTab::Bluetooth),
        ),
        section_label("PERANGKAT KERAS"),
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
        section_label("PERSONALISASI & SISTEM"),
        fluent_sidebar_nav_item(
            "Personalisasi & Dock",
            icons::SPARKLES,
            state.active_tab == SettingsTab::Appearance,
            Color::from_rgb(0.55, 0.36, 0.96),
            Message::SelectSettingsTab(SettingsTab::Appearance),
        ),
        fluent_sidebar_nav_item(
            "Tentang Sistem",
            icons::INFO,
            state.active_tab == SettingsTab::About,
            FLUENT_PRIMARY,
            Message::SelectSettingsTab(SettingsTab::About),
        ),
    ]
    .spacing(4);

    let sidebar_footer = container(
        column![
            text("HyperXOS Fluent v2.2")
                .size(11)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
            text("Labwc Wayland + Iced WGPU")
                .size(9)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_MUTED),
                }),
        ]
        .spacing(2),
    )
    .padding([8, 10])
    .width(Length::Fill)
    .style(fluent_card_style);

    let sidebar = container(
        column![
            brand,
            sep,
            nav_links,
            container("").height(Length::Fill),
            sidebar_footer,
        ]
        .spacing(10)
        .padding(12),
    )
    .width(Length::Fixed(220.0))
    .height(Length::Fill)
    .style(fluent_sidebar_container_style);

    // 2. Right Panel Header
    let (tab_title, tab_icon, tab_accent) = match state.active_tab {
        SettingsTab::Network => ("Wi-Fi & Jaringan", icons::WIFI, FLUENT_PRIMARY),
        SettingsTab::Bluetooth => ("Perangkat Bluetooth", icons::BLUETOOTH, Color::from_rgb(0.35, 0.45, 0.95)),
        SettingsTab::Audio => ("Suara & Audio (PipeWire)", icons::VOLUME_HIGH, ACCENT_AMBER),
        SettingsTab::Display => ("Tampilan & Monitor", icons::GPU, ACCENT_GREEN),
        SettingsTab::Appearance => ("Personalisasi Wallpaper & Dock", icons::SPARKLES, ACCENT_PURPLE),
        SettingsTab::About => ("Tentang Perangkat & HyperXOS", icons::INFO, FLUENT_PRIMARY),
    };

    let top_header = mouse_area(fluent_window_header_bar(
        "Pengaturan",
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
            // A. Wi-Fi Status Card
            let status_card = container(
                row![
                    container(svg_icon(
                        if net.wifi_enabled { icons::WIFI } else { icons::WIFI_OFF },
                        24.0,
                        if net.wifi_enabled { FLUENT_PRIMARY } else { FLUENT_TEXT_MUTED },
                    ))
                    .width(Length::Fixed(44.0))
                    .height(Length::Fixed(44.0))
                    .center_x(Length::Fixed(44.0))
                    .center_y(Length::Fixed(44.0))
                    .style(move |_| container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.0, 0.47, 0.83, 0.08))),
                        border: Border::default().rounded(Radius::from(8.0)),
                        ..container::Style::default()
                    }),
                    column![
                        text(if net.wifi_enabled {
                            if net.active_ssid.is_empty() { "Wi-Fi Aktif (Tidak Terhubung)" } else { &net.active_ssid }
                        } else {
                            "Radio Wi-Fi Mati"
                        })
                        .size(13)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                        text(format!("IPv4: {}  •  Gateway: {}", net.ip_address, net.gateway))
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    ]
                    .spacing(2),
                    container("").width(Length::Fill),
                    button(
                        row![
                            svg_icon(icons::REFRESH, 13.0, FLUENT_PRIMARY),
                            text("Pindai Ulang").size(11).font(FONT_BODY_SEMIBOLD).style(|_| text::Style { color: Some(FLUENT_PRIMARY) }),
                        ]
                        .spacing(5)
                        .align_y(Alignment::Center),
                    )
                    .padding([6, 12])
                    .style(fluent_ribbon_button_style)
                    .on_press(Message::RefreshNetwork),
                    fluent_toggle_switch(net.wifi_enabled, Message::ToggleWifiRadio),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(14)
            .width(Length::Fill)
            .style(fluent_card_style);

            // B. Password connection card if SSID selected
            let connect_box: Option<Element<'a, Message>> = state.selected_ssid.as_ref().map(|ssid| {
                container(
                    column![
                        row![
                            svg_icon(icons::LOCK, 14.0, FLUENT_PRIMARY),
                            text(format!("Hubungkan ke '{}'", ssid))
                                .size(12)
                                .font(FONT_DISPLAY_BOLD)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center),
                        row![
                            text_input("Ketik Kata Sandi Wi-Fi...", &state.wifi_password_input)
                                .size(12)
                                .padding([7, 10])
                                .secure(true)
                                .style(fluent_input_style)
                                .on_input(Message::WifiPasswordChanged)
                                .on_submit(Message::ConnectSelectedWifi),
                            button(
                                text("Sambungkan")
                                    .size(11)
                                    .font(FONT_BODY_SEMIBOLD)
                                    .style(|_| text::Style { color: Some(Color::WHITE) }),
                            )
                            .padding([7, 14])
                            .style(fluent_button_style(FLUENT_PRIMARY, true))
                            .on_press(Message::ConnectSelectedWifi),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(8),
                )
                .padding(12)
                .width(Length::Fill)
                .style(fluent_card_style)
                .into()
            });

            // C. SSID List
            let mut list_col = column![].spacing(6);
            for ap in &net.networks {
                let is_active = ap.ssid == net.active_ssid && net.wifi_enabled;
                let is_selected = state.selected_ssid.as_deref() == Some(&ap.ssid);

                let ssid_card = button(
                    row![
                        svg_icon(
                            if !ap.security.is_empty() && !ap.security.eq_ignore_ascii_case("open") { icons::LOCK } else { icons::WIFI },
                            15.0,
                            if is_active { FLUENT_PRIMARY } else { FLUENT_TEXT_MUTED },
                        ),
                        column![
                            text(&ap.ssid)
                                .size(12)
                                .font(if is_active { FONT_DISPLAY_BOLD } else { FONT_BODY_SEMIBOLD })
                                .style(move |_| text::Style {
                                    color: Some(if is_active { FLUENT_PRIMARY } else { FLUENT_TEXT_DARK }),
                                }),
                            text(format!("Kekuatan Sinyal: {}%  •  {}", ap.signal, ap.security))
                                .size(10)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                        ]
                        .spacing(2),
                        container("").width(Length::Fill),
                        if is_active {
                            status_pill("Terhubung", FLUENT_PRIMARY, true)
                        } else if is_selected {
                            status_pill("Dipilih", ACCENT_AMBER, false)
                        } else {
                            status_pill("Pilih", Color::from_rgb(0.55, 0.58, 0.64), false)
                        },
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                )
                .padding([8, 12])
                .width(Length::Fill)
                .style(move |_theme, status| {
                    let (bg, border_color) = if is_selected || is_active {
                        (Color::WHITE, FLUENT_PRIMARY)
                    } else {
                        match status {
                            button::Status::Hovered => (Color::from_rgba(0.97, 0.98, 1.0, 1.0), Color::from_rgba(0.0, 0.0, 0.0, 0.12)),
                            _ => (Color::WHITE, FLUENT_BORDER),
                        }
                    };
                    button::Style {
                        background: Some(Background::Color(bg)),
                        border: Border::default().rounded(Radius::from(6.0)).width(1.0).color(border_color),
                        ..button::Style::default()
                    }
                })
                .on_press(Message::SelectWifiSsid(ap.ssid.clone()));

                list_col = list_col.push(ssid_card);
            }

            let mut main_col = column![status_card].spacing(10);
            if let Some(c) = connect_box {
                main_col = main_col.push(c);
            }
            main_col = main_col.push(
                text("JARINGAN TERSEDIA")
                    .size(11)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
            );
            main_col = main_col.push(list_col);

            glass_scrollable(main_col.spacing(10).padding([0, 4])).into()
        }

        SettingsTab::Bluetooth => {
            let bt_card = container(
                row![
                    container(svg_icon(icons::BLUETOOTH, 24.0, Color::from_rgb(0.35, 0.45, 0.95)))
                        .width(Length::Fixed(44.0))
                        .height(Length::Fixed(44.0))
                        .center_x(Length::Fixed(44.0))
                        .center_y(Length::Fixed(44.0))
                        .style(|_| container::Style {
                            background: Some(Background::Color(Color::from_rgba(0.35, 0.45, 0.95, 0.10))),
                            border: Border::default().rounded(Radius::from(8.0)),
                            ..container::Style::default()
                        }),
                    column![
                        text(if bluetooth_enabled { "Bluetooth Aktif" } else { "Bluetooth Nonaktif" })
                            .size(13)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                        text(if bluetooth_enabled {
                            "Dapat ditemukan sebagai 'HyperX-PC'. Siap memasangkan perangkat baru."
                        } else {
                            "Aktifkan Bluetooth untuk menghubungkan headset, keyboard, atau mouse nirkabel."
                        })
                        .size(11)
                        .font(FONT_BODY_MEDIUM)
                        .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    ]
                    .spacing(2),
                    container("").width(Length::Fill),
                    fluent_toggle_switch(bluetooth_enabled, Message::ToggleBluetooth),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(14)
            .width(Length::Fill)
            .style(fluent_card_style);

            let devices = [
                ("HyperX Cloud III Wireless Headset", "Audio & Mikrofon Terhubung", true, icons::VOLUME_HIGH),
                ("Logitech MX Master 3S Mouse", "Perangkat Input • Baterai 85%", true, icons::COMPUTER),
                ("Xbox Wireless Controller", "Terputus • Klik untuk menghubungkan", false, icons::SPARKLES),
            ];

            let mut dev_col = column![].spacing(6);
            for (name, desc, connected, ic) in devices {
                let dev_row = container(
                    row![
                        svg_icon(ic, 18.0, if connected { Color::from_rgb(0.35, 0.45, 0.95) } else { FLUENT_TEXT_MUTED }),
                        column![
                            text(name)
                                .size(12)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                            text(desc)
                                .size(10)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                        ]
                        .spacing(2),
                        container("").width(Length::Fill),
                        if connected {
                            status_pill("Terhubung", ACCENT_GREEN, true)
                        } else {
                            status_pill("Pasangkan", FLUENT_TEXT_MUTED, false)
                        },
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                )
                .padding([10, 12])
                .width(Length::Fill)
                .style(fluent_card_style);

                dev_col = dev_col.push(dev_row);
            }

            glass_scrollable(
                column![
                    bt_card,
                    text("PERANGKAT TERSIMPAN & TERHUBUNG")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    dev_col,
                ]
                .spacing(12)
                .padding([0, 4]),
            )
            .into()
        }

        SettingsTab::Audio => {
            let vol_percent = (volume * 100.0).round() as u32;

            let master_card = container(
                column![
                    row![
                        svg_icon(
                            if muted || volume < 0.01 { icons::VOLUME_X } else { icons::VOLUME_HIGH },
                            22.0,
                            if muted { ACCENT_RED } else { ACCENT_AMBER },
                        ),
                        column![
                            text(if muted { "Volume Master (Dibungkam)" } else { "Volume Master" })
                                .size(13)
                                .font(FONT_DISPLAY_BOLD)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                            text(format!("Tingkat Volume: {}%  •  PipeWire Audio Server", if muted { 0 } else { vol_percent }))
                                .size(11)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                        ]
                        .spacing(2),
                        container("").width(Length::Fill),
                        button(
                            text(if muted { "Bunyikan" } else { "Bungkam" })
                                .size(11)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(move |_| text::Style {
                                    color: Some(if muted { Color::WHITE } else { FLUENT_TEXT_DARK }),
                                }),
                        )
                        .padding([6, 12])
                        .style(move |theme, status| {
                            if muted {
                                fluent_button_style(ACCENT_RED, true)(theme, status)
                            } else {
                                fluent_ribbon_button_style(theme, status)
                            }
                        })
                        .on_press(Message::ToggleMute),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                    row![
                        svg_icon(icons::VOLUME_X, 14.0, FLUENT_TEXT_MUTED),
                        slider(0.0..=1.0, volume, Message::VolumeChanged)
                            .step(0.02_f32)
                            .style(fluent_slider_style)
                            .width(Length::Fill),
                        svg_icon(icons::VOLUME_HIGH, 14.0, FLUENT_TEXT_MUTED),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                ]
                .spacing(12),
            )
            .padding(14)
            .width(Length::Fill)
            .style(fluent_card_style);

            let dev_card = container(
                row![
                    svg_icon(icons::VOLUME_HIGH, 20.0, FLUENT_PRIMARY),
                    column![
                        text("Perangkat Keluaran Utama")
                            .size(12)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                        text(output_device)
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    ]
                    .spacing(2),
                    container("").width(Length::Fill),
                    status_pill("Default ALSA/PipeWire", FLUENT_PRIMARY, true),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .padding([12, 14])
            .width(Length::Fill)
            .style(fluent_card_style);

            glass_scrollable(
                column![
                    master_card,
                    text("PERANGKAT AUDIO & KELUARAN")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    dev_card,
                ]
                .spacing(12)
                .padding([0, 4]),
            )
            .into()
        }

        SettingsTab::Display => {
            let bright_percent = (brightness * 100.0).round() as u32;

            let bright_card = container(
                column![
                    row![
                        svg_icon(icons::SUN, 22.0, ACCENT_AMBER),
                        column![
                            text("Kecerahan Layar")
                                .size(13)
                                .font(FONT_DISPLAY_BOLD)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                            text(format!("Tingkat Kecerahan: {}%", bright_percent))
                                .size(11)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                        ]
                        .spacing(2),
                        container("").width(Length::Fill),
                        text(format!("{}%", bright_percent))
                            .size(13)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(FLUENT_PRIMARY) }),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                    row![
                        svg_icon(icons::SUN, 13.0, FLUENT_TEXT_MUTED),
                        slider(0.1..=1.0, brightness, Message::BrightnessChanged)
                            .step(0.05_f32)
                            .style(fluent_slider_style)
                            .width(Length::Fill),
                        svg_icon(icons::SUN, 18.0, ACCENT_AMBER),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                ]
                .spacing(12),
            )
            .padding(14)
            .width(Length::Fill)
            .style(fluent_card_style);

            let res_card = container(
                row![
                    svg_icon(icons::GPU, 20.0, ACCENT_GREEN),
                    column![
                        text("Resolusi & Refresh Rate Layar")
                            .size(12)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                        text(format!("Monitor Utama: {} @ {}", resolution, refresh_rate))
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    ]
                    .spacing(2),
                    container("").width(Length::Fill),
                    status_pill("100% Skala Asli", ACCENT_GREEN, true),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .padding([12, 14])
            .width(Length::Fill)
            .style(fluent_card_style);

            let night_card = container(
                row![
                    svg_icon(icons::MOON, 20.0, Color::from_rgb(0.55, 0.36, 0.96)),
                    column![
                        text("Cahaya Malam (Night Light)")
                            .size(12)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                        text("Mengurangi pancaran sinar biru layar untuk kenyamanan mata di malam hari.")
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    ]
                    .spacing(2),
                    container("").width(Length::Fill),
                    fluent_toggle_switch(night_light, Message::ToggleNightLight),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .padding([12, 14])
            .width(Length::Fill)
            .style(fluent_card_style);

            glass_scrollable(
                column![
                    bright_card,
                    text("PENGATURAN MONITOR & KENYAMANAN MATA")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    res_card,
                    night_card,
                ]
                .spacing(12)
                .padding([0, 4]),
            )
            .into()
        }

        SettingsTab::Appearance => {
            let wp_themes = [
                ("Cyber Crimson", "Gradien gelap aksen merah keunguan", WallpaperTheme::CyberCrimson),
                ("Deep Nebula", "Gradien biru kosmis ruang angkasa", WallpaperTheme::DeepNebula),
                ("Emerald Aurora", "Gradien hijau zamrud utara", WallpaperTheme::EmeraldAurora),
            ];

            let mut themes_row = row![].spacing(8);
            for (title, desc, theme) in wp_themes {
                let is_sel = wallpaper_theme == theme;
                let btn = button(
                    column![
                        text(title)
                            .size(12)
                            .font(FONT_DISPLAY_BOLD)
                            .style(move |_| text::Style {
                                color: Some(if is_sel { FLUENT_PRIMARY } else { FLUENT_TEXT_DARK }),
                            }),
                        text(desc)
                            .size(10)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    ]
                    .spacing(3),
                )
                .padding([10, 12])
                .width(Length::FillPortion(1))
                .style(move |_theme, status| {
                    let border_c = if is_sel {
                        FLUENT_PRIMARY
                    } else {
                        match status {
                            button::Status::Hovered => Color::from_rgba(0.0, 0.47, 0.83, 0.5),
                            _ => FLUENT_BORDER,
                        }
                    };
                    button::Style {
                        background: Some(Background::Color(Color::WHITE)),
                        border: Border::default().rounded(Radius::from(8.0)).width(if is_sel { 2.0 } else { 1.0 }).color(border_c),
                        ..button::Style::default()
                    }
                })
                .on_press(Message::SelectWallpaper(theme));

                themes_row = themes_row.push(btn);
            }

            let custom_wp_card = container(
                column![
                    row![
                        svg_icon(icons::FOLDER, 16.0, FLUENT_PRIMARY),
                        text("Wallpaper Kustom dari Berkas Gambar")
                            .size(12)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                    row![
                        text_input("Jalur berkas gambar PNG/JPG/WebP...", &state.custom_wallpaper_input)
                            .size(12)
                            .padding([7, 10])
                            .style(fluent_input_style)
                            .on_input(Message::CustomWallpaperInputChanged)
                            .on_submit(Message::ApplyCustomWallpaperInput),
                        button(
                            text("Terapkan")
                                .size(11)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(|_| text::Style { color: Some(Color::WHITE) }),
                        )
                        .padding([7, 14])
                        .style(fluent_button_style(FLUENT_PRIMARY, true))
                        .on_press(Message::ApplyCustomWallpaperInput),
                        button(
                            text("Buat Neon")
                                .size(11)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(|_| text::Style { color: Some(FLUENT_PRIMARY) }),
                        )
                        .padding([7, 12])
                        .style(fluent_ribbon_button_style)
                        .on_press(Message::GenerateAndSetSampleCustomWallpaper),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
                .spacing(10),
            )
            .padding(14)
            .width(Length::Fill)
            .style(fluent_card_style);

            let dock_size_card = container(
                column![
                    row![
                        svg_icon(icons::LAYERS, 16.0, ACCENT_PURPLE),
                        column![
                            text("Ukuran Ikon Taskbar Dock")
                                .size(12)
                                .font(FONT_DISPLAY_BOLD)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                            text(format!("Ukuran: {:.0} px", dock_icon_size))
                                .size(11)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                        ]
                        .spacing(2),
                        container("").width(Length::Fill),
                        text(format!("{:.0}px", dock_icon_size))
                            .size(13)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(FLUENT_PRIMARY) }),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                    slider(28.0..=48.0, dock_icon_size, Message::DockSizeChanged)
                        .step(2.0_f32)
                        .style(fluent_slider_style)
                        .width(Length::Fill),
                ]
                .spacing(10),
            )
            .padding(14)
            .width(Length::Fill)
            .style(fluent_card_style);

            glass_scrollable(
                column![
                    text("PILIHAN TEMA WALLPAPER")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    themes_row,
                    custom_wp_card,
                    text("TASKBAR DOCK")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    dock_size_card,
                ]
                .spacing(12)
                .padding([0, 4]),
            )
            .into()
        }

        SettingsTab::About => {
            let info_items: [(&'static str, String, &'static str); 5] = [
                ("Sistem Operasi", "HyperXOS Linux 2026.10 (Rolling Release)".to_string(), icons::COMPUTER),
                ("Prosesor (CPU)", cpu_brand.to_string(), icons::CPU),
                ("Memori (RAM)", format!("{:.2} GB terpakai dari {:.2} GB", ram_used_gb, ram_total_gb), icons::MEMORY),
                ("Compositor", format!("Labwc Wayland ({})", wayland_socket), icons::GPU),
                ("Mesin Grafis", "Iced 0.14 WGPU (Direct Rendering / KMS Vulkan)".to_string(), icons::SPARKLES),
            ];

            let mut info_col = column![].spacing(6);
            for (label, val, ic) in &info_items {
                let item_row = container(
                    row![
                        svg_icon(ic, 18.0, FLUENT_PRIMARY),
                        column![
                            text(*label)
                                .size(10)
                                .font(FONT_BODY_MEDIUM)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                            text(val.clone())
                                .size(12)
                                .font(FONT_BODY_SEMIBOLD)
                                .style(|_| text::Style { color: Some(FLUENT_TEXT_DARK) }),
                        ]
                        .spacing(2),
                    ]
                    .spacing(10)
                    .align_y(Alignment::Center),
                )
                .padding([10, 14])
                .width(Length::Fill)
                .style(fluent_card_style);

                info_col = info_col.push(item_row);
            }

            let usage_card = container(
                row![
                    column![
                        text("Penggunaan CPU Global")
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                        text(format!("{:.1}%", cpu_usage))
                            .size(18)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(FLUENT_PRIMARY) }),
                    ]
                    .width(Length::FillPortion(1)),
                    column![
                        text("Penggunaan Memori")
                            .size(11)
                            .font(FONT_BODY_MEDIUM)
                            .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                        text(format!("{:.1}%", (ram_used_gb / ram_total_gb.max(0.1)) * 100.0))
                            .size(18)
                            .font(FONT_DISPLAY_BOLD)
                            .style(|_| text::Style { color: Some(ACCENT_GREEN) }),
                    ]
                    .width(Length::FillPortion(1)),
                ]
                .spacing(16),
            )
            .padding(14)
            .width(Length::Fill)
            .style(fluent_card_style);

            glass_scrollable(
                column![
                    usage_card,
                    text("SPESIFIKASI PERANGKAT & SUBSISTEM")
                        .size(11)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style { color: Some(FLUENT_TEXT_MUTED) }),
                    info_col,
                ]
                .spacing(12)
                .padding([0, 4]),
            )
            .into()
        }
    };

    // 4. Combine Top Window Header with Left Sidebar & Right Content Panel
    let right_panel = container(page_body)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(14)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.97, 0.98, 0.99))),
            ..container::Style::default()
        });

    let main_body = row![sidebar, right_panel]
        .width(Length::Fill)
        .height(Length::Fill);

    container(
        column![
            top_header,
            main_body,
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(w)
    .height(h)
    .style(fluent_window_style)
    .into()
}
