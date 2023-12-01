/// Run and pretty-print a day's solution.
/// 
/// In order, the parameters are:
/// - Your solution type.
/// - The day to solve.
#[macro_export]
macro_rules! solve {
    ($sols:ty, $day:expr) => {
        <$sols as ::lib_aoc::Solution<$day>>::run();
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
            <$sols as ::lib_aoc::Solution<N>>::run();
        })
    };
}

/// Generates an array of solution closures. Useful if you'd like to defer
/// picking which solution to execute until runtime.
/// 
/// Example usage:
/// ``` ignore
/// fn main() {
///     let sol_arr = solution_array!(Solutions, 25);
///     let target = std::env::var("SOLUTION")
///         .unwrap()
///         .parse::<usize>()
///         .unwrap();
/// 
///     sol_arr[target - 1]();
/// }
/// ```
#[macro_export]
macro_rules! solution_array {
    ($sols:ty, $up_to:literal) => {
        ::lib_aoc::seq!(N in 1..=$up_to {
            [
                #(
                || { <$sols as ::lib_aoc::Solution<N>>::run(); },
                )*
            ]
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
                let input = <$sols as ::lib_aoc::Solver>::load_test($day, PART_ONE);
                let parsed = <$sols as ::lib_aoc::Solution<$day>>::parse(&input);
                let outcome = <$sols as ::lib_aoc::Solution<$day>>::part_one(&parsed);
                assert_eq!(outcome, expected);
            }

            #[test]
            fn part_two() {
                let expected = <$sols as ::lib_aoc::Test<$day>>::expected(true);
                let input = <$sols as ::lib_aoc::Solver>::load_test($day, PART_TWO);
                let parsed = <$sols as ::lib_aoc::Solution<$day>>::parse(&input);
                let outcome = <$sols as ::lib_aoc::Solution<$day>>::part_two(&parsed);
                assert_eq!(outcome, expected);
            }
        }
    };
}