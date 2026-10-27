use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};

use crate::theme::{self, ACCENT_AMBER, ACCENT_BLUE, ACCENT_GREEN, ACCENT_PURPLE, TEXT_PRIMARY, TEXT_SECONDARY};

#[derive(Debug, Clone)]
pub enum IslandState {
    Idle,
    ExpandedMedia {
        title: String,
        artist: String,
        playing: bool,
        progress: f32,
    },
    ToastVolume {
        level: u8,
        ticks_left: u8,
    },
    ToastBrightness {
        level: u8,
        ticks_left: u8,
    },
    Notification {
        app_name: String,
        title: String,
        body: String,
        ticks_left: u8,
    },
}

#[derive(Debug, Clone)]
pub enum IslandAction {
    ToggleExpand,
    PlayPause,
    NextTrack,
    PrevTrack,
    DismissToast,
}

pub struct DynamicIsland {
    pub state: IslandState,
    pub waveform_tick: u8,
}

impl Default for DynamicIsland {
    fn default() -> Self {
        Self {
            state: IslandState::Idle,
            waveform_tick: 0,
        }
    }
}

impl DynamicIsland {
    pub fn update(&mut self, action: IslandAction) {
        match action {
            IslandAction::ToggleExpand => {
                self.state = match self.state {
                    IslandState::Idle => IslandState::ExpandedMedia {
                        title: "Neon Horizon".to_string(),
                        artist: "HyperX Synthwave".to_string(),
                        playing: true,
                        progress: 0.42,
                    },
                    IslandState::ExpandedMedia { .. } => IslandState::Idle,
                    _ => IslandState::Idle,
                };
            }
            IslandAction::PlayPause => {
                if let IslandState::ExpandedMedia { ref mut playing, .. } = self.state {
                    *playing = !*playing;
                }
            }
            IslandAction::NextTrack => {
                if let IslandState::ExpandedMedia {
                    ref mut title,
                    ref mut artist,
                    ref mut progress,
                    ..
                } = self.state
                {
                    *title = "Cyber Pulse (Remix)".to_string();
                    *artist = "HyperX Wave".to_string();
                    *progress = 0.05;
                }
            }
            IslandAction::PrevTrack => {
                if let IslandState::ExpandedMedia {
                    ref mut title,
                    ref mut artist,
                    ref mut progress,
                    ..
                } = self.state
                {
                    *title = "Aurora Nights".to_string();
                    *artist = "HyperX Chill".to_string();
                    *progress = 0.88;
                }
            }
            IslandAction::DismissToast => {
                self.state = IslandState::Idle;
            }
        }
    }

    pub fn show_volume_toast(&mut self, level: u8) {
        self.state = IslandState::ToastVolume {
            level,
            ticks_left: 3,
        };
    }

    pub fn show_brightness_toast(&mut self, level: u8) {
        self.state = IslandState::ToastBrightness {
            level,
            ticks_left: 3,
        };
    }

    pub fn show_notification(&mut self, app: &str, title: &str, body: &str) {
        self.state = IslandState::Notification {
            app_name: app.to_string(),
            title: title.to_string(),
            body: body.to_string(),
            ticks_left: 5,
        };
    }

    pub fn tick(&mut self) {
        self.waveform_tick = (self.waveform_tick + 1) % 4;

        match &mut self.state {
            IslandState::ToastVolume { ticks_left, .. }
            | IslandState::ToastBrightness { ticks_left, .. }
            | IslandState::Notification { ticks_left, .. } => {
                if *ticks_left > 1 {
                    *ticks_left -= 1;
                } else {
                    self.state = IslandState::Idle;
                }
            }
            IslandState::ExpandedMedia { progress, playing, .. } => {
                if *playing {
                    *progress += 0.02;
                    if *progress > 1.0 {
                        *progress = 0.0;
                    }
                }
            }
            IslandState::Idle => {}
        }
    }

