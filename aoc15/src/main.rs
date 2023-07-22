use std::time::{Duration, Instant};

use anyhow::Context;
use clap::Parser;

use aoc15::*;

#[derive(Parser)]
struct Cli {
    day: Option<usize>,
    #[arg(long)]
    sample: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let days = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        // day06::run,
        // day07::run,
        // day08::run,
        // day09::run,
        // day10::run,
        // day11::run,
        // day12::run,
        // day13::run,
        // day14::run,
        // day15::run,
        // day16::run,
        // day17::run,
        // day18::run,
        // day19::run,
        // day20::run,
        // day21::run,
        // day22::run,
        // day23::run,
        // day24::run,
        // day25::run,
    ];

    if let Some(day) = cli.day {
        if day > days.len() {
            anyhow::bail!("Day {} not yet solved!", day)
        }

        let start = Instant::now();
        let input = day_input(day, cli.sample)?;
        let f = days[day - 1];
        let solve = f(&input, cli.sample)?;
        print_solve(day, &solve, start.elapsed());
    } else {
        for (day, f) in days.iter().enumerate() {
            let start = Instant::now();
            let day = day + 1;
            let input = day_input(day, cli.sample)?;
            let solve = f(&input, cli.sample)?;
            print_solve(day, &solve, start.elapsed());
            println!();
        }
    }

    Ok(())
}

fn day_input(day: usize, sample: bool) -> anyhow::Result<String> {
    let fname = if sample {
        format!("aoc15/inputs/day{:02}.sample.txt", day)
    } else {
        format!("aoc15/inputs/day{:02}.input.txt", day)
    };
    std::fs::read_to_string(&fname).with_context(|| format!("Reading file {}", fname))
}

fn print_solve(day: usize, solve: &SolveInfo, duration: Duration) {
    println!("--- Day {:02} ({:?}) ---", day, duration);
    println!("  Part 1: {}", solve.part01);
    println!("  Part 2: {}", solve.part02);
}
