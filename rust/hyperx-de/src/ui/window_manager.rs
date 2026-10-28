use iced::{Color, Point, Size};
use hyperx_ui_kit::{icons, ACCENT_AMBER, ACCENT_CYAN, ACCENT_GREEN, ACCENT_PURPLE};

use crate::ui::DesktopWindowId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSnapMode {
    Floating,
    Maximized,
    LeftHalf,
    RightHalf,
}

#[derive(Debug, Clone)]
pub struct FloatingWindowState {
    pub _id: DesktopWindowId,
    pub open: bool,
    pub minimized: bool,
    pub snap: WindowSnapMode,
    pub pos: Point,
    pub size: Size,
}

impl FloatingWindowState {
    pub fn new(id: DesktopWindowId, pos: Point, size: Size) -> Self {
        Self {
            _id: id,
            open: false,
            minimized: false,
            snap: WindowSnapMode::Floating,
            pos,
            size,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.open && !self.minimized
    }
}

pub struct FloatingWindowManager {
    pub files: FloatingWindowState,
    pub settings: FloatingWindowState,
    pub terminal: FloatingWindowState,
    pub notes: FloatingWindowState,
    pub z_order: Vec<DesktopWindowId>,
    pub dragging_window: Option<(DesktopWindowId, Point)>,
    pub resizing_window: Option<(DesktopWindowId, Point, Size)>,
    pub show_desktop_active: bool,
    pub pre_show_desktop_visible: Vec<DesktopWindowId>,
}

impl Default for FloatingWindowManager {
    fn default() -> Self {
        Self {
            files: FloatingWindowState::new(
                DesktopWindowId::Files,
                Point::new(150.0, 26.0),
                Size::new(980.0, 575.0),
            ),
            settings: FloatingWindowState::new(
                DesktopWindowId::Settings,
                Point::new(210.0, 38.0),
                Size::new(940.0, 560.0),
            ),
            terminal: FloatingWindowState::new(
                DesktopWindowId::Terminal,
                Point::new(260.0, 52.0),
                Size::new(860.0, 520.0),
            ),
            notes: FloatingWindowState::new(
                DesktopWindowId::Notes,
                Point::new(310.0, 64.0),
                Size::new(820.0, 510.0),
            ),
            z_order: vec![
                DesktopWindowId::Notes,
                DesktopWindowId::Terminal,
                DesktopWindowId::Settings,
                DesktopWindowId::Files,
            ],
            dragging_window: None,
            resizing_window: None,
            show_desktop_active: false,
            pre_show_desktop_visible: Vec::new(),
        }
    }
}

impl FloatingWindowManager {
    pub fn state(&self, id: DesktopWindowId) -> &FloatingWindowState {
        match id {
            DesktopWindowId::Files => &self.files,
            DesktopWindowId::Settings => &self.settings,
            DesktopWindowId::Terminal => &self.terminal,
            DesktopWindowId::Notes => &self.notes,
        }
    }

    pub fn state_mut(&mut self, id: DesktopWindowId) -> &mut FloatingWindowState {
        match id {
            DesktopWindowId::Files => &mut self.files,
            DesktopWindowId::Settings => &mut self.settings,
            DesktopWindowId::Terminal => &mut self.terminal,
            DesktopWindowId::Notes => &mut self.notes,
        }
    }

    pub fn focus(&mut self, id: DesktopWindowId) {
        self.z_order.retain(|w| *w != id);
        self.z_order.push(id);
    }

    pub fn open_or_focus(&mut self, id: DesktopWindowId) {
        self.show_desktop_active = false;
        let st = self.state_mut(id);
        st.open = true;
        st.minimized = false;
        self.focus(id);
    }

