use clap::Parser;
use std::{fs, path::Path};

const TEMPLATE: &str = r###"use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> u32 {
    0
}

pub fn part02(input: &str) -> u32 {
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
    let day = format!("day{:02}", cli.day);
    let fp = Path::new("src").join(format!("{}.rs", day));
    if fp.exists() {
        anyhow::bail!("{} already exists!", fp.to_string_lossy());
    }
    fs::write(fp, TEMPLATE.replace("::DAY::", &day))?;

    fs::File::create(Path::new("inputs").join(format!("{}.sample.txt", day)))?;
    fs::File::create(Path::new("inputs").join(format!("{}.input.txt", day)))?;

    let fp_main = Path::new("src").join("main.rs");
    let contents = fs::read_to_string(&fp_main)?;
    let contents = contents.replace(
        "// GENERATE DAY FUNCTION",
        &format!("{}::run,\n        // GENERATE DAY FUNCTION", day),
    );
    fs::write(&fp_main, contents)?;

    let fp_lib = Path::new("src").join("lib.rs");
    let contents = fs::read_to_string(&fp_lib)?;
    let contents = contents.replace(
        "// GENERATE MOD HEADER",
        &format!("pub mod {};\n// GENERATE MOD HEADER", day),
    );
    fs::write(&fp_lib, contents)?;

    Ok(())
}
