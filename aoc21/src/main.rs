use std::time::{Duration, Instant};

use anyhow::Context;
use clap::Parser;

use aoc21::*;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

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
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
        // day19::run,
        // day20::run,
        // day21::run,
        // day22::run,
        // day23::run,
        // day24::run,
        // day25::run,
    ];

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
