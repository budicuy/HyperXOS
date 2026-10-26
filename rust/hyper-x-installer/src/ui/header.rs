use iced::border::Radius;
use iced::mouse;
use iced::widget::{button, container, mouse_area, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};

use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

pub fn header<'a, Message: Clone + 'a>(
    hovered_btn: Option<&'static str>,
    on_hover: impl Fn(Option<&'static str>) -> Message + 'a,
    on_open_url: impl Fn(&'static str) -> Message + 'a,
    on_minimize: Message,
    on_close: Message,
) -> Element<'a, Message> {
    let link_btn = |id: &'static str,
                    label: &'static str,
                    icon_svg: &'static str,
                    accent_color: Color,
                    url: &'static str|
     -> Element<'a, Message> {
        let is_h = hovered_btn == Some(id);
        let (bg, border, fg) = if is_h {
            (
                Color::WHITE,
                Color::WHITE,
                Color::from_rgb(0.06, 0.07, 0.10),
            )
        } else {
            (
                Color::from_rgb(0.06, 0.08, 0.12),
                Color::from_rgba(1.0, 1.0, 1.0, 0.22),
                Color::WHITE,
            )
        };
        let icon_c = if is_h {
            Color::from_rgb(0.06, 0.07, 0.10)
        } else {
            accent_color
        };

        let content = container(
            row![
                svg_icon(icon_svg, 12.5, icon_c),
                text(label)
                    .size(11)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(move |_| text::Style {
                        color: Some(fg),
                    }),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding([5, 10])
        .style(move |_| container::Style {
            background: Some(Background::Color(bg)),
            border: Border::default()
                .rounded(Radius::from(8.0))
                .width(1.2)
                .color(border),
            ..container::Style::default()
        });

        mouse_area(content)
            .interaction(mouse::Interaction::Pointer)
            .on_enter(on_hover(Some(id)))
            .on_exit(on_hover(None))
            .on_press(on_open_url(url))
            .into()
    };

    let brand = row![
        icons::logo_image(28.0),
        text("HyperXOS Installer")
            .size(14)
            .font(FONT_DISPLAY_BOLD)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN)
            }),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let links = row![
        link_btn(
            "github",
            "GitHub",
            icons::GITHUB,
            Color::WHITE,
            "https://github.com/budicuy/HyperXOS"
        ),
        link_btn(
            "releases",
            "Releases",
            icons::TAG,
            ACCENT_CYAN,
            "https://github.com/budicuy/HyperXOS/releases"
        ),
        link_btn(
            "issues",
            "Issues",
            icons::BUG,
            ACCENT_AMBER,
            "https://github.com/budicuy/HyperXOS/issues"
        ),
        link_btn(
            "docs",
            "Docs",
            icons::DOCS,
            ACCENT_TEAL,
            "https://github.com/budicuy/HyperXOS/wiki"
        ),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let win_controls = row![
        button(svg_icon(icons::MINUS, 12.0, TEXT_MUTED))
            .on_press(on_minimize)
            .style(secondary_button_style())
            .padding([5, 8]),
        button(svg_icon(icons::CLOSE, 12.0, ACCENT_RED))
            .on_press(on_close)
            .style(secondary_button_style())
            .padding([5, 8]),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    container(
        row![
            brand,
            row![].width(Length::Fill),
            links,
            row![].width(Length::Fixed(16.0)),
            win_controls,
        ]
        .align_y(Alignment::Center),
    )
    .padding([12, 20])
    .into()
}
