mod core;
mod ui;

use std::time::Duration;
use crossbeam_channel::{unbounded, Receiver, Sender};
use iced::widget::{column, container, row};
use iced::window;
use iced::{Element, Length, Size, Subscription, Task};

use crate::core::kernel_manager::{KernelEntry, KernelManager};
use crate::core::maintenance::Maintenance;
use crate::core::package_manager::{PackageManager, StorePackage};
use crate::core::snapshot_manager::{SnapshotManager, SnapshotRecord};
use crate::core::system_monitor::{SystemMetrics, SystemMonitor};
use crate::core::system_tweaker::SystemTweaker;
use crate::ui::sidebar::{sidebar, Page};
use hyperx_ui_kit::{
    load_window_icon, open_external_url, tools_header, FONT_BODY, FONT_INTER_BYTES,
    FONT_PLUS_JAKARTA_BYTES, TEXT_MAIN,
};

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: Size::new(1060.0, 720.0),
        min_size: Some(Size::new(920.0, 620.0)),
        transparent: true,
        blur: true,
        decorations: false,
        icon: load_window_icon(),
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "hyper-x-tools".to_string(),
            override_redirect: false,
        },
        ..Default::default()
    };

    iced::application(
        App::default,
        App::update,
        App::view,
    )
    .title("HyperXTools — Control Center & System Dashboard")
    .font(FONT_PLUS_JAKARTA_BYTES)
    .font(FONT_INTER_BYTES)
    .default_font(FONT_BODY)
    .window(window_settings)
    .subscription(App::subscription)
    .theme(|_: &App| iced::Theme::Dark)
    .style(|_: &App, _: &iced::Theme| iced::theme::Style {
        background_color: iced::Color::TRANSPARENT,
        text_color: TEXT_MAIN,
    })
    .run()
}

pub struct App {
    current_page: Page,
    monitor: SystemMonitor,
    metrics: SystemMetrics,

    // Kernel Manager
    running_kernel_name: String,
    running_kernel_ver: String,
    kernels: Vec<KernelEntry>,

    // Gaming Tweaks
    current_governor: String,
    max_map_count_val: String,
    is_max_map_opt: bool,
    is_split_lock_disabled: bool,
    is_gamemode_active: bool,

    // Snapshots
    is_btrfs: bool,
    fs_type_name: String,
    snapshot_input: String,
    snapshots: Vec<SnapshotRecord>,

    // Store
    store_cat: &'static str,
    store_search: String,
    all_packages: Vec<StorePackage>,
    filtered_packages: Vec<StorePackage>,

    // Header state
    hovered_header_btn: Option<&'static str>,

    // Terminal Logging
    terminal_log: String,
    log_tx: Sender<String>,
    log_rx: Receiver<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    SelectPage(Page),
    RefreshMetrics,
    ClearLog,

    // Window Management
    WindowDrag,
    WindowMinimize,
    WindowMaximize,
    WindowClose,

    // Kernel
    InstallKernel(String),
    RemoveKernel(String),
    RebuildInitramfs,

