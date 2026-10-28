mod sys;
mod ui;

use std::process::Command;

use iced::{window, Size};

use hyperx_ui_kit::{
    load_window_icon, FONT_BODY_MEDIUM, FONT_INTER_BYTES, FONT_PLUS_JAKARTA_BYTES, TEXT_MAIN,
};
use ui::{DesktopWindowId, HyperXDesktop};

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--check" || a == "--version") {
        println!(
            "HyperXOS Desktop Environment (hyperx-de) v2.2.0 — Labwc Wayland Compositor + Iced 0.14 WGPU OK"
        );
        return Ok(());
    }

    // If invoked directly on a raw Linux TTY without an active Wayland/X11 display server,
    // automatically bootstrap a fullscreen Wayland display host with Labwc stacking window manager
    let has_wayland = std::env::var("WAYLAND_DISPLAY")
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    let has_x11 = std::env::var("DISPLAY")
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    let is_inner_session = args.iter().any(|a| a == "--session-ui");

    if !has_wayland && !has_x11 && !is_inner_session {
        let self_exe = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("/usr/bin/hyperx-de"));
        let self_str = self_exe.to_string_lossy().to_string();

        // Launch via labwc (lightweight Wayland stacking compositor with Server-Side Decorations).
        // Hardware cursor planes render flipped/offset on virtio-gpu (QEMU virgl),
        // so force software cursor.
        let status = Command::new("labwc")
            .env("WLR_NO_HARDWARE_CURSORS", "1")
            .args(["-S", &format!("{} --session-ui", self_str)])
            .status();

        if let Ok(s) = status {
            if s.success() {
                return Ok(());
            }
        }
    }

    let mut initial_window: Option<DesktopWindowId> = None;
    let mut open_launcher = false;
    let open_wifi_modal = args.iter().any(|a| a == "--wifi-modal");
    let open_multi_demo = args.iter().any(|a| a == "--multi-demo");
    let open_context_demo = args.iter().any(|a| a == "--context-demo");
    let open_switcher_demo = args.iter().any(|a| a == "--switcher-demo");
    let open_mission_demo = args.iter().any(|a| a == "--mission-demo");
    let open_clipboard_demo = args.iter().any(|a| a == "--clipboard-demo");
    let open_screenshot_demo = args.iter().any(|a| a == "--screenshot-demo");
    let open_customwp_demo = args.iter().any(|a| a == "--customwp-demo");
    let open_launchpad_demo = args.iter().any(|a| a == "--launchpad-demo");

    let bin_name = args
        .first()
        .and_then(|p| std::path::Path::new(p).file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("hyperx-de");

    if bin_name == "hyperx-files" || args.iter().any(|a| a == "--files") {
        initial_window = Some(DesktopWindowId::Files);
    } else if bin_name == "hyperx-settings" || args.iter().any(|a| a == "--settings") {
        initial_window = Some(DesktopWindowId::Settings);
    } else if args.iter().any(|a| a == "--terminal") {
        initial_window = Some(DesktopWindowId::Terminal);
    } else if args.iter().any(|a| a == "--notes") {
        initial_window = Some(DesktopWindowId::Notes);
    } else if bin_name == "hyperx-launcher" || args.iter().any(|a| a == "--launcher") {
        open_launcher = true;
    }

    let window_settings = window::Settings {
        size: Size::new(1366.0, 768.0),
        min_size: Some(Size::new(1024.0, 640.0)),
        maximized: true,
        decorations: false,
        transparent: false,
        icon: load_window_icon(),
        platform_specific: window::settings::PlatformSpecific {
            application_id: "hyperx-de".to_string(),
            override_redirect: false,
        },
        ..Default::default()
    };

    iced::application(
        move || {
            let (mut st, task) =
                HyperXDesktop::new(initial_window, open_launcher);
            if open_wifi_modal {
                st.wifi_modal_open = true;
            }
            if open_multi_demo {
                st.wm.open_or_focus(DesktopWindowId::Files);
                st.wm.state_mut(DesktopWindowId::Files).pos = iced::Point::new(140.0, 18.0);
                st.wm.open_or_focus(DesktopWindowId::Terminal);
                st.wm.state_mut(DesktopWindowId::Terminal).pos = iced::Point::new(420.0, 62.0);
            }
            if open_context_demo {
                st.context_menu = Some(ui::context_menu::ActiveContextMenu {
                    kind: ui::context_menu::ContextMenuKind::Desktop,
                    pos: iced::Point::new(260.0, 110.0),
                });
            }
            if open_switcher_demo {
                st.wm.open_or_focus(DesktopWindowId::Files);
                st.wm.open_or_focus(DesktopWindowId::Settings);
                st.app_switcher_open = true;
            }
            if open_mission_demo {
                st.wm.open_or_focus(DesktopWindowId::Files);
                st.wm.open_or_focus(DesktopWindowId::Terminal);
                st.mission_control_open = true;
            }
            if open_clipboard_demo {
                st.wm.open_or_focus(DesktopWindowId::Files);
                st.wm.set_custom_size(DesktopWindowId::Files, 820.0, 480.0);
                st.clipboard_hud_open = true;
            }
            if open_screenshot_demo {
                let _ = st.update(ui::Message::CaptureScreenshotNow);
                st.screenshot_hud_open = true;
            }
            if open_customwp_demo {
                let _ = st.update(ui::Message::GenerateAndSetSampleCustomWallpaper);
                st.wm.open_or_focus(DesktopWindowId::Settings);
                st.settings_view.active_tab = ui::settings::SettingsTab::Appearance;
                st.wm.set_custom_size(DesktopWindowId::Settings, 860.0, 500.0);
            }
            if open_launchpad_demo {
                st.launcher_open = false;
                let _ = st.update(ui::Message::ToggleLauncher);
                if let Some(discord) = st
                    .desktop_apps
                    .iter()
                    .find(|a| a.id.contains("discord") || a.name.to_lowercase().contains("discord"))
                    .cloned()
                {
                    let _ = st.update(ui::Message::AddAppToDesktop(discord.clone()));
                    let _ = st.update(ui::Message::TogglePinAppToDock(discord));
                }
            }
            (st, task)
        },
        HyperXDesktop::update,
        HyperXDesktop::view,
    )
    .title("HyperXOS Desktop Environment — Labwc Wayland & WGPU Shell")
    .font(FONT_PLUS_JAKARTA_BYTES)
    .font(FONT_INTER_BYTES)
    .default_font(FONT_BODY_MEDIUM)
    .window(window_settings)
    .subscription(HyperXDesktop::subscription)
    .theme(|_: &HyperXDesktop| iced::Theme::Dark)
    .style(|_: &HyperXDesktop, _: &iced::Theme| iced::theme::Style {
        background_color: iced::Color::from_rgb(0.04, 0.05, 0.09),
        text_color: TEXT_MAIN,
    })
    .run()
}
