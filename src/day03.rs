use std::collections::HashSet;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mid = line.len() / 2;
        let left: HashSet<char> = HashSet::from_iter(line[..mid].chars());
        let right: HashSet<char> = HashSet::from_iter(line[mid..].chars());
        let ch = left.intersection(&right).next().unwrap().to_owned();
        sum += calc_priority(ch);
    }
    sum
}

pub fn part02(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let a = HashSet::<char>::from_iter(chunk[0].chars());
            let b = HashSet::<char>::from_iter(chunk[1].chars());
            let c = HashSet::<char>::from_iter(chunk[2].chars());
            let ch = [b, c]
                .iter()
                .fold(a, |acc, hs| acc.intersection(hs).cloned().collect())
                .iter()
                .next()
                .unwrap()
                .to_owned();
            calc_priority(ch)
        })
        .sum()
}

fn calc_priority(ch: char) -> u32 {
    if ch.is_ascii_uppercase() {
        ch as u32 - 'A' as u32 + 27
    } else {
        ch as u32 - 'a' as u32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day03.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day03.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(157, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(8515, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(70, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(2434, ans);
    }
}
