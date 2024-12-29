use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (min, max) = parse_input(input);
    Ok((min..=max)
        .filter(|n| {
            let digits = digits(*n);
            is_sorted(digits) && counts(digits).iter().any(|c| *c >= 2)
        })
        .count())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (min, max) = parse_input(input);
    Ok((min..=max)
        .filter(|n| {
            let digits = digits(*n);
            is_sorted(digits) && counts(digits).iter().any(|c| *c == 2)
        })
        .count())
}

#[inline]
fn digits(n: usize) -> [usize; 6] {
    [
        n % 10,
        n / 10 % 10,
        n / 100 % 10,
        n / 1000 % 10,
        n / 10000 % 10,
        n / 100000 % 10,
    ]
}

#[inline]
fn is_sorted(digits: [usize; 6]) -> bool {
    digits[0] <= digits[1]
        && digits[1] <= digits[2]
        && digits[2] <= digits[3]
        && digits[3] <= digits[4]
        && digits[4] <= digits[5]
}

#[inline]
fn counts(digits: [usize; 6]) -> [usize; 9] {
    let mut counts = [0usize; 9];
    counts[digits[0]] += 1;
    counts[digits[1]] += 1;
    counts[digits[2]] += 1;
    counts[digits[3]] += 1;
    counts[digits[4]] += 1;
    counts[digits[5]] += 1;
    counts
}

fn parse_input(input: &str) -> (usize, usize) {
    input
        .trim()
        .split('-')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day04.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1246, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(814, ans);
    }
}
