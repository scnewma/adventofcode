use std::str::FromStr;

use anyhow::Context;

pub fn part01(input: &str) -> usize {
    parse_input(input)
        .filter(|(i1, i2)| Interval::fully_overlaps(i1, i2))
        .count()
}

pub fn part02(input: &str) -> usize {
    parse_input(input)
        .filter(|(i1, i2)| Interval::overlaps(i1, i2))
        .count()
}

struct Interval(u32, u32);

impl Interval {
    fn fully_overlaps(i1: &Interval, i2: &Interval) -> bool {
        (i1.0 <= i2.0 && i1.1 >= i2.1) || (i2.0 <= i1.0 && i2.1 >= i1.1)
    }

    fn overlaps(i1: &Interval, i2: &Interval) -> bool {
        let (low, high) = if i1.0 > i2.0 { (i2, i1) } else { (i1, i2) };
        low.1 >= high.0
    }
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (l, r) = input.split_once('-').context("malformed interval")?;
        Ok(Self(l.parse::<u32>()?, r.parse::<u32>()?))
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Interval, Interval)> + '_ {
    input
        .lines()
        .filter_map(|l| l.split_once(','))
        .filter_map(
            |(left, right)| match (left.parse::<Interval>(), right.parse::<Interval>()) {
                (Ok(i1), Ok(i2)) => Some((i1, i2)),
                _ => None,
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let ans = part01(include_str!("../inputs/4.sample.txt"));
        assert_eq!(2, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(include_str!("../inputs/4.input.txt"));
        assert_eq!(651, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(include_str!("../inputs/4.sample.txt"));
        assert_eq!(4, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(include_str!("../inputs/4.input.txt"));
        assert_eq!(956, ans);
    }
}
