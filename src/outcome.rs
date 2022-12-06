use std::fmt::Debug;
use colored::{Colorize, ColoredString};

use crate::Timer;

/// Represents the final product of a [`Solution`](crate::Solution).
pub struct Outcome<T: Debug> {
    /// The computed answer to part one, if any.
    pub part_one: Option<T>,
    /// The computer answer to part two, if any.
    pub part_two: Option<T>,
    /// Internal benchmark timing buffer.
    pub(crate) timer: Timer,
    /// The day of the source [`Solution`](crate::Solution).
    pub day: u8,
}

impl<T: Debug> std::fmt::Display for Outcome<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "--- DAY {} ---", self.day.to_string().bright_cyan().bold())?;
        writeln!(f, "{}: {}", "Part 1".bold(), format_answer(&self.part_one))?;
        writeln!(f, "{}: {}", "Part 2".bold(), format_answer(&self.part_two))?;

        let opt_target = match cfg!(debug_assertions) {
            true => "(DEBUG)".yellow().bold(),
            false => "(RELEASE)".green().bold()
        };

        writeln!(f, "\n--- BENCH {opt_target} ---\n{}", self.timer)?;

        Ok(())
    }
}

fn format_answer(ans: &Option<impl Debug>) -> ColoredString {
    match ans {
        Some(answer) => format!("{answer:?}").green(),
        None => "unimplemented".red()
    }
}