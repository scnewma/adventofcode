use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let dir_sizes = parse_input(input)?;

    Ok(dir_sizes
        .into_iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum())
}

const TOTAL_SPACE: u32 = 70000000;
const FREE_SPACE_NEEDED: u32 = 30000000;

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let dir_sizes = parse_input(input)?;
    let free_space = TOTAL_SPACE - *dir_sizes.get("/").context("missing root directory size")?;
    let free_space_needed = FREE_SPACE_NEEDED - free_space;

    dir_sizes
        .into_iter()
        .filter(|(_, size)| *size > free_space_needed)
        .map(|(_, size)| size)
        .min()
        .context("why no min?")
}

fn parse_input(input: &str) -> anyhow::Result<HashMap<String, u32>> {
    let mut dir_sizes = HashMap::new();
    dir_sizes.insert("/".to_string(), 0);
    let mut cwd = PathBuf::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        match words.next().context("malformed line")? {
            "$" => {
                let command = words.next().context("no command after $")?;
                match command {
                    "cd" => {
                        let dir = words.next().context("no directory for cd command")?;
                        if dir == ".." {
                            cwd.pop();
                        } else {
                            cwd.push(dir);
                        }
                    }
                    "ls" => {
                        // fallthrough and collect files in next iterations
                    }
                    _ => anyhow::bail!("unknown command {}", command),
                }
            }
            "dir" => {
                // do nothing
            }
            // these are files: <size> <name>
            size => {
                let size: u32 = size.parse().context("parsing file size")?;
                let name = words.next().context("malformed: missing name")?;

                let mut filename = cwd.join(name);
                while let Some(dir) = filename.parent() {
                    dir_sizes
                        .entry(dir.to_str().unwrap().to_string())
                        .and_modify(|e| *e += size)
                        .or_insert(size);

                    filename = dir.to_owned();
                }

                cwd.to_str().context("invalid cwd name")?;
            }
        }
    }
    Ok(dir_sizes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day07.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day07.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(95437, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1453349, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(24933642, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(2948823, ans);
    }
}
