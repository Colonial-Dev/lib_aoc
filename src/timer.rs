use std::time::{Instant, Duration};

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

    pub(crate) fn buffer(&self) -> &[(&'static str, Duration)] {
        &self.buffer
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