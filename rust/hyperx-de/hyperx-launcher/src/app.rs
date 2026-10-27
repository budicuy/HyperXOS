use std::process::Command;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::alignment::Alignment;
use iced::border::Radius;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{color, Border, Color, Element, Event, Length, Shadow, Subscription, Task, Theme, Vector};

use crate::calc::evaluate_math;
use crate::scanner::{scan_applications, AppItem};

#[derive(Debug, Clone)]
pub enum Message {
    QueryChanged(String),
    SelectIndex(usize),
    LaunchSelected,
    LaunchApp(String),
    EventReceived(Event),
}

pub struct LauncherApp {
    pub query: String,
    pub all_apps: Vec<AppItem>,
    pub filtered: Vec<AppItem>,
    pub selected_index: usize,
    pub math_result: Option<f64>,
}

impl Default for LauncherApp {
    fn default() -> Self {
        let apps = scan_applications();
        let initial = apps.iter().take(8).cloned().collect();
        Self {
            query: String::new(),
            all_apps: apps,
            filtered: initial,
            selected_index: 0,
            math_result: None,
        }
    }
}

impl LauncherApp {
    pub fn title(&self) -> String {
        "Spotlight Search".to_string()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::EventReceived)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::QueryChanged(new_query) => {
                self.query = new_query.clone();
                self.selected_index = 0;

                // Evaluate math expression
                self.math_result = evaluate_math(&new_query);

                if new_query.trim().is_empty() {
                    self.filtered = self.all_apps.iter().take(8).cloned().collect();
                } else {
                    let matcher = SkimMatcherV2::default();
                    let mut scored: Vec<(AppItem, i64)> = self
                        .all_apps
                        .iter()
                        .filter_map(|app| {
                            let score_name = matcher.fuzzy_match(&app.name, &new_query).unwrap_or(0);
                            let score_desc = matcher.fuzzy_match(&app.description, &new_query).unwrap_or(0);
                            let score = score_name.max(score_desc / 2);
                            if score > 0 {
                                Some((app.clone(), score))
                            } else {
                                None
                            }
                        })
                        .collect();

                    scored.sort_by(|a, b| b.1.cmp(&a.1));
                    self.filtered = scored.into_iter().map(|(app, _)| app).take(10).collect();
                }
            }
            Message::SelectIndex(idx) => {
                self.selected_index = idx;
            }
            Message::LaunchSelected => {
                if let Some(app) = self.filtered.get(self.selected_index) {
                    Self::spawn_cmd(&app.exec);
                    std::process::exit(0);
                }
            }
            Message::LaunchApp(exec) => {
                Self::spawn_cmd(&exec);
                std::process::exit(0);
            }
            Message::EventReceived(Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                modifiers: _,
                ..
            })) => match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                    if !self.filtered.is_empty() {
                        self.selected_index = (self.selected_index + 1).min(self.filtered.len() - 1);
                    }
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                    return self.update(Message::LaunchSelected);
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
                    std::process::exit(0);
                }
                _ => {}
            },
            Message::EventReceived(_) => {}
        }
        Task::none()
    }

    fn spawn_cmd(cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if let Some((bin, args)) = parts.split_first() {
            let _ = Command::new(bin).args(args).spawn();
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let search_icon = text("🔍").size(20).color(color!(0x38, 0xbd, 0xf8));

        let input_box = text_input(
            "Spotlight Search (apps, calculate math, actions)...",
            &self.query,
        )
        .on_input(Message::QueryChanged)
        .on_submit(Message::LaunchSelected)
        .size(16)
        .padding([8, 12])
        .style(|_theme: &Theme, _status| text_input::Style {
            background: Color::TRANSPARENT.into(),
            border: Border::default(),
            icon: Color::TRANSPARENT,
            placeholder: Color::from_rgba(0.6, 0.65, 0.75, 0.6),
            value: Color::WHITE,
            selection: Color::from_rgba(0.2, 0.45, 0.95, 0.4),
        });

        let esc_badge = container(text("esc").size(10).color(color!(0x94, 0xa3, 0xb8)))
            .padding([3, 7])
            .style(|_| container::Style {
                background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.08).into()),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            });

        let search_bar = row![search_icon, input_box, esc_badge]
            .spacing(12)
            .padding([8, 16])
            .align_y(Alignment::Center);

        let mut content = column![search_bar].spacing(10);

        // Instant math calculation card
        if let Some(calc_val) = self.math_result {
            let math_card = container(
                row![
                    text("🧮").size(22),
                    text(format!("= {:.4}", calc_val))
                        .size(18)
                        .color(color!(0x10, 0xb9, 0x81)),
                    Space::new().width(Length::Fill),
                    text("Calculation").size(11).color(color!(0x94, 0xa3, 0xb8)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding([10, 16])
            .style(|_| container::Style {
                background: Some(Color::from_rgba(0.1, 0.15, 0.22, 0.9).into()),
                border: Border {
                    color: Color::from_rgba(0.06, 0.72, 0.5, 0.3),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            });

            content = content.push(math_card);
        }

        // App results list
        let mut results_col = column![].spacing(4);

        for (idx, app) in self.filtered.iter().enumerate() {
            let is_selected = idx == self.selected_index;

            let icon_text = text(if app.icon.starts_with('/') {
                "📦"
            } else if app.icon.len() <= 4 {
                &app.icon
            } else {
                "⚡"
            })
            .size(20);

            let row_content = row![
                icon_text,
                column![
                    text(&app.name).size(13).color(Color::WHITE),
                    text(&app.description)
                        .size(10)
                        .color(Color::from_rgba(0.7, 0.75, 0.85, 0.8)),
                ]
                .spacing(2),
                Space::new().width(Length::Fill),
                text(&app.category).size(10).color(color!(0x64, 0x74, 0x8b)),
            ]
            .spacing(12)
            .align_y(Alignment::Center);

            let exec_cmd = app.exec.clone();
            let app_card = button(
                container(row_content)
                    .width(Length::Fill)
                    .padding([8, 12])
                    .style(move |_| container::Style {
                        background: if is_selected {
                            Some(Color::from_rgba(0.2, 0.45, 0.95, 0.35).into())
                        } else {
                            None
                        },
                        border: Border {
                            color: if is_selected {
                                Color::from_rgba(0.22, 0.74, 0.97, 0.5)
                            } else {
                                Color::TRANSPARENT
                            },
                            width: 1.0,
                            radius: 8.0.into(),
                        },
                        ..Default::default()
                    }),
            )
            .padding(0)
            .style(button::text)
            .on_press(Message::LaunchApp(exec_cmd));

            results_col = results_col.push(app_card);
        }

        if !self.filtered.is_empty() {
            content = content.push(
                container(scrollable(results_col).height(Length::Shrink))
                    .padding([4, 6])
            );
        }

        let modal_box = container(content)
            .width(620)
            .padding(10)
            .style(|_| container::Style {
                background: Some(Color::from_rgba(0.08, 0.09, 0.13, 0.95).into()),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                    width: 1.0,
                    radius: Radius::from(16.0),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.75),
                    offset: Vector::new(0.0, 12.0),
                    blur_radius: 36.0,
                },
                ..Default::default()
            });

        // Center modal
        container(modal_box)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .padding(iced::Padding {
                top: 60.0,
                ..Default::default()
            })
            .into()
    }
}
