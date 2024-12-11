use arrayvec::ArrayVec;
use fxhash::FxHashMap;
use itertools::Itertools;
use num::Integer;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(solve(input, 25))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(solve(input, 75))
}

fn solve(input: &str, blinks: usize) -> usize {
    let mut stones = FxHashMap::default();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .for_each(|n| *stones.entry(n).or_default() += 1);

    for _ in 0..blinks {
        let mut updated = FxHashMap::default();

        for (stone, amt) in stones {
            if stone == 0 {
                *updated.entry(1).or_default() += amt;
                continue;
            }

            if let Some((left, right)) = split(stone) {
                *updated.entry(left).or_default() += amt;
                *updated.entry(right).or_default() += amt;
                continue;
            }

            *updated.entry(stone * 2024).or_default() += amt;
        }

        stones = updated;
    }
    stones.values().sum()
}

fn split(mut n: usize) -> Option<(usize, usize)> {
    let mut cnt = 0;
    let mut digits = ArrayVec::<usize, 64>::new();
    while n != 0 {
        let (q, digit) = n.div_mod_floor(&10);
        n = q;
        cnt += 1;
        digits.push(digit);
    }
    if cnt % 2 != 0 {
        None
    } else {
        digits.reverse();
        [&digits[..cnt / 2], &digits[cnt / 2..]]
            .into_iter()
            .map(|digits| digits.iter().fold(0, |acc, d| acc * 10 + d))
            .collect_tuple()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day11.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(211306, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(250783680217283, ans);
    }
}
