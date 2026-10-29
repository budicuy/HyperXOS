use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::{Alignment, Color, Element, Length};
use crate::core::configurator::UserConfig;
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    cfg: &UserConfig,
    confirm_password: &str,
    use_same_root_password: bool,
    on_change_fullname: impl Fn(String) -> Message + 'a,
    on_change_username: impl Fn(String) -> Message + 'a,
    on_change_hostname: impl Fn(String) -> Message + 'a,
    on_change_password: impl Fn(String) -> Message + 'a,
    on_change_confirm: impl Fn(String) -> Message + 'a,
    on_toggle_same_root: impl Fn(bool) -> Message + 'a,
    on_change_root_password: impl Fn(String) -> Message + 'a,
    on_back: Message,
    on_next: Message,
) -> Element<'a, Message> {
    let is_valid_username = !cfg.username.trim().is_empty()
        && cfg.username.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
    let is_valid_password = !cfg.password.is_empty() && cfg.password == confirm_password;
    let is_valid_root = use_same_root_password || !cfg.root_password.is_empty();
    let is_valid_hostname = !cfg.hostname.trim().is_empty();

    let can_proceed = is_valid_username && is_valid_password && is_valid_root && is_valid_hostname;

    let field_box = |label: &'static str, input_element: Element<'a, Message>| {
        column![
            text(label).size(11).font(FONT_BODY_SEMIBOLD).style(|_| text::Style {
                color: Some(TEXT_SUB),
            }),
            input_element,
        ]
        .spacing(4)
        .width(Length::Fill)
    };

    let fullname_in = text_input("Full Name (e.g. John Doe)", &cfg.fullname)
        .on_input(on_change_fullname)
        .padding([10, 14])
        .size(13)
        .style(custom_text_input_style());

    let username_in = text_input("Username (e.g. hyperx)", &cfg.username)
        .on_input(on_change_username)
        .padding([10, 14])
        .size(13)
        .style(custom_text_input_style());

    let hostname_in = text_input("Hostname (e.g. hyperxos)", &cfg.hostname)
        .on_input(on_change_hostname)
        .padding([10, 14])
        .size(13)
        .style(custom_text_input_style());

    let password_in = text_input("Password", &cfg.password)
        .on_input(on_change_password)
        .secure(true)
        .padding([10, 14])
        .size(13)
        .style(custom_text_input_style());

    let confirm_in = text_input("Confirm Password", confirm_password)
        .on_input(on_change_confirm)
        .secure(true)
        .padding([10, 14])
        .size(13)
        .style(custom_text_input_style());

    let mut user_form = column![
        row![
            field_box("YOUR FULL NAME", fullname_in.into()),
            field_box("LOGIN USERNAME", username_in.into()),
        ]
        .spacing(16),
        field_box("COMPUTER HOSTNAME", hostname_in.into()),
        row![
            field_box("USER PASSWORD", password_in.into()),
            field_box("CONFIRM PASSWORD", confirm_in.into()),
        ]
        .spacing(16),
    ]
    .spacing(12);

    let same_pwd_cb = row![
        checkbox(use_same_root_password)
            .on_toggle(on_toggle_same_root)
            .size(16),
        text("Use the same password for administrator (root) account")
            .size(12)
            .font(FONT_BODY)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN),
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    user_form = user_form.push(same_pwd_cb);

    if !use_same_root_password {
        let root_pwd_in = text_input("Root Administrator Password", &cfg.root_password)
            .on_input(on_change_root_password)
            .secure(true)
            .padding([10, 14])
            .size(13)
            .style(custom_text_input_style());

        user_form = user_form.push(field_box("ROOT (ADMIN) PASSWORD", root_pwd_in.into()));
    }

    let form_card = container(user_form)
        .padding(20)
        .width(Length::Fill)
        .style(glass_card_style);

    let back_btn = button(text("Back").size(12).font(FONT_BODY_MEDIUM))
        .on_press(on_back)
        .style(secondary_button_style())
        .padding([8, 20]);

    let mut next_btn = button(
        row![
            text("Next: Summary")
                .size(13)
                .font(FONT_BODY_SEMIBOLD),
            svg_icon(icons::ROCKET, 13.0, Color::WHITE),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .style(primary_button_style(ACCENT_CYAN))
    .padding([8, 20]);

    if can_proceed {
        next_btn = next_btn.on_press(on_next);
    }

    column![
        column![
            text("User & System Credentials")
                .size(20)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN)
                }),
            text("Set up your primary user account, hostname, and administrator passwords")
                .size(12)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED)
                }),
        ]
        .spacing(4),
        form_card,
        column![].height(Length::Fill),
        row![
            back_btn,
            row![].width(Length::Fill),
            next_btn,
        ]
        .align_y(Alignment::Center),
    ]
    .spacing(16)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
