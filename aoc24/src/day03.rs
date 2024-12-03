use regex::Regex;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    Ok(re.find_iter(input).map(|m| do_multiply(m.as_str())).sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    Ok(re
        .find_iter(input)
        .fold((0, true), |(sum, enabled), m| match m.as_str() {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            _ if enabled => (sum + do_multiply(m.as_str()), enabled),
            _ => (sum, enabled),
        })
        .0)
}

fn do_multiply(s: &str) -> usize {
    let s = &s["mul(".len()..s.len() - 1];
    let (l, r) = s.split_once(',').unwrap();
    l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day03.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(187194524, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(127092535, ans);
    }
}
