use std::time::{Duration, Instant};

use anyhow::Context;
use clap::Parser;

use aoc24::*;

#[derive(Parser)]
struct Cli {
    day: usize,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let days = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
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

    if cli.day > days.len() {
        anyhow::bail!("Day {} not yet solved!", cli.day)
    }

    let input = std::io::read_to_string(std::io::stdin()).context("could not read stdin")?;

    let start = Instant::now();
    let f = days[cli.day - 1];
    let solve = f(&input)?;
    print_solve(cli.day, &solve, start.elapsed());
    Ok(())
}

fn print_solve(day: usize, solve: &SolveInfo, duration: Duration) {
    println!("--- Day {:02} ({:?}) ---", day, duration);
    println!("  Part 1: {}", solve.part01);
    println!("  Part 2: {}", solve.part02);
}
