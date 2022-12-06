use clap::Parser;
use std::{fs, path::Path};

const TEMPLATE: &str = r###"use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

fn part01(input: &str) -> u32 {
    0
}

fn part02(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/::DAY::.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/::DAY::.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(0, ans);
    }
}
"###;

#[derive(Parser)]
struct Cli {
    day: usize,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let fp = Path::new("src").join(format!("day{:02}.rs", cli.day));
    if fp.exists() {
        anyhow::bail!("{} already exists!", fp.to_string_lossy())
    }

    fs::write(fp, TEMPLATE.replace("::DAY::", &cli.day.to_string()))?;
    fs::File::create(Path::new("inputs").join(format!("{}.sample.txt", cli.day)))?;
    fs::File::create(Path::new("inputs").join(format!("{}.input.txt", cli.day)))?;

    let fp_main = Path::new("src").join("main.rs");
    let contents = fs::read_to_string(&fp_main)?;
    let contents = contents.replace(
        "// GENERATE MODULE HEADER",
        &format!("mod day{:02};\n// GENERATE MODULE HEADER", cli.day),
    );
    let contents = contents.replace(
        "// GENERATE DAY FUNCTION",
        &format!("day{:02}::run,\n        // GENERATE DAY FUNCTION", cli.day),
    );
    fs::write(&fp_main, contents)?;

    Ok(())
}
