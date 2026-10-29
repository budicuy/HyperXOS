//! Smooth Animation & Motion Easing Engine for HyperXOS Desktop Apps
//! Pure Rust, zero extra dependencies, zero idle CPU overhead.

use std::f32::consts::TAU;

/// Cubic ease-out curve: decelerates smoothly to target
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

/// Linear interpolation between a and b
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Dynamically smoothed value with exponential easing (ideal for progress bars, gauges, meters)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SmoothValue {
    pub current: f32,
    pub target: f32,
    pub speed: f32,
}

impl SmoothValue {
    pub fn new(initial: f32, speed: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            speed,
        }
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn snap(&mut self, val: f32) {
        self.current = val;
        self.target = val;
    }

    /// Advances the smooth value by `dt` seconds. Returns `true` if still animating.
    pub fn update(&mut self, dt: f32) -> bool {
        let diff = self.target - self.current;
        if diff.abs() < 0.001 {
            self.current = self.target;
            false
        } else {
            let factor = 1.0 - (-self.speed * dt).exp();
            self.current += diff * factor;
            true
        }
    }

    pub fn is_animating(&self) -> bool {
        (self.target - self.current).abs() >= 0.001
    }
}

/// Manages cross-fade and micro-slide page/step transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionState {
    pub progress: f32,
    pub duration: f32,
    pub active: bool,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            progress: 1.0,
            duration: 0.18,
            active: false,
        }
    }
}

impl TransitionState {
    pub fn new(duration: f32) -> Self {
        Self {
            progress: 1.0,
            duration,
            active: false,
        }
    }

    pub fn start(&mut self) {
        self.progress = 0.0;
        self.active = true;
    }

    /// Advances transition timer. Returns `true` if still animating.
    pub fn update(&mut self, dt: f32) -> bool {
        if !self.active {
            return false;
        }
        self.progress += dt / self.duration.max(0.001);
        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.active = false;
            false
        } else {
            true
        }
    }

    pub fn is_animating(&self) -> bool {
        self.active && self.progress < 1.0
    }

    pub fn alpha(&self) -> f32 {
        ease_out_cubic(self.progress)
    }

    pub fn offset_y(&self, max_offset: f32) -> f32 {
        (1.0 - ease_out_cubic(self.progress)) * max_offset
    }
}

/// Breathing / pulsing phase tracker for running operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PulseState {
    pub phase: f32,
    pub speed: f32,
}

impl Default for PulseState {
    fn default() -> Self {
        Self {
            phase: 0.0,
            speed: 3.5,
        }
    }
}

impl PulseState {
    pub fn update(&mut self, dt: f32) {
        self.phase = (self.phase + self.speed * dt) % TAU;
    }

    pub fn breathing(&self, min: f32, max: f32) -> f32 {
        let s = (self.phase.sin() + 1.0) / 2.0;
        min + (max - min) * s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ease_out_cubic_boundaries() {
        assert_eq!(ease_out_cubic(0.0), 0.0);
        assert_eq!(ease_out_cubic(1.0), 1.0);
        assert_eq!(ease_out_cubic(-0.5), 0.0);
        assert_eq!(ease_out_cubic(1.5), 1.0);
        assert!(ease_out_cubic(0.5) > 0.5); // Ease out decelerates, so midpoint is > 0.5
    }

    #[test]
    fn test_smooth_value_settles_at_target() {
        let mut val = SmoothValue::new(0.0, 10.0);
        val.set_target(100.0);
        assert!(val.is_animating());

        for _ in 0..100 {
            val.update(0.016);
        }

        assert_eq!(val.current, 100.0);
        assert!(!val.is_animating());
    }

    #[test]
    fn test_transition_state_completes() {
        let mut trans = TransitionState::new(0.1);
        trans.start();
        assert!(trans.is_animating());
        assert_eq!(trans.progress, 0.0);
        assert_eq!(trans.alpha(), 0.0);

        // Advance 120ms (more than 100ms duration)
        let animating = trans.update(0.12);
        assert!(!animating);
        assert!(!trans.is_animating());
        assert_eq!(trans.progress, 1.0);
        assert_eq!(trans.alpha(), 1.0);
        assert_eq!(trans.offset_y(10.0), 0.0);
    }

    #[test]
    fn test_pulse_state_bounds() {
        let mut pulse = PulseState::default();
        for _ in 0..50 {
            pulse.update(0.016);
            let b = pulse.breathing(0.3, 1.0);
            assert!(b >= 0.299 && b <= 1.001);
        }
    }
}
