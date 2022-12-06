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

/// Run and pretty-print the solutions for all days between 1..N, inclusive.
/// 
/// In order, the parameters are:
/// - Your solution type.
/// - The day to solve through.
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
#[macro_export]
macro_rules! derive_tests {
    ($sols:ty, $day:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn part_one() {
                let (expected, _) = <$sols as ::lib_aoc::Test<$day>>::expected();
                if expected.is_none() { panic!("Expected input not provided!") }

                let outcome = <$sols as ::lib_aoc::Solution<$day>>::run(true).part_one;
                assert_eq!(outcome, expected);
            }

            #[test]
            fn part_two() {
                let (_, expected) = <$sols as ::lib_aoc::Test<$day>>::expected();
                if expected.is_none() { panic!("Expected input not provided!") }

                let outcome = <$sols as ::lib_aoc::Solution<$day>>::run(true).part_two;
                assert_eq!(outcome, expected);
            }
        }
    };
}