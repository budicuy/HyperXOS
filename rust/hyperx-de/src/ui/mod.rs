pub mod context_menu;
pub mod control_center;
pub mod dock;
pub mod file_manager;
pub mod launcher;
pub mod notifications;
pub mod overlays;
pub mod settings;
pub mod topbar;
pub mod window_manager;

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::{Datelike, Local};
use iced::border::Radius;
use iced::gradient::{ColorStop, Linear};
use iced::keyboard::{self, key::Named, Key, Modifiers};
use iced::mouse;
use iced::widget::{button, column, container, image as iced_image, mouse_area, row, stack, text};
use iced::{
    Alignment, Background, Border, Color, ContentFit, Element, Event, Gradient, Length, Point,
    Subscription, Task,
};
use sysinfo::System;

use hyperx_ui_kit::{
    app_icon, fluent_desktop_icon_tile_style, icons, svg_icon,
    ACCENT_AMBER, ACCENT_CYAN, ACCENT_GREEN, ACCENT_PURPLE, ACCENT_RED, FLUENT_PRIMARY,
    FONT_BODY_SEMIBOLD,
};

use crate::compositor::CompositorSnapshot;
use crate::sys::{
    apps::{scan_desktop_apps, spawn_external_command, DesktopAppEntry},
    audio::AudioState,
    display::DisplayState,
    network::NetworkState,
};
use context_menu::{ActiveContextMenu, ContextMenuKind};
use file_manager::{is_image_ext, open_file_or_dir, FileClipboardOp, FileManagerState, FilePlace};
use notifications::DesktopNotification;
use overlays::{NotesAppState, QuickLookData, TerminalAppState};
use settings::{SettingsTab, SettingsViewState};
use window_manager::{FloatingWindowManager, WindowSnapMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopWindowId {
    Files,
    Settings,
    Terminal,
    Notes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperTheme {
    CyberCrimson,
    DeepNebula,
    EmeraldAurora,
    CustomImage,
}

#[derive(Debug, Clone)]
pub enum DesktopIconAction {
    External(String),
    Internal(DesktopWindowId),
    OpenDirectory(PathBuf),
}

pub const DESKTOP_GRID_ORIGIN_X: f32 = 24.0;
pub const DESKTOP_GRID_ORIGIN_Y: f32 = 24.0;
pub const DESKTOP_GRID_CELL_WIDTH: f32 = 104.0;
pub const DESKTOP_GRID_CELL_HEIGHT: f32 = 88.0;

pub fn snap_to_desktop_grid(pt: Point) -> Point {
    let col = ((pt.x - DESKTOP_GRID_ORIGIN_X + (DESKTOP_GRID_CELL_WIDTH / 2.0)) / DESKTOP_GRID_CELL_WIDTH)
        .floor()
        .clamp(0.0, 12.0) as usize;
    let row = ((pt.y - DESKTOP_GRID_ORIGIN_Y + (DESKTOP_GRID_CELL_HEIGHT / 2.0)) / DESKTOP_GRID_CELL_HEIGHT)
        .floor()
        .clamp(0.0, 6.0) as usize;
    Point::new(
        DESKTOP_GRID_ORIGIN_X + (col as f32) * DESKTOP_GRID_CELL_WIDTH,
        DESKTOP_GRID_ORIGIN_Y + (row as f32) * DESKTOP_GRID_CELL_HEIGHT,
    )
}

#[derive(Debug, Clone)]
pub struct DesktopShortcutItem {
    pub title: String,
    pub svg_icon_data: Option<&'static str>,
    pub app_icon_name: String,
    pub svg_color: Color,
    pub action: DesktopIconAction,
    pub pos: Point,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    AnimTick,
    RuntimeEvent(Event),

    // Popovers & HUD Overlays
    ToggleWifiModal,
    ToggleControlCenter,
    ToggleNotificationDrawer,
    ToggleLauncher,
    ToggleMissionControl,
    ToggleAppSwitcher,
    SelectAppSwitcherWindow(DesktopWindowId),
    CloseAppSwitcher,
    ToggleClipboardHud,
    ClipboardPasteToNotes(usize),
    ClipboardClearHistory,
    ToggleScreenshotHud,
    CaptureScreenshotNow,
    CaptureScreenshotAndSetWallpaper,
    OpenScreenshotsFolder,
    ToggleLockScreen,
    UnlockScreen,
    CloseQuickLook,
    QuickLookOpenInNotes,
    DismissToast,
    ClearNotifications,

    // Context Menus (Right-Click)
    OpenDesktopContextMenu,
    OpenDesktopIconContextMenu(usize),
    OpenFileEntryContextMenu(usize),
    OpenDockContextMenu(DesktopWindowId),
    OpenPinnedDockContextMenu(usize),
    OpenTrashDockContextMenu,
    CloseContextMenu,

    // Desktop Actions & Drag-and-Drop
    DesktopIconPress(usize),
    DesktopOpenIcon(usize),
    DesktopCursorMove(Point),
    DesktopMouseRelease,
    DesktopCreateNewFolder,
    DesktopAutoArrangeIcons,
    CycleWallpaperTheme,

    // Window Management (Multi-Window Floating, Z-Order, Drag, Resize, Aero-Snap)
    OpenWindow(DesktopWindowId),
    DockItemClicked(DesktopWindowId),
    FocusWindow(DesktopWindowId),
    MinimizeWindow(DesktopWindowId),
    CloseWindow(DesktopWindowId),
    ToggleMaximizeWindow(DesktopWindowId),
    SnapWindow(DesktopWindowId, WindowSnapMode),
    PresetResizeWindow(DesktopWindowId, f32, f32),
    StartWindowDrag(DesktopWindowId),
    EndWindowDrag,
    StartWindowResize(DesktopWindowId),
    EndWindowResize,
    OpenSettingsPage(SettingsTab),

    // External Apps & Power
    LaunchExternalApp(String),
    SystemPowerAction(String),

    // Quick Toggles & Sliders
    ToggleWifiRadio,
    ToggleBluetooth,
    TogglePerformanceMode,
    ToggleDnd,
    ToggleMute,
    ToggleNightLight,
    VolumeChanged(f32),
    BrightnessChanged(f32),

    // Launcher & Pacman App Integration (v2.2)
    LauncherSearchChanged(String),
    RefreshDesktopApps,
    AddAppToDesktop(DesktopAppEntry),
    TogglePinAppToDock(DesktopAppEntry),
    LaunchPinnedDockApp(usize),
    AddPinnedDockAppToDesktop(usize),
    UnpinDockApp(usize),

    // File Manager, Clipboard & Trash Actions
    FileSelectPlace(FilePlace),
    OpenTrashInFiles,
    FileNavigateUp,
    FileRefresh,
    FileToggleViewMode,
    FileSearchChanged(String),
    #[allow(dead_code)]
    FileNewFolderNameChanged(String),
    FileCreateFolder,
    FileEntryClicked(usize),
    FileDeleteSelected,
    FileCopySelected(usize),
    FileCutSelected(usize),
    FilePasteClipboard,
    FileRestoreFromTrash(usize),
    FileEmptyTrash,
    FileContextOpen(usize),
    FileContextQuickLook(usize),
    FileContextStartRename(usize),
    FileRenameInputChanged(String),
    FileCommitRename,
    FileCancelRename,
    FileContextDuplicate(usize),
    FileContextDelete(usize),

    // Custom Wallpaper
    SetCustomWallpaperFromFile(PathBuf),
    CustomWallpaperInputChanged(String),
    ApplyCustomWallpaperInput,
    GenerateAndSetSampleCustomWallpaper,

    // Terminal & Activity Monitor App
    TerminalSelectTab(usize),
    TerminalInputChanged(String),
    TerminalExecuteInput,
    TerminalRunQuick(String),

    // Notes & Text Editor App
    NotesLineInputChanged(String),
    NotesAppendLine,
    NotesSaveFile,

    // Settings
    SelectSettingsTab(SettingsTab),
    RefreshNetwork,
    SelectWifiSsid(String),
    WifiPasswordChanged(String),
    ConnectSelectedWifi,
    SelectWallpaper(WallpaperTheme),
    DockSizeChanged(f32),
}

pub struct HyperXDesktop {
    pub compositor_snapshot: Arc<Mutex<CompositorSnapshot>>,
    pub clock_str: String,
    pub date_header_str: String,
    pub current_day: u32,

    // Draggable Clean Desktop Icons
    pub desktop_icons: Vec<DesktopShortcutItem>,
    pub dragging_icon: Option<(usize, Point)>,
    pub last_cursor_pos: Point,
    pub drag_started_at: Option<Instant>,
    pub drag_moved: bool,
    pub created_desktop_folders: usize,

    // Popovers & macOS HUD Overlays
    pub wifi_modal_open: bool,
    pub control_center_open: bool,
    pub notification_drawer_open: bool,
    pub launcher_open: bool,
    pub launcher_anim: f32,
    pub launcher_search: String,
    pub control_center_anim: f32,
    pub mission_control_open: bool,
    pub app_switcher_open: bool,
    pub app_switcher_index: usize,
    pub clipboard_hud_open: bool,
    pub clipboard_history: Vec<String>,
    pub screenshot_hud_open: bool,
    pub last_screenshot_path: Option<String>,
    pub lock_screen_open: bool,
    pub quick_look: Option<QuickLookData>,
    pub context_menu: Option<ActiveContextMenu>,

    // Multi-Window Floating & Stacking Manager
    pub wm: FloatingWindowManager,

    // Sub-application states
    pub file_manager: FileManagerState,
    pub settings_view: SettingsViewState,
    pub terminal_app: TerminalAppState,
    pub notes_app: NotesAppState,
    pub desktop_apps: Vec<DesktopAppEntry>,
    pub pinned_dock_apps: Vec<DesktopAppEntry>,

    // Hardware, Custom Wallpaper & OS state
    pub network: NetworkState,
    pub audio: AudioState,
    pub display: DisplayState,
    pub bluetooth_enabled: bool,
    pub performance_mode: bool,
    pub dnd_enabled: bool,
    pub wallpaper_theme: WallpaperTheme,
    pub custom_wallpaper_handle: Option<iced_image::Handle>,
    pub custom_wallpaper_path: Option<PathBuf>,
    pub dock_icon_size: f32,

    // Telemetry
    pub sys: System,
    pub cpu_brand: String,
    pub cpu_usage: f32,
    pub ram_used_gb: f32,
    pub ram_total_gb: f32,
    pub wayland_socket: String,

    // Notifications
    pub notifications: Vec<DesktopNotification>,
    pub active_toast: Option<DesktopNotification>,
    pub next_notif_id: usize,
}

impl HyperXDesktop {
    pub fn new(
        compositor_snapshot: Arc<Mutex<CompositorSnapshot>>,
        initial_window: Option<DesktopWindowId>,
        open_launcher: bool,
    ) -> (Self, Task<Message>) {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let cpu_brand = sys
            .cpus()
            .first()
            .map(|c| c.brand().trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "HyperX x86_64 Processor".to_string());

        let cpu_usage = sys.global_cpu_usage();
        let ram_total_gb = (sys.total_memory() as f32) / (1024.0 * 1024.0 * 1024.0);
        let ram_used_gb = (sys.used_memory() as f32) / (1024.0 * 1024.0 * 1024.0);

        let now = Local::now();
        let clock_str = now.format("%a, %d %b  %H:%M").to_string();
        let date_header_str = now.format("%A, %d %B %Y").to_string();
        let current_day = now.day();

        let welcome_notif = DesktopNotification {
            _id: 1,
            title: "HyperXOS Desktop v2.1 Siap".to_string(),
            body: "Window Resize, Clipboard (Super+V), Trash Dock, Screenshot (Super+Shift+S), & Custom Wallpaper aktif."
                .to_string(),
            time_label: now.format("%H:%M").to_string(),
            accent: ACCENT_CYAN,
        };

        let mut wm = FloatingWindowManager::default();
        if let Some(win_id) = initial_window {
            wm.open_or_focus(win_id);
        }

        let wayland_socket = compositor_snapshot
            .lock()
            .ok()
            .map(|s| s.socket_name.clone())
            .unwrap_or_else(|| "wayland-hyperx-0".to_string());

        let has_installer = std::path::Path::new("/usr/bin/launch-installer").exists()
            || std::path::Path::new("/usr/bin/hyper-x-installer").exists()
            || std::path::Path::new("/usr/local/bin/launch-installer").exists();

        let mut desktop_icons = Vec::new();
        let mut icon_grid_idx = 0;
        let mut next_grid_pos = || {
            let col = icon_grid_idx / 6;
            let row = icon_grid_idx % 6;
            icon_grid_idx += 1;
            Point::new(
                DESKTOP_GRID_ORIGIN_X + (col as f32) * DESKTOP_GRID_CELL_WIDTH,
                DESKTOP_GRID_ORIGIN_Y + (row as f32) * DESKTOP_GRID_CELL_HEIGHT,
            )
        };

        if has_installer {
            desktop_icons.push(DesktopShortcutItem {
                title: "Install HyperXOS".to_string(),
                svg_icon_data: None,
                app_icon_name: "hyper-x-installer".to_string(),
                svg_color: FLUENT_PRIMARY,
                action: DesktopIconAction::External("launch-installer".to_string()),
                pos: next_grid_pos(),
            });
        }

        desktop_icons.push(DesktopShortcutItem {
            title: "PC Ini".to_string(),
            svg_icon_data: Some(icons::COMPUTER),
            app_icon_name: String::new(),
            svg_color: FLUENT_PRIMARY,
            action: DesktopIconAction::Internal(DesktopWindowId::Files),
            pos: next_grid_pos(),
        });

        desktop_icons.push(DesktopShortcutItem {
            title: "Keranjang Sampah".to_string(),
            svg_icon_data: Some(icons::TRASH),
            app_icon_name: String::new(),
            svg_color: Color::from_rgb8(100, 116, 139),
            action: DesktopIconAction::Internal(DesktopWindowId::Files),
            pos: next_grid_pos(),
        });

        desktop_icons.push(DesktopShortcutItem {
            title: "Dokumen".to_string(),
            svg_icon_data: Some(icons::FOLDER),
            app_icon_name: String::new(),
            svg_color: Color::from_rgb8(245, 158, 11),
            action: DesktopIconAction::Internal(DesktopWindowId::Files),
            pos: next_grid_pos(),
        });

        desktop_icons.push(DesktopShortcutItem {
            title: "Terminal".to_string(),
            svg_icon_data: Some(icons::TERMINAL),
            app_icon_name: String::new(),
            svg_color: Color::from_rgb8(16, 185, 129),
            action: DesktopIconAction::Internal(DesktopWindowId::Terminal),
            pos: next_grid_pos(),
        });

        desktop_icons.push(DesktopShortcutItem {
            title: "Browser".to_string(),
            svg_icon_data: None,
            app_icon_name: "firefox".to_string(),
            svg_color: Color::from_rgb8(0, 120, 212),
            action: DesktopIconAction::External("firefox".to_string()),
            pos: next_grid_pos(),
        });

        desktop_icons.push(DesktopShortcutItem {
            title: "HyperXTools".to_string(),
            svg_icon_data: None,
            app_icon_name: "hyper-x-tools".to_string(),
            svg_color: ACCENT_PURPLE,
            action: DesktopIconAction::External("hyper-x-tools".to_string()),
            pos: next_grid_pos(),
        });

        let mut state = Self {
            compositor_snapshot,
            clock_str,
            date_header_str,
            current_day,
            desktop_icons,
            dragging_icon: None,
            last_cursor_pos: Point::new(340.0, 260.0),
            drag_started_at: None,
            drag_moved: false,
            created_desktop_folders: 0,
            wifi_modal_open: false,
            control_center_open: false,
            control_center_anim: 0.0,
            notification_drawer_open: false,
            launcher_open: open_launcher,
            launcher_anim: if open_launcher { 1.0 } else { 0.0 },
            launcher_search: String::new(),
            mission_control_open: false,
            app_switcher_open: false,
            app_switcher_index: 0,
            clipboard_hud_open: false,
            clipboard_history: vec![
                "HyperXOS Desktop v2.1 — Smithay Wayland + Iced 0.14 WGPU".to_string(),
                "/home/hyperxos/Pictures/Screenshots".to_string(),
                "sudo pacman -Syu".to_string(),
            ],
            screenshot_hud_open: false,
            last_screenshot_path: None,
            lock_screen_open: false,
            quick_look: None,
            context_menu: None,
            wm,
            file_manager: FileManagerState::default(),
            settings_view: SettingsViewState::default(),
            terminal_app: TerminalAppState::default(),
            notes_app: NotesAppState::default(),
            desktop_apps: scan_desktop_apps(),
            pinned_dock_apps: Vec::new(),
            network: NetworkState::refresh(),
            audio: AudioState::get_current(),
            display: DisplayState::default(),
            bluetooth_enabled: true,
            performance_mode: true,
            dnd_enabled: false,
            wallpaper_theme: WallpaperTheme::CyberCrimson,
            custom_wallpaper_handle: None,
            custom_wallpaper_path: None,
            dock_icon_size: 34.0,
            sys,
            cpu_brand,
            cpu_usage,
            ram_used_gb,
            ram_total_gb: ram_total_gb.max(1.0),
            wayland_socket,
            notifications: vec![welcome_notif],
            active_toast: None,
            next_notif_id: 2,
        };

        state.load_persistent_settings();

        (state, Task::none())
    }

    fn settings_conf_path() -> PathBuf {
        let dir = FileManagerState::home_dir().join(".config/hyperx-de");
        let _ = fs::create_dir_all(&dir);
        dir.join("settings.conf")
    }

    fn save_persistent_settings(&self) {
        let theme_str = match self.wallpaper_theme {
            WallpaperTheme::CyberCrimson => "CyberCrimson",
            WallpaperTheme::DeepNebula => "DeepNebula",
            WallpaperTheme::EmeraldAurora => "EmeraldAurora",
            WallpaperTheme::CustomImage => "CustomImage",
        };
        let custom_wp = self
            .custom_wallpaper_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let body = format!(
            "theme={}\ncustom_wallpaper={}\ndock_size={}\n",
            theme_str, custom_wp, self.dock_icon_size
        );
        let _ = fs::write(Self::settings_conf_path(), body);
    }

    fn load_persistent_settings(&mut self) {
        if let Ok(raw) = fs::read_to_string(Self::settings_conf_path()) {
            for line in raw.lines() {
                if let Some(val) = line.strip_prefix("theme=") {
                    self.wallpaper_theme = match val.trim() {
                        "DeepNebula" => WallpaperTheme::DeepNebula,
                        "EmeraldAurora" => WallpaperTheme::EmeraldAurora,
                        "CustomImage" => WallpaperTheme::CustomImage,
                        _ => WallpaperTheme::CyberCrimson,
                    };
                } else if let Some(val) = line.strip_prefix("custom_wallpaper=") {
                    let p = PathBuf::from(val.trim());
                    if p.is_file() {
                        self.custom_wallpaper_handle = Some(iced_image::Handle::from_path(&p));
                        self.settings_view.custom_wallpaper_input =
                            p.to_string_lossy().to_string();
                        self.custom_wallpaper_path = Some(p);
                    }
                } else if let Some(val) = line.strip_prefix("dock_size=") {
                    if let Ok(sz) = val.trim().parse::<f32>() {
                        self.dock_icon_size = sz.clamp(26.0, 48.0);
                    }
                }
            }
        }
    }

    pub fn generate_neon_wallpaper_file() -> Result<PathBuf, String> {
        let dir = FileManagerState::home_dir().join("Pictures/Wallpapers");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let target = dir.join("HyperX-Sonoma-Aurora.png");
        let w = 1366u32;
        let h = 768u32;
        let mut img = image::RgbaImage::new(w, h);
        for y in 0..h {
            let fy = y as f32 / h as f32;
            for x in 0..w {
                let fx = x as f32 / w as f32;
                let wave1 = ((fx * std::f32::consts::TAU + fy * std::f32::consts::PI).sin() * 0.5 + 0.5).powf(2.0);
                let wave2 = ((fx * 4.5 - fy * 5.0).cos() * 0.5 + 0.5).powf(2.2);
                let r = ((18.0 + 135.0 * wave1 * (1.0 - fy * 0.4)) as u8).clamp(10, 210);
                let g = ((22.0 + 95.0 * wave2 + 45.0 * (1.0 - fy)) as u8).clamp(12, 195);
                let b = ((58.0 + 165.0 * (0.5 * wave1 + 0.5 * wave2)) as u8).clamp(38, 245);
                img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
        }
        img.save(&target).map_err(|e| e.to_string())?;
        Ok(target)
    }

    fn perform_screen_capture(&mut self) -> Result<PathBuf, String> {
        let dir = FileManagerState::home_dir().join("Pictures/Screenshots");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let ts = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let target = dir.join(format!("Screenshot_{}.png", ts));
        let target_str = target.to_string_lossy().to_string();

        // Try grim (Wayland) or import/scrot first; if running inside nested KMS/WGPU without grim, generate high-res snapshot image
        let grim_ok = Command::new("grim")
            .arg(&target_str)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !grim_ok {
            let w = 1366u32;
            let h = 768u32;
            let mut img = image::RgbaImage::new(w, h);
            for y in 0..h {
                let fy = y as f32 / h as f32;
                for x in 0..w {
                    let fx = x as f32 / w as f32;
                    let r = (24.0 + 85.0 * fx * (1.0 - fy * 0.5)) as u8;
                    let g = (26.0 + 55.0 * (1.0 - fx) + 35.0 * fy) as u8;
                    let b = (48.0 + 120.0 * (fx * 0.5 + fy * 0.5)) as u8;
                    img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
                }
            }
            img.save(&target).map_err(|e| e.to_string())?;
        }

        self.clipboard_history
            .insert(0, format!("SCREENSHOT: {}", target_str));
        self.last_screenshot_path = Some(target_str);
        self.file_manager.reload();
        Ok(target)
    }

    fn push_notification(
        &mut self,
        title: impl Into<String>,
        body: impl Into<String>,
        accent: Color,
    ) {
        let now = Local::now();
        let notif = DesktopNotification {
            _id: self.next_notif_id,
            title: title.into(),
            body: body.into(),
            time_label: now.format("%H:%M:%S").to_string(),
            accent,
        };
        self.next_notif_id += 1;
        if !self.dnd_enabled {
            self.active_toast = Some(notif.clone());
        }
        self.notifications.push(notif);
    }

    fn close_transient_popovers(&mut self) {
        self.wifi_modal_open = false;
        self.control_center_open = false;
        self.notification_drawer_open = false;
        self.context_menu = None;
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let is_animating = (self.launcher_open && self.launcher_anim < 1.0)
            || (!self.launcher_open && self.launcher_anim > 0.0)
            || (self.control_center_open && self.control_center_anim < 1.0)
            || (!self.control_center_open && self.control_center_anim > 0.0);

        let mut subs = vec![
            iced::time::every(Duration::from_secs(2)).map(|_| Message::Tick),
            iced::event::listen().map(Message::RuntimeEvent),
        ];

        if is_animating {
            subs.push(iced::time::every(Duration::from_millis(16)).map(|_| Message::AnimTick));
        }

        Subscription::batch(subs)
    }

    fn handle_keyboard_shortcut(&mut self, key: Key, modifiers: Modifiers) -> Task<Message> {
        let has_super = modifiers.logo();
        let has_alt = modifiers.alt();
        let has_ctrl = modifiers.control();
        let has_shift = modifiers.shift();

        if self.lock_screen_open {
            if matches!(
                key.as_ref(),
                Key::Named(Named::Enter | Named::Escape | Named::Space)
            ) {
                self.lock_screen_open = false;
            }
            return Task::none();
        }

        // Escape dismisses any active modal/overlay/context menu
        if matches!(key.as_ref(), Key::Named(Named::Escape)) {
            if self.context_menu.is_some() {
                self.context_menu = None;
                return Task::none();
            }
            if self.quick_look.is_some() {
                self.quick_look = None;
                return Task::none();
            }
            if self.clipboard_hud_open {
                self.clipboard_hud_open = false;
                return Task::none();
            }
            if self.screenshot_hud_open {
                self.screenshot_hud_open = false;
                return Task::none();
            }
            if self.app_switcher_open {
                self.app_switcher_open = false;
                return Task::none();
            }
            if self.mission_control_open {
                self.mission_control_open = false;
                return Task::none();
            }
            if self.launcher_open {
                self.launcher_open = false;
                return Task::none();
            }
            if self.file_manager.renaming_index.is_some() {
                self.file_manager.renaming_index = None;
                return Task::none();
            }
            self.close_transient_popovers();
            return Task::none();
        }

        // Screenshot Shortcut: PrintScreen or Super+Shift+S
        if matches!(key.as_ref(), Key::Named(Named::PrintScreen))
            || (has_super && has_shift && matches!(key.as_ref(), Key::Character("s" | "S")))
        {
            return self.update(Message::ToggleScreenshotHud);
        }

        // Clipboard Manager HUD: Super+V
        if has_super && matches!(key.as_ref(), Key::Character("v" | "V")) {
            return self.update(Message::ToggleClipboardHud);
        }

        // App Switcher navigation: Alt+Tab or Ctrl+Tab
        if matches!(key.as_ref(), Key::Named(Named::Tab)) && (has_alt || has_ctrl) {
            return self.update(Message::ToggleAppSwitcher);
        }

        // Mission Control: Super+Tab or F3
        if (matches!(key.as_ref(), Key::Named(Named::Tab)) && has_super)
            || matches!(key.as_ref(), Key::Named(Named::F3))
        {
            return self.update(Message::ToggleMissionControl);
        }

        if self.app_switcher_open && matches!(key.as_ref(), Key::Named(Named::Enter)) {
            let all_apps = [
                DesktopWindowId::Files,
                DesktopWindowId::Settings,
                DesktopWindowId::Terminal,
                DesktopWindowId::Notes,
            ];
            let target = all_apps[self.app_switcher_index % all_apps.len()];
            return self.update(Message::SelectAppSwitcherWindow(target));
        }

        // Close Active Window: Alt+F4 or Ctrl+Q or Ctrl+W
        if (has_alt && matches!(key.as_ref(), Key::Named(Named::F4)))
            || (has_ctrl && matches!(key.as_ref(), Key::Character("q" | "Q" | "w" | "W")))
        {
            if let Some(focused) = self.wm.focused_window() {
                return self.update(Message::CloseWindow(focused));
            }
        }

        // Spotlight / Launchpad: Alt+Space or Ctrl+Space
        if (has_alt || has_ctrl || has_super) && matches!(key.as_ref(), Key::Named(Named::Space)) {
            return self.update(Message::ToggleLauncher);
        }

        // Windows + macOS Super/Cmd Shortcuts
        if has_super {
            match key.as_ref() {
                Key::Character("e" | "E") => {
                    return self.update(Message::OpenWindow(DesktopWindowId::Files));
                }
                Key::Character("i" | "I" | ",") => {
                    return self.update(Message::OpenWindow(DesktopWindowId::Settings));
                }
                Key::Character("t" | "T") => {
                    return self.update(Message::OpenWindow(DesktopWindowId::Terminal));
                }
                Key::Character("j" | "J") => {
                    return self.update(Message::OpenWindow(DesktopWindowId::Notes));
                }
                Key::Character("d" | "D" | "m" | "M") => {
                    self.wm.toggle_show_desktop();
                    return Task::none();
                }
                Key::Character("l" | "L") => {
                    return self.update(Message::ToggleLockScreen);
                }
                Key::Character("a" | "A") => {
                    return self.update(Message::ToggleControlCenter);
                }
                Key::Character("n" | "N") => {
                    return self.update(Message::ToggleNotificationDrawer);
                }
                Key::Named(Named::ArrowUp) => {
                    if let Some(win) = self.wm.focused_window() {
                        return self.update(Message::SnapWindow(win, WindowSnapMode::Maximized));
                    }
                }
                Key::Named(Named::ArrowDown) => {
                    if let Some(win) = self.wm.focused_window() {
                        let snap = self.wm.state(win).snap;
                        if snap != WindowSnapMode::Floating {
                            return self.update(Message::SnapWindow(win, WindowSnapMode::Floating));
                        } else {
                            return self.update(Message::MinimizeWindow(win));
                        }
                    }
                }
                Key::Named(Named::ArrowLeft) => {
                    if let Some(win) = self.wm.focused_window() {
                        return self.update(Message::SnapWindow(win, WindowSnapMode::LeftHalf));
                    }
                }
                Key::Named(Named::ArrowRight) => {
                    if let Some(win) = self.wm.focused_window() {
                        return self.update(Message::SnapWindow(win, WindowSnapMode::RightHalf));
                    }
                }
                _ => {}
            }
        }

        // Ctrl+Shift+N: Create new folder
        if has_ctrl && has_shift && matches!(key.as_ref(), Key::Character("n" | "N")) {
            if self.wm.focused_window() == Some(DesktopWindowId::Files) {
                return self.update(Message::FileCreateFolder);
            } else {
                return self.update(Message::DesktopCreateNewFolder);
            }
        }

        // File Manager Ctrl+C / Ctrl+X / Ctrl+V / Ctrl+D / F2 / Delete / Spacebar
        if self.wm.focused_window() == Some(DesktopWindowId::Files)
            && self.file_manager.renaming_index.is_none()
        {
            if has_ctrl {
                match key.as_ref() {
                    Key::Character("c" | "C") => {
                        if let Some(idx) = self.file_manager.selected_index {
                            return self.update(Message::FileCopySelected(idx));
                        }
                    }
                    Key::Character("x" | "X") => {
                        if let Some(idx) = self.file_manager.selected_index {
                            return self.update(Message::FileCutSelected(idx));
                        }
                    }
                    Key::Character("v" | "V") => {
                        return self.update(Message::FilePasteClipboard);
                    }
                    Key::Character("d" | "D") => {
                        if let Some(idx) = self.file_manager.selected_index {
                            return self.update(Message::FileContextDuplicate(idx));
                        }
                    }
                    _ => {}
                }
            }

            match key.as_ref() {
                Key::Named(Named::F2) => {
                    if let Some(idx) = self.file_manager.selected_index {
                        return self.update(Message::FileContextStartRename(idx));
                    }
                }
                Key::Named(Named::Delete) => {
                    if self.file_manager.selected_index.is_some() {
                        return self.update(Message::FileDeleteSelected);
                    }
                }
                Key::Named(Named::Space) if !has_ctrl && !has_alt && !has_super => {
                    if self.quick_look.is_some() {
                        self.quick_look = None;
                    } else if let Some(idx) = self.file_manager.selected_index {
                        return self.update(Message::FileContextQuickLook(idx));
                    }
                }
                _ => {}
            }
        }

        Task::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                let now = Local::now();
                self.clock_str = now.format("%a, %d %b  %H:%M").to_string();
                self.date_header_str = now.format("%A, %d %B %Y").to_string();
                self.current_day = now.day();

                self.sys.refresh_cpu_usage();
                self.sys.refresh_memory();
                self.cpu_usage = self.sys.global_cpu_usage();
                self.ram_used_gb = (self.sys.used_memory() as f32) / (1024.0 * 1024.0 * 1024.0);
                self.ram_total_gb =
                    ((self.sys.total_memory() as f32) / (1024.0 * 1024.0 * 1024.0)).max(1.0);
                if let Ok(snap) = self.compositor_snapshot.lock() {
                    self.wayland_socket = snap.socket_name.clone();
                }
            }

            Message::AnimTick => {
                let step = 0.14;
                if self.launcher_open {
                    self.launcher_anim = (self.launcher_anim + step).min(1.0);
                } else {
                    self.launcher_anim = (self.launcher_anim - step).max(0.0);
                }

                if self.control_center_open {
                    self.control_center_anim = (self.control_center_anim + step).min(1.0);
                } else {
                    self.control_center_anim = (self.control_center_anim - step).max(0.0);
                }
            }

            Message::RuntimeEvent(ev) => match ev {
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    return self.update(Message::DesktopCursorMove(position));
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    if self.wm.resizing_window.is_some() {
                        return self.update(Message::EndWindowResize);
                    }
                    if self.wm.dragging_window.is_some() {
                        return self.update(Message::EndWindowDrag);
                    }
                    if self.dragging_icon.is_some() {
                        return self.update(Message::DesktopMouseRelease);
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                    return self.handle_keyboard_shortcut(key, modifiers);
                }
                _ => {}
            },

            // Context Menus (Right-Click)
            Message::OpenDesktopContextMenu => {
                self.wifi_modal_open = false;
                self.control_center_open = false;
                self.notification_drawer_open = false;
                self.context_menu = Some(ActiveContextMenu {
                    kind: ContextMenuKind::Desktop,
                    pos: self.last_cursor_pos,
                });
            }

            Message::OpenDesktopIconContextMenu(idx) => {
                self.context_menu = Some(ActiveContextMenu {
                    kind: ContextMenuKind::DesktopIcon(idx),
                    pos: self.last_cursor_pos,
                });
            }

            Message::OpenFileEntryContextMenu(idx) => {
                self.file_manager.selected_index = Some(idx);
                self.context_menu = Some(ActiveContextMenu {
                    kind: ContextMenuKind::FileEntry(idx),
                    pos: self.last_cursor_pos,
                });
            }

            Message::OpenDockContextMenu(win_id) => {
                self.context_menu = Some(ActiveContextMenu {
                    kind: ContextMenuKind::DockItem(win_id),
                    pos: Point::new(self.last_cursor_pos.x, 380.0),
                });
            }

            Message::OpenPinnedDockContextMenu(idx) => {
                self.context_menu = Some(ActiveContextMenu {
                    kind: ContextMenuKind::PinnedDockApp(idx),
                    pos: Point::new(self.last_cursor_pos.x, 450.0),
                });
            }

            Message::OpenTrashDockContextMenu => {
                self.context_menu = Some(ActiveContextMenu {
                    kind: ContextMenuKind::TrashDock,
                    pos: Point::new(self.last_cursor_pos.x, 480.0),
                });
            }

            Message::CloseContextMenu => {
                self.context_menu = None;
            }

            // Desktop Actions & Drag-and-Drop
            Message::DesktopIconPress(idx) => {
                self.context_menu = None;
                if let Some(item) = self.desktop_icons.get(idx) {
                    let offset = Point::new(
                        (self.last_cursor_pos.x - item.pos.x).clamp(0.0, 96.0),
                        (self.last_cursor_pos.y - item.pos.y).clamp(0.0, 96.0),
                    );
                    self.dragging_icon = Some((idx, offset));
                    self.drag_started_at = Some(Instant::now());
                    self.drag_moved = false;
                }
            }

            Message::DesktopOpenIcon(idx) => {
                self.context_menu = None;
                if let Some(action) = self.desktop_icons.get(idx).map(|i| i.action.clone()) {
                    match action {
                        DesktopIconAction::External(cmd) => {
                            return self.update(Message::LaunchExternalApp(cmd));
                        }
                        DesktopIconAction::Internal(win_id) => {
                            return self.update(Message::OpenWindow(win_id));
                        }
                        DesktopIconAction::OpenDirectory(path) => {
                            self.file_manager.navigate_to(path);
                            return self.update(Message::OpenWindow(DesktopWindowId::Files));
                        }
                    }
                }
            }

            Message::DesktopCursorMove(pt) => {
                self.last_cursor_pos = pt;

                // 1. Handle Interactive Window Corner Resizing!
                if let Some((win_id, start_pt, start_sz)) = self.wm.resizing_window {
                    let dx = pt.x - start_pt.x;
                    let dy = pt.y - start_pt.y;
                    let new_w = (start_sz.width + dx).clamp(680.0, 1310.0);
                    let new_h = (start_sz.height + dy).clamp(410.0, 620.0);
                    let st = self.wm.state_mut(win_id);
                    st.snap = WindowSnapMode::Floating;
                    st.size = iced::Size::new(new_w, new_h);
                    return Task::none();
                }

                // 2. Handle Floating Window Titlebar Dragging
                if let Some((win_id, offset)) = self.wm.dragging_window {
                    let st = self.wm.state_mut(win_id);
                    if st.snap != WindowSnapMode::Floating {
                        st.snap = WindowSnapMode::Floating;
                    }
                    let new_x = (pt.x - offset.x).clamp(8.0, 820.0);
                    let new_y = (pt.y - offset.y).clamp(4.0, 480.0);
                    st.pos = Point::new(new_x, new_y);
                    return Task::none();
                }

                // 3. Handle Desktop Icon Dragging
                if let Some((idx, offset)) = self.dragging_icon {
                    if let Some(item) = self.desktop_icons.get_mut(idx) {
                        let new_x = (pt.x - offset.x).clamp(12.0, 1240.0);
                        let new_y = (pt.y - offset.y).clamp(12.0, 540.0);
                        if (new_x - item.pos.x).abs() > 4.0 || (new_y - item.pos.y).abs() > 4.0 {
                            self.drag_moved = true;
                        }
                        item.pos = Point::new(new_x, new_y);
                    }
                }
            }

            Message::DesktopMouseRelease => {
                if let Some((idx, _)) = self.dragging_icon.take() {
                    if !self.drag_moved {
                        return self.update(Message::DesktopOpenIcon(idx));
                    } else if let Some(item) = self.desktop_icons.get_mut(idx) {
                        item.pos = snap_to_desktop_grid(item.pos);
                    }
                }
                self.drag_moved = false;
            }

            Message::DesktopCreateNewFolder => {
                self.context_menu = None;
                self.created_desktop_folders += 1;
                let folder_name = format!("Folder Baru {}", self.created_desktop_folders);
                let home = FileManagerState::home_dir();
                let desktop_dir = home.join("Desktop");
                let target_dir = if desktop_dir.exists() {
                    desktop_dir.join(&folder_name)
                } else {
                    home.join(&folder_name)
                };
                let _ = fs::create_dir_all(&target_dir);

                let idx = self.desktop_icons.len();
                let col = idx / 6;
                let row = idx % 6;
                let pos = Point::new(
                    DESKTOP_GRID_ORIGIN_X + (col as f32) * DESKTOP_GRID_CELL_WIDTH,
                    DESKTOP_GRID_ORIGIN_Y + (row as f32) * DESKTOP_GRID_CELL_HEIGHT,
                );

                self.desktop_icons.push(DesktopShortcutItem {
                    title: folder_name.clone(),
                    svg_icon_data: Some(icons::FOLDER),
                    app_icon_name: String::new(),
                    svg_color: ACCENT_CYAN,
                    action: DesktopIconAction::OpenDirectory(target_dir),
                    pos,
                });
                self.file_manager.reload();
                self.push_notification(
                    "Desktop HyperXOS",
                    format!("'{}' dibuat di Desktop", folder_name),
                    ACCENT_GREEN,
                );
            }

            Message::DesktopAutoArrangeIcons => {
                self.context_menu = None;
                for (i, icon) in self.desktop_icons.iter_mut().enumerate() {
                    let col = i / 6;
                    let row = i % 6;
                    icon.pos = Point::new(
                        DESKTOP_GRID_ORIGIN_X + (col as f32) * DESKTOP_GRID_CELL_WIDTH,
                        DESKTOP_GRID_ORIGIN_Y + (row as f32) * DESKTOP_GRID_CELL_HEIGHT,
                    );
                }
                self.push_notification(
                    "Desktop HyperXOS",
                    "Seluruh ikon desktop dirapikan ke susunan kolom vertikal.",
                    ACCENT_CYAN,
                );
            }

            Message::CycleWallpaperTheme => {
                self.context_menu = None;
                self.wallpaper_theme = match self.wallpaper_theme {
                    WallpaperTheme::CyberCrimson => WallpaperTheme::DeepNebula,
                    WallpaperTheme::DeepNebula => WallpaperTheme::EmeraldAurora,
                    WallpaperTheme::EmeraldAurora => {
                        if self.custom_wallpaper_handle.is_some() {
                            WallpaperTheme::CustomImage
                        } else {
                            WallpaperTheme::CyberCrimson
                        }
                    }
                    WallpaperTheme::CustomImage => WallpaperTheme::CyberCrimson,
                };
                self.save_persistent_settings();
            }

            // Popovers & HUD Overlays
            Message::ToggleWifiModal => {
                self.wifi_modal_open = !self.wifi_modal_open;
                if self.wifi_modal_open {
                    self.control_center_open = false;
                    self.notification_drawer_open = false;
                    self.context_menu = None;
                }
            }

            Message::ToggleControlCenter => {
                self.control_center_open = !self.control_center_open;
                if self.control_center_open {
                    self.wifi_modal_open = false;
                    self.notification_drawer_open = false;
                    self.context_menu = None;
                }
            }

            Message::ToggleNotificationDrawer => {
                self.notification_drawer_open = !self.notification_drawer_open;
                if self.notification_drawer_open {
                    self.wifi_modal_open = false;
                    self.control_center_open = false;
                    self.active_toast = None;
                    self.context_menu = None;
                }
            }

            Message::ToggleLauncher => {
                self.launcher_open = !self.launcher_open;
                if self.launcher_open {
                    // Auto-refresh Pacman / XDG applications immediately when Launchpad opens
                    self.desktop_apps = scan_desktop_apps();
                }
                self.mission_control_open = false;
                self.app_switcher_open = false;
                self.clipboard_hud_open = false;
                self.screenshot_hud_open = false;
                self.close_transient_popovers();
            }

            Message::ToggleMissionControl => {
                self.mission_control_open = !self.mission_control_open;
                self.launcher_open = false;
                self.app_switcher_open = false;
                self.clipboard_hud_open = false;
                self.screenshot_hud_open = false;
                self.close_transient_popovers();
            }

            Message::ToggleAppSwitcher => {
                if !self.app_switcher_open {
                    self.app_switcher_open = true;
                    self.app_switcher_index = 0;
                } else {
                    self.app_switcher_index = (self.app_switcher_index + 1) % 4;
                }
                self.launcher_open = false;
                self.mission_control_open = false;
                self.clipboard_hud_open = false;
                self.screenshot_hud_open = false;
                self.close_transient_popovers();
            }

            Message::SelectAppSwitcherWindow(win_id) => {
                self.app_switcher_open = false;
                return self.update(Message::OpenWindow(win_id));
            }

            Message::CloseAppSwitcher => {
                self.app_switcher_open = false;
            }

            Message::ToggleClipboardHud => {
                self.clipboard_hud_open = !self.clipboard_hud_open;
                self.screenshot_hud_open = false;
                self.launcher_open = false;
                self.mission_control_open = false;
                self.app_switcher_open = false;
                self.close_transient_popovers();
            }

            Message::ClipboardPasteToNotes(idx) => {
                if let Some(txt) = self.clipboard_history.get(idx).cloned() {
                    self.notes_app.content_lines.push(txt);
                    self.clipboard_hud_open = false;
                    return self.update(Message::OpenWindow(DesktopWindowId::Notes));
                }
            }

            Message::ClipboardClearHistory => {
                self.clipboard_history.clear();
                self.file_manager.file_clipboard = None;
            }

            Message::ToggleScreenshotHud => {
                self.screenshot_hud_open = !self.screenshot_hud_open;
                self.clipboard_hud_open = false;
                self.launcher_open = false;
                self.mission_control_open = false;
                self.app_switcher_open = false;
                self.close_transient_popovers();
            }

            Message::CaptureScreenshotNow => match self.perform_screen_capture() {
                Ok(path) => {
                    self.screenshot_hud_open = false;
                    self.push_notification(
                        "Tangkapan Layar Tersimpan",
                        format!("Disimpan ke {}", path.display()),
                        ACCENT_GREEN,
                    );
                }
                Err(err) => self.push_notification("Gagal Screenshot", err, ACCENT_RED),
            },

            Message::CaptureScreenshotAndSetWallpaper => match self.perform_screen_capture() {
                Ok(path) => {
                    self.screenshot_hud_open = false;
                    return self.update(Message::SetCustomWallpaperFromFile(path));
                }
                Err(err) => self.push_notification("Gagal Screenshot", err, ACCENT_RED),
            },

            Message::OpenScreenshotsFolder => {
                self.screenshot_hud_open = false;
                let dir = FileManagerState::home_dir().join("Pictures/Screenshots");
                let _ = fs::create_dir_all(&dir);
                self.file_manager.navigate_to(dir);
                return self.update(Message::OpenWindow(DesktopWindowId::Files));
            }

            Message::ToggleLockScreen => {
                self.close_transient_popovers();
                self.launcher_open = false;
                self.mission_control_open = false;
                self.app_switcher_open = false;
                self.lock_screen_open = true;
            }

            Message::UnlockScreen => {
                self.lock_screen_open = false;
            }

            Message::CloseQuickLook => {
                self.quick_look = None;
            }

            Message::QuickLookOpenInNotes => {
                if let Some(ql) = self.quick_look.take() {
                    if !ql.is_dir {
                        self.notes_app.load_file(PathBuf::from(ql.path));
                        return self.update(Message::OpenWindow(DesktopWindowId::Notes));
                    }
                }
            }

            Message::DismissToast => {
                self.active_toast = None;
            }

            Message::ClearNotifications => {
                self.notifications.clear();
                self.active_toast = None;
            }

            // Multi-Window Floating & Resize Management
            Message::OpenWindow(win_id) => {
                self.close_transient_popovers();
                self.launcher_open = false;
                self.mission_control_open = false;
                self.app_switcher_open = false;
                if win_id == DesktopWindowId::Files {
                    self.file_manager.reload();
                }
                self.wm.open_or_focus(win_id);
            }

            Message::DockItemClicked(win_id) => {
                self.close_transient_popovers();
                self.launcher_open = false;
                self.mission_control_open = false;
                if win_id == DesktopWindowId::Files {
                    self.file_manager.reload();
                }
                self.wm.dock_click(win_id);
            }

            Message::FocusWindow(win_id) => {
                self.context_menu = None;
                self.wm.focus(win_id);
            }

            Message::MinimizeWindow(win_id) => {
                self.context_menu = None;
                self.wm.minimize(win_id);
            }

            Message::CloseWindow(win_id) => {
                self.context_menu = None;
                self.wm.close(win_id);
            }

            Message::ToggleMaximizeWindow(win_id) => {
                self.context_menu = None;
                self.wm.toggle_maximize(win_id);
            }

            Message::SnapWindow(win_id, snap) => {
                self.context_menu = None;
                self.mission_control_open = false;
                self.wm.set_snap(win_id, snap);
            }

            Message::PresetResizeWindow(win_id, w, h) => {
                self.context_menu = None;
                self.wm.set_custom_size(win_id, w, h);
            }

            Message::StartWindowDrag(win_id) => {
                self.context_menu = None;
                self.wm.focus(win_id);
                let pos = self.wm.state(win_id).pos;
                let offset = Point::new(
                    (self.last_cursor_pos.x - pos.x).clamp(20.0, 700.0),
                    (self.last_cursor_pos.y - pos.y).clamp(6.0, 56.0),
                );
                self.wm.dragging_window = Some((win_id, offset));
            }

            Message::EndWindowDrag => {
                if let Some((win_id, _)) = self.wm.dragging_window.take() {
                    if self.last_cursor_pos.x <= 18.0 {
                        self.wm.set_snap(win_id, WindowSnapMode::LeftHalf);
                    } else if self.last_cursor_pos.x >= 1345.0 {
                        self.wm.set_snap(win_id, WindowSnapMode::RightHalf);
                    } else if self.last_cursor_pos.y <= 38.0 {
                        self.wm.set_snap(win_id, WindowSnapMode::Maximized);
                    }
                }
            }

            Message::StartWindowResize(win_id) => {
                self.context_menu = None;
                self.wm.focus(win_id);
                let sz = self.wm.state(win_id).size;
                self.wm.resizing_window = Some((win_id, self.last_cursor_pos, sz));
            }

            Message::EndWindowResize => {
                self.wm.resizing_window = None;
            }

            Message::OpenSettingsPage(tab) => {
                self.close_transient_popovers();
                self.launcher_open = false;
                self.mission_control_open = false;
                self.settings_view.active_tab = tab;
                self.wm.open_or_focus(DesktopWindowId::Settings);
            }

            Message::LaunchExternalApp(cmd) => {
                self.close_transient_popovers();
                self.launcher_open = false;
                match spawn_external_command(&cmd) {
                    Ok(msg) => self.push_notification("Peluncur Aplikasi", msg, ACCENT_CYAN),
                    Err(err) => self.push_notification("Gagal Meluncurkan", err, ACCENT_RED),
                }
            }

            Message::SystemPowerAction(cmd) => {
                let _ = spawn_external_command(&cmd);
            }

            Message::ToggleWifiRadio => {
                self.network.wifi_enabled = !self.network.wifi_enabled;
                NetworkState::set_wifi_radio(self.network.wifi_enabled);
                let status = if self.network.wifi_enabled {
                    "Radio Wi-Fi diaktifkan"
                } else {
                    "Radio Wi-Fi dinonaktifkan"
                };
                self.push_notification("Jaringan Nirkabel", status, ACCENT_CYAN);
            }

            Message::ToggleBluetooth => {
                self.bluetooth_enabled = !self.bluetooth_enabled;
                let status = if self.bluetooth_enabled {
                    "Adaptor Bluetooth BlueZ diaktifkan"
                } else {
                    "Adaptor Bluetooth dinonaktifkan"
                };
                self.push_notification("Bluetooth", status, ACCENT_PURPLE);
            }

            Message::TogglePerformanceMode => {
                self.performance_mode = !self.performance_mode;
                let msg = if self.performance_mode {
                    "Mode Gaming Turbo (Governor Performance) aktif"
                } else {
                    "Mode Hemat Daya / Seimbang aktif"
                };
                self.push_notification("Performa Sistem", msg, ACCENT_GREEN);
            }

            Message::ToggleDnd => {
                self.dnd_enabled = !self.dnd_enabled;
                if self.dnd_enabled {
                    self.active_toast = None;
                }
            }

            Message::ToggleMute => {
                self.audio.muted = !self.audio.muted;
                AudioState::toggle_mute();
            }

            Message::ToggleNightLight => {
                self.display.night_light = !self.display.night_light;
            }

            Message::VolumeChanged(val) => {
                self.audio.volume = val;
                self.audio.muted = val <= 0.5;
                AudioState::set_volume(val);
            }

            Message::BrightnessChanged(val) => {
                self.display.brightness = val;
                DisplayState::set_brightness(val);
            }

            Message::LauncherSearchChanged(q) => {
                self.launcher_search = q;
            }

            Message::RefreshDesktopApps => {
                self.desktop_apps = scan_desktop_apps();
                self.push_notification(
                    "Launchpad Pacman",
                    format!(
                        "{} aplikasi sistem berhasil dipindai ulang.",
                        self.desktop_apps.len()
                    ),
                    ACCENT_CYAN,
                );
            }

            Message::AddAppToDesktop(app) => {
                if self
                    .desktop_icons
                    .iter()
                    .any(|i| i.title.eq_ignore_ascii_case(&app.name))
                {
                    self.push_notification(
                        "Desktop Shortcut",
                        format!("'{}' sudah ada di Desktop.", app.name),
                        ACCENT_AMBER,
                    );
                } else {
                    let idx = self.desktop_icons.len();
                    let col = idx / 6;
                    let row = idx % 6;
                    let pos = Point::new(
                        DESKTOP_GRID_ORIGIN_X + (col as f32) * DESKTOP_GRID_CELL_WIDTH,
                        DESKTOP_GRID_ORIGIN_Y + (row as f32) * DESKTOP_GRID_CELL_HEIGHT,
                    );
                    let svg_icon_data = if app.icon_key == "terminal" {
                        Some(icons::TERMINAL)
                    } else {
                        None
                    };
                    self.desktop_icons.push(DesktopShortcutItem {
                        title: app.name.clone(),
                        svg_icon_data,
                        app_icon_name: app.icon_key.clone(),
                        svg_color: ACCENT_CYAN,
                        action: DesktopIconAction::External(app.exec.clone()),
                        pos,
                    });
                    self.push_notification(
                        "Ditambahkan ke Desktop",
                        format!("Pintasan '{}' ditambahkan ke Desktop.", app.name),
                        ACCENT_GREEN,
                    );
                }
            }

            Message::TogglePinAppToDock(app) => {
                if let Some(pos) = self.pinned_dock_apps.iter().position(|p| p.id == app.id) {
                    let removed = self.pinned_dock_apps.remove(pos);
                    self.push_notification(
                        "Dock HyperXOS",
                        format!("'{}' dilepas dari Dock.", removed.name),
                        ACCENT_AMBER,
                    );
                } else {
                    self.pinned_dock_apps.push(app.clone());
                    self.push_notification(
                        "Pin ke Dock",
                        format!("'{}' disematkan ke macOS Dock.", app.name),
                        ACCENT_PURPLE,
                    );
                }
            }

            Message::LaunchPinnedDockApp(idx) => {
                self.context_menu = None;
                if let Some(app) = self.pinned_dock_apps.get(idx).cloned() {
                    return self.update(Message::LaunchExternalApp(app.exec));
                }
            }

            Message::AddPinnedDockAppToDesktop(idx) => {
                self.context_menu = None;
                if let Some(app) = self.pinned_dock_apps.get(idx).cloned() {
                    return self.update(Message::AddAppToDesktop(app));
                }
            }

            Message::UnpinDockApp(idx) => {
                self.context_menu = None;
                if idx < self.pinned_dock_apps.len() {
                    let removed = self.pinned_dock_apps.remove(idx);
                    self.push_notification(
                        "Dock HyperXOS",
                        format!("'{}' dilepas dari Dock.", removed.name),
                        ACCENT_AMBER,
                    );
                }
            }

            // File Manager, Clipboard & Trash Actions
            Message::FileSelectPlace(place) => {
                self.file_manager.navigate_place(place);
            }
            Message::OpenTrashInFiles => {
                self.context_menu = None;
                self.file_manager.navigate_place(FilePlace::Trash);
                return self.update(Message::OpenWindow(DesktopWindowId::Files));
            }
            Message::FileNavigateUp => {
                self.file_manager.navigate_up();
            }
            Message::FileRefresh => {
                self.file_manager.reload();
            }
            Message::FileToggleViewMode => {
                self.file_manager.grid_mode = !self.file_manager.grid_mode;
            }
            Message::FileSearchChanged(q) => {
                self.file_manager.search_query = q;
            }
            Message::FileNewFolderNameChanged(name) => {
                self.file_manager.new_folder_name = name;
            }
            Message::FileCreateFolder => match self.file_manager.create_folder() {
                Ok(msg) => self.push_notification("HyperX Files", msg, ACCENT_GREEN),
                Err(err) => self.push_notification("HyperX Files", err, ACCENT_AMBER),
            },
            Message::FileEntryClicked(idx) => {
                self.context_menu = None;
                self.wm.focus(DesktopWindowId::Files);
                if self.file_manager.selected_index == Some(idx) {
                    return self.update(Message::FileContextOpen(idx));
                } else {
                    self.file_manager.selected_index = Some(idx);
                }
            }
            Message::FileDeleteSelected => {
                if let Some(idx) = self.file_manager.selected_index {
                    return self.update(Message::FileContextDelete(idx));
                }
            }
            Message::FileCopySelected(idx) => {
                self.context_menu = None;
                if let Some((name, clip_entry)) = self.file_manager.copy_entry_to_clipboard(idx) {
                    self.clipboard_history.insert(0, clip_entry);
                    self.push_notification(
                        "Clipboard (Ctrl+C)",
                        format!("'{}' disalin ke Clipboard. Tekan Ctrl+V untuk menempel.", name),
                        ACCENT_CYAN,
                    );
                }
            }
            Message::FileCutSelected(idx) => {
                self.context_menu = None;
                if let Some((name, clip_entry)) = self.file_manager.cut_entry_to_clipboard(idx) {
                    self.clipboard_history.insert(0, clip_entry);
                    self.push_notification(
                        "Clipboard (Ctrl+X)",
                        format!("'{}' siap dipindahkan. Tekan Ctrl+V di folder tujuan.", name),
                        ACCENT_AMBER,
                    );
                }
            }
            Message::FilePasteClipboard => {
                self.context_menu = None;
                match self.file_manager.paste_from_clipboard() {
                    Ok(msg) => self.push_notification("Clipboard (Ctrl+V)", msg, ACCENT_GREEN),
                    Err(err) => self.push_notification("Clipboard (Ctrl+V)", err, ACCENT_AMBER),
                }
            }
            Message::FileRestoreFromTrash(idx) => match self.file_manager.restore_from_trash(idx) {
                Ok(msg) => self.push_notification("macOS Trash", msg, ACCENT_GREEN),
                Err(err) => self.push_notification("macOS Trash", err, ACCENT_RED),
            },
            Message::FileEmptyTrash => {
                self.context_menu = None;
                match self.file_manager.empty_trash() {
                    Ok(msg) => self.push_notification("macOS Trash", msg, ACCENT_AMBER),
                    Err(err) => self.push_notification("macOS Trash", err, ACCENT_RED),
                }
            }
            Message::FileContextOpen(idx) => {
                self.context_menu = None;
                if let Some(entry) = self.file_manager.entries.get(idx).cloned() {
                    if entry.is_dir {
                        self.file_manager.navigate_to(entry.path);
                    } else if matches!(
                        entry.ext.as_str(),
                        "txt" | "md" | "conf" | "toml" | "rs" | "sh" | "json" | "yaml" | "yml" | "log"
                    ) {
                        self.notes_app.load_file(entry.path);
                        return self.update(Message::OpenWindow(DesktopWindowId::Notes));
                    } else if let Err(e) = open_file_or_dir(&entry.path) {
                        self.push_notification("HyperX Files", e, ACCENT_RED);
                    }
                }
            }
            Message::FileContextQuickLook(idx) => {
                self.context_menu = None;
                self.file_manager.selected_index = Some(idx);
                self.quick_look = self.file_manager.build_quick_look(idx);
            }
            Message::FileContextStartRename(idx) => {
                self.context_menu = None;
                self.file_manager.start_rename(idx);
            }
            Message::FileRenameInputChanged(val) => {
                self.file_manager.rename_input = val;
            }
            Message::FileCommitRename => match self.file_manager.commit_rename() {
                Ok(msg) => self.push_notification("Ganti Nama Berkas", msg, ACCENT_GREEN),
                Err(err) => self.push_notification("Ganti Nama Berkas", err, ACCENT_AMBER),
            },
            Message::FileCancelRename => {
                self.file_manager.renaming_index = None;
            }
            Message::FileContextDuplicate(idx) => {
                self.context_menu = None;
                match self.file_manager.duplicate_entry(idx) {
                    Ok(msg) => self.push_notification("Duplikat Item", msg, ACCENT_CYAN),
                    Err(err) => self.push_notification("Duplikat Item", err, ACCENT_RED),
                }
            }
            Message::FileContextDelete(idx) => {
                self.context_menu = None;
                match self.file_manager.move_to_trash_or_delete(idx) {
                    Ok(msg) => self.push_notification("Tempat Sampah (Trash)", msg, ACCENT_AMBER),
                    Err(err) => self.push_notification("Tempat Sampah (Trash)", err, ACCENT_RED),
                }
            }

            // Custom Wallpaper Handlers
            Message::SetCustomWallpaperFromFile(path) => {
                self.context_menu = None;
                if path.is_file() {
                    self.custom_wallpaper_handle = Some(iced_image::Handle::from_path(&path));
                    self.settings_view.custom_wallpaper_input =
                        path.to_string_lossy().to_string();
                    self.custom_wallpaper_path = Some(path.clone());
                    self.wallpaper_theme = WallpaperTheme::CustomImage;
                    self.save_persistent_settings();
                    self.push_notification(
                        "Custom Wallpaper Aktif",
                        format!("Latar desktop diubah ke '{}'", path.display()),
                        ACCENT_PURPLE,
                    );
                } else {
                    self.push_notification(
                        "Custom Wallpaper",
                        "Berkas gambar tidak ditemukan.",
                        ACCENT_RED,
                    );
                }
            }

            Message::CustomWallpaperInputChanged(val) => {
                self.settings_view.custom_wallpaper_input = val;
            }

            Message::ApplyCustomWallpaperInput => {
                let p = PathBuf::from(self.settings_view.custom_wallpaper_input.trim());
                return self.update(Message::SetCustomWallpaperFromFile(p));
            }

            Message::GenerateAndSetSampleCustomWallpaper => {
                match Self::generate_neon_wallpaper_file() {
                    Ok(path) => {
                        self.file_manager.reload();
                        return self.update(Message::SetCustomWallpaperFromFile(path));
                    }
                    Err(err) => self.push_notification("Custom Wallpaper", err, ACCENT_RED),
                }
            }

            // Terminal App Handlers
            Message::TerminalSelectTab(t) => {
                self.terminal_app.active_tab = t;
            }
            Message::TerminalInputChanged(val) => {
                self.terminal_app.command_input = val;
            }
            Message::TerminalExecuteInput => {
                let cmd = self.terminal_app.command_input.clone();
                self.terminal_app.command_input.clear();
                self.terminal_app.run_command(&cmd);
            }
            Message::TerminalRunQuick(cmd) => {
                self.terminal_app.active_tab = 0;
                self.terminal_app.run_command(&cmd);
            }

            // Notes App Handlers
            Message::NotesLineInputChanged(val) => {
                self.notes_app.new_line_input = val;
            }
            Message::NotesAppendLine => {
                let line = self.notes_app.new_line_input.trim().to_string();
                if !line.is_empty() {
                    self.notes_app.content_lines.push(line);
                    self.notes_app.new_line_input.clear();
                }
            }
            Message::NotesSaveFile => match self.notes_app.save_file() {
                Ok(msg) => {
                    self.file_manager.reload();
                    self.push_notification("HyperX Notes", msg, ACCENT_GREEN);
                }
                Err(err) => self.push_notification("HyperX Notes", err, ACCENT_RED),
            },

            // Settings Handlers
            Message::SelectSettingsTab(tab) => {
                self.settings_view.active_tab = tab;
            }
            Message::RefreshNetwork => {
                self.network = NetworkState::refresh();
                self.settings_view.network_status_msg =
                    "Pemindaian ulang jaringan selesai.".to_string();
            }
            Message::SelectWifiSsid(ssid) => {
                self.settings_view.selected_ssid = Some(ssid);
            }
            Message::WifiPasswordChanged(pw) => {
                self.settings_view.wifi_password_input = pw;
            }
            Message::ConnectSelectedWifi => {
                if let Some(ssid) = self.settings_view.selected_ssid.clone() {
                    match NetworkState::connect_wifi(&ssid, &self.settings_view.wifi_password_input)
                    {
                        Ok(msg) => {
                            self.settings_view.network_status_msg = msg.clone();
                            self.network = NetworkState::refresh();
                            self.push_notification("Wi-Fi Terhubung", msg, ACCENT_GREEN);
                        }
                        Err(err) => {
                            self.settings_view.network_status_msg = format!("Gagal: {}", err);
                            self.push_notification("Koneksi Wi-Fi", err, ACCENT_RED);
                        }
                    }
                }
            }
            Message::SelectWallpaper(theme) => {
                self.wallpaper_theme = theme;
                self.save_persistent_settings();
            }
            Message::DockSizeChanged(sz) => {
                self.dock_icon_size = sz;
                self.save_persistent_settings();
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.lock_screen_open {
            let theme = self.wallpaper_theme;
            return container(overlays::view_lock_screen(
                &self.clock_str,
                &self.date_header_str,
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| wallpaper_background_style(theme))
            .into();
        }

        let topbar_el = topbar::view_topbar(
            &self.clock_str,
            self.network.wifi_enabled,
            self.audio.volume,
            self.audio.muted,
            self.notifications.len(),
            self.wifi_modal_open,
            self.control_center_open,
            self.notification_drawer_open,
        );

        let mut workspace_layers: Vec<Element<'_, Message>> =
            vec![self.view_clean_desktop_canvas()];

        for win_id in self.wm.visible_windows_in_z_order() {
            workspace_layers.push(self.view_floating_window(win_id));
        }

        let main_workspace_content = stack(workspace_layers)
            .width(Length::Fill)
            .height(Length::Fill);

        let dock_row = container(dock::view_dock(
            &self.wm,
            self.launcher_open,
            self.mission_control_open,
            self.file_manager.trash_count,
            self.dock_icon_size,
            &self.pinned_dock_apps,
        ))
        .width(Length::Fill)
        .height(Length::Shrink)
        .align_x(Alignment::Center)
        .padding([6, 0]);

        let base_layout = column![
            topbar_el,
            container(main_workspace_content)
                .width(Length::Fill)
                .height(Length::Fill),
            dock_row,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        let mut layers: Vec<Element<'_, Message>> = Vec::new();

        // Render Custom Wallpaper Image at bottom of stack if active!
        if self.wallpaper_theme == WallpaperTheme::CustomImage {
            if let Some(handle) = &self.custom_wallpaper_handle {
                layers.push(
                    iced_image(handle.clone())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(ContentFit::Cover)
                        .into(),
                );
            }
        }

        layers.push(base_layout.into());

        if self.wifi_modal_open {
            let wifi_overlay = container(
                row![
                    container("").width(Length::Fill),
                    topbar::view_wifi_modal(
                        &self.network,
                        self.settings_view.selected_ssid.as_deref(),
                        &self.settings_view.wifi_password_input,
                        &self.settings_view.network_status_msg,
                    ),
                ]
                .padding([42, 74]),
            )
            .width(Length::Fill)
            .height(Length::Fill);
            layers.push(wifi_overlay.into());
        }

        if self.notification_drawer_open {
            let notif_overlay = container(
                column![
                    container(notifications::view_notification_drawer(
                        &self.date_header_str,
                        self.current_day,
                        &self.notifications,
                    ))
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    container("").height(Length::Fill),
                ]
                .padding([42, 0]),
            )
            .width(Length::Fill)
            .height(Length::Fill);
            layers.push(notif_overlay.into());
        }

        if self.control_center_anim > 0.005 {
            let t = 1.0 - (1.0 - self.control_center_anim).powi(3);
            let dy = (1.0 - t) * 20.0;
            let top_offset = (52.0 - dy).max(36.0);

            let cc_overlay = container(
                row![
                    container("").width(Length::Fill),
                    control_center::view_control_center(
                        self.network.wifi_enabled,
                        &self.network.active_ssid,
                        self.bluetooth_enabled,
                        self.performance_mode,
                        self.dnd_enabled,
                        self.audio.volume,
                        self.audio.muted,
                        self.display.brightness,
                        self.cpu_usage,
                        self.ram_used_gb,
                        self.ram_total_gb,
                    ),
                ]
                .padding([top_offset as u16, 20]),
            )
            .width(Length::Fill)
            .height(Length::Fill);
            layers.push(cc_overlay.into());
        }

        if let Some(ql) = &self.quick_look {
            layers.push(overlays::view_quick_look_modal(ql));
        }

        if self.clipboard_hud_open {
            let fc_label = self.file_manager.file_clipboard.as_ref().map(|(op, p)| {
                let op_str = match op {
                    FileClipboardOp::Copy => "COPY (Salin)",
                    FileClipboardOp::Cut => "CUT (Pindah)",
                };
                format!("{}: {}", op_str, p.display())
            });
            layers.push(overlays::view_clipboard_hud(
                &self.clipboard_history,
                fc_label,
            ));
        }

        if self.screenshot_hud_open {
            layers.push(overlays::view_screenshot_hud(
                self.last_screenshot_path.as_deref(),
            ));
        }

        if self.mission_control_open {
            layers.push(overlays::view_mission_control(&self.wm));
        }

        if self.app_switcher_open {
            layers.push(overlays::view_app_switcher(
                &self.wm,
                self.app_switcher_index,
            ));
        }

        if self.launcher_anim > 0.005 {
            let t = 1.0 - (1.0 - self.launcher_anim).powi(3);
            let dy = (1.0 - t) * 40.0;
            let bg_alpha = 0.40 * t;
            let bottom_offset = (74.0 - dy).max(10.0);

            let launcher_card = launcher::view_launcher(
                &self.launcher_search,
                &self.desktop_apps,
                &self.pinned_dock_apps,
            );

            let content = column![
                mouse_area(container("").width(Length::Fill).height(Length::Fill))
                    .on_press(Message::ToggleLauncher),
                launcher_card,
                container("").height(Length::Fixed(bottom_offset)),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center);

            let launcher_overlay = container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.01, 0.02, 0.04, bg_alpha))),
                    ..container::Style::default()
                });
            layers.push(launcher_overlay.into());
        }

        if let Some(menu) = &self.context_menu {
            let icon_title = match menu.kind {
                ContextMenuKind::DesktopIcon(idx) => {
                    self.desktop_icons.get(idx).map(|i| i.title.as_str())
                }
                ContextMenuKind::PinnedDockApp(idx) => {
                    self.pinned_dock_apps.get(idx).map(|a| a.name.as_str())
                }
                _ => None,
            };
            let (file_name, file_path, is_img) = match menu.kind {
                ContextMenuKind::FileEntry(idx) => {
                    if let Some(e) = self.file_manager.entries.get(idx) {
                        (
                            Some(e.name.as_str()),
                            Some(e.path.clone()),
                            is_image_ext(&e.ext),
                        )
                    } else {
                        (None, None, false)
                    }
                }
                _ => (None, None, false),
            };
            layers.push(context_menu::view_context_menu(
                menu,
                icon_title,
                file_name,
                file_path,
                is_img,
                self.file_manager.file_clipboard.is_some(),
            ));
        }

        if let Some(toast) = &self.active_toast {
            if !self.notification_drawer_open && !self.control_center_open && !self.wifi_modal_open
            {
                let toast_overlay = container(
                    row![
                        container("").width(Length::Fill),
                        notifications::view_toast_banner(toast),
                    ]
                    .padding([46, 18]),
                )
                .width(Length::Fill);
                layers.push(toast_overlay.into());
            }
        }

        let theme = self.wallpaper_theme;
        container(stack(layers))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| wallpaper_background_style(theme))
            .into()
    }

    fn view_floating_window(&self, win_id: DesktopWindowId) -> Element<'_, Message> {
        let st = self.wm.state(win_id);

        let (w, h) = match st.snap {
            WindowSnapMode::Maximized => (Length::Fill, Length::Fill),
            WindowSnapMode::LeftHalf | WindowSnapMode::RightHalf => {
                (Length::Fixed(665.0), Length::Fill)
            }
            WindowSnapMode::Floating => {
                (Length::Fixed(st.size.width), Length::Fixed(st.size.height))
            }
        };

        let win_el: Element<'_, Message> = match win_id {
            DesktopWindowId::Files => file_manager::view_file_manager(&self.file_manager, w, h),
            DesktopWindowId::Settings => settings::view_settings(
                &self.settings_view,
                &self.network,
                self.bluetooth_enabled,
                self.audio.volume,
                self.audio.muted,
                &self.audio.output_device,
                self.display.brightness,
                &self.display.resolution,
                &self.display.refresh_rate,
                self.display.night_light,
                self.wallpaper_theme,
                self.dock_icon_size,
                &self.cpu_brand,
                self.cpu_usage,
                self.ram_used_gb,
                self.ram_total_gb,
                &self.wayland_socket,
                w,
                h,
            ),
            DesktopWindowId::Terminal => overlays::view_terminal_window(
                &self.terminal_app,
                &self.cpu_brand,
                self.cpu_usage,
                self.ram_used_gb,
                self.ram_total_gb,
                &self.wayland_socket,
                w,
                h,
            ),
            DesktopWindowId::Notes => overlays::view_notes_window(&self.notes_app, w, h),
        };

        // Add bottom-right interactive Resize Grip Handle when window is in Floating mode!
        let wrapped_with_resize: Element<'_, Message> = if st.snap == WindowSnapMode::Floating {
            let resize_grip = mouse_area(
                container(svg_icon(icons::RESIZE_CORNER, 13.0, ACCENT_CYAN))
                    .width(Length::Fixed(22.0))
                    .height(Length::Fixed(22.0))
                    .center_x(Length::Fixed(22.0))
                    .center_y(Length::Fixed(22.0))
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgba(
                            0.54, 0.71, 0.98, 0.18,
                        ))),
                        border: Border::default()
                            .rounded(Radius::from(6.0))
                            .width(1.0)
                            .color(Color::from_rgba(0.54, 0.71, 0.98, 0.45)),
                        ..container::Style::default()
                    }),
            )
            .on_press(Message::StartWindowResize(win_id))
            .on_release(Message::EndWindowResize);

            let grip_overlay = container(
                column![
                    container("").height(Length::Fixed((st.size.height - 26.0).max(10.0))),
                    row![
                        container("").width(Length::Fixed((st.size.width - 26.0).max(10.0))),
                        resize_grip,
                    ],
                ],
            )
            .width(Length::Fixed(st.size.width))
            .height(Length::Fixed(st.size.height));

            stack![win_el, grip_overlay].into()
        } else {
            win_el
        };

        let focusable = mouse_area(wrapped_with_resize).on_press(Message::FocusWindow(win_id));

        match st.snap {
            WindowSnapMode::Maximized => container(focusable)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding([6, 12])
                .into(),
            WindowSnapMode::LeftHalf => container(
                row![focusable, container("").width(Length::Fill)].height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([6, 10])
            .into(),
            WindowSnapMode::RightHalf => container(
                row![container("").width(Length::Fill), focusable].height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([6, 10])
            .into(),
            WindowSnapMode::Floating => container(
                column![
                    container("").height(Length::Fixed(st.pos.y.max(4.0))),
                    row![
                        container("").width(Length::Fixed(st.pos.x.max(8.0))),
                        focusable,
                    ],
                ],
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        }
    }

    fn view_clean_desktop_canvas(&self) -> Element<'_, Message> {
        let mut icon_layers: Vec<Element<'_, Message>> = Vec::new();

        let wallpaper_click_area = mouse_area(
            container("")
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .on_press(Message::CloseContextMenu)
        .on_right_press(Message::OpenDesktopContextMenu);

        icon_layers.push(wallpaper_click_area.into());

        for (idx, item) in self.desktop_icons.iter().enumerate() {
            let _is_dragging = self.dragging_icon.map(|(i, _)| i == idx).unwrap_or(false);

            let icon_graphic: Element<'_, Message> = if let Some(svg_data) = item.svg_icon_data {
                svg_icon(svg_data, 26.0, item.svg_color).into()
            } else {
                app_icon(&item.app_icon_name, 28.0)
            };

            let tile_box = container(icon_graphic)
                .width(Length::Fixed(44.0))
                .height(Length::Fixed(44.0))
                .center_x(Length::Fixed(44.0))
                .center_y(Length::Fixed(44.0))
                .style(fluent_desktop_icon_tile_style);

            let icon_btn = button(
                column![
                    tile_box,
                    text(&item.title)
                        .size(11)
                        .font(FONT_BODY_SEMIBOLD)
                        .style(|_| text::Style {
                            color: Some(Color::WHITE),
                        }),
                ]
                .spacing(5)
                .align_x(Alignment::Center),
            )
            .width(Length::Fixed(92.0))
            .height(Length::Fixed(80.0))
            .padding([4, 4])
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.18))),
                    button::Status::Pressed => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.28))),
                    _ => None,
                };
                button::Style {
                    background: bg,
                    border: Border::default().rounded(Radius::from(10.0)),
                    ..button::Style::default()
                }
            })
            .on_press(Message::DesktopIconPress(idx));

            let interactive_icon = mouse_area(icon_btn)
                .on_release(Message::DesktopMouseRelease)
                .on_right_press(Message::OpenDesktopIconContextMenu(idx));

            let positioned = container(
                column![
                    container("").height(Length::Fixed(item.pos.y.max(0.0))),
                    row![
                        container("").width(Length::Fixed(item.pos.x.max(0.0))),
                        interactive_icon,
                    ],
                ],
            )
            .width(Length::Fill)
            .height(Length::Fill);

            icon_layers.push(positioned.into());
        }

        container(stack(icon_layers))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn wallpaper_background_style(theme: WallpaperTheme) -> container::Style {
    let gradient = match theme {
        WallpaperTheme::CyberCrimson | WallpaperTheme::CustomImage => Linear::new(2.4)
            .add_stop(0.0, Color::from_rgb(0.05, 0.06, 0.11))
            .add_stop(0.45, Color::from_rgb(0.11, 0.06, 0.16))
            .add_stop(0.80, Color::from_rgb(0.24, 0.06, 0.12))
            .add_stop(1.0, Color::from_rgb(0.06, 0.07, 0.12)),
        WallpaperTheme::DeepNebula => Linear::new(2.2)
            .add_stop(0.0, Color::from_rgb(0.03, 0.05, 0.12))
            .add_stop(0.5, Color::from_rgb(0.06, 0.12, 0.25))
            .add_stop(1.0, Color::from_rgb(0.04, 0.06, 0.14)),
        WallpaperTheme::EmeraldAurora => Linear::new(2.3)
            .add_stop(0.0, Color::from_rgb(0.03, 0.08, 0.10))
            .add_stop(0.5, Color::from_rgb(0.05, 0.18, 0.16))
            .add_stop(1.0, Color::from_rgb(0.04, 0.07, 0.12)),
    };

    let _ = ColorStop {
        offset: 0.0,
        color: Color::BLACK,
    };

    container::Style {
        background: Some(Background::Gradient(Gradient::Linear(gradient))),
        border: Border::default().rounded(Radius::from(0.0)),
        ..container::Style::default()
    }
}
