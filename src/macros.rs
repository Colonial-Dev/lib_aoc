/// Run and pretty-print a day's solution.
/// 
/// In order, the parameters are:
/// - Your solution type.
/// - The day to solve.
#[macro_export]
macro_rules! solve {
    ($sols:ty, $day:expr) => {
        <$sols as ::lib_aoc::Solution<$day>>::run(false);
    };
}

/// Run and pretty-print the solutions for all days in the range `1..=N`.
/// 
/// In order, the parameters are:
/// - Your solution type.
/// - The day to solve through. Must be an integer literal due to macro
/// limitations.
/// 
/// Trying to solve through a range with unimplemented solutions will result
/// in a compilation error.
#[macro_export]
macro_rules! solve_through {
    ($sols:ty, $up_to:literal) => {
        ::lib_aoc::seq!(N in 1..=$up_to {
            <$sols as ::lib_aoc::Solution<N>>::run(false);
        })
    };
}

/// Derive test cases for a day's solution.
/// 
/// In order, the parameters are:
/// - Your solution type.
/// - The day to generate test cases for.
/// 
/// This expands out into a new module called `tests`, which can cause
/// conflicts if you want to derive tests for multiple solutions in the same file.
/// Consider breaking up your solutions into separate modules if you encounter this issue.
#[macro_export]
macro_rules! derive_tests {
    ($sols:ty, $day:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn part_one() {
                let expected = <$sols as ::lib_aoc::Test<$day>>::expected(false);
                let outcome = <$sols as ::lib_aoc::Solution<$day>>::run(true).part_one;
                assert_eq!(outcome, Some(expected));
            }

            #[test]
            fn part_two() {
                let expected = <$sols as ::lib_aoc::Test<$day>>::expected(true);
                let outcome = <$sols as ::lib_aoc::Solution<$day>>::run(true).part_two;
                assert_eq!(outcome, Some(expected));
            }
        }
    };
}