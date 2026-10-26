use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length};

use crate::core::system_monitor::SystemMetrics;
use crate::ui::components::{glass_scrollable, metric_card};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    metrics: &SystemMetrics,
    on_quick_clean: Message,
    on_quick_tweaks: Message,
    on_quick_kernel: Message,
) -> Element<'a, Message> {
    // 1. Metric Cards Grid (2x2)
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
    let ram_total_gb = metrics.ram_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
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
    let disk_total_gb = metrics.disk_total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
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

    // 2. System Information Glass Panel
    let info_header = row![
        svg_icon(icons::INFO, 15.0, ACCENT_CYAN),
        text("Informasi Sistem Operasi & Sesi")
            .size(13)
            .style(|_| text::Style {
                color: Some(ACCENT_CYAN),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let info_item = |label: &'static str, val: String, accent: Option<Color>| -> Element<'a, Message> {
        column![
            text(label).size(10).style(|_| text::Style {
                color: Some(TEXT_MUTED),
            }),
            text(val).size(12).style(move |_| text::Style {
                color: Some(accent.unwrap_or(TEXT_MAIN)),
            }),
        ]
        .spacing(2)
        .into()
    };

    let info_grid = row![
        info_item("Distribusi OS:", metrics.os_name.clone(), None),
        container("").width(Length::Fill),
        info_item("Kernel Aktif:", metrics.kernel_version.clone(), Some(ACCENT_GREEN)),
        container("").width(Length::Fill),
        info_item("Sesi Layar:", metrics.display_session.clone(), None),
        container("").width(Length::Fill),
        info_item("Waktu Aktif:", metrics.uptime_string.clone(), None),
    ]
    .align_y(Alignment::Center)
    .padding([4, 8]);

    let info_panel = container(column![info_header, info_grid].spacing(10).padding(14))
        .width(Length::Fill)
        .style(glass_card_style);

    // 3. Quick Action Buttons
    let action_btn = |label: &'static str, icon_svg: &'static str, msg: Message| {
        button(
            row![
                svg_icon(icon_svg, 14.0, ACCENT_CYAN),
                text(label).size(11)
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
        column![
            text("Pemantauan Perangkat Keras Real-time")
                .size(16)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
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
        info_panel,
        quick_actions,
    ]
    .spacing(18)
    .padding(20);

    glass_scrollable(content).height(Length::Fill).into()
}
