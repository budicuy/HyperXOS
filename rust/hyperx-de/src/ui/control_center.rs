use iced::widget::{button, column, container, row, slider, text};
use iced::{Alignment, Color, Element, Length};

use hyperx_ui_kit::{
    active_toggle_button_style, danger_button_style, glass_action_button, glass_card_accent_style,
    glass_card_style, glass_slider_style, icons, metric_card, popover_glass_style, svg_icon,
    ACCENT_AMBER, ACCENT_CYAN, ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED, ACTIVE_DARK_FG,
    FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD, FONT_DISPLAY_BOLD, TEXT_MAIN, TEXT_MUTED,
};

use crate::ui::settings::SettingsTab;
use crate::ui::Message;

pub fn view_control_center<'a>(
    wifi_enabled: bool,
    wifi_ssid: &'a str,
    bluetooth_enabled: bool,
    performance_mode: bool,
    dnd_enabled: bool,
    volume: f32,
    muted: bool,
    brightness: f32,
    cpu_usage: f32,
    ram_used_gb: f32,
    ram_total_gb: f32,
) -> Element<'a, Message> {
    let header = row![
        svg_icon(icons::SLIDERS, 18.0, ACCENT_CYAN),
        column![
            text("Pusat Kendali HyperXOS")
                .size(14)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text("Pengaturan Cepat Sistem & Perangkat Keras")
                .size(10)
                .font(FONT_BODY_MEDIUM)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED),
                }),
        ]
        .spacing(1),
        container("").width(Length::Fill),
        button(svg_icon(icons::CLOSE, 13.0, ACCENT_RED))
            .padding([4, 8])
            .style(glass_action_button)
            .on_press(Message::ToggleControlCenter),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let quick_tile = |title: &'static str,
                      subtitle: String,
                      icon: &'static str,
                      is_active: bool,
                      accent: Color,
                      msg: Message|
     -> Element<'a, Message> {
        let icon_col = if is_active { ACTIVE_DARK_FG } else { accent };
        let sub_col = if is_active {
            ACTIVE_DARK_FG
        } else {
            TEXT_MUTED
        };

        button(
            row![
                svg_icon(icon, 18.0, icon_col),
                column![
                    text(title).size(12).font(FONT_DISPLAY_BOLD),
                    text(subtitle).size(10).font(FONT_BODY_SEMIBOLD).style(move |_| text::Style {
                        color: Some(sub_col),
                    }),
                ]
                .spacing(1),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([10, 12])
        .style(active_toggle_button_style(is_active, accent))
        .on_press(msg)
        .into()
    };

    let tile_wifi = quick_tile(
        "Wi-Fi",
        if wifi_enabled {
            wifi_ssid.to_string()
        } else {
            "Nonaktif".to_string()
        },
        if wifi_enabled {
            icons::WIFI
        } else {
            icons::WIFI_OFF
        },
        wifi_enabled,
        ACCENT_CYAN,
        Message::ToggleWifiRadio,
    );

    let tile_bt = quick_tile(
        "Bluetooth",
        if bluetooth_enabled {
            "Aktif".to_string()
        } else {
            "Nonaktif".to_string()
        },
        icons::BLUETOOTH,
        bluetooth_enabled,
        ACCENT_PURPLE,
        Message::ToggleBluetooth,
    );

    let tile_perf = quick_tile(
        "Mode Performa",
        if performance_mode {
            "Gaming Turbo".to_string()
        } else {
            "Seimbang".to_string()
        },
        icons::ZAP,
        performance_mode,
        ACCENT_GREEN,
        Message::TogglePerformanceMode,
    );

    let tile_dnd = quick_tile(
        "Jangan Ganggu",
        if dnd_enabled {
            "Senyap Aktif".to_string()
        } else {
            "Normal".to_string()
        },
        icons::MOON_DND,
        dnd_enabled,
        ACCENT_AMBER,
        Message::ToggleDnd,
    );

    let toggles_grid = column![
        row![tile_wifi, tile_bt].spacing(8),
        row![tile_perf, tile_dnd].spacing(8),
    ]
    .spacing(8);

    // Brightness Slider Card
    let brightness_card = container(
        column![
            row![
                svg_icon(icons::SUN_BRIGHTNESS, 16.0, ACCENT_AMBER),
                text("Kecerahan Layar")
                    .size(12)
                    .font(FONT_DISPLAY_BOLD),
                container("").width(Length::Fill),
                text(format!("{:.0}%", brightness))
                    .size(12)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_AMBER),
                    }),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            slider(10.0..=100.0, brightness, Message::BrightnessChanged)
                .style(glass_slider_style(ACCENT_AMBER)),
        ]
        .spacing(8)
        .padding(12),
    )
    .width(Length::Fill)
    .style(glass_card_accent_style(ACCENT_AMBER));

    // Audio Volume Slider Card
    let vol_icon = if muted || volume <= 0.5 {
        icons::VOLUME_MUTE
    } else {
        icons::VOLUME_HIGH
    };
    let volume_card = container(
        column![
            row![
                svg_icon(vol_icon, 16.0, ACCENT_CYAN),
                text("Volume Suara (PipeWire)")
                    .size(12)
                    .font(FONT_DISPLAY_BOLD),
                container("").width(Length::Fill),
                button(
                    text(if muted { "Unmute" } else { "Mute" })
                        .size(10)
                        .font(FONT_BODY_SEMIBOLD)
                )
                .padding([3, 8])
                .style(glass_action_button)
                .on_press(Message::ToggleMute),
                text(format!("{:.0}%", volume))
                    .size(12)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_CYAN),
                    }),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            slider(0.0..=100.0, volume, Message::VolumeChanged)
                .style(glass_slider_style(ACCENT_CYAN)),
        ]
        .spacing(8)
        .padding(12),
    )
    .width(Length::Fill)
    .style(glass_card_accent_style(ACCENT_CYAN));

    // System Resource Telemetry
    let ram_ratio = if ram_total_gb > 0.0 {
        (ram_used_gb / ram_total_gb).clamp(0.0, 1.0)
    } else {
        0.25
    };
    let cpu_card = metric_card(
        "Beban CPU & Memori",
        icons::CPU,
        format!("{:.0}%", cpu_usage),
        (cpu_usage / 100.0).clamp(0.0, 1.0),
        format!("RAM: {:.1} / {:.1} GB ({:.0}%)", ram_used_gb, ram_total_gb, ram_ratio * 100.0),
        Some("Smithay WGPU Aktif"),
        ACCENT_GREEN,
    );

    // Bottom Quick Action Row
    let footer_actions = container(
        row![
            button(
                row![
                    svg_icon(icons::WIFI, 13.0, ACCENT_CYAN),
                    text("Wi-Fi").size(11).font(FONT_BODY_SEMIBOLD),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([6, 10])
            .style(glass_action_button)
            .on_press(Message::OpenSettingsPage(SettingsTab::Network)),
            button(
                row![
                    svg_icon(icons::SETTINGS_GEAR, 13.0, ACCENT_PURPLE),
                    text("Settings").size(11).font(FONT_BODY_SEMIBOLD),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .padding([6, 10])
            .style(glass_action_button)
            .on_press(Message::OpenWindow(crate::ui::DesktopWindowId::Settings)),
            container("").width(Length::Fill),
            button(
                row![
                    svg_icon(icons::REFRESH, 13.0, ACCENT_AMBER),
                    text("Reboot").size(11).font(FONT_BODY_SEMIBOLD),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .padding([6, 10])
            .style(glass_action_button)
            .on_press(Message::SystemPowerAction("systemctl reboot".to_string())),
            button(
                row![
                    svg_icon(icons::POWER, 13.0, ACCENT_RED),
                    text("Matikan").size(11).font(FONT_BODY_SEMIBOLD),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .padding([6, 10])
            .style(danger_button_style())
            .on_press(Message::SystemPowerAction("systemctl poweroff".to_string())),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .padding(10)
    .width(Length::Fill)
    .style(glass_card_style);

    let content = column![
        header,
        toggles_grid,
        brightness_card,
        volume_card,
        cpu_card,
        footer_actions,
    ]
    .spacing(10)
    .padding(14);

    container(content)
        .width(Length::Fixed(370.0))
        .style(popover_glass_style(ACCENT_CYAN))
        .into()
}
