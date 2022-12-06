use anyhow::Context;
use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

struct SolveInfo {
    part01: String,
    part02: String,
}

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
        format!("inputs/{}.sample.txt", day)
    } else {
        format!("inputs/{}.input.txt", day)
    };
    std::fs::read_to_string(&fname).with_context(|| format!("Reading file {}", fname))
}

fn print_solve(day: usize, solve: &SolveInfo) {
    println!("--- Day {:02} ---", day);
    println!("  Part 1: {}", solve.part01);
    println!("  Part 2: {}", solve.part02);
}
