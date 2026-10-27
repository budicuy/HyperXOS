use sysinfo::{Disks, System};

use iced::alignment::Alignment;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Color, Element, Length};

use crate::theme::{self, ACCENT_BLUE, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub struct SystemSpecs {
    pub os_name: String,
    pub kernel: String,
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub total_memory_gb: f32,
    pub used_memory_gb: f32,
    pub total_disk_gb: f32,
    pub available_disk_gb: f32,
}

impl Default for SystemSpecs {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let os_name = "HyperXOS 2026.10 (Rolling Edition)".to_string();
        let kernel = System::kernel_version().unwrap_or_else(|| "Linux 6.17-hyperx".to_string());

        let cpu_model = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "x86_64 Multi-Core Processor".to_string());
        let cpu_cores = sys.cpus().len();

        let total_memory_gb = sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
        let used_memory_gb = sys.used_memory() as f32 / (1024.0 * 1024.0 * 1024.0);

        let disks = Disks::new_with_refreshed_list();
        let mut total_disk_bytes: u64 = 0;
        let mut avail_disk_bytes: u64 = 0;
        for disk in &disks {
            total_disk_bytes += disk.total_space();
            avail_disk_bytes += disk.available_space();
        }

        let total_disk_gb = total_disk_bytes as f32 / (1024.0 * 1024.0 * 1024.0);
        let available_disk_gb = avail_disk_bytes as f32 / (1024.0 * 1024.0 * 1024.0);

        Self {
            os_name,
            kernel,
            cpu_model,
            cpu_cores,
            total_memory_gb,
            used_memory_gb,
            total_disk_gb,
            available_disk_gb,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AboutMessage {
    CheckForUpdates,
}

pub fn view<'a, Message: 'static + Clone>(
    specs: &'a SystemSpecs,
    map_fn: impl Fn(AboutMessage) -> Message + Copy + 'static,
) -> Element<'a, Message> {
    let title = text("About HyperXOS").size(22).color(TEXT_PRIMARY);

    let logo_card = container(
        column![
            text("⚡").size(48).color(ACCENT_BLUE),
            text(&specs.os_name).size(18).color(TEXT_PRIMARY),
            text("Pure Rust Desktop Environment powered by Smithay & Iced").size(12).color(TEXT_SECONDARY),
        ]
        .spacing(6)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .padding(20)
    .center_x(Length::Fill)
    .style(theme::card_style);

    let spec_item = |label: &'static str, val: String| {
        row![
            text(label).size(12).color(TEXT_SECONDARY).width(140),
            text(val).size(12).color(TEXT_PRIMARY),
        ]
        .spacing(10)
        .align_y(Alignment::Center)
    };

    let specs_card = container(
        column![
            spec_item("Operating System", specs.os_name.clone()),
            spec_item("Linux Kernel", format!("Linux {}", specs.kernel)),
            spec_item("Desktop Shell", "HyperX DE v1.0.0 (Smithay Wayland)".to_string()),
            spec_item("Processor", format!("{} ({} Threads)", specs.cpu_model, specs.cpu_cores)),
            spec_item("Memory (RAM)", format!("{:.1} GB / {:.1} GB Installed", specs.used_memory_gb, specs.total_memory_gb)),
            spec_item("Storage", format!("{:.1} GB Available of {:.1} GB", specs.available_disk_gb, specs.total_disk_gb)),
        ]
        .spacing(12),
    )
    .padding(16)
    .style(theme::card_style);

    let update_btn = row![
        Space::new().width(Length::Fill),
        button(text("Check for System Updates...").size(12).color(Color::WHITE))
            .padding([8, 16])
            .style(button::primary)
            .on_press(map_fn(AboutMessage::CheckForUpdates)),
    ];

    let content = column![title, logo_card, specs_card, update_btn].spacing(14);

    scrollable(content).height(Length::Fill).into()
}
