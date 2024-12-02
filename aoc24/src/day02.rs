use arrayvec::ArrayVec;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const WIDTH: usize = 10;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(parse_input(input).filter(|v| check(&v)).count())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(parse_input(input)
        .filter(|v| {
            (0..v.len()).any(|i| {
                let mut v = v.clone();
                v.remove(i);
                check(&v)
            })
        })
        .count())
}

fn check(v: &[usize]) -> bool {
    let mut inc = true;
    let mut dec = true;
    let mut bounds = true;
    for (a, b) in v.iter().tuple_windows() {
        inc &= a < b;
        dec &= a > b;
        bounds &= a.abs_diff(*b) <= 3;
    }
    (inc || dec) && bounds
}

fn parse_input(input: &str) -> impl Iterator<Item = ArrayVec<usize, WIDTH>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day02.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(390, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(439, ans);
    }
}
