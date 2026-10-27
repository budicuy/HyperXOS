mod app;
mod app_menu;
mod control_center;
mod dock;
mod dynamic_island;
mod network_menu;
mod notifications;
mod theme;
mod top_bar;

use app::ShellApp;

fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        transparent: true,
        decorations: false,
        maximized: true,
        ..Default::default()
    };

    iced::application(ShellApp::default, ShellApp::update, ShellApp::view)
        .title("HyperX Desktop Shell")
        .theme(ShellApp::theme)
        .subscription(ShellApp::subscription)
        .window(window_settings)
        .style(|_: &ShellApp, _: &iced::Theme| iced::theme::Style {
            background_color: iced::Color::TRANSPARENT,
            text_color: iced::Color::WHITE,
        })
        .run()
}

