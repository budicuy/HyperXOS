mod core;
mod ui;

use std::thread;
use std::time::Duration;

use crossbeam_channel::{unbounded, Receiver, Sender};
use iced::widget::{column, container, row};
use iced::window;
use iced::{Element, Length, Size, Subscription, Task};

use crate::core::bootloader::LimineBootloader;
use crate::core::configurator::{SystemConfigurator, UserConfig};
use crate::core::disk::{DiskInfo, DiskManager};
use crate::core::filesystem::FilesystemManager;
use crate::core::pacstrap::PacstrapInstaller;
use crate::core::partitioner::Partitioner;
use crate::core::preflight::PreflightStatus;

use crate::ui::header::header;
use crate::ui::stepper::{stepper_sidebar, StepId};
use crate::ui::steps::step7_install::InstallState;
use crate::ui::steps::{
    step1_welcome, step2_location, step3_keyboard, step4_disk, step5_users, step6_summary,
    step7_install, step8_finished,
};

#[derive(Debug, Clone)]
pub enum InstallerEvent {
    Stage { name: String, pct: f32 },
    Log(String),
    Finished(Result<(), String>),
}

#[derive(Debug, Clone)]
pub enum Message {
    HeaderHover(Option<&'static str>),
    OpenUrl(&'static str),
    Minimize,
    Close,

    // Step 1: Preflight
    RefreshPreflight,
    GotoLocation,

    // Step 2: Location
    SelectTimezone(String),
    SelectLocale(String),
    GotoWelcome,
    GotoKeyboard,

    // Step 3: Keyboard
    SelectKeyboard(String),
    ChangeTestText(String),
    GotoDisk,

    // Step 4: Storage
    SelectDisk(String),
    RefreshDisks,
    GotoUsers,

    // Step 5: Users
    ChangeFullname(String),
    ChangeUsername(String),
    ChangeHostname(String),
    ChangePassword(String),
    ChangeConfirmPassword(String),
    ToggleSameRoot(bool),
    ChangeRootPassword(String),
    GotoSummary,

    // Step 6: Summary & Confirm
    StartInstallation,

    // Step 7: Progress & Real-time engine
    EngineTick,
    EngineEvent(InstallerEvent),

    // Step 8: Finished
    ToggleRestart(bool),
    CompleteAndExit,
}

fn load_window_icon() -> Option<iced::window::Icon> {
    let bytes = include_bytes!("../assets/logo.png");
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "--auto-install" || args[1] == "-a") {
        let disk = args.get(2).cloned().unwrap_or_else(|| "/dev/vda".to_string());
        println!("============================================================");
        println!("  HyperXOS Installer (Pure Rust) — Headless Automated Mode  ");
        println!("  Target Disk: {}", disk);
        println!("============================================================");

        let (tx, rx) = unbounded();
        let handle = thread::spawn(move || {
            App::execute_installation(disk, UserConfig::default(), tx);
        });

        while let Ok(event) = rx.recv() {
            match event {
                InstallerEvent::Stage { name, pct } => {
                    println!("\n[STAGE {:.0}%] {}", pct, name);
                }
                InstallerEvent::Log(line) => {
                    println!("  -> {}", line);
                }
                InstallerEvent::Finished(res) => match res {
                    Ok(()) => {
                        println!("\n[✓] HyperXOS Installation COMPLETED SUCCESSFULLY!");
                        let _ = handle.join();
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("\n[-] INSTALLATION FAILED: {}", e);
                        let _ = handle.join();
                        std::process::exit(1);
                    }
                },
            }
        }
        let _ = handle.join();
        return Ok(());
    }

    let window_settings = window::Settings {
        size: Size::new(1060.0, 720.0),
        min_size: Some(Size::new(960.0, 640.0)),
        transparent: true,
        blur: true,
        decorations: false,
        icon: load_window_icon(),
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "hyper-x-installer".to_string(),
            override_redirect: false,
        },
        ..Default::default()
    };

