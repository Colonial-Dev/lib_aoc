#![warn(clippy::perf, clippy::style, warnings)]

//! `lib_aoc` is a simple trait-based framework for the annual [Advent of Code](https://adventofcode.com/) programming challenge.
//! 
//! Focus less on the boilerplate and more on the problem by automatically wiring up your solutions with input loading, pretty-printing
//! and granular benchmarking.
//! 
//! ## Getting Started
//! Create a new binary crate and add `lib_aoc` as a dependency.
//! ``` shell
//! $ cargo new advent_of_code && cd advent_of_code
//! $ cargo add lib_aoc
//! ```
//! Then, import the `lib_aoc` prelude and create a new struct to link your solutions.
//! ``` ignore
//! use lib_aoc::prelude::*;
//! 
//! // Can be named whatever you'd like.
//! struct Solutions {}
//! 
//! fn main() { /* ... */ }
//! ```
//! When solving a problem, you'll implement the [`Solution`] trait on this struct, and `lib_aoc` will
//! take care of connecting everything together.
//! 
//! Before you can do that, however, you'll need to implement the [`Solver`] trait on the struct, which
//! (among other, optional things) tells `lib_aoc` how you'd like puzzle inputs to be loaded. 
//! 
//! The simple approach is to just read the input from disk, but more complex approaches
//! (such as scraping the Advent of Code website directly) are certainly possible.
//! ``` ignore
//! impl Solver for Solutions {
//!     fn load(day: u8, testing: bool) -> String {
//!        let path = match testing {
//!            false => Path::new("src/inputs").join(format!("{day:02}.txt")),
//!            true => Path::new("src/inputs").join(format!("{day:02}_test.txt"))
//!        };
//! 
//!        std::fs::read_to_string(path).expect("Puzzle input could not be read.")
//!    }
//! }
//! ```
//! With [`Solver`] implemented, you can now begin solving problems!
//! 
//! ## Implementing a Solution
//! For demonstration purposes, we'll assume a simple first problem:
//! - The input is a list of integers, one per line.
//! - Part one wants the sum of all the integers.
//! - Part two wants us to square each integer, *then* sum them.
//! 
//! Start by implementing [`Solution<DAY_01>`] for your solutions struct; at minimum, you need to provide 
//! type definitions for [`Input`](Solution::Input) and [`Output`](Solution::Output), 
//! as well as an implementation of [`parse`](Solution::parse).
//! ``` ignore
//! impl Solution<DAY_01> for Solutions {
//!     type Input<'i> = Vec<u64>;
//!     type Output = u64;
//! 
//!     fn parse(puzzle: &str) -> Self::Input<'_> {
//!         puzzle
//!             .lines()
//!             .map(str::parse::<u64>())
//!             .map(Result::unwrap)
//!             .collect::<Vec<_>>() 
//!     }
//! }
//! ```
//! At this point, the solution is technically ready to be run. You can use the [`solve_through`] macro to execute
//! all solutions up to a certain day, like so:
//! ``` ignore
//! fn main() {
//!     // Notes: 
//!     // - Due to macro limitations, you must use an integer literal for the day cap.
//!     // - Trying to solve through days you haven't implemented yet is a compile error.
//!     solve_through!(Solutions, 1);
//! }
//! ```
//! Assuming your [`load`](Solver::load) implementation works, the program should output something like this:
//! ``` shell
//! --- DAY 1 ---
//! Part 1: unimplemented
//! Part 2: unimplemented
//!
//! --- BENCH (DEBUG) ---
//! Parsing: 0 μs / 133 ns
//! Part 1: 0 μs / 53 ns
//! Part 2: 0 μs / 46 ns
//! Total: 0 μs / 436 ns
//! ```
//! It looks like the actual solution logic is unimplemented! Fortunately, that's easy to fix - we just implement
//! the [`part_one`](Solution::part_one) and [`part_two`](Solution::part_two) methods.
//! ``` ignore
//! impl Solution<DAY_01> for Solutions {
//!     type Input<'i> = Vec<u64>;
//!     type Output = u64;
//! 
//!     fn parse(puzzle: &str) -> Self::Input<'_> {
//!         puzzle
//!             .lines()
//!             .map(str::parse::<u64>())
//!             .map(Result::unwrap)
//!             .collect::<Vec<_>>() 
//!     }
//! 
//!     fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
//!         input.iter()
//!             .sum::<u64>()
//!             .into()
//!     }
//!
//!     fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
//!         input.iter()
//!             .map(|x| x.pow(2) )
//!             .sum::<u64>()
//!             .into()
//!     }
//! }
//! ```
//! As you can see, the signatures of the solver methods are identical apart from their names - they take
//! a shared reference to a value of type [`Input`](Solution::Input) and return an [`Option<Output>`]. 
//! 
//! The default implementations simply return [`None`], which is how `lib_aoc` knew to display 
//! `unimplemented` when the program was run earlier. By overriding them with implementations that return [`Some`],
//! the result will be displayed instead:
//! ``` shell
//! --- DAY 1 ---
//! Part 1: 66306
//! Part 2: 195292
//!
//! --- BENCH (DEBUG) ---
//! Parsing: 533 μs / 533339 ns
//! Part 1: 0 μs / 67 ns
//! Part 2: 0 μs / 976 ns
//! Total: 534 μs / 534839 ns
//! ```
//! And that's it - you've implemented a solution!
//! 
//! ## Deriving Tests
//! Because Advent of Code provides a test case in the description of every problem, `lib_aoc` also comes with a macro for 
//! deriving tests from your [`Solution`] implementations.
//! 
//! Assuming your [`loader`](Solver::load) already correctly loads the test case instead of the full input when prompted, all you need to do is implement 
//! the [`Test`] trait on your solution to provide the expected results:
//! ``` ignore
//! impl Test<DAY_01> for Solutions {
//!     fn expected() -> (Option<Self::Output>, Option<Self::Output>) {
//!         (Some("PART_ONE_EXPECTED"), Some("PART_TWO_EXPECTED"))
//!     }
//! }
//! ```
//! Then you can invoke the [`derive_tests`] macro to auto-generate the tests:
//! ``` ignore
//! derive_tests!(Solutions, DAY_01);
//! ```
//! This expands into a new module with a test function for each part of the solution, and can be run normally via `cargo test`.

