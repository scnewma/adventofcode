use std::collections::HashSet;

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    solve::<2>(input)
}

pub fn part02(input: &str) -> usize {
    solve::<10>(input)
}

fn solve<const N: usize>(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut knots = [(0, 0); N];
    visited.insert((0, 0));
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, n)| (dir, n.parse::<u32>().unwrap()))
        .for_each(|(dir, n)| {
            for _ in 0..n {
                match dir {
                    "R" => knots[0].1 += 1,
                    "L" => knots[0].1 -= 1,
                    "U" => knots[0].0 += 1,
                    "D" => knots[0].0 -= 1,
                    _ => unreachable!(),
                }

                for i in 1..knots.len() {
                    // if head is >2 away from tail in any direction move in that direction
                    let dx: i32 = knots[i - 1].1 - knots[i].1;
                    let dy: i32 = knots[i - 1].0 - knots[i].0;
                    if dx.abs() > 1 || dy.abs() > 1 {
                        knots[i] = (knots[i].0 + dy.signum(), knots[i].1 + dx.signum());
                    }

                    // we only keep track of the locations of the tail
                    if i == knots.len() - 1 {
                        visited.insert(knots[knots.len() - 1]);
                    }
                }
            }
        });

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../inputs/day09.sample.txt");
    const INPUT: &str = include_str!("../inputs/day09.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(13, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(6384, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(1, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(2734, ans);
    }
}
