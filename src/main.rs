use anyhow::Context;
use clap::Parser;

use adventofcode22::*;

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
        day06::run,
        day07::run,
        day08::run,
        day09::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        // GENERATE DAY FUNCTION
    ];

    if let Some(day) = cli.day {
        if day > days.len() {
            anyhow::bail!("Day {} not yet solved!", day)
        }

        let input = day_input(day, cli.sample)?;
        let f = days[day - 1];
        let solve = f(&input)?;
        print_solve(day, &solve);
    } else {
        for (day, f) in days.iter().enumerate() {
            let day = day + 1;
            let input = day_input(day, cli.sample)?;
            let solve = f(&input)?;
            print_solve(day, &solve);
            println!();
        }
    }

    Ok(())
}

fn day_input(day: usize, sample: bool) -> anyhow::Result<String> {
    let fname = if sample {
        format!("inputs/day{:02}.sample.txt", day)
    } else {
        format!("inputs/day{:02}.input.txt", day)
    };
    std::fs::read_to_string(&fname).with_context(|| format!("Reading file {}", fname))
}

fn print_solve(day: usize, solve: &SolveInfo) {
    println!("--- Day {:02} ---", day);
    println!("  Part 1: {}", solve.part01);
    println!("  Part 2: {}", solve.part02);
}
