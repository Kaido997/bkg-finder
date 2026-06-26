# TODO

## Rust Rewrite Review

### Done

- [x] Add Rust correctness tests in `tests/rust_correctness.rs`.
- [x] Add in-process Rust benchmark in `src/bin/bench_rust.rs`.
- [x] Add C benchmark harness in `bench/bench_c.c`.
- [x] Add benchmark runner in `scripts/benchmark.py`.
- [x] Change `find_combination` to return `Result<Vec<Vec<f64>>, ErrorType>`.
- [x] Change recursive success/failure return from `u8` to `bool`.

### Next

- [x] Run `cargo fmt` and keep the Rust code formatted.
- [x] Parse Rust CLI flags so `-max` and `-ex` match the C version.
- [x] Replace `dbg!` CLI output with stable user-facing stdout.
- [ ] Fix the recursive base case so block index `0` can be used safely.
- [x] Make pruning tolerant: compare `current_sum > target + EPSILON`.
- [x] Reset `upperbound_count` at the start of each public search.
- [ ] Stop cloning `self.bkg_set` on every search call.
- [ ] Split "found a combination" from "hit recursion limit" in the recursive return path.
- [ ] Add bounds checks or dynamic storage for `sub_sets`, `item`, and `exclusions`.
- [ ] Add regression tests for `0.1001`, `0.1020`, `-max`, and `-ex`.
- [ ] Re-run the benchmark with the corrected edge cases included.

### Notes

- `cargo test` passes.
- `cargo fmt -- --check` currently fails.
- `cargo clippy --all-targets --all-features -- -D warnings` currently fails on `clippy::too_many_arguments` for the recursive helper.
