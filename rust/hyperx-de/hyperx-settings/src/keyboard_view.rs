use iced::alignment::Alignment;
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Element, Length};

use crate::theme::{self, ACCENT_BLUE, TEXT_PRIMARY, TEXT_SECONDARY};

pub fn view<'a, Message: 'static + Clone>() -> Element<'a, Message> {
    let title = text("Keyboard & Shortcuts").size(22).color(TEXT_PRIMARY);

    let shortcut_row = |label: &'static str, key_combo: &'static str, desc: &'static str| {
        let badge = container(text(key_combo).size(11).color(ACCENT_BLUE))
            .padding([3, 8])
            .style(|_| container::Style {
                background: Some(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.08).into()),
                border: iced::Border {
                    color: iced::Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            });

        container(
            row![
                column![
                    text(label).size(13).color(TEXT_PRIMARY),
                    text(desc).size(10).color(TEXT_SECONDARY),
                ]
                .spacing(2),
                Space::new().width(Length::Fill),
                badge,
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .padding([8, 12])
        .style(theme::card_style)
    };

    let shortcuts_col = column![
        shortcut_row("Spotlight Search", "⌘ Space", "Search apps, run calculations, and system commands"),
        shortcut_row("HyperX File Manager", "Super + E", "Open Finder-style Miller Columns File Browser"),
        shortcut_row("Terminal Console", "Super + T", "Open hardware-accelerated terminal"),
        shortcut_row("Control Center", "Super + C", "Open macOS-style Quick Toggles & Sliders"),
        shortcut_row("Lock Desktop Screen", "Super + L", "Lock active user session"),
        shortcut_row("Quick Look Preview", "Spacebar", "Preview highlighted file in File Manager instantly"),
    ]
    .spacing(8);

    let content = column![title, shortcuts_col].spacing(14);

    scrollable(content).height(Length::Fill).into()
}
