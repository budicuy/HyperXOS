use iced::border::Radius;
use iced::widget::{button, column, container, image, row, text};
use iced::{Alignment, Border, Color, Element, Length};

use crate::core::system_monitor::SystemMetrics;
use crate::ui::components::{glass_scrollable, metric_card, status_pill};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    metrics: &SystemMetrics,
    on_quick_clean: Message,
    on_quick_tweaks: Message,
    on_quick_kernel: Message,
) -> Element<'a, Message> {
    // --- 1. TOP BANNER: OS & Complete Hardware Specifications with Logo ---
    let logo_data: &'static [u8] = include_bytes!("../../../assets/logo.png");
    let logo_img = image(image::Handle::from_bytes(logo_data))
        .width(60)
        .height(60);

    let logo_box = container(logo_img)
        .padding(8)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
            border: Border::default()
                .rounded(Radius::from(12.0))
                .width(1.0)
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.15)),
            ..container::Style::default()
        });

    let spec_col = |label: &'static str, val: String, accent: Option<Color>| -> Element<'a, Message> {
        column![
            text(label).size(9).font(FONT_DISPLAY_BOLD).style(|_| text::Style {
                color: Some(Color::from_rgb(0.70, 0.75, 0.85)),
            }),
            text(val).size(12).font(FONT_BODY_SEMIBOLD).style(move |_| text::Style {
                color: Some(accent.unwrap_or(Color::WHITE)),
            }),
        ]
        .spacing(2)
        .into()
    };

    let ram_total_gb = metrics.ram_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let disk_total_gb = metrics.disk_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

    let banner_specs_row1 = row![
        spec_col("SISTEM OPERASI", format!("{} (Arch)", metrics.os_name), Some(ACCENT_CYAN)),
        container("").width(Length::Fill),
        spec_col("KERNEL AKTIF", metrics.kernel_version.clone(), Some(ACCENT_GREEN)),
        container("").width(Length::Fill),
        spec_col("PERANGKAT KERAS", format!("{} • {}", metrics.hardware_board, metrics.host_name), None),
    ]
    .align_y(Alignment::Center);

    let banner_specs_row2 = row![
        spec_col("PROSESOR (CPU)", format!("{} ({} Cores)", metrics.cpu_model, metrics.cpu_cores), Some(ACCENT_AMBER)),
        container("").width(Length::Fill),
        spec_col("AKSELERASI GRAFIS", metrics.gpu_model.clone(), Some(ACCENT_PURPLE)),
        container("").width(Length::Fill),
        spec_col("DESKTOP & SESI", format!("{} ({}) • Uptime {}", metrics.desktop_env, metrics.display_session, metrics.uptime_string), None),
    ]
    .align_y(Alignment::Center);

    let banner_right = column![
        row![
            text("HyperXOS Linux")
                .size(16)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(Color::WHITE),
                }),
            status_pill("Sistem Aktif", ACCENT_GREEN, true),
            status_pill("Edisi Rolling", ACCENT_CYAN, true),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        container("").height(1).width(Length::Fill).style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.12))),
            ..container::Style::default()
        }),
        banner_specs_row1,
        banner_specs_row2,
    ]
    .spacing(8)
    .width(Length::Fill);

    let top_banner = container(
        row![logo_box, banner_right]
            .spacing(16)
            .align_y(Alignment::Center)
            .padding(16)
    )
    .width(Length::Fill)
    .style(glass_card_accent_style(Color::from_rgb(0.92, 0.32, 0.12))); // Colored border

    // --- 2. Metric Cards Grid (2x2) ---
    // CPU
    let cpu_sub = format!(
        "{} Cores • {}",
        metrics.cpu_cores,
        metrics.cpu_model.chars().take(28).collect::<String>()
    );
    let cpu_temp_opt = if metrics.cpu_temp_celsius > 0 {
        Some(format!("Suhu: {}°C", metrics.cpu_temp_celsius))
    } else {
        None
    };
    let cpu_val = format!("{:.1}%", metrics.cpu_usage_percent);
    let cpu_card = metric_card(
        "Prosesor (CPU)",
        icons::CPU,
        cpu_val,
        metrics.cpu_usage_percent / 100.0,
        cpu_sub,
        cpu_temp_opt,
        ACCENT_CYAN,
    );

    // RAM
    let ram_used_gb = metrics.ram_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let ram_sub = format!("Terpakai: {:.1} GB / {:.1} GB", ram_used_gb, ram_total_gb);
    let swap_opt = if metrics.swap_total_bytes > 0 {
        let swap_used_gb = metrics.swap_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        Some(format!("Swap: {:.1} GB", swap_used_gb))
    } else {
        None
    };
    let ram_val = format!("{:.1}%", metrics.ram_usage_percent);
    let ram_card = metric_card(
        "Memori (RAM & Swap)",
        icons::RAM,
        &ram_val,
        metrics.ram_usage_percent / 100.0,
        ram_sub,
        swap_opt,
        ACCENT_PURPLE,
    );

    // Disk
    let disk_used_gb = metrics.disk_used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let disk_sub = format!("Terpakai: {:.1} GB / {:.1} GB", disk_used_gb, disk_total_gb);
    let fs_str = format!("FS: {}", metrics.disk_fs_type);
    let disk_val = format!("{:.1}%", metrics.disk_usage_percent);
    let disk_card = metric_card(
        "Penyimpanan Disk (Root /)",
        icons::DISK,
        disk_val,
        metrics.disk_usage_percent / 100.0,
        disk_sub,
        Some(fs_str),
        ACCENT_TEAL,
    );

    // GPU
    let gpu_val = format!("{:.1}%", metrics.gpu_usage_percent);
    let gpu_sub = metrics.gpu_model.chars().take(30).collect::<String>();
    let vram_opt = if metrics.vram_total_bytes > 0 {
        let vram_used_mb = metrics.vram_used_bytes / (1024 * 1024);
        let vram_total_mb = metrics.vram_total_bytes / (1024 * 1024);
        Some(format!("VRAM: {} / {} MB", vram_used_mb, vram_total_mb))
    } else {
        None
    };
    let gpu_card = metric_card(
        "Akselerasi Grafis (GPU)",
        icons::GPU,
        gpu_val,
        metrics.gpu_usage_percent / 100.0,
        gpu_sub,
        vram_opt,
        ACCENT_GREEN,
    );

    let row1 = row![cpu_card, ram_card].spacing(16);
    let row2 = row![disk_card, gpu_card].spacing(16);

    // --- 3. Quick Action Buttons ---
    let action_btn = |label: &'static str, icon_svg: &'static str, msg: Message| {
        button(
            row![
                svg_icon(icon_svg, 14.0, ACCENT_CYAN),
                text(label).size(11).font(FONT_BODY_SEMIBOLD)
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .padding([8, 14])
        .style(glass_action_button)
        .on_press(msg)
    };

    let quick_actions = row![
        action_btn("Bersihkan Cache Sistem", icons::MAINTENANCE, on_quick_clean),
        action_btn("Buka Tweak Gaming", icons::GAMING, on_quick_tweaks),
        action_btn("Kelola Kernel CachyOS", icons::KERNEL, on_quick_kernel),
    ]
    .spacing(12);

    let content = column![
        top_banner,
        column![
            text("Pemantauan Perangkat Keras Real-time")
                .size(16)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(Color::WHITE),
                }),
            text("Status sumber daya komputasi HyperXOS diperbarui secara langsung.")
                .size(11)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                }),
        ]
        .spacing(2),
        row1,
        row2,
        quick_actions,
    ]
    .spacing(18)
    .padding(20);

    glass_scrollable(content).height(Length::Fill).into()
}
