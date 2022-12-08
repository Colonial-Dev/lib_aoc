# Changelog

## v 0.6.1/0.6.2
- Fixed a (very silly) bug with the display implementation of `Timings`.

## v 0.6.0
- The `Test` trait's `expected` method now takes a boolean indicating which part is being requested and returns a single instance of `Self::Output`.
- The prelude now contains the constants `PART_ONE` and `PART_TWO` (corresponding to `false` and `true` respectively) that can be used when implementing the aformentioned `expected` method.
- The `Solver` trait's `load` method has been split into `load` and `load_test`, with each method now only taking the requested day (as a `u8`.) `load_test` has a panicking default implementation.
- `Outcome` now exposes benchmark durations via the `Timings` type.