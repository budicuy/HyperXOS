use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};
use crate::core::preflight::PreflightStatus;
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    preflight: &PreflightStatus,
    on_refresh: Message,
    on_next: Message,
) -> Element<'a, Message> {
    let check_row = |title: &'static str, detail: String, passed: bool, icon: &'static str| {
        let (badge_text, badge_color) = if passed {
            ("PASSED", ACCENT_GREEN)
        } else {
            ("FAILED", ACCENT_RED)
        };

        container(
            row![
                svg_icon(icon, 18.0, if passed { ACCENT_GREEN } else { ACCENT_RED }),
                column![
                    text(title).size(13).font(FONT_BODY_SEMIBOLD).style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                    text(detail).size(11).font(FONT_BODY).style(|_| text::Style {
                        color: Some(TEXT_MUTED),
                    }),
                ]
                .spacing(2)
                .width(Length::Fill),
                container(
                    text(badge_text)
                        .size(10)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(move |_| text::Style {
                            color: Some(badge_color),
                        }),
                )
                .padding([4, 8])
                .style(glass_card_accent_style(badge_color)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding(12)
        .width(Length::Fill)
        .style(glass_card_style)
    };

    let uefi_row = check_row(
        "UEFI Firmware Mode",
        if preflight.is_uefi {
            "System booted with standard modern UEFI interface (/sys/firmware/efi)".to_string()
        } else {
            "Legacy BIOS detected. UEFI boot mode is required for Limine".to_string()
        },
        preflight.is_uefi,
        icons::SHIELD_CHECK,
    );

    let ram_row = check_row(
        "System Memory (RAM)",
        format!(
            "{:.1} GB detected (Minimum 3.5 GB required for HyperXOS installation)",
            preflight.total_ram_gb
        ),
        preflight.has_min_ram,
        icons::RAM_CHIP,
    );

    let net_row = check_row(
        "Active Internet Connectivity",
        if preflight.has_internet {
            "Connected to online mirrors for fresh pacstrap package installation".to_string()
        } else {
            "No internet access detected. Please connect to Wi-Fi or Ethernet".to_string()
        },
        preflight.has_internet,
        icons::WIFI,
    );

    let storage_row = check_row(
        "Target Storage Capacity",
        if preflight.has_min_disk {
            "At least one non-loop disk with >= 20.0 GB available".to_string()
        } else {
            "No suitable storage drive with >= 20.0 GB detected".to_string()
        },
        preflight.has_min_disk,
        icons::HARD_DRIVE,
    );

    let power_row = check_row(
        "Power Source",
        if preflight.is_plugged_in {
            "AC Power connected or desktop system".to_string()
        } else {
            "Running on battery power. Plug in power to prevent interruption".to_string()
        },
        preflight.is_plugged_in,
        icons::POWER,
    );

    let can_continue = preflight.can_proceed();

    let mut next_btn = button(
        row![
            text("Next: Regional Settings")
                .size(13)
                .font(FONT_BODY_SEMIBOLD),
            svg_icon(icons::ROCKET, 13.0, Color::WHITE),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([8, 20])
    .style(primary_button_style(ACCENT_CYAN));

    if can_continue {
        next_btn = next_btn.on_press(on_next);
    }

    let refresh_btn = button(
        row![
            svg_icon(icons::REFRESH, 12.0, TEXT_MAIN),
            text("Recheck System")
                .size(12)
                .font(FONT_BODY_MEDIUM),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .on_press(on_refresh)
    .style(secondary_button_style())
    .padding([8, 16]);

    column![
        column![
            text("Welcome to HyperXOS Linux")
                .size(22)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN)
                }),
            text("Pure Rust Native Installer with Limine Bootloader & HyperX Desktop Environment")
                .size(13)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED)
                }),
        ]
        .spacing(4),
        column![
            text("PRE-FLIGHT DIAGNOSTICS & SYSTEM COMPLIANCE")
                .size(11)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(ACCENT_CYAN)
                }),
            uefi_row,
            ram_row,
            net_row,
            storage_row,
            power_row,
        ]
        .spacing(8),
        column![].height(Length::Fill),
        row![
            refresh_btn,
            row![].width(Length::Fill),
            next_btn,
        ]
        .align_y(Alignment::Center),
    ]
    .spacing(14)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