    pub fn view<Message: 'static + Clone>(&self, map_fn: impl Fn(IslandAction) -> Message + Copy + 'static) -> Element<'_, Message> {
        match &self.state {
            IslandState::Idle => self.view_idle(map_fn),
            IslandState::ExpandedMedia {
                title,
                artist,
                playing,
                progress,
            } => self.view_expanded_media(title, artist, *playing, *progress, map_fn),
            IslandState::ToastVolume { level, .. } => self.view_toast_volume(*level, map_fn),
            IslandState::ToastBrightness { level, .. } => self.view_toast_brightness(*level, map_fn),
            IslandState::Notification {
                app_name,
                title,
                body,
                ..
            } => self.view_notification(app_name, title, body, map_fn),
        }
    }

    fn view_idle<Message: 'static + Clone>(&self, map_fn: impl Fn(IslandAction) -> Message + 'static) -> Element<'_, Message> {
        let content = row![
            text("⚡").size(14).color(ACCENT_BLUE),
            text("HyperXOS").size(12).color(TEXT_PRIMARY),
            text("●").size(6).color(ACCENT_GREEN),
            text("Audio Ready").size(11).color(TEXT_SECONDARY),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        button(
            container(content)
                .padding([4, 16])
                .style(theme::dynamic_island_style),
        )
        .padding(0)
        .on_press(map_fn(IslandAction::ToggleExpand))
        .into()
    }

    fn view_expanded_media<Message: 'static + Clone>(
        &self,
        title: &str,
        artist: &str,
        playing: bool,
        progress: f32,
        map_fn: impl Fn(IslandAction) -> Message + Copy + 'static,
    ) -> Element<'_, Message> {
        let wave_str = match self.waveform_tick {
            0 => " ▂▃▅▆▇",
            1 => "▃▅▇▆▃ ",
            2 => "▇▆▃ ▂▃",
            _ => "▅▃ ▂▃▅",
        };

        let header = row![
            text("🎧").size(24),
            column![
                text(title.to_string()).size(13).color(TEXT_PRIMARY),
                text(artist.to_string()).size(11).color(TEXT_SECONDARY),
            ]
            .spacing(2),
            Space::new().width(Length::Fill),
            text(wave_str).size(14).color(ACCENT_PURPLE),
            button(text("✕").size(12).color(TEXT_SECONDARY))
                .padding([2, 6])
                .on_press(map_fn(IslandAction::ToggleExpand)),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        // Progress bar simulation
        let total_chars = 24;
        let filled_chars = (progress * total_chars as f32).round() as usize;
        let mut bar = String::new();
        for i in 0..total_chars {
            if i < filled_chars {
                bar.push('█');
            } else {
                bar.push('░');
            }
        }

        let progress_row = row![
            text("01:14").size(10).color(TEXT_SECONDARY),
            text(bar).size(10).color(ACCENT_BLUE),
            text("03:30").size(10).color(TEXT_SECONDARY),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let controls = row![
            Space::new().width(Length::Fill),
            button(text("⏮").size(16).color(TEXT_PRIMARY))
                .padding([4, 8])
                .on_press(map_fn(IslandAction::PrevTrack)),
            button(text(if playing { "⏸" } else { "▶" }).size(18).color(TEXT_PRIMARY))
                .padding([4, 12])
                .on_press(map_fn(IslandAction::PlayPause)),
            button(text("⏭").size(16).color(TEXT_PRIMARY))
                .padding([4, 8])
                .on_press(map_fn(IslandAction::NextTrack)),
            Space::new().width(Length::Fill),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let body = column![header, progress_row, controls].spacing(8);

        container(body)
            .width(360)
            .padding([10, 16])
            .style(theme::dynamic_island_style)
            .into()
    }

    fn view_toast_volume<Message: 'static + Clone>(
        &self,
        level: u8,
        map_fn: impl Fn(IslandAction) -> Message + 'static,
    ) -> Element<'_, Message> {
        let total = 14;
        let filled = ((level as f32 / 100.0) * total as f32).round() as usize;
        let mut bar = String::new();
        for i in 0..total {
            if i < filled {
                bar.push('━');
            } else {
                bar.push('─');
            }
        }

        let content = row![
            text(if level == 0 { "🔇" } else { "🔊" }).size(15),
            text(format!("{}%", level)).size(12).color(TEXT_PRIMARY),
            text(bar).size(12).color(ACCENT_BLUE),
            button(text("✕").size(10).color(TEXT_SECONDARY))
                .padding([2, 4])
                .on_press(map_fn(IslandAction::DismissToast)),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        container(content)
            .padding([6, 16])
            .style(theme::dynamic_island_style)
            .into()
    }

    fn view_toast_brightness<Message: 'static + Clone>(
        &self,
        level: u8,
        map_fn: impl Fn(IslandAction) -> Message + 'static,
    ) -> Element<'_, Message> {
        let total = 14;
        let filled = ((level as f32 / 100.0) * total as f32).round() as usize;
        let mut bar = String::new();
        for i in 0..total {
            if i < filled {
                bar.push('━');
            } else {
                bar.push('─');
            }
        }

        let content = row![
            text("☀️").size(15),
            text(format!("{}%", level)).size(12).color(TEXT_PRIMARY),
            text(bar).size(12).color(ACCENT_AMBER),
            button(text("✕").size(10).color(TEXT_SECONDARY))
                .padding([2, 4])
                .on_press(map_fn(IslandAction::DismissToast)),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        container(content)
            .padding([6, 16])
            .style(theme::dynamic_island_style)
            .into()
    }

    fn view_notification<Message: 'static + Clone>(
        &self,
        app_name: &str,
        title: &str,
        body: &str,
        map_fn: impl Fn(IslandAction) -> Message + 'static,
    ) -> Element<'_, Message> {
        let header = row![
            text("🔔").size(14).color(ACCENT_BLUE),
            text(app_name.to_string()).size(12).color(ACCENT_BLUE),
            Space::new().width(Length::Fill),
            text("now").size(10).color(TEXT_SECONDARY),
            button(text("✕").size(10).color(TEXT_SECONDARY))
                .padding([2, 4])
                .on_press(map_fn(IslandAction::DismissToast)),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let details = column![
            text(title.to_string()).size(13).color(TEXT_PRIMARY),
            text(body.to_string()).size(11).color(TEXT_SECONDARY),
        ]
        .spacing(2);

        let content = column![header, details].spacing(6);

        container(content)
            .width(360)
            .padding([10, 16])
            .style(theme::dynamic_island_style)
            .into()
    }
}
