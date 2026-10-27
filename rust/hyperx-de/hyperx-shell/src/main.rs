mod app;
mod app_menu;
mod control_center;
mod dock;
mod dynamic_island;
mod notifications;
mod theme;
mod top_bar;

use app::ShellApp;

fn main() -> iced::Result {
    iced::application(ShellApp::default, ShellApp::update, ShellApp::view)
        .title("HyperX Desktop Shell")
        .theme(ShellApp::theme)
        .subscription(ShellApp::subscription)
        .run()
}
