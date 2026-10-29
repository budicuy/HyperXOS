use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Color, Element, Length};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub const KEYBOARDS: &[(&str, &str)] = &[
    ("us", "English (US) - Standard QWERTY"),
    ("gb", "English (UK) - UK Layout"),
    ("de", "German - QWERTZ"),
    ("fr", "French - AZERTY"),
    ("es", "Spanish - Español"),
    ("jp", "Japanese - OADG 109A"),
];

pub fn view<'a, Message: Clone + 'a>(
    selected_layout: &str,
    test_text: &str,
    on_select_layout: impl Fn(String) -> Message + 'a,
    on_change_test_text: impl Fn(String) -> Message + 'a,
    on_back: Message,
    on_next: Message,
) -> Element<'a, Message> {
    let mut layout_col = column![].spacing(8);

    for (code, name) in KEYBOARDS {
        let is_sel = *code == selected_layout;
        let code_str = code.to_string();

        let btn = button(
            row![
                svg_icon(icons::KEYBOARD, 16.0, if is_sel { FLUENT_PRIMARY } else { TEXT_MUTED }),
                text(*name)
                    .size(13)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(move |_| text::Style {
                        color: Some(if is_sel { FLUENT_PRIMARY } else { TEXT_MAIN }),
                    }),
                row![].width(Length::Fill),
                if is_sel {
                    svg_icon(icons::CHECK_CIRCLE, 16.0, FLUENT_PRIMARY)
                } else {
                    svg_icon(icons::MINUS, 0.0, Color::TRANSPARENT)
                },
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding([12, 16])
        .width(Length::Fill)
        .style(move |_, _| {
            let style = selection_card_style(is_sel)(&iced::Theme::Dark);
            button::Style {
                background: style.background,
                border: style.border,
                text_color: FLUENT_TEXT_DARK,
                ..button::Style::default()
            }
        })
        .on_press(on_select_layout(code_str));

        layout_col = layout_col.push(btn);
    }

    let test_box = column![
        text("TEST KEYBOARD INPUT")
            .size(11)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(FLUENT_PRIMARY)
            }),
        text_input("Type here to test special keys, symbols, and accents...", test_text)
            .on_input(on_change_test_text)
            .padding([12, 16])
            .size(13)
            .style(custom_text_input_style()),
    ]
    .spacing(8);

    let back_btn = button(text("Back").size(12).font(FONT_BODY_MEDIUM))
        .on_press(on_back)
        .style(secondary_button_style())
        .padding([8, 20]);

    let next_btn = button(
        row![
            text("Next: Partitions")
                .size(13)
                .font(FONT_BODY_SEMIBOLD),
            svg_icon(icons::ROCKET, 13.0, Color::WHITE),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .on_press(on_next)
    .style(primary_button_style(FLUENT_PRIMARY))
    .padding([8, 20]);

    column![
        column![
            text("Keyboard Layout")
                .size(20)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN)
                }),
            text("Select your physical keyboard layout and test keys below")
                .size(12)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED)
                }),
        ]
        .spacing(4),
        column![
            text("AVAILABLE KEYBOARD MODELS")
                .size(11)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(FLUENT_PRIMARY)
                }),
            layout_col,
        ]
        .spacing(8),
        test_box,
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
