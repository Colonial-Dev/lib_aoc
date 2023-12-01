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
//!     fn load(day: u8) -> String {
//!         std::fs::read_to_string(format!("src/inputs/{day:02}.txt"))
//!             .expect("Puzzle input could not be read.")
//!     }
//!     
//!     // Note that a test loading implementation can be elided if one is not desired;
//!     // the default implementation will simply panic.
//!     fn load_test(day: u8, part: bool) -> String {
//!         std::fs::read_to_string(format!("src/inputs/test_{day:02}.txt"))
//!             .expect("Puzzle input could not be read.")
//!     }
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
//! --- BENCH (RELEASE) ---
//! Parsing: 20 ns
//! Part 1: 20 ns
//! Part 2: 20 ns
//! Total: 60 ns
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
//!     fn part_one(input: &Self::Input<'_>) -> Self::Output {
//!         input.iter()
//!             .sum::<u64>()
//!     }
//!
//!     fn part_two(input: &Self::Input<'_>) -> Self::Output {
//!         input.iter()
//!             .map(|x| x.pow(2) )
//!             .sum::<u64>()
//!     }
//! }
//! ```
//! As you can see, the signatures of the solver methods are identical apart from their names - they take
//! a shared reference to a value of type [`Input`](Solution::Input) and return an [`Output`](Solution::Output). 
//! 
//! The default implementations of these methods *panic*, which (by using [`std::panic::catch_unwind`]) is how `lib_aoc` 
//! knew to display `unimplemented` when the program was run earlier. By overriding them with implementations that 
//! *don't* panic and instead return a proper value, the result will be displayed instead:
//! ``` shell
//! --- DAY 1 ---
//! Part 1: 2506
//! Part 2: 95843
//! 
//! --- BENCH (RELEASE) ---
//! Parsing: 7.223 µs
//! Part 1: 73.838 µs
//! Part 2: 81.042 µs
//! Total: 162.244 µs
//! ```
//! And that's it - you've implemented a solution!
//! 
//! ## Deriving Tests
//! Because Advent of Code provides a test case in the description of every problem, `lib_aoc` also comes with a macro for 
//! deriving tests from your [`Solution`] implementations.
//! 
//! Assuming your [`load_test`](Solver::load_test) implementation already correctly loads test cases, all you need to do is implement 
//! the [`Test`] trait on your solution to provide the expected results:
//! ``` ignore
//! impl Test<DAY_01> for Solutions {
//!     fn expected(part: bool) -> Self::Output {
//!         // If you don't know the expected result for a part yet, you can just
//!         // substitute a panicking macro.
//!         match part {
//!             // PART_ONE and PART_TWO are constants from the prelude.
//!             PART_ONE => 24_000,
//!             PART_TWO => 34_000
//!         }
//!     }
//! } 
//! ```
//! Then you can invoke the [`derive_tests`] macro to auto-generate the tests:
//! ``` ignore
//! derive_tests!(Solutions, DAY_01);
//! ```
//! This expands into a new module with a test function for each part of the solution, and can be run normally via `cargo test`.
//! 
//! ## Notes on Benchmarking
//! `lib_aoc` provides basic benchmarking of solution implementations via [`std::time::Instant`]. While the
//! measurements it provides are good approximations, a crate like [`criterion`](https://docs.rs/criterion/latest/criterion/)
//! is a better choice if you want a more rigorous solution.
//! 
//! Also note that execution clock is started *after* your [`Solver::load`] implementation returns, 
//! immediately before [`Solution::parse`] is invoked. This means the time spent loading the puzzle input is not considered
//! by the benchmark.
//! 
//! ## Additional Customization Options
//! By overriding the [`Solver::display`] and [`Solver::finalize`] methods, it's possible to define custom behavior
//! that is invoked once a solution finishes executing in a non-test context.
//! 
//! [`display`](Solver::display) has a default implementation that pretty-prints the solution outcome,
//! while [`finalize`](Solver::finalize) defaults to a no-op. Both methods take a shared reference to an [`Outcome<impl Display>`].
//! 
//! Want to add some awesome extra behavior like submitting your solution to AoC right from the command line? You can do that here!

mod macros;
mod outcome;
mod timer;

mod constants {
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
    pub const PART_ONE: bool = false;
    pub const PART_TWO: bool = true;
}

/// Library prelude; glob-import to bring all important items into scope.
pub mod prelude {
    pub use crate::{solve, solve_through, solution_array, derive_tests};
    pub use crate::outcome::{Outcome, Timings};
    pub use crate::{Solution, Solver, Test};
    pub use crate::constants::*;
}

// This re-export is necessary for the solve_through! macro to work.
#[doc(hidden)]
pub use seq_macro::seq;

use std::{
    fmt::{Display, Debug},
    panic::{self, UnwindSafe, RefUnwindSafe}
};

