use std::time::{Duration, Instant};

/// A timer for easily measuing elapsed time.
pub struct Timer {
    /// The instant the timer started at.
    start: Instant,
}

impl Timer {
    /// Construct a new timer, starting now.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Get the elapsed time since start.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Format the elapsed time in a human readable format.
    pub fn format_elapsed(&self) -> String {
        let duration = self.elapsed();
        if duration.as_micros() == 0 {
            return "<1 μs".into();
        }
        if duration.as_micros() < 1000 {
            return format!("{} μs", duration.as_micros());
        }
        if duration.as_millis() < 1000 {
            return format!("{} ms", duration.as_millis());
        }
        format!("{} s", duration.as_secs())
    }
}
