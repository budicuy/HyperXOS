use iced::border::Radius;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length};
use crate::ui::icons::{self, svg_icon};
use crate::ui::theme::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepId {
    Welcome = 1,
    Location = 2,
    Keyboard = 3,
    Disk = 4,
    Users = 5,
    Summary = 6,
    Install = 7,
    Finished = 8,
}

impl StepId {
    pub fn all() -> &'static [StepId] {
        &[
            StepId::Welcome,
            StepId::Location,
            StepId::Keyboard,
            StepId::Disk,
            StepId::Users,
            StepId::Summary,
            StepId::Install,
            StepId::Finished,
        ]
    }

    pub fn title(&self) -> &'static str {
        match self {
            StepId::Welcome => "Welcome",
            StepId::Location => "Location",
            StepId::Keyboard => "Keyboard",
            StepId::Disk => "Partitions",
            StepId::Users => "Users",
            StepId::Summary => "Summary",
            StepId::Install => "Install",
            StepId::Finished => "Finished",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            StepId::Welcome => icons::SHIELD_CHECK,
            StepId::Location => icons::GLOBE,
            StepId::Keyboard => icons::KEYBOARD,
            StepId::Disk => icons::HARD_DRIVE,
            StepId::Users => icons::USER,
            StepId::Summary => icons::CHECK_CIRCLE,
            StepId::Install => icons::ROCKET,
            StepId::Finished => icons::CHECK_CIRCLE,
        }
    }
}

pub fn stepper_sidebar<'a, Message: 'a>(current_step: StepId) -> Element<'a, Message> {
    let mut step_items = column![].spacing(10);

    for step in StepId::all() {
        let is_current = *step == current_step;
        let is_completed = (*step as u8) < (current_step as u8);

        let (bg, border, fg_color, icon_color) = if is_current {
            (
                Color::WHITE,
                Color::WHITE,
                Color::from_rgb(0.06, 0.07, 0.10),
                Color::from_rgb(0.06, 0.07, 0.10),
            )
        } else if is_completed {
            (
                Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                Color::from_rgba(1.0, 1.0, 1.0, 0.35),
                Color::WHITE,
                ACCENT_GREEN,
            )
        } else {
            (
                Color::TRANSPARENT,
                Color::TRANSPARENT,
                Color::from_rgba(1.0, 1.0, 1.0, 0.65),
                Color::from_rgba(1.0, 1.0, 1.0, 0.45),
            )
        };

        let num_str = format!("{}", *step as u8);
        let item = container(
            row![
                text(num_str)
                    .size(11)
                    .font(FONT_BODY_SEMIBOLD)
                    .style(move |_| text::Style {
                        color: Some(icon_color)
                    }),
                svg_icon(step.icon(), 14.0, icon_color),
                text(step.title())
                    .size(13)
                    .font(if is_current {
                        FONT_BODY_SEMIBOLD
                    } else {
                        FONT_BODY
                    })
                    .style(move |_| text::Style {
                        color: Some(fg_color)
                    }),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .padding([8, 12])
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(bg)),
            border: Border::default()
                .rounded(Radius::from(9.0))
                .width(1.2)
                .color(border),
            ..container::Style::default()
        });

        step_items = step_items.push(item);
    }

    container(
        column![
            // Top Logo Banner in Sidebar
            container(icons::logo_image(110.0))
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .padding([10, 0]),
            step_items,
        ]
        .spacing(18),
    )
    .width(Length::Fixed(220.0))
    .height(Length::Fill)
    .padding(16)
    .style(stepper_bar_style)
    .into()
}
