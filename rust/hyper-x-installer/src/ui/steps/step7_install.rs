use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{column, container, progress_bar, row, scrollable, text};
use iced::{Alignment, Element, Length};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

#[derive(Debug, Clone)]
pub struct InstallState {
    pub stage_name: String,
    pub progress_pct: f32,
    pub log_lines: Vec<String>,
    pub is_failed: bool,
    pub error_message: Option<String>,
}

impl Default for InstallState {
    fn default() -> Self {
        Self {
            stage_name: "Preparing installation engine...".to_string(),
            progress_pct: 0.0,
            log_lines: Vec::new(),
            is_failed: false,
            error_message: None,
        }
    }
}

pub fn view<'a, Message: Clone + 'a>(
    state: &'a InstallState,
) -> Element<'a, Message> {
    let mut logs_col = column![].spacing(3);
    for line in &state.log_lines {
        let is_warn = line.starts_with("[WARN]");
        let is_err = line.starts_with("[ERR]");
        let text_color = if is_err {
            ACCENT_RED
        } else if is_warn {
            ACCENT_AMBER
        } else {
            ACCENT_GREEN
        };

        logs_col = logs_col.push(
            text(line.clone())
                .size(11)
                .font(FONT_BODY)
                .style(move |_| text::Style {
                    color: Some(text_color),
                }),
        );
    }

    let scrollbar = Scrollbar::new()
        .width(5.0)
        .scroller_width(5.0)
        .margin(6.0);

    let terminal_box = container(
        scrollable(logs_col)
            .direction(Direction::Vertical(scrollbar))
            .height(Length::Fixed(320.0))
            .style(glass_scrollable_style),
    )
    .padding(12)
    .width(Length::Fill)
    .style(terminal_container_style);

    let progress_widget = column![
        row![
            text(&state.stage_name)
                .size(13)
                .font(FONT_BODY_SEMIBOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN),
                }),
            row![].width(Length::Fill),
            text(format!("{:.0}%", state.progress_pct))
                .size(13)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(ACCENT_CYAN),
                }),
        ]
        .align_y(Alignment::Center),
        progress_bar(0.0..=100.0, state.progress_pct)
            .length(Length::Fill)
            .girth(8.0)
            .style(custom_progress_bar_style(ACCENT_CYAN)),
    ]
    .spacing(8);

    let error_view = if let Some(ref err) = state.error_message {
        container(
            row![
                svg_icon(icons::ALERT_TRIANGLE, 24.0, ACCENT_RED),
                column![
                    text("INSTALLATION ERROR OCCURRED")
                        .size(12)
                        .font(FONT_DISPLAY_BOLD)
                        .style(|_| text::Style {
                            color: Some(ACCENT_RED),
                        }),
                    text(err)
                        .size(11)
                        .font(FONT_BODY)
                        .style(|_| text::Style {
                            color: Some(TEXT_SUB),
                        }),
                ]
                .spacing(2)
                .width(Length::Fill),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding(12)
        .width(Length::Fill)
        .style(glass_card_accent_style(ACCENT_RED))
    } else {
        container(row![].height(Length::Shrink))
    };

    column![
        column![
            text("Installing HyperXOS Linux")
                .size(20)
                .font(FONT_DISPLAY_BOLD)
                .style(|_| text::Style {
                    color: Some(TEXT_MAIN)
                }),
            text("Deploying base packages, Limine bootloader, and KDE Plasma 6 Wayland desktop")
                .size(12)
                .font(FONT_BODY)
                .style(|_| text::Style {
                    color: Some(TEXT_MUTED)
                }),
        ]
        .spacing(4),
        progress_widget,
        error_view,
        column![
            row![
                svg_icon(icons::TERMINAL, 14.0, ACCENT_GREEN),
                text("REAL-TIME INSTALLATION TERMINAL OUTPUT")
                    .size(11)
                    .font(FONT_DISPLAY_BOLD)
                    .style(|_| text::Style {
                        color: Some(ACCENT_GREEN)
                    }),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            terminal_box,
        ]
        .spacing(6),
    ]
    .spacing(14)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
