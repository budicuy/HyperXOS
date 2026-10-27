mod app;
mod calc;
mod scanner;

use app::LauncherApp;

fn main() -> iced::Result {
    iced::application(
        LauncherApp::default,
        LauncherApp::update,
        LauncherApp::view,
    )
    .title("Spotlight Search")
    .theme(LauncherApp::theme)
    .subscription(LauncherApp::subscription)
    .run()
}
