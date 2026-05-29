# Advent of Code 2020

To execute a solution for a given day, run the following from the root of the
repository:

```sh
cargo run --release -p aoc20 <DAY> < <INPUT_FILE>
```

## Benchmarks

All benchmarks were ran on a MacBook Air with Apple M4 and 16 GB memory. The
time taken to parse the input is included in each part's benchmark.

Benchmarks can be ran with `cargo bench`.

| Day | Part 1    | Part 2    |
| --- | ------    | ------    |
| 1   | 5.7480 µs | 2.9198 µs |
| 2   | 39.942 µs | 38.441 µs |
| 3   | 3.3284 µs | 16.693 µs |
| 4   | 22.839 µs | 24.490 µs |
