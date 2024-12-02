use std::collections::HashSet;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(parse_input(input)
        .map(|(winning, mine)| {
            let count = winning.intersection(&mine).count();
            if count == 0 {
                0
            } else {
                2usize.pow(count as u32 - 1)
            }
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut card_copies = vec![1; input.lines().count()];
    for (i, (winning, mine)) in parse_input(input).enumerate() {
        for j in 1..=winning.intersection(&mine).count() {
            card_copies[i + j] += card_copies[i];
        }
    }
    Ok(card_copies.iter().sum())
}

fn parse_input(input: &str) -> impl Iterator<Item = (HashSet<u32>, HashSet<u32>)> + '_ {
    input.lines().map(|line| {
        let (_, info) = line.split_once(": ").unwrap();
        let (winning, mine) = info.split_once(" | ").unwrap();
        let winning: HashSet<u32> = winning
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mine: HashSet<u32> = mine
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        (winning, mine)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day04.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(20117, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(13768818, ans);
    }
}
