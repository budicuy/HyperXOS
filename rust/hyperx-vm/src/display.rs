use std::path::{Path, PathBuf};
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
    pub fn detect(root: &Path) -> (DisplayBackend, Vec<String>) {
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

        let bundled_bin = root.join("rust/hyperx-vm/.bin/bin/qemu-system-x86_64");
        let bin = if bundled_bin.exists() {
            bundled_bin
        } else {
            PathBuf::from("qemu-system-x86_64")
        };

        let lib_dir = root.join("rust/hyperx-vm/.bin/lib/qemu");

        let test_args = |args: &[&str]| -> bool {
            let mut cmd = Command::new(&bin);
            if lib_dir.exists() {
                cmd.env("QEMU_MODULE_DIR", &lib_dir);
                cmd.env("LD_LIBRARY_PATH", &lib_dir);
            }
            cmd.args(args);
            cmd.output().map(|o| o.status.success()).unwrap_or(false)
        };

        // Test SDL OpenGL (virtio-vga-gl)
        if test_args(&["-device", "virtio-vga-gl", "-display", "sdl,gl=on", "-help"]) {
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
        if test_args(&["-device", "virtio-vga-gl", "-display", "gtk,gl=on", "-help"]) {
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
