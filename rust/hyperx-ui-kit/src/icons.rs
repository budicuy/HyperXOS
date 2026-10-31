use std::path::PathBuf;
use std::sync::LazyLock;
use iced::widget::image;
use iced::widget::svg::{Handle, Svg};
use iced::{Color, Element, Length};

pub const LOGO_PNG_BYTES: &[u8] = include_bytes!("../assets/logo.png").as_slice();

static LOGO_HANDLE: LazyLock<image::Handle> = LazyLock::new(|| {
    image::Handle::from_bytes(LOGO_PNG_BYTES)
});

pub fn logo_image(size: f32) -> image::Image<image::Handle> {
    image(LOGO_HANDLE.clone())
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
}

// Official Application Icons (64x64 RGBA)
static ICON_BRAVE: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/brave-bin.png").as_slice()));
static ICON_BTOP: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/btop.png").as_slice()));
static ICON_CHROMIUM: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/chromium.png").as_slice()));
static ICON_CODE: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/code.png").as_slice()));
static ICON_DISCORD: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/discord.png").as_slice()));
static ICON_DOCKER: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/docker.png").as_slice()));
static ICON_FASTFETCH: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/fastfetch.png").as_slice()));
static ICON_FIREFOX: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/firefox.png").as_slice()));
static ICON_GIMP: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/gimp.png").as_slice()));
static ICON_GIT: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/git.png").as_slice()));
static ICON_HEROIC: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/heroic-games-launcher-bin.png").as_slice()));
static ICON_HTOP: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/htop.png").as_slice()));
static ICON_LUTRIS: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/lutris.png").as_slice()));
static ICON_MPV: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/mpv.png").as_slice()));
static ICON_NEOVIM: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/neovim.png").as_slice()));
static ICON_OBS: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/obs-studio.png").as_slice()));
static ICON_STEAM: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/steam.png").as_slice()));
static ICON_TIMESHIFT: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/timeshift.png").as_slice()));
static ICON_VLC: LazyLock<image::Handle> = LazyLock::new(|| image::Handle::from_bytes(include_bytes!("../assets/apps/vlc.png").as_slice()));

fn app_icon_handle(package_name: &str) -> Option<image::Handle> {
    let lower = package_name.trim().to_lowercase();
    let stem = std::path::Path::new(&lower)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(&lower);

    match stem {
        "hyper-x-installer" | "hyper-x-tools" | "hyperx" => Some(LOGO_HANDLE.clone()),
        "brave-bin" | "brave" | "brave-browser" => Some(ICON_BRAVE.clone()),
        "btop" => Some(ICON_BTOP.clone()),
        "chromium" => Some(ICON_CHROMIUM.clone()),
        "code" | "code-oss" | "visual-studio-code" | "vscodium" => Some(ICON_CODE.clone()),
        "discord" | "vesktop" => Some(ICON_DISCORD.clone()),
        "docker" => Some(ICON_DOCKER.clone()),
        "fastfetch" => Some(ICON_FASTFETCH.clone()),
        "firefox" => Some(ICON_FIREFOX.clone()),
        "gimp" => Some(ICON_GIMP.clone()),
        "git" => Some(ICON_GIT.clone()),
        "heroic-games-launcher-bin" | "heroic" => Some(ICON_HEROIC.clone()),
        "htop" => Some(ICON_HTOP.clone()),
        "lutris" => Some(ICON_LUTRIS.clone()),
        "mpv" => Some(ICON_MPV.clone()),
        "neovim" | "nvim" => Some(ICON_NEOVIM.clone()),
        "obs-studio" | "obs" | "com.obsproject.studio" => Some(ICON_OBS.clone()),
        "steam" => Some(ICON_STEAM.clone()),
        "timeshift" => Some(ICON_TIMESHIFT.clone()),
        "vlc" => Some(ICON_VLC.clone()),
        _ => resolve_system_png_handle(package_name.trim()),
    }
}

fn resolve_system_png_handle(icon_name: &str) -> Option<image::Handle> {
    if icon_name.is_empty() {
        return None;
    }
    let direct = PathBuf::from(icon_name);
    if direct.is_file() {
        let ext = direct
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp" | "ico") {
            return Some(image::Handle::from_path(direct));
        }
    }

    let search_dirs = [
        "/usr/share/pixmaps",
        "/usr/share/icons/hicolor/128x128/apps",
        "/usr/share/icons/hicolor/64x64/apps",
        "/usr/share/icons/hicolor/48x48/apps",
        "/usr/share/icons/hicolor/256x256/apps",
    ];
    for dir in search_dirs {
        let candidate = PathBuf::from(dir).join(format!("{}.png", icon_name));
        if candidate.is_file() {
            return Some(image::Handle::from_path(candidate));
        }
    }
    None
}

fn resolve_system_svg_handle(icon_name: &str) -> Option<Handle> {
    if icon_name.is_empty() {
        return None;
    }
    let direct = PathBuf::from(icon_name);
    if direct.is_file()
        && direct
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("svg"))
            .unwrap_or(false)
    {
        return Some(Handle::from_path(direct));
    }

    let svg_dirs = [
        "/usr/share/icons/hicolor/scalable/apps",
        "/usr/share/pixmaps",
    ];
    for dir in svg_dirs {
        let candidate = PathBuf::from(dir).join(format!("{}.svg", icon_name));
        if candidate.is_file() {
            return Some(Handle::from_path(candidate));
        }
    }
    None
}