    pub fn dock_click(&mut self, id: DesktopWindowId) {
        self.show_desktop_active = false;
        let is_top_focused = self.focused_window() == Some(id);
        let st = self.state_mut(id);
        if !st.open {
            st.open = true;
            st.minimized = false;
            self.focus(id);
        } else if st.minimized {
            st.minimized = false;
            self.focus(id);
        } else if is_top_focused {
            // Clicking already-focused dock item minimizes it (like Windows/macOS click toggle)
            st.minimized = true;
        } else {
            self.focus(id);
        }
    }

    pub fn minimize(&mut self, id: DesktopWindowId) {
        let st = self.state_mut(id);
        if st.open {
            st.minimized = true;
        }
    }

    pub fn close(&mut self, id: DesktopWindowId) {
        let st = self.state_mut(id);
        st.open = false;
        st.minimized = false;
        st.snap = WindowSnapMode::Floating;
    }

    pub fn toggle_maximize(&mut self, id: DesktopWindowId) {
        self.focus(id);
        let st = self.state_mut(id);
        st.minimized = false;
        st.snap = match st.snap {
            WindowSnapMode::Maximized => WindowSnapMode::Floating,
            _ => WindowSnapMode::Maximized,
        };
    }

    pub fn set_snap(&mut self, id: DesktopWindowId, snap: WindowSnapMode) {
        self.focus(id);
        let st = self.state_mut(id);
        st.open = true;
        st.minimized = false;
        st.snap = snap;
    }

    pub fn set_custom_size(&mut self, id: DesktopWindowId, w: f32, h: f32) {
        self.focus(id);
        let st = self.state_mut(id);
        st.open = true;
        st.minimized = false;
        st.snap = WindowSnapMode::Floating;
        st.size = Size::new(w.clamp(680.0, 1280.0), h.clamp(410.0, 620.0));
    }

    pub fn focused_window(&self) -> Option<DesktopWindowId> {
        self.z_order
            .iter()
            .rev()
            .copied()
            .find(|id| self.state(*id).is_visible())
    }

    pub fn visible_windows_in_z_order(&self) -> Vec<DesktopWindowId> {
        self.z_order
            .iter()
            .copied()
            .filter(|id| self.state(*id).is_visible())
            .collect()
    }

    #[allow(dead_code)]
    pub fn running_windows(&self) -> Vec<DesktopWindowId> {
        let all = [
            DesktopWindowId::Files,
            DesktopWindowId::Settings,
            DesktopWindowId::Terminal,
            DesktopWindowId::Notes,
        ];
        all.into_iter()
            .filter(|id| self.state(*id).open)
            .collect()
    }

    pub fn toggle_show_desktop(&mut self) {
        let currently_visible = self.visible_windows_in_z_order();
        if !currently_visible.is_empty() {
            self.pre_show_desktop_visible = currently_visible.clone();
            for id in currently_visible {
                self.state_mut(id).minimized = true;
            }
            self.show_desktop_active = true;
        } else if !self.pre_show_desktop_visible.is_empty() {
            let to_restore = self.pre_show_desktop_visible.clone();
            for id in to_restore {
                let st = self.state_mut(id);
                if st.open {
                    st.minimized = false;
                }
            }
            self.show_desktop_active = false;
        }
    }
}

pub fn window_meta(id: DesktopWindowId) -> (&'static str, &'static str, &'static str, Color) {
    match id {
        DesktopWindowId::Files => (
            "HyperX Files",
            "Manajer Berkas Finder",
            icons::FOLDER,
            ACCENT_CYAN,
        ),
        DesktopWindowId::Settings => (
            "HyperX Settings",
            "Pengaturan Sistem OS",
            icons::SETTINGS_GEAR,
            ACCENT_AMBER,
        ),
        DesktopWindowId::Terminal => (
            "HyperX Terminal & Monitor",
            "Konsol Perintah & Telemetri",
            icons::TERMINAL,
            ACCENT_GREEN,
        ),
        DesktopWindowId::Notes => (
            "HyperX Notes & Editor",
            "Editor Teks & Catatan Cepat",
            icons::EDIT_PEN,
            ACCENT_PURPLE,
        ),
    }
}