    iced::application(App::default, App::update, App::view)
        .title("HyperXOS Installer (Pure Rust)")
        .font(include_bytes!("../assets/fonts/PlusJakartaSans.ttf").as_slice())
        .font(include_bytes!("../assets/fonts/Inter.ttf").as_slice())
        .default_font(crate::ui::theme::FONT_BODY)
        .window(window_settings)
        .subscription(App::subscription)
        .theme(|_: &App| iced::Theme::Dark)
        .style(|_: &App, _: &iced::Theme| iced::theme::Style {
            background_color: iced::Color::TRANSPARENT,
            text_color: crate::ui::theme::TEXT_MAIN,
        })
        .run()?;

    Ok(())
}

pub struct App {
    step: StepId,
    hovered_btn: Option<&'static str>,

    // Step 1 State
    preflight: PreflightStatus,

    // Step 2 State
    selected_tz: String,
    selected_locale: String,

    // Step 3 State
    selected_keyboard: String,
    test_keyboard_text: String,

    // Step 4 State
    disks: Vec<DiskInfo>,
    selected_disk_path: Option<String>,

    // Step 5 State
    user_config: UserConfig,
    confirm_password: String,
    use_same_root_password: bool,

    // Step 7 State
    install_state: InstallState,
    engine_rx: Option<Receiver<InstallerEvent>>,
    is_installing: bool,

    // Step 8 State
    restart_checked: bool,
}

