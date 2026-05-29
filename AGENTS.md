# AGENTS.md

For AoC I prefer to write code with the following characteristics:

- maximizes performance
- minimizes allocations
- skips some error handling robustness for ease of reading

Keep these characteristics in mind when reviewing code.

## AoC Review Style

Assume AoC inputs are valid and trusted. Do not mention lack of validation, skipped
parse errors, malformed input behavior, underflow/panic risks from impossible
puzzle states, or other robustness issues unless they affect valid puzzle input.
Focus on correctness for the puzzle, performance, allocations, and readability.

## Avoid This Feedback

Duplicate parsing between `part01` and `part02`. We are intentionally
benchmarking the full solve time, including parsing, for each part.

Allocations via `to_string()` in `run()`. Those aren't counted in the benchmark.