mod macros;
mod outcome;
mod timer;

mod days {
    pub const DAY_01: u8 = 1;
    pub const DAY_02: u8 = 2;
    pub const DAY_03: u8 = 3;
    pub const DAY_04: u8 = 4;
    pub const DAY_05: u8 = 5;
    pub const DAY_06: u8 = 6;
    pub const DAY_07: u8 = 7;
    pub const DAY_08: u8 = 8;
    pub const DAY_09: u8 = 9;
    pub const DAY_10: u8 = 10;
    pub const DAY_11: u8 = 11;
    pub const DAY_12: u8 = 12;
    pub const DAY_13: u8 = 13;
    pub const DAY_14: u8 = 14;
    pub const DAY_15: u8 = 15;
    pub const DAY_16: u8 = 16;
    pub const DAY_17: u8 = 17;
    pub const DAY_18: u8 = 18;
    pub const DAY_19: u8 = 19;
    pub const DAY_20: u8 = 20;
    pub const DAY_21: u8 = 21;
    pub const DAY_22: u8 = 22;
    pub const DAY_23: u8 = 23;
    pub const DAY_24: u8 = 24;
    pub const DAY_25: u8 = 25;
}

/// Library prelude; glob-import to bring all important items into scope.
pub mod prelude {
    pub use crate::*;
    pub use crate::days::*;
    pub use crate::{Solution, Solver, Test};
}

// This re-export is necessary for the solve_through! macro to work.
#[doc(hidden)]
pub use seq_macro::seq;
pub use outcome::Outcome;

use std::fmt::Debug;
use timer::Timer;

/// Implements the solution to a single Advent of Code problem.
/// 
/// Should be implemented on a marker struct (e.g. `struct Solutions {}`);
/// see [the getting started guide](crate) for more information.
#[allow(unused_variables)]
pub trait Solution<const DAY: u8> : Solver {
    /// The type representing the parsed form of the puzzle input.
    /// 
    /// The generic lifetime parameter `<'i>` can typically be elided; it is only
    /// necessary if the concrete type definition contains references to the puzzle input
    /// or other data.
    type Input<'i>;
    /// The type representing the puzzle's solution.
    type Output: Debug;

    /// Parse textual puzzle input into a value of type [`Input`](Solution::Input).
    fn parse(puzzle: &str) -> Self::Input<'_>;

    /// Compute the solution to part one of the problem.
    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        None
    }

    /// Compute the solution to part two of the problem.
    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        None
    }

    /// Execute the solution from start to finish. This method
    /// handles wiring everything together and should not be overriden.
    fn run(testing: bool) -> Outcome<Self::Output> {
        let puzzle = Self::load(DAY, testing);
        let mut timer = Timer::new();

        let input = Self::parse(&puzzle);
        timer.mark("Parsing");

        let part_one = Self::part_one(&input);
        timer.mark("Part 1");

        let part_two = Self::part_two(&input);
        timer.mark("Part 2");
        timer.mark_total("Total");

        let outcome = Outcome {
            part_one,
            part_two,
            timer,
            day: DAY
        };

        Self::display(&outcome, testing);
        Self::finalize(&outcome, testing);
        outcome
    }
}

/// Interface for testing Advent of Code puzzle solutions.
/// 
/// See [the getting started guide](crate) for more information.
pub trait Test<const DAY: u8> : Solution<DAY> {
    /// Provides the expected results for the official test case.
    /// 
    /// The default implementation of this method returns `(None, None)`,
    /// which will panic the test cases derived by [`derive_tests`], so this
    /// method must be overriden with the actual expected values for tests to work.
    fn expected() -> (Option<Self::Output>, Option<Self::Output>);
}

/// Interface for running Advent of Code puzzle solutions.
/// 
/// See [the getting started guide](crate) for more information.
pub trait Solver {
    /// Load the puzzle input matching the parameters.
    fn load(day: u8, testing: bool) -> String;

    /// Callback executed after puzzle completion.
    /// 
    /// The default implementation of this method is a no-op;
    /// override it to add custom behavior.
    #[allow(unused_variables)]
    fn finalize(outcome: &Outcome<impl Debug>, testing: bool) {

    }

    /// Callback executed after puzzle completion to print the outcome.
    /// 
    /// The default implementation of this method pretty-prints the outcome
    /// only for non-test runs.
    fn display(outcome: &Outcome<impl Debug>, testing: bool) {
        if !testing {
            print!("{outcome}")
        }
    }
}