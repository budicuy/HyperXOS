mod about_view;
mod appearance_view;
mod app;
mod bluetooth_view;
mod displays_view;
mod keyboard_view;
mod network;
mod network_view;
mod power_view;
mod sound_view;
mod theme;

use app::{Page, SettingsApp};
use clap::Parser;
use iced::window::Settings as WindowSettings;
use iced::Size;

#[derive(Parser, Debug)]
#[command(
    name = "hyperx-settings",
    about = "macOS-style System Settings & Network Manager for HyperXOS",
    version = "1.0.0"
)]
struct Args {
    /// Jump directly to a settings page (network, bluetooth, displays, appearance, sound, power, keyboard, about)
    #[arg(short, long)]
    page: Option<String>,
}

fn main() -> iced::Result {
    let args = Args::parse();
    let mut initial_app = SettingsApp::default();

    if let Some(ref p) = args.page {
        match p.to_lowercase().as_str() {
            "network" | "wifi" | "net" => initial_app.current_page = Page::Network,
            "bluetooth" | "bt" => initial_app.current_page = Page::Bluetooth,
            "displays" | "display" | "screen" => initial_app.current_page = Page::Displays,
            "appearance" | "theme" => initial_app.current_page = Page::Appearance,
            "sound" | "audio" | "volume" => initial_app.current_page = Page::Sound,
            "power" | "battery" => initial_app.current_page = Page::Power,
            "keyboard" | "shortcuts" => initial_app.current_page = Page::Keyboard,
            "about" => initial_app.current_page = Page::About,
            _ => {}
        }
    }

    iced::application(
        move || (initial_app.clone(), iced::Task::none()),
        SettingsApp::update,
        SettingsApp::view,
    )
    .title(SettingsApp::title)
    .theme(SettingsApp::theme)
    .window(WindowSettings {
        size: Size::new(960.0, 640.0),
        min_size: Some(Size::new(760.0, 500.0)),
        resizable: true,
        decorations: true,
        ..Default::default()
    })
    .run()
}
