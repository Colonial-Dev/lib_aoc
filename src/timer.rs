use std::time::{Instant, Duration};

use colored::Colorize;

pub struct Timer {
    start: Instant,
    prev: Instant,
    buffer: Vec<(&'static str, Duration)>
}

impl Timer {
    pub(crate) fn new() -> Self {
        let buffer = Vec::with_capacity(4);
        let init = Instant::now();
        Self { 
            start: init,
            prev: init,
            buffer
        }
    }

    pub(crate) fn mark(&mut self, label: &'static str) {
        self.buffer.push((
            label,
            self.prev.elapsed()
        ));
        self.prev = Instant::now();
    }

    pub(crate) fn mark_total(&mut self, label: &'static str) {
        self.buffer.push((
            label,
            self.start.elapsed()
        ));
    }
}

impl std::fmt::Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = &self.buffer
            .iter()
            .map(|(label, time)| format!(
                "{}: {} μs / {} ns\n",
                label.bold(),
                time.as_micros().to_string().green(),
                time.as_nanos().to_string().green()
            ))
            .collect::<String>();
        
        write!(f, "{}", output.trim())?;
        Ok(())
    }
}