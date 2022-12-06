use anyhow::Context;
use std::{collections::VecDeque, str::FromStr};

pub(crate) fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

fn part01(input: &str) -> String {
    solve(input, Model::CM9000)
}

fn part02(input: &str) -> String {
    solve(input, Model::CM9001)
}

fn solve(input: &str, model: Model) -> String {
    let (current, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = current.parse::<Stacks>().unwrap();
    moves
        .lines()
        .filter_map(|s| s.parse::<Move>().ok())
        .for_each(|m| stacks.perform(&m, &model));
    stacks
        .0
        .into_iter()
        .filter_map(|mut stk| stk.pop_back())
        .collect::<String>()
}

enum Model {
    CM9000,
    CM9001,
}

#[derive(Debug)]
struct Stacks(Vec<VecDeque<char>>);

impl Stacks {
    fn perform(&mut self, m: &Move, model: &Model) {
        match model {
            Model::CM9000 => {
                for _ in 0..m.amt {
                    let crt = self.0[m.from - 1].pop_back().unwrap();
                    self.0[m.to - 1].push_back(crt);
                }
            }
            Model::CM9001 => {
                // I'm sure there is a more efficient way to do this, but I'm running short on time
                // atm. :)
                let mut stk = VecDeque::new();
                for _ in 0..m.amt {
                    let crt = self.0[m.from - 1].pop_back().unwrap();
                    stk.push_back(crt);
                }
                for _ in 0..m.amt {
                    self.0[m.to - 1].push_back(stk.pop_back().unwrap());
                }
            }
        }
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = Vec::new();
        let mut lines = s.lines().rev();
        // initialize stacks with correct size
        for _ in 0..lines.next().unwrap().chars().skip(1).step_by(4).count() {
            stacks.push(VecDeque::new());
        }

        for ln in lines {
            // skip initial whitespace
            for (idx, ch) in ln.chars().skip(1).step_by(4).enumerate() {
                if ch != ' ' {
                    stacks[idx].push_back(ch);
                }
            }
        }

        Ok(Stacks(stacks))
    }
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amt: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        // skip "move"
        words.next();
        let amt = words.next().context("malformed")?.parse::<usize>()?;
        // skip "from"
        words.next();
        let from = words.next().context("malformed")?.parse::<usize>()?;
        // skip "to"
        words.next();
        let to = words.next().context("malformed")?.parse::<usize>()?;
        Ok(Move { from, to, amt })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/5.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/5.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!("CMZ", ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!("WSFTMRHPP", ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!("MCD", ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!("GSLCMFBRP", ans);
    }
}
