use iced::widget::{button, checkbox, column, container, row, text};
use iced::{Alignment, Color, Element, Length};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    restart_checked: bool,
    on_toggle_restart: impl Fn(bool) -> Message + 'a,
    on_finish: Message,
) -> Element<'a, Message> {
    let restart_row = row![
        checkbox(restart_checked)
            .on_toggle(on_toggle_restart)
            .size(18),
        text("Restart computer now into your new HyperXOS system")
            .size(13)
            .font(FONT_BODY_SEMIBOLD)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN),
            }),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let success_card = container(
        column![
            svg_icon(icons::CHECK_CIRCLE, 56.0, ACCENT_GREEN),
            text("Installation Complete!")
                .size(24)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            text("HyperXOS Linux has been successfully installed to your computer.")
                .size(13)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_SUB),
                }),
            column![
                text("• Limine Bootloader is registered and ready in UEFI NVRAM").size(12).font(FONT_BODY).style(|_| text::Style { color: Some(TEXT_MUTED) }),
                text("• Root filesystem is configured on Btrfs with @ and @home subvolumes").size(12).font(FONT_BODY).style(|_| text::Style { color: Some(TEXT_MUTED) }),
                text("• KDE Plasma 6 Wayland environment and SDDM display manager are enabled").size(12).font(FONT_BODY).style(|_| text::Style { color: Some(TEXT_MUTED) }),
            ]
            .spacing(4)
            .padding([10, 0]),
            restart_row,
        ]
        .spacing(14)
        .align_x(Alignment::Center),
    )
    .padding(32)
    .width(Length::Fill)
    .style(glass_card_accent_style(ACCENT_GREEN));

    let finish_btn = button(
        row![
            svg_icon(icons::CHECK_CIRCLE, 14.0, Color::WHITE),
            text("Done")
                .size(13)
                .font(FONT_BODY_BOLD),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .on_press(on_finish)
    .style(primary_button_style(ACCENT_GREEN))
    .padding([10, 28]);

    column![
        column![
            text("All Done")
                .size(20)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN)
                }),
            text("You may now restart into your new system or continue testing the live environment")
                .size(12)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED)
                }),
        ]
        .spacing(4),
        success_card,
        column![].height(Length::Fill),
        row![
            row![].width(Length::Fill),
            finish_btn,
        ]
        .align_y(Alignment::Center),
    ]
    .spacing(16)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