pub fn app_icon<'a, Message: 'a>(package_name: &str, size: f32) -> Element<'a, Message> {
    if let Some(handle) = app_icon_handle(package_name) {
        image(handle)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .into()
    } else if let Some(svg_h) = resolve_system_svg_handle(package_name.trim()) {
        iced::widget::svg(svg_h)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .into()
    } else {
        svg_icon(STORE, size, Color::from_rgb(0.54, 0.71, 0.98)).into()
    }
}

// --- Active Lucide Vector SVG Icons ---
pub const DASHBOARD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>"#;
pub const KERNEL: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="16" height="16" x="4" y="4" rx="2"/><rect width="6" height="6" x="9" y="9" rx="1"/><path d="M15 2v2"/><path d="M15 20v2"/><path d="M2 15h2"/><path d="M2 9h2"/><path d="M20 15h2"/><path d="M20 9h2"/><path d="M9 2v2"/><path d="M9 20v2"/></svg>"#;
pub const GAMING: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="6" x2="10" y1="12" y2="12"/><line x1="8" x2="8" y1="10" y2="14"/><line x1="15" x2="15.01" y1="13" y2="13"/><line x1="18" x2="18.01" y1="11" y2="11"/><rect width="20" height="12" x="2" y="6" rx="6"/></svg>"#;
pub const SNAPSHOT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/><circle cx="12" cy="13" r="3"/></svg>"#;
pub const MAINTENANCE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>"#;
pub const STORE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16.5 9.4 7.55 4.24"/><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.29 7 12 12 20.71 7"/><line x1="12" x2="12" y1="22" y2="12"/></svg>"#;
pub const RAM: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>"#;
pub const DISK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="22" x2="2" y1="12" y2="12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/><line x1="6" x2="6.01" y1="16" y2="16"/><line x1="10" x2="10.01" y1="16" y2="16"/></svg>"#;
pub const GPU: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="14" x="2" y="3" rx="2"/><line x1="8" x2="16" y1="21" y2="21"/><line x1="12" x2="12" y1="17" y2="21"/></svg>"#;
pub const REFRESH: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M3 21v-5h5"/></svg>"#;
pub const TERMINAL: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 17 10 11 4 5"/><line x1="12" x2="20" y1="19" y2="19"/></svg>"#;
pub const DOWNLOAD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>"#;
pub const TRASH: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>"#;
pub const CHECK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>"#;
pub const CHECK_CIRCLE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#;
pub const ALERT_TRIANGLE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" x2="12" y1="9" y2="13"/><line x1="12" x2="12.01" y1="17" y2="17"/></svg>"#;
pub const ZAP: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>"#;
pub const SPARKLES: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"/></svg>"#;
pub const SHIELD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/></svg>"#;
pub const SHIELD_CHECK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10"/><path d="m9 12 2 2 4-4"/></svg>"#;
pub const SLIDERS: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="4" x2="4" y1="21" y2="14"/><line x1="4" x2="4" y1="10" y2="3"/><line x1="12" x2="12" y1="21" y2="12"/><line x1="12" x2="12" y1="8" y2="3"/><line x1="20" x2="20" y1="21" y2="16"/><line x1="20" x2="20" y1="12" y2="3"/><line x1="1" x2="7" y1="14" y2="14"/><line x1="9" x2="15" y1="8" y2="8"/><line x1="17" x2="23" y1="16" y2="16"/></svg>"#;
pub const GITHUB: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/><path d="M9 18c-4.51 2-5-2-7-2"/></svg>"#;
pub const TAG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z"/><circle cx="7.5" cy="7.5" r=".5" fill="currentColor"/></svg>"#;
pub const WIFI: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12.55a11 11 0 0 1 14.08 0"/><path d="M1.42 9a16 16 0 0 1 21.16 0"/><path d="M8.53 16.11a6 6 0 0 1 6.95 0"/><line x1="12" x2="12.01" y1="20" y2="20"/></svg>"#;
pub const POWER: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18.36 6.64a9 9 0 1 1-12.73 0"/><line x1="12" x2="12" y1="2" y2="12"/></svg>"#;
pub const ROCKET: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"/><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/></svg>"#;
pub const GLOBE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" x2="22" y1="12" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>"#;
pub const USER: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;
pub const EYE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0"/><circle cx="12" cy="12" r="3"/></svg>"#;
pub const EYE_OFF: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49"/><path d="M14.084 14.158a3 3 0 0 1-4.242-4.242"/><path d="M17.479 17.499A10.75 10.75 0 0 1 2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 2.827-4.14"/><line x1="2" x2="22" y1="2" y2="22"/></svg>"#;
pub const KEYBOARD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 8h.001"/><path d="M10 8h.001"/><path d="M14 8h.001"/><path d="M18 8h.001"/><path d="M8 12h.001"/><path d="M12 12h.001"/><path d="M16 12h.001"/><path d="M7 16h10"/></svg>"#;

pub fn svg_icon<'a>(data: &'static str, size: f32, color: Color) -> Svg<'a> {
    iced::widget::svg(Handle::from_memory(data.as_bytes()))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_theme, _status| iced::widget::svg::Style {
            color: Some(color),
        })
}
