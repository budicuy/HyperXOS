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
    on_drag: Message,
    on_minimize: Message,
    on_maximize: Message,
    on_close: Message,
) -> Element<'a, Message> {
    // Helper to create header link button with dark-to-white hover and black text + black icon
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

    let github_btn = link_btn(
        "github",
        "GitHub",
        icons::GITHUB,
        Color::WHITE,
        "https://github.com/budicuy/HyperXOS",
    );

    let releases_btn = link_btn(
        "releases",
        "Releases",
        icons::TAG,
        ACCENT_CYAN,
        "https://github.com/budicuy/HyperXOS/releases",
    );

    let issues_btn = link_btn(
        "issues",
        "Issues",
        icons::BUG,
        ACCENT_AMBER,
        "https://github.com/budicuy/HyperXOS/issues",
    );

    let docs_btn = link_btn(
        "docs",
        "Docs",
        icons::DOCS,
        ACCENT_GREEN,
        "https://github.com/budicuy/HyperXOS#readme",
    );

    let links_group = row![github_btn, releases_btn, issues_btn, docs_btn]
        .spacing(8)
        .align_y(Alignment::Center);

    // 2. Draggable area across the header
    let drag_area = mouse_area(
        container("")
            .width(Length::Fill)
            .height(28),
    )
    .on_press(on_drag);

    // 3. Custom Window Controls (Minimize, Maximize, Close)
    let win_min = button(svg_icon(icons::MINUS, 12.0, ACCENT_AMBER))
        .padding([5, 8])
        .style(window_control_btn(ACCENT_AMBER, false))
        .on_press(on_minimize);

    let win_max = button(svg_icon(icons::SQUARE, 11.0, ACCENT_GREEN))
        .padding([5, 8])
        .style(window_control_btn(ACCENT_GREEN, false))
        .on_press(on_maximize);

    let win_close = button(svg_icon(icons::CLOSE, 12.0, ACCENT_RED))
        .padding([5, 8])
        .style(window_control_btn(ACCENT_RED, true))
        .on_press(on_close);

    let window_controls = row![win_min, win_max, win_close]
        .spacing(6)
        .align_y(Alignment::Center);

    let bar = row![
        links_group,
        drag_area,
        window_controls,
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .padding([10, 16]);

    container(bar)
        .width(Length::Fill)
        .style(header_container_style)
        .into()
}