impl Default for App {
    fn default() -> Self {
        let preflight = PreflightStatus::check();
        let disks = DiskManager::list_disks();
        let first_disk = disks.first().map(|d| d.path.clone());

        Self {
            step: StepId::Welcome,
            hovered_btn: None,
            preflight,
            selected_tz: "Asia/Jakarta".to_string(),
            selected_locale: "en_US.UTF-8".to_string(),
            selected_keyboard: "us".to_string(),
            test_keyboard_text: String::new(),
            disks,
            selected_disk_path: first_disk,
            user_config: UserConfig::default(),
            confirm_password: "password".to_string(),
            use_same_root_password: true,
            install_state: InstallState::default(),
            engine_rx: None,
            is_installing: false,
            restart_checked: true,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HeaderHover(id) => {
                self.hovered_btn = id;
                Task::none()
            }
            Message::OpenUrl(url) => {
                let _ = std::process::Command::new("xdg-open").arg(url).spawn();
                Task::none()
            }
            Message::Minimize => window::latest().and_then(|id| window::minimize(id, true)),
            Message::Close => window::latest().and_then(window::close),

            // Step 1: Preflight
            Message::RefreshPreflight => {
                self.preflight = PreflightStatus::check();
                self.disks = DiskManager::list_disks();
                Task::none()
            }
            Message::GotoLocation => {
                self.step = StepId::Location;
                Task::none()
            }

            // Step 2: Location
            Message::SelectTimezone(tz) => {
                self.selected_tz = tz;
                Task::none()
            }
            Message::SelectLocale(loc) => {
                self.selected_locale = loc;
                Task::none()
            }
            Message::GotoWelcome => {
                self.step = StepId::Welcome;
                Task::none()
            }
            Message::GotoKeyboard => {
                self.step = StepId::Keyboard;
                Task::none()
            }

            // Step 3: Keyboard
            Message::SelectKeyboard(layout) => {
                self.selected_keyboard = layout;
                Task::none()
            }
            Message::ChangeTestText(val) => {
                self.test_keyboard_text = val;
                Task::none()
            }
            Message::GotoDisk => {
                self.step = StepId::Disk;
                Task::none()
            }

            // Step 4: Storage
            Message::SelectDisk(path) => {
                self.selected_disk_path = Some(path);
                Task::none()
            }
            Message::RefreshDisks => {
                self.disks = DiskManager::list_disks();
                if self.selected_disk_path.is_none() {
                    self.selected_disk_path = self.disks.first().map(|d| d.path.clone());
                }
                Task::none()
            }
            Message::GotoUsers => {
                self.step = StepId::Users;
                Task::none()
            }

            // Step 5: Users
            Message::ChangeFullname(val) => {
                self.user_config.fullname = val;
                Task::none()
            }
            Message::ChangeUsername(val) => {
                self.user_config.username = val;
                Task::none()
            }
            Message::ChangeHostname(val) => {
                self.user_config.hostname = val;
                Task::none()
            }
            Message::ChangePassword(val) => {
                self.user_config.password = val.clone();
                if self.use_same_root_password {
                    self.user_config.root_password = val;
                }
                Task::none()
            }
            Message::ChangeConfirmPassword(val) => {
                self.confirm_password = val;
                Task::none()
            }
            Message::ToggleSameRoot(enabled) => {
                self.use_same_root_password = enabled;
                if enabled {
                    self.user_config.root_password = self.user_config.password.clone();
                }
                Task::none()
            }
            Message::ChangeRootPassword(val) => {
                self.user_config.root_password = val;
                Task::none()
            }
            Message::GotoSummary => {
                self.user_config.timezone = self.selected_tz.clone();
                self.user_config.locale = self.selected_locale.clone();
                self.user_config.keymap = self.selected_keyboard.clone();
                self.step = StepId::Summary;
                Task::none()
            }

            // Step 6: Start Install
            Message::StartInstallation => {
                self.step = StepId::Install;
                self.is_installing = true;
                self.install_state = InstallState::default();

                let (tx, rx) = unbounded();
                self.engine_rx = Some(rx);

                let disk_path = self.selected_disk_path.clone().unwrap_or_default();
                let user_cfg = self.user_config.clone();

                thread::spawn(move || {
                    Self::execute_installation(disk_path, user_cfg, tx);
                });

                Task::none()
            }

            // Step 7: Engine Realtime Loop
            Message::EngineTick => {
                if let Some(ref rx) = self.engine_rx {
                    while let Ok(event) = rx.try_recv() {
                        match event {
                            InstallerEvent::Stage { name, pct } => {
                                self.install_state.stage_name = name;
                                self.install_state.progress_pct = pct;
                            }
                            InstallerEvent::Log(line) => {
                                self.install_state.log_lines.push(line);
                            }
                            InstallerEvent::Finished(result) => {
                                self.is_installing = false;
                                match result {
                                    Ok(()) => {
                                        self.install_state.progress_pct = 100.0;
                                        self.step = StepId::Finished;
                                    }
                                    Err(err) => {
                                        self.install_state.is_failed = true;
                                        self.install_state.error_message = Some(err);
                                    }
                                }
                            }
                        }
                    }
                }
                Task::none()
            }
            Message::EngineEvent(_evt) => {
                // Direct event handler if needed
                Task::none()
            }

            // Step 8: Done
            Message::ToggleRestart(val) => {
                self.restart_checked = val;
                Task::none()
            }
            Message::CompleteAndExit => {
                if self.restart_checked {
                    let _ = std::process::Command::new("systemctl")
                        .arg("reboot")
                        .status();
                }
                window::latest().and_then(window::close)
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_installing {
            iced::time::every(Duration::from_millis(100)).map(|_| Message::EngineTick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let step_content: Element<Message> = match self.step {
            StepId::Welcome => step1_welcome::view(
                &self.preflight,
                Message::RefreshPreflight,
                Message::GotoLocation,
            ),
            StepId::Location => step2_location::view(
                &self.selected_tz,
                &self.selected_locale,
                Message::SelectTimezone,
                Message::SelectLocale,
                Message::GotoWelcome,
                Message::GotoKeyboard,
            ),
            StepId::Keyboard => step3_keyboard::view(
                &self.selected_keyboard,
                &self.test_keyboard_text,
                Message::SelectKeyboard,
                Message::ChangeTestText,
                Message::GotoLocation,
                Message::GotoDisk,
            ),
            StepId::Disk => step4_disk::view(
                &self.disks,
                self.selected_disk_path.as_deref(),
                Message::SelectDisk,
                Message::RefreshDisks,
                Message::GotoKeyboard,
                Message::GotoUsers,
            ),
            StepId::Users => step5_users::view(
                &self.user_config,
                &self.confirm_password,
                self.use_same_root_password,
                Message::ChangeFullname,
                Message::ChangeUsername,
                Message::ChangeHostname,
                Message::ChangePassword,
                Message::ChangeConfirmPassword,
                Message::ToggleSameRoot,
                Message::ChangeRootPassword,
                Message::GotoDisk,
                Message::GotoSummary,
            ),
            StepId::Summary => {
                let selected_disk = self
                    .disks
                    .iter()
                    .find(|d| Some(d.path.as_str()) == self.selected_disk_path.as_deref());
                step6_summary::view(
                    selected_disk,
                    &self.user_config,
                    Message::GotoUsers,
                    Message::StartInstallation,
                )
            }
            StepId::Install => {
                step7_install::view(&self.install_state)
            }
            StepId::Finished => step8_finished::view(
                self.restart_checked,
                Message::ToggleRestart,
                Message::CompleteAndExit,
            ),
        };

        let main_view = container(
            row![
                stepper_sidebar(self.step),
                column![
                    header(
                        self.hovered_btn,
                        Message::HeaderHover,
                        Message::OpenUrl,
                        Message::Minimize,
                        Message::Close,
                    ),
                    step_content,
                ]
                .width(Length::Fill)
                .height(Length::Fill),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(crate::ui::theme::glass_window_style);

        main_view.into()
    }

    /// Background thread executing the actual system installation steps in pure Rust
    fn execute_installation(
        disk_path: String,
        user_config: UserConfig,
        tx: Sender<InstallerEvent>,
    ) {
        if disk_path.is_empty() {
            let _ = tx.send(InstallerEvent::Finished(Err("No target disk specified".to_string())));
            return;
        }

        // Helper closures
        let send_log = {
            let tx = tx.clone();
            move |msg: &str| {
                let _ = tx.send(InstallerEvent::Log(msg.to_string()));
            }
        };

        // 1. Partitioning
        let _ = tx.send(InstallerEvent::Stage {
            name: format!("Partitioning {} (4GB FAT32 Boot + Btrfs Root)...", disk_path),
            pct: 10.0,
        });

        let layout = match Partitioner::partition_disk(&disk_path, send_log.clone()) {
            Ok(l) => l,
            Err(e) => {
                let _ = tx.send(InstallerEvent::Finished(Err(format!("Partitioning error: {}", e))));
                return;
            }
        };

        // 2. Formatting & Subvolumes
        let _ = tx.send(InstallerEvent::Stage {
            name: "Formatting Filesystems & Creating Subvolumes (@, @home)...".to_string(),
            pct: 22.0,
        });

        let fs_ctx = match FilesystemManager::setup_filesystems(&layout, send_log.clone()) {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(InstallerEvent::Finished(Err(format!("Filesystem error: {}", e))));
                return;
            }
        };

        // 3. Pacman preparation
        let _ = tx.send(InstallerEvent::Stage {
            name: "Preparing Pacman Mirrors, Keys, and Repositories...".to_string(),
            pct: 32.0,
        });

        if let Err(e) = PacstrapInstaller::prepare_pacman_environment(send_log.clone()) {
            let _ = tx.send(InstallerEvent::Finished(Err(format!("Pacman setup error: {}", e))));
            return;
        }

        // 4. Pacstrap fresh online installation
        let _ = tx.send(InstallerEvent::Stage {
            name: "Downloading & Installing Base System + KDE Plasma 6 (pacstrap)...".to_string(),
            pct: 45.0,
        });

        if let Err(e) = PacstrapInstaller::run_pacstrap(send_log.clone()) {
            let _ = tx.send(InstallerEvent::Finished(Err(format!("Pacstrap installation error: {}", e))));
            return;
        }

        // 5. System Configuration
        let _ = tx.send(InstallerEvent::Stage {
            name: "Configuring Target System (fstab, users, locale, SDDM)...".to_string(),
            pct: 80.0,
        });

        if let Err(e) = SystemConfigurator::configure_system(&fs_ctx, &user_config, send_log.clone()) {
            let _ = tx.send(InstallerEvent::Finished(Err(format!("Configuration error: {}", e))));
            return;
        }

        // 6. Limine Bootloader Installation
        let _ = tx.send(InstallerEvent::Stage {
            name: "Deploying Limine Bootloader & Registering UEFI NVRAM...".to_string(),
            pct: 92.0,
        });

        if let Err(e) = LimineBootloader::install(&disk_path, &fs_ctx, send_log.clone()) {
            let _ = tx.send(InstallerEvent::Finished(Err(format!("Bootloader installation error: {}", e))));
            return;
        }

        // 7. Complete!
        let _ = tx.send(InstallerEvent::Stage {
            name: "Installation Successfully Completed!".to_string(),
            pct: 100.0,
        });
        let _ = tx.send(InstallerEvent::Log("System installation and configuration complete. Ready to reboot.".to_string()));
        let _ = tx.send(InstallerEvent::Finished(Ok(())));
    }
}