    // Gaming Tweaks
    SetGovernor(&'static str),
    ToggleMaxMapCount,
    ToggleSplitLock,
    ToggleGameMode,
    InstallGamingRuntimes,

    // Snapshots
    SnapshotInputChanged(String),
    CreateSnapshot,
    RestoreSnapshot(String),
    DeleteSnapshot(String),

    // Maintenance
    CleanCache,
    VacuumJournal,
    RemoveOrphans,
    RankMirrors,
    FixKeyrings,

    // Store
    StoreCategorySelected(&'static str),
    StoreSearchChanged(String),
    InstallPackage(String),
    RemovePackage(String),

    // External Links
    OpenUrl(&'static str),
    HeaderBtnHovered(Option<&'static str>),
}

impl Default for App {
    fn default() -> Self {
        let (log_tx, log_rx) = unbounded();
        let mut monitor = SystemMonitor::new();
        let metrics = monitor.collect_metrics();

        let (running_name, running_ver) = KernelManager::get_running_kernel_info();
        let kernels = KernelManager::get_catalog();

        let current_governor = SystemTweaker::get_current_governor();
        let (max_map_count_val, is_max_map_opt) = SystemTweaker::get_max_map_count();
        let is_split_lock_disabled = SystemTweaker::is_split_lock_disabled();
        let is_gamemode_active = SystemTweaker::is_gamemode_active();

        let is_btrfs = SnapshotManager::is_btrfs_filesystem();
        let fs_type_name = if is_btrfs { "BTRFS" } else { "EXT4/XFS" }.to_string();
        let snapshots = SnapshotManager::list_snapshots();

        let all_packages = PackageManager::get_catalog();
        let filtered_packages = PackageManager::filter_catalog(&all_packages, "Semua", "");

        Self {
            current_page: Page::Dashboard,
            monitor,
            metrics,

            running_kernel_name: running_name,
            running_kernel_ver: running_ver,
            kernels,

            current_governor,
            max_map_count_val,
            is_max_map_opt,
            is_split_lock_disabled,
            is_gamemode_active,

            is_btrfs,
            fs_type_name,
            snapshot_input: String::new(),
            snapshots,

            store_cat: "Semua",
            store_search: String::new(),
            all_packages,
            filtered_packages,

            hovered_header_btn: None,

            terminal_log: "HyperXTools v2.2.0 (Rust + Iced Glass WGPU) Siap.\n".to_string(),
            log_tx,
            log_rx,
        }
    }
}

impl App {
    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(1000)).map(|_| Message::Tick)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        // Drain any incoming logs from background threads
        while let Ok(msg) = self.log_rx.try_recv() {
            self.terminal_log.push_str(&msg);
        }

        match message {
            Message::Tick => {
                self.metrics = self.monitor.collect_metrics();
                Task::none()
            }
            Message::RefreshMetrics => {
                self.metrics = self.monitor.collect_metrics();
                Task::none()
            }
            Message::SelectPage(page) => {
                self.current_page = page;
                match page {
                    Page::KernelManager => self.refresh_kernel_state(),
                    Page::GameTweaks => self.refresh_tweaks_state(),
                    Page::Snapshots => self.refresh_snapshots_state(),
                    Page::PackageStore => self.refresh_store_state(),
                    _ => {}
                }
                Task::none()
            }
            Message::ClearLog => {
                self.terminal_log = "Terminal dibersihkan.\n".to_string();
                Task::none()
            }
            Message::WindowDrag => {
                window::latest().and_then(window::drag)
            }
            Message::WindowMinimize => {
                window::latest().and_then(|id| window::minimize(id, true))
            }
            Message::WindowMaximize => {
                window::latest().and_then(window::toggle_maximize)
            }
            Message::WindowClose => {
                window::latest().and_then(window::close)
            }
            Message::OpenUrl(url) => {
                open_external_url(url);
                Task::none()
            }
            Message::HeaderBtnHovered(btn) => {
                self.hovered_header_btn = btn;
                Task::none()
            }

            // --- KERNEL MANAGER ---
            Message::InstallKernel(pkg) => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    let headers = format!("{}-headers", pkg);
                    let _ = KernelManager::execute_action(
                        "pkexec",
                        &["pacman", "-S", "--needed", "--noconfirm", &pkg, &headers],
                        move |s| {
                            let _ = tx.send(s);
                        },
                    );
                });
                Task::none()
            }
            Message::RemoveKernel(pkg) => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    let headers = format!("{}-headers", pkg);
                    let _ = KernelManager::execute_action(
                        "pkexec",
                        &["pacman", "-Rns", "--noconfirm", &pkg, &headers],
                        move |s| {
                            let _ = tx.send(s);
                        },
                    );
                });
                Task::none()
            }
            Message::RebuildInitramfs => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    let _ = KernelManager::execute_action(
                        "pkexec",
                        &["mkinitcpio", "-P"],
                        move |s| {
                            let _ = tx.send(s);
                        },
                    );
                });
                Task::none()
            }

            // --- GAMING TWEAKS ---
            Message::SetGovernor(gov) => {
                let tx = self.log_tx.clone();
                let gov_str = gov.to_string();
                std::thread::spawn(move || {
                    SystemTweaker::set_governor(&gov_str, move |s| {
                        let _ = tx.send(s);
                    });
                });
                self.current_governor = gov.to_string();
                Task::none()
            }
            Message::ToggleMaxMapCount => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    SystemTweaker::optimize_max_map_count(move |s| {
                        let _ = tx.send(s);
                    });
                });
                self.is_max_map_opt = true;
                self.max_map_count_val = "2147483642".to_string();
                Task::none()
            }
            Message::ToggleSplitLock => {
                let tx = self.log_tx.clone();
                let new_disabled = !self.is_split_lock_disabled;
                std::thread::spawn(move || {
                    SystemTweaker::toggle_split_lock(new_disabled, move |s| {
                        let _ = tx.send(s);
                    });
                });
                self.is_split_lock_disabled = new_disabled;
                Task::none()
            }
            Message::ToggleGameMode => {
                let tx = self.log_tx.clone();
                let new_active = !self.is_gamemode_active;
                std::thread::spawn(move || {
                    SystemTweaker::toggle_gamemode(new_active, move |s| {
                        let _ = tx.send(s);
                    });
                });
                self.is_gamemode_active = new_active;
                Task::none()
            }
            Message::InstallGamingRuntimes => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    SystemTweaker::install_gaming_runtimes(move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }

            // --- SNAPSHOTS ---
            Message::SnapshotInputChanged(input) => {
                self.snapshot_input = input;
                Task::none()
            }
            Message::CreateSnapshot => {
                let tx = self.log_tx.clone();
                let desc = self.snapshot_input.clone();
                self.snapshot_input.clear();
                std::thread::spawn(move || {
                    SnapshotManager::create_snapshot(&desc, move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }
            Message::RestoreSnapshot(name) => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    SnapshotManager::restore_snapshot(&name, move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }
            Message::DeleteSnapshot(name) => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    SnapshotManager::delete_snapshot(&name, move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }

            // --- MAINTENANCE ---
            Message::CleanCache => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    Maintenance::clean_cache(move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }
            Message::VacuumJournal => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    Maintenance::clean_journal(move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }
            Message::RemoveOrphans => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    Maintenance::remove_orphans(move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }
            Message::RankMirrors => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    Maintenance::rank_mirrors(move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }
            Message::FixKeyrings => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    Maintenance::fix_keyrings(move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }

            // --- STORE ---
            Message::StoreCategorySelected(cat) => {
                self.store_cat = cat;
                self.filtered_packages =
                    PackageManager::filter_catalog(&self.all_packages, cat, &self.store_search);
                Task::none()
            }
            Message::StoreSearchChanged(q) => {
                self.store_search = q.clone();
                self.filtered_packages =
                    PackageManager::filter_catalog(&self.all_packages, self.store_cat, &q);
                Task::none()
            }
            Message::InstallPackage(pkg) => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    PackageManager::install_package(&pkg, move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }
            Message::RemovePackage(pkg) => {
                let tx = self.log_tx.clone();
                std::thread::spawn(move || {
                    PackageManager::remove_package(&pkg, move |s| {
                        let _ = tx.send(s);
                    });
                });
                Task::none()
            }
        }
    }

    fn refresh_kernel_state(&mut self) {
        let (name, ver) = KernelManager::get_running_kernel_info();
        self.running_kernel_name = name;
        self.running_kernel_ver = ver;
        self.kernels = KernelManager::get_catalog();
    }

    fn refresh_tweaks_state(&mut self) {
        self.current_governor = SystemTweaker::get_current_governor();
        let (val, is_opt) = SystemTweaker::get_max_map_count();
        self.max_map_count_val = val;
        self.is_max_map_opt = is_opt;
        self.is_split_lock_disabled = SystemTweaker::is_split_lock_disabled();
        self.is_gamemode_active = SystemTweaker::is_gamemode_active();
    }

    fn refresh_snapshots_state(&mut self) {
        self.is_btrfs = SnapshotManager::is_btrfs_filesystem();
        self.fs_type_name = if self.is_btrfs { "BTRFS" } else { "EXT4/XFS" }.to_string();
        self.snapshots = SnapshotManager::list_snapshots();
    }

    fn refresh_store_state(&mut self) {
        self.all_packages = PackageManager::get_catalog();
        self.filtered_packages = PackageManager::filter_catalog(
            &self.all_packages,
            self.store_cat,
            &self.store_search,
        );
    }

    pub fn view(&self) -> Element<'_, Message> {
        let side = sidebar(self.current_page, Message::SelectPage, Message::WindowDrag);
        let head = tools_header(
            self.hovered_header_btn,
            Message::HeaderBtnHovered,
            Message::OpenUrl,
            Message::WindowDrag,
            Message::WindowMinimize,
            Message::WindowMaximize,
            Message::WindowClose,
        );

        let main_content: Element<'_, Message> = match self.current_page {
            Page::Dashboard => ui::pages::dashboard::view(
                &self.metrics,
                Message::CleanCache,
                Message::SelectPage(Page::GameTweaks),
                Message::SelectPage(Page::KernelManager),
            ),
            Page::KernelManager => ui::pages::kernel_manager::view(
                &self.running_kernel_name,
                &self.running_kernel_ver,
                &self.kernels,
                &self.terminal_log,
                Message::InstallKernel,
                Message::RemoveKernel,
                Message::RebuildInitramfs,
                Message::ClearLog,
            ),
            Page::GameTweaks => ui::pages::game_tweaks::view(
                &self.current_governor,
                &self.max_map_count_val,
                self.is_max_map_opt,
                self.is_split_lock_disabled,
                self.is_gamemode_active,
                &self.terminal_log,
                Message::SetGovernor,
                Message::ToggleMaxMapCount,
                Message::ToggleSplitLock,
                Message::ToggleGameMode,
                Message::InstallGamingRuntimes,
                Message::ClearLog,
            ),
            Page::Snapshots => ui::pages::snapshots::view(
                self.is_btrfs,
                &self.fs_type_name,
                &self.snapshot_input,
                &self.snapshots,
                &self.terminal_log,
                Message::SnapshotInputChanged,
                Message::CreateSnapshot,
                Message::RestoreSnapshot,
                Message::DeleteSnapshot,
                Message::ClearLog,
            ),
            Page::Maintenance => ui::pages::maintenance::view(
                &self.terminal_log,
                Message::CleanCache,
                Message::VacuumJournal,
                Message::RemoveOrphans,
                Message::RankMirrors,
                Message::FixKeyrings,
                Message::ClearLog,
            ),
            Page::PackageStore => ui::pages::package_store::view(
                self.store_cat,
                &self.store_search,
                &self.filtered_packages,
                &self.terminal_log,
                Message::StoreSearchChanged,
                Message::StoreCategorySelected,
                Message::InstallPackage,
                Message::RemovePackage,
                Message::ClearLog,
            ),
        };

        let right_panel = column![head, main_content].width(Length::Fill);

        let divider = container("")
            .width(1)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.08))),
                ..container::Style::default()
            });

        let app_layout = row![side, divider, right_panel]
            .width(Length::Fill)
            .height(Length::Fill);

        container(app_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(crate::ui::theme::glass_window_style)
            .into()
    }
}