use outcome::Outcome;
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
    type Input<'i>: RefUnwindSafe;
    /// The type representing the puzzle's solution.
    type Output: Display;

    /// Parse textual puzzle input into a value of type [`Input`](Solution::Input).
    fn parse(puzzle: &str) -> Self::Input<'_>;

    /// Compute the solution to part one of the problem.
    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        panic::panic_any(Unimplemented {})
    }

    /// Compute the solution to part two of the problem.
    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        panic::panic_any(Unimplemented {})
    }

    /// Execute the solution from start to finish. This method
    /// handles wiring everything together and should not be overriden.
    fn run() -> Outcome<Self::Output> {
        let puzzle = Self::load(DAY);
        let mut timer = Timer::new();

        let input = Self::parse(&puzzle);
        timer.mark("Parsing");

        let part_one = catch_unimplemented(|| Self::part_one(&input));
        timer.mark("Part 1");

        let part_two = catch_unimplemented(|| Self::part_two(&input));
        timer.mark("Part 2");
        timer.mark_total("Total");

        let outcome = Outcome {
            part_one,
            part_two,
            timings: timer.into(),
            day: DAY
        };

        Self::display(&outcome);
        Self::finalize(&outcome);
        outcome
    }
}

/// Marker struct used to indicate panics triggered by unimplemented solutions.
struct Unimplemented {}

fn catch_unimplemented<T, F>(operation: F) -> Option<T> where 
    T: Display,
    F: Fn() -> T + UnwindSafe
{   
    // Install a custom panic hook that surpresses output *only* for
    // panics generated by unimplemented solutions, indicated by a payload
    // of the unit type.
    let noisy_hook = panic::take_hook();
    panic::set_hook(
        Box::new(move |panic| {
            match panic.payload().downcast_ref::<Unimplemented>()
            {
                Some(_) => (),
                None => noisy_hook(panic)
            }
        })
    );

    let outcome = match panic::catch_unwind(operation) {
        Ok(result) => Some(result),
        Err(panic) => {
            match panic.downcast_ref::<Unimplemented>()
            {
                Some(_) => None,
                None => {
                    let _ = panic::take_hook();
                    panic::resume_unwind(panic);
                }
            }
        }
    };

    let _ = panic::take_hook();
    outcome
}

/// Interface for testing Advent of Code puzzle solutions.
/// 
/// See [the getting started guide](crate) for more information.
#[allow(unused_variables)]
pub trait Test<const DAY: u8> : Solution<DAY> {
    /// Provides the expected results for the official test case.
    /// 
    /// The default implementation of this method panics, so it should
    /// overriden with the actual expected values for tests to work.
    fn expected(part: bool) -> Self::Output {
        panic!("Expected inputs not provided.")
    }
}

/// Interface for running Advent of Code puzzle solutions.
/// 
/// See [the getting started guide](crate) for more information.
#[allow(unused_variables)]
pub trait Solver {
    /// Load the full puzzle input for the specified day.
    fn load(day: u8) -> String;

    /// Load the test puzzle input for the specified day and (optionally) part.
    /// 
    /// The default implementation of this method panics;
    /// override it if you intend to use `lib_aoc`'s testing
    /// features.
    fn load_test(day: u8, part: bool) -> String {
        panic!("Test loading has not been implemented.")
    }

    /// Callback executed after puzzle completion.
    /// 
    /// The default implementation of this method is a no-op;
    /// override it to add custom behavior.
    fn finalize(outcome: &Outcome<impl Display>) {

    }

    /// Callback executed after puzzle completion to print the outcome.
    /// 
    /// The default implementation of this method pretty-prints the outcome.
    fn display(outcome: &Outcome<impl Display>) {
        print!("{outcome}")
    }
}

/// Wrapper enum for problems with answer types that differ between parts.
/// 
/// Example usage:
/// ``` no_run
/// # use lib_aoc::prelude::*;
/// # struct Solutions {}
/// # impl Solver for Solutions {
/// #   fn load(day: u8) -> String { panic!() }
/// #   fn load_test(day: u8, part: bool) -> String { panic!() }
/// # }
/// use lib_aoc::Split;
/// 
/// impl Solution<DAY_01> for Solutions {
///     type Input<'i> = usize;
///     type Output = Split<usize, String>;
///     
///     fn parse(puzzle: &str) -> Self::Input<'_> {
///         puzzle.parse::<usize>().unwrap()
///     }
/// 
///     fn part_one(input: &Self::Input<'_>) -> Self::Output {
///         Split::P1(*input)
///     }
/// 
///     fn part_two(input: &Self::Input<'_>) -> Self::Output {
///         Split::P2(input.to_string())
///     }
/// }
#[derive(Debug, PartialEq, Eq)]
pub enum Split<A, B> where
    A: Display + Debug,
    B: Display + Debug
{
    P1(A),
    P2(B)
}

impl<A, B> Display for Split<A, B> where
    A: Display + Debug,
    B: Display + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Split::P1(value) => write!(f, "{value}"),
            Split::P2(value) => write!(f, "{value}")
        }
    }
}