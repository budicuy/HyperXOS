use iced::widget::{button, column, container, row, slider, text};
use iced::{Alignment, Background, Color, Element, Length};

use hyperx_ui_kit::{
    fluent_acrylic_modal_style, fluent_quick_toggle_style, fluent_ribbon_button_style,
    fluent_slider_style, icons, svg_icon, FLUENT_PRIMARY, FLUENT_TEXT_DARK,
    FLUENT_TEXT_MUTED, FONT_BODY_MEDIUM, FONT_BODY_SEMIBOLD,
};

use crate::ui::{DesktopWindowId, Message};

pub fn view_control_center<'a>(
    wifi_enabled: bool,
    wifi_ssid: &'a str,
    bluetooth_enabled: bool,
    performance_mode: bool,
    dnd_enabled: bool,
    volume: f32,
    muted: bool,
    brightness: f32,
    _cpu_usage: f32,
    _ram_used_gb: f32,
    _ram_total_gb: f32,
) -> Element<'a, Message> {
    // Header Bar
    let header = row![
        text("Pengaturan Cepat")
            .size(13)
            .font(FONT_BODY_SEMIBOLD)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_DARK),
            }),
        container("").width(Length::Fill),
        button(svg_icon(icons::CLOSE, 12.0, FLUENT_TEXT_MUTED))
            .padding([3, 6])
            .style(fluent_ribbon_button_style)
            .on_press(Message::ToggleControlCenter),
    ]
    .align_y(Alignment::Center);

    // Fluent Quick Toggle Pill Builder
    let make_toggle = |title: &'static str,
                       subtitle: String,
                       icon: &'static str,
                       is_active: bool,
                       msg: Message|
     -> Element<'a, Message> {
        let (icon_col, title_col, sub_col) = if is_active {
            (Color::WHITE, Color::WHITE, Color::from_rgba(1.0, 1.0, 1.0, 0.85))
        } else {
            (FLUENT_TEXT_DARK, FLUENT_TEXT_DARK, FLUENT_TEXT_MUTED)
        };

        button(
            row![
                svg_icon(icon, 18.0, icon_col),
                column![
                    text(title).size(11).font(FONT_BODY_SEMIBOLD).style(move |_| text::Style {
                        color: Some(title_col),
                    }),
                    text(subtitle).size(9).font(FONT_BODY_MEDIUM).style(move |_| text::Style {
                        color: Some(sub_col),
                    }),
                ]
                .spacing(1),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([8, 10])
        .style(fluent_quick_toggle_style(is_active))
        .on_press(msg)
        .into()
    };

    let toggle_wifi = make_toggle(
        "Wi-Fi",
        if wifi_enabled {
            if wifi_ssid.is_empty() {
                "Aktif".to_string()
            } else {
                wifi_ssid.to_string()
            }
        } else {
            "Mati".to_string()
        },
        if wifi_enabled { icons::WIFI } else { icons::WIFI_OFF },
        wifi_enabled,
        Message::ToggleWifiRadio,
    );

    let toggle_bt = make_toggle(
        "Bluetooth",
        if bluetooth_enabled { "Aktif".to_string() } else { "Mati".to_string() },
        icons::BLUETOOTH,
        bluetooth_enabled,
        Message::ToggleBluetooth,
    );

    let toggle_dnd = make_toggle(
        "Fokus",
        if dnd_enabled { "Aktif".to_string() } else { "Mati".to_string() },
        icons::MOON_DND,
        dnd_enabled,
        Message::ToggleDnd,
    );

    let toggle_perf = make_toggle(
        "Performa",
        if performance_mode { "Turbo".to_string() } else { "Seimbang".to_string() },
        icons::ZAP,
        performance_mode,
        Message::TogglePerformanceMode,
    );

    let toggle_night = make_toggle(
        "Lampu Malam",
        "Normal".to_string(),
        icons::SUN_BRIGHTNESS,
        false,
        Message::ToggleNightLight,
    );

    let toggle_mute = make_toggle(
        "Suara",
        if muted { "Hening".to_string() } else { "Normal".to_string() },
        if muted { icons::VOLUME_MUTE } else { icons::VOLUME_HIGH },
        muted,
        Message::ToggleMute,
    );

    let toggles_grid = column![
        row![toggle_wifi, toggle_bt].spacing(8),
        row![toggle_dnd, toggle_perf].spacing(8),
        row![toggle_night, toggle_mute].spacing(8),
    ]
    .spacing(8);

    // Brightness Slider Row
    let brightness_row = row![
        svg_icon(icons::SUN_BRIGHTNESS, 16.0, FLUENT_TEXT_DARK),
        slider(10.0..=100.0, brightness, Message::BrightnessChanged)
            .style(fluent_slider_style)
            .width(Length::Fill),
        text(format!("{:.0}%", brightness))
            .size(11)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_MUTED),
            }),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    // Volume Slider Row
    let vol_icon = if muted || volume <= 0.5 {
        icons::VOLUME_MUTE
    } else {
        icons::VOLUME_HIGH
    };
    let volume_row = row![
        button(svg_icon(vol_icon, 16.0, FLUENT_PRIMARY))
            .padding([2, 4])
            .style(fluent_ribbon_button_style)
            .on_press(Message::ToggleMute),
        slider(0.0..=100.0, volume, Message::VolumeChanged)
            .style(fluent_slider_style)
            .width(Length::Fill),
        text(format!("{:.0}%", volume))
            .size(11)
            .font(FONT_BODY_MEDIUM)
            .style(|_| text::Style {
                color: Some(FLUENT_TEXT_MUTED),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let subtle_divider = container("")
        .width(Length::Fill)
        .height(1)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.06))),
            ..container::Style::default()
        });

    // Footer Bar (Battery + Settings Gear + Power)
    let footer_bar = row![
        row![
            svg_icon(icons::BATTERY_CHARGING, 16.0, Color::from_rgb8(16, 185, 129)),
            text("100% Tersedia")
                .size(11)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
        container("").width(Length::Fill),
        button(
            row![
                svg_icon(icons::SETTINGS_GEAR, 14.0, FLUENT_TEXT_DARK),
                text("Pengaturan").size(11).font(FONT_BODY_MEDIUM).style(|_| text::Style {
                    color: Some(FLUENT_TEXT_DARK),
                }),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .padding([4, 8])
        .style(fluent_ribbon_button_style)
        .on_press(Message::OpenWindow(DesktopWindowId::Settings)),
        button(svg_icon(icons::POWER, 14.0, Color::from_rgb8(232, 17, 35)))
            .padding([4, 6])
            .style(fluent_ribbon_button_style)
            .on_press(Message::SystemPowerAction("systemctl poweroff".to_string())),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let content = column![
        header,
        toggles_grid,
        column![brightness_row, volume_row].spacing(10),
        subtle_divider,
        footer_bar,
    ]
    .spacing(12)
    .padding(14);

    container(content)
        .width(Length::Fixed(350.0))
        .style(fluent_acrylic_modal_style)
        .into()
}
