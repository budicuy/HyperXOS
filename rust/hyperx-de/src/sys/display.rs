use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct DisplayState {
    pub brightness: f32, // 10.0 ..= 100.0
    pub resolution: String,
    pub refresh_rate: String,
    pub night_light: bool,
}

impl Default for DisplayState {
    fn default() -> Self {
        Self {
            brightness: Self::read_brightness(),
            resolution: "1920 x 1080 (Full HD 16:9)".to_string(),
            refresh_rate: "60 Hz".to_string(),
            night_light: false,
        }
    }
}

impl DisplayState {
    pub fn read_brightness() -> f32 {
        let backlight_dir = Path::new("/sys/class/backlight");
        if let Ok(entries) = fs::read_dir(backlight_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let cur = fs::read_to_string(path.join("brightness"))
                    .ok()
                    .and_then(|s| s.trim().parse::<f32>().ok());
                let max = fs::read_to_string(path.join("max_brightness"))
                    .ok()
                    .and_then(|s| s.trim().parse::<f32>().ok());
                if let (Some(c), Some(m)) = (cur, max) {
                    if m > 0.0 {
                        return ((c / m) * 100.0).clamp(10.0, 100.0);
                    }
                }
            }
        }
        90.0
    }

    pub fn set_brightness(percent: f32) {
        let p = percent.clamp(10.0, 100.0) as u32;
        let _ = Command::new("brightnessctl")
            .args(["set", &format!("{}%", p)])
            .status();
    }
}
