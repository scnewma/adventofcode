use anyhow::Context;
use std::str::FromStr;

pub(crate) fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input),
        part02: part02(input),
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
        .filter_map(|mut stk| stk.pop())
        .collect::<String>()
}

enum Model {
    CM9000,
    CM9001,
}

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn perform(&mut self, m: &Move, model: &Model) {
        unsafe {
            // unsafe to get mutable access to two DIFFERENT array indicies at same time
            let from_stk: &mut Vec<char> = &mut *(self.0.get_unchecked_mut(m.from - 1) as *mut _);
            let to_stk: &mut Vec<char> = &mut *(self.0.get_unchecked_mut(m.to - 1) as *mut _);

            let to_move = from_stk.drain(from_stk.len() - m.amt..);
            match model {
                Model::CM9000 => {
                    to_stk.extend(to_move.rev());
                }
                Model::CM9001 => {
                    to_stk.extend(to_move);
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
            stacks.push(Vec::new());
        }

        for ln in lines {
            // skip initial whitespace
            for (idx, ch) in ln.chars().skip(1).step_by(4).enumerate() {
                if ch != ' ' {
                    stacks[idx].push(ch);
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

    // #[test]
    // fn test_part_two() {
    //     let ans = part02(INPUT);
    //     assert_eq!("GSLCMFBRP", ans);
    // }
}
