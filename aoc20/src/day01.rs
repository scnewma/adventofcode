use anyhow::Context;
use std::{cmp::Ordering, collections::HashSet};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let nums: HashSet<_> = input.lines().flat_map(str::parse::<usize>).collect();
    nums.iter()
        .find(|&n| nums.contains(&(2020 - n)))
        .map(|n| n * (2020 - n))
        .context("no solution")
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut nums: Vec<_> = input.lines().flat_map(str::parse::<usize>).collect();
    nums.sort_unstable();
    for i in 0..nums.len() {
        let (mut l, mut r) = (i + 1, nums.len() - 1);
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            match sum.cmp(&2020) {
                Ordering::Equal => return Ok(nums[i] * nums[l] * nums[r]),
                Ordering::Greater => r -= 1,
                Ordering::Less => l += 1,
            }
        }
    }
    anyhow::bail!("no solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day01.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(878724, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(201251610, ans);
    }
}
