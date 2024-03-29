# `lib_aoc`
`lib_aoc` is a simple trait-based framework for the annual [Advent of Code](https://adventofcode.com/) programming challenge.

Focus less on the boilerplate and more on the problem by automatically wiring up your solutions with input loading, pretty-printing
and granular benchmarking.

## Getting Started
Create a new binary crate and add `lib_aoc` as a dependency.
``` shell
$ cargo new advent_of_code && cd advent_of_code
$ cargo add lib_aoc
```
Then, import the `lib_aoc` prelude and create a new struct to link your solutions.
``` rs
use lib_aoc::prelude::*;

// Can be named whatever you'd like.
struct Solutions {}

fn main() { /* ... */ }
```
When solving a problem, you'll implement the `Solution` trait on this struct, and `lib_aoc` will
take care of connecting everything together.

Before you can do that, however, you'll need to implement the `Solver` trait on the struct, which
(among other, optional things) tells `lib_aoc` how you'd like puzzle inputs to be loaded. 

The simple approach is to just read the input from disk, but more complex approaches
(such as scraping the Advent of Code website directly) are certainly possible.
``` rs
impl Solver for Solutions {
    fn load(day: u8) -> String {
        std::fs::read_to_string(format!("src/inputs/{day:02}.txt"))
            .expect("Puzzle input could not be read.")
    }
    
    // Note that a test loading implementation can be elided if one is not desired;
    // the default implementation will simply panic.
    fn load_test(day: u8, part: bool) -> String {
        std::fs::read_to_string(format!("src/inputs/test_{day:02}.txt"))
            .expect("Puzzle input could not be read.")
    }
}
```
With `Solver` implemented, you can now begin solving problems!

## Implementing a Solution
For demonstration purposes, we'll assume a simple first problem:
- The input is a list of integers, one per line.
- Part one wants the sum of all the integers.
- Part two wants us to square each integer, *then* sum them.

Start by implementing `Solution<DAY_01>` for your solutions struct; at minimum, you need to provide 
type definitions for `Input` and `Output`, 
as well as an implementation of `parse`.
``` rs
impl Solution<DAY_01> for Solutions {
    type Input<'i> = Vec<u64>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .lines()
            .map(str::parse::<u64>())
            .map(Result::unwrap)
            .collect::<Vec<_>>() 
    }
}
```
At this point, the solution is technically ready to be run. You can use the `solve_through` macro to execute
all solutions up to a certain day, like so:
``` rs
fn main() {
    // Notes: 
    // - Due to macro limitations, you must use an integer literal for the day cap.
    // - Trying to solve through days you haven't implemented yet is a compile error.
    solve_through!(Solutions, 1);
}
```
Assuming your `load` implementation works, the program should output something like this:
``` shell
--- DAY 1 ---
Part 1: unimplemented
Part 2: unimplemented

--- BENCH (RELEASE) ---
Parsing: 20 ns
Part 1: 20 ns
Part 2: 20 ns
Total: 60 ns
```
It looks like the actual solution logic is unimplemented! Fortunately, that's easy to fix - we just implement
the `part_one` and `part_two` methods.
``` rs
impl Solution<DAY_01> for Solutions {
    type Input<'i> = Vec<u64>;
    type Output = u64;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        puzzle
            .lines()
            .map(str::parse::<u64>())
            .map(Result::unwrap)
            .collect::<Vec<_>>() 
    }

    fn part_one(input: &Self::Input<'_>) -> Self::Output {
        input.iter()
            .sum::<u64>()
            .into()
    }

    fn part_two(input: &Self::Input<'_>) -> Self::Output {
        input.iter()
            .map(|x| x.pow(2) )
            .sum::<u64>()
            .into()
    }
}
```
As you can see, the signatures of the solver methods are identical apart from their names - they take
a shared reference to a value of type `Input` and return an `Output`. 

The default implementations of these methods *panic*, which (by using `std::panic::catch_unwind`) is how `lib_aoc` knew to display `unimplemented` when the program was run earlier. By overriding them with implementations that *don't* panic and
instead return a proper value, the result will be displayed instead:
``` shell
--- DAY 1 ---
Part 1: 2506
Part 2: 95843

--- BENCH (RELEASE) ---
Parsing: 7.223 µs
Part 1: 73.838 µs
Part 2: 81.042 µs
Total: 162.244 µs
```
And that's it - you've implemented a solution!

## Deriving Tests
Because Advent of Code provides a test case in the description of every problem, `lib_aoc` also comes with a macro for 
deriving tests from your `Solution` implementations.

Assuming your `load_test` implementation already correctly loads test cases, all you need to do is implement 
the `Test` trait on your solution to provide the expected results:
``` rs
impl Test<DAY_01> for Solutions {
    fn expected(part: bool) -> Self::Output {
        // If you don't know the expected result for a part yet, you can just
        // substitute a panicking macro.
        match part {
            // PART_ONE and PART_TWO are constants from the prelude.
            PART_ONE => 24_000,
            PART_TWO => 34_000
        }
    }
} 
```
Then you can invoke the `derive_tests` macro to auto-generate the tests:
``` rs
derive_tests!(Solutions, DAY_01);
```
This expands into a new module with a test function for each part of the solution, and can be run normally via `cargo test`.

## Notes on Benchmarking
`lib_aoc` provides basic benchmarking of solution implementations via `std::time::Instant`. While the
measurements it provides are good approximations, a crate like [`criterion`](https://docs.rs/criterion/latest/criterion/)
is a better choice if you want a more rigorous solution.

Also note that execution clock is started *after* your `Solver::load` implementation returns, 
immediately before `Solution::parse` is invoked. This means the time spent loading the puzzle input is not considered
by the benchmark.

## Additional Customization Options
By overriding the `Solver::display` and `Solver::finalize` methods, it's possible to define custom behavior
that is invoked once a solution finishes executing in a non-test context.

`display` has a default implementation that pretty-prints the solution outcome,
while `finalize` defaults to a no-op. Both methods take a shared reference to an `Outcome<impl Display>`.

Want to add some awesome extra behavior like submitting your solution to AoC right from the command line? You can do that here!