mod app;
mod entry;
mod miller;
mod quick_look;
mod sidebar;

use app::FileManagerApp;

fn main() -> iced::Result {
    iced::application(
        FileManagerApp::default,
        FileManagerApp::update,
        FileManagerApp::view,
    )
    .title("HyperX File Manager")
    .theme(|_: &FileManagerApp| iced::Theme::Dark)
    .run()
}
