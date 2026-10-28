use std::process::Command;

#[derive(Debug, Clone)]
pub struct AudioState {
    pub volume: f32, // 0.0 ..= 100.0
    pub muted: bool,
    pub output_device: String,
}

impl Default for AudioState {
    fn default() -> Self {
        Self::get_current()
    }
}

impl AudioState {
    pub fn get_current() -> Self {
        // Query PipeWire via wpctl first, fallback to pactl
        if let Ok(out) = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout);
                // Example: "Volume: 0.75 [MUTED]"
                let muted = s.contains("MUTED");
                let vol = s
                    .split_whitespace()
                    .nth(1)
                    .and_then(|v| v.parse::<f32>().ok())
                    .map(|v| (v * 100.0).clamp(0.0, 100.0))
                    .unwrap_or(80.0);

                return Self {
                    volume: vol,
                    muted,
                    output_device: "PipeWire HD Audio (Default Sink)".to_string(),
                };
            }
        }

        Self {
            volume: 80.0,
            muted: false,
            output_device: "PipeWire Stereo Output".to_string(),
        }
    }

    pub fn set_volume(percent: f32) {
        let clamped = percent.clamp(0.0, 100.0) as u32;
        let arg = format!("{}%", clamped);
        if Command::new("wpctl")
            .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &arg])
            .status()
            .is_err()
        {
            let _ = Command::new("pactl")
                .args(["set-sink-volume", "@DEFAULT_SINK@", &arg])
                .status();
        }
    }

    pub fn toggle_mute() {
        if Command::new("wpctl")
            .args(["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"])
            .status()
            .is_err()
        {
            let _ = Command::new("pactl")
                .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
                .status();
        }
    }
}
