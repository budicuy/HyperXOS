use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayBackend {
    SdlOpenGL,
    GtkOpenGL,
    GtkStandard,
    Headless,
}

pub struct DisplayManager;

impl DisplayManager {
    /// Detect optimal display backend with Direct OpenGL Hardware Acceleration
    pub fn detect() -> (DisplayBackend, Vec<String>) {
        let has_display = std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();

        if !has_display {
            println!("  -> Display server not detected: Using headless mode (-display none).");
            return (
                DisplayBackend::Headless,
                vec![
                    "-device".to_string(),
                    "virtio-vga".to_string(),
                    "-display".to_string(),
                    "none".to_string(),
                ],
            );
        }

        // Test SDL OpenGL (virtio-vga-gl)
        let test_sdl = Command::new("qemu-system-x86_64")
            .args(["-device", "virtio-vga-gl", "-display", "sdl,gl=on", "-help"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if test_sdl {
            println!("  [✓] Direct Host GPU Acceleration: SDL2 OpenGL (virtio-vga-gl + sdl,gl=on)");
            return (
                DisplayBackend::SdlOpenGL,
                vec![
                    "-device".to_string(),
                    "virtio-vga-gl".to_string(),
                    "-display".to_string(),
                    "sdl,gl=on".to_string(),
                ],
            );
        }

        // Test GTK OpenGL
        let test_gtk_gl = Command::new("qemu-system-x86_64")
            .args(["-device", "virtio-vga-gl", "-display", "gtk,gl=on", "-help"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if test_gtk_gl {
            println!("  [✓] Direct Host GPU Acceleration: GTK OpenGL (virtio-vga-gl + gtk,gl=on)");
            return (
                DisplayBackend::GtkOpenGL,
                vec![
                    "-device".to_string(),
                    "virtio-vga-gl".to_string(),
                    "-display".to_string(),
                    "gtk,gl=on".to_string(),
                ],
            );
        }

        // Fallback to standard GTK
        println!("  -> Using standard GTK display backend (-device virtio-vga -display gtk).");
        (
            DisplayBackend::GtkStandard,
            vec![
                "-device".to_string(),
                "virtio-vga".to_string(),
                "-display".to_string(),
                "gtk".to_string(),
            ],
        )
    }
}
