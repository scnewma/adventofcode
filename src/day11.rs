use anyhow::Context;
use itertools::process_results;
use num::integer::Integer;
use std::{cell::RefCell, cmp::Reverse, collections::VecDeque, str::FromStr};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let monkeys: Vec<Monkey> =
        process_results(input.split("\n\n").map(Monkey::from_str), |it| it.collect())?;

    simulate(20, monkeys, |current_worry| current_worry / 3)
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let monkeys: Vec<Monkey> =
        process_results(input.split("\n\n").map(Monkey::from_str), |it| it.collect())?;

    let lcm = monkeys
        .iter()
        .map(|m| m.test)
        .fold(1u64, |acc, n| acc.lcm(&n));

    simulate(10000, monkeys, |current_worry| lcm + (current_worry % lcm))
}

fn simulate<F>(rounds: usize, monkeys: Vec<Monkey>, reduce_worry: F) -> anyhow::Result<u64>
where
    F: Fn(&u64) -> u64,
{
    let monkeys: Vec<RefCell<Monkey>> = monkeys.into_iter().map(RefCell::new).collect();
    let mut inspected = vec![0; monkeys.len()];

    for _round in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            let mut monkey = monkey.borrow_mut();
            while let Some(mut item) = monkey.items.pop_front() {
                inspected[i] += 1;
                match monkey.op {
                    Op::Add(i) => item += i,
                    Op::Mult(i) => item *= i,
                    Op::Square => item *= item.clone(),
                }
                item = reduce_worry(&item);
                if &item % monkey.test == 0 {
                    monkeys[monkey.next.0].borrow_mut().items.push_back(item);
                } else {
                    monkeys[monkey.next.1].borrow_mut().items.push_back(item);
                }
            }
        }
    }
    inspected.sort_by_key(|i| Reverse(*i));
    Ok(inspected[0] * inspected[1])
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    // tests if divisible by
    test: u64,
    // indicies to throw to next for (true, false)
    next: (usize, usize),
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items: Option<VecDeque<u64>> = None;
        let mut op: Option<Op> = None;
        let mut test: Option<u64> = None;
        let mut if_true: Option<usize> = None;
        let mut if_false: Option<usize> = None;

        for line in s.lines().skip(1) {
            let (mut desc, data) = line.split_once(": ").context("malformed")?;
            desc = desc.trim_start();
            match desc.trim_start() {
                "Starting items" => {
                    items = Some(process_results(
                        data.split(", ").map(u64::from_str),
                        |it| it.collect(),
                    )?);
                }
                "Operation" => {
                    let mut words = data.split_whitespace().skip(3);
                    let operator = words.next().context("no operator")?;
                    let n = words.next().context("no rhs on expr")?;
                    op = Some(match (operator, n) {
                        ("*", "old") => Op::Square,
                        ("*", n) => Op::Mult(n.parse()?),
                        ("+", n) => Op::Add(n.parse()?),
                        _ => unreachable!(),
                    });
                }
                "Test" => {
                    test = Some(
                        data.split_whitespace()
                            .last()
                            .context("invalid test line")?
                            .parse()?,
                    );
                }
                "If true" => {
                    if_true = Some(
                        data.split_whitespace()
                            .last()
                            .context("invalid if true line")?
                            .parse()?,
                    );
                }
                "If false" => {
                    if_false = Some(
                        data.split_whitespace()
                            .last()
                            .context("invalid if false line")?
                            .parse()?,
                    );
                }
                _ => unreachable!(),
            }
        }
        Ok(Monkey {
            items: items.context("no items")?,
            op: op.context("no op")?,
            test: test.context("no test")?,
            next: (if_true.context("no true")?, if_false.context("no false")?),
        })
    }
}

#[derive(Debug)]
enum Op {
    Add(u64),
    Mult(u64),
    Square,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day11.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day11.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(10605, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(113232, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(2713310158, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(29703395016, ans);
    }
}
