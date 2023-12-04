# Advent of Code 2023 Solutions

This repository contains Rust solutions for [Advent of Code 2023](https://adventofcode.com/2023) puzzles. Each directory is a separate Cargo crate corresponding to the solution for a day and part number.

To run the solutions, [follow the getting started guide for Cargo and Rust](https://doc.rust-lang.org/cargo/getting-started/index.html). Then, run the following commands:

```
cd [directory name]
cargo run --release
```

This will build and run an optimized version of the solution.

You may also change the `input.txt` file within the directory to your puzzle input. This file is included in source control as the Rust compiler expects an `input.txt` file at that location; not including one will cause the compilation to fail.

> ℹ️ As indicated by the [Progress Tracker](#progress-tracker) table, some solutions require [installing Rust nightly](https://www.oreilly.com/library/view/rust-programming-by/9781788390637/e07dc768-de29-482e-804b-0274b4bef418.xhtml).

## Progress Tracker

This table tracks progress on released Advent of Code puzzles for 2023.

| Day and part # | Progress | Notes |
| -------------- | -------- | ----- |
| `day01a`       | ✅ Completed      |       |
| `day01b`       | ✅ Completed      |       |
| `day02a`       | ✅ Completed      |       |
| `day02b`       | ✅ Completed      |       |
| `day03a`       | ✅ Completed      | Requires Rust nightly.      |
| `day03b`       | ✅ Completed      | Requires Rust nightly.      |
| `day04a`       | ⏳ Waiting | |
| `day04b`       | ⏳ Waiting | |