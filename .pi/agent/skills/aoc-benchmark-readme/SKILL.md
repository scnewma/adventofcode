---
name: aoc-benchmark-readme
description: |
  Generate or update an Advent of Code year README benchmark table with per-day
  Criterion timings. Use when asked to update benchmarks for aocYY, update
  benchmarks for aocYY day N, create README performance measurements, benchmark
  solved AoC days/parts, or make a README similar to aoc24/README.md.
---

# AoC Benchmark README

Use this skill to generate/update `aocYY/README.md` benchmark tables from this
repo's Rust Criterion benchmarks.

Example user requests that should trigger this skill:

- "update benchmarks for aoc20"
- "update benchmarks for aoc20 day 2"
- "refresh aoc25 README performance numbers"
- "benchmark solved days for aoc23 and update the README"

## Goal

Create a README like `aoc24/README.md` with one row per implemented day:

```md
| Day | Part 1    | Part 2    |
| --- | ------    | ------    |
| 1   | 22.723 µs | 24.340 µs |
```

Benchmark command for a day:

```sh
bin/aoc -y YEAR -d DAY -b
```

`YEAR` is two digits (`20` for `aoc20`). `DAY` is an integer (`4` for
`src/day04.rs`).

## Workflow

1. Identify the target year from the user request or the crate path (`aoc25` ->
   `25`). If ambiguous, ask.
2. Identify the requested day scope:
   - `update benchmarks for aoc20` means all solved days in `aoc20`.
   - `update benchmarks for aoc20 day 2` means only `aoc20/src/day02.rs` and
     only the README row for day 2 should change.
   - Accept day formats like `day 2`, `day02`, `d2`, and `2` when the rest of
     the request is clearly about a single day.
3. Inspect the scoped `aocYY/src/dayNN.rs` files to find implemented days/parts.
4. Treat a part as unsolved if its function body is just a placeholder such as:
   - `Ok(0)`
   - `todo!()` / `unimplemented!()`
   - an ignored/unused input plus a constant zero result
5. Only run benchmarks for scoped days with at least one solved part. If one part
   is unsolved, keep that table cell as `-` and ignore any benchmark timing for
   it.
6. Run each scoped benchmark from the repo root:
   ```sh
   bin/aoc -y YY -d D -b
   ```
7. Parse Criterion's estimate line for each solved part. Use the middle value in
   the bracketed range:
   ```txt
   day04::part01           time:   [22.598 µs 22.723 µs 22.864 µs]
   day04::part02           time:   [24.287 µs 24.340 µs 24.451 µs]
   ```
   This becomes:
   ```md
   | 4   | 22.723 µs | 24.340 µs |
   ```
8. Find the current machine information before writing benchmark text. Do not
   blindly copy hardware details from another README. On macOS, use:
   ```sh
   system_profiler SPHardwareDataType | rg 'Model Name|Model Identifier|Chip|Processor|Memory'
   ```
   Summarize it in the README benchmark paragraph, e.g.
   `MacBook Air with Apple M4 and 16 GB memory`.
9. Update or create `aocYY/README.md`. Preserve the standard intro text from
   existing README files when possible, but update the hardware sentence to the
   current machine.
   - For whole-year requests, regenerate the benchmark table for all solved days.
   - For single-day requests, update only that day's row and preserve all other
     existing rows exactly when possible. If the row does not exist, insert it in
     numeric day order.
10. Run a quick verification by checking that every scoped solved part has a
    non-`-` cell, every scoped placeholder part has `-`, and the README hardware
    sentence matches the machine used for the benchmark run.

## Helpful Commands

List day files:

```sh
find aocYY/src -maxdepth 1 -name 'day*.rs' | sort
```

Find obvious placeholders:

```sh
rg 'Ok\(0\)|todo!\(|unimplemented!\(' aocYY/src/day*.rs
```

Benchmark all solved days with shell output saved for parsing:

```sh
for f in aocYY/src/day*.rs; do
  d=$(basename "$f" .rs | sed 's/day//; s/^0//')
  bin/aoc -y YY -d "$d" -b 2>&1 | tee "/tmp/aocYY-day${d}-bench.txt"
done
```

Benchmark one requested day:

```sh
bin/aoc -y YY -d D -b 2>&1 | tee "/tmp/aocYY-dayD-bench.txt"
```

A compact parser pattern for Criterion output:

```regex
^day(\d{2})::part0([12])\s+time:\s+\[[^\]]*?\s+([0-9.]+\s+(?:ns|µs|us|ms|s))\s+[^\]]+\]
```

Prefer the `µs` symbol as emitted by Criterion. Do not convert units unless the
output uses `us`.

## README Template

````md
# Advent of Code 20YY

To execute a solution for a given day, run the following from the root of the
repository:

```sh
cargo run --release -p aocYY <DAY> < <INPUT_FILE>
```

## Benchmarks

All benchmarks were ran on a MacBook Air with Apple M4 and 16 GB memory. The
time taken to parse the input is included in each part's benchmark.

Benchmarks can be ran with `cargo bench`.

| Day | Part 1    | Part 2    |
| --- | ------    | ------    |
| 1   | ...       | ...       |
````

Match the existing repo style, including the wording above, unless the user asks
for different text.
