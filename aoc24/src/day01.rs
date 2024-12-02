use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (left, right) = parse_input(input);
    Ok(left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (left, right) = parse_input(input);
    let counts = right.into_iter().counts();
    Ok(left
        .into_iter()
        .map(|l| l * counts.get(&l).unwrap_or(&0))
        .sum())
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    input.lines().map(str::split_whitespace).for_each(|mut it| {
        left.push(it.next().unwrap().parse().unwrap());
        right.push(it.next().unwrap().parse().unwrap());
    });
    left.sort();
    right.sort();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day01.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1879048, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(21024792, ans);
    }
}
