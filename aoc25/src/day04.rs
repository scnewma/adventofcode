use std::collections::HashSet;

use crate::DELTAS8;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let rolls = parse_input(input);

    let mut accessible_rolls = 0;
    for (r, c) in rolls.iter() {
        let count: usize = DELTAS8
            .into_iter()
            .map(|(dr, dc)| {
                let npos = (r + dr, c + dc);
                if rolls.contains(&npos) { 1 } else { 0 }
            })
            .sum();
        if count < 4 {
            accessible_rolls += 1;
        }
    }
    Ok(accessible_rolls)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut rolls = parse_input(input);

    let mut rolls_removed = 0;
    loop {
        let mut rolls_removed_turn = 0;

        for (r, c) in rolls.clone().iter() {
            let count: usize = DELTAS8
                .into_iter()
                .map(|(dr, dc)| {
                    let npos = (r + dr, c + dc);
                    if rolls.contains(&npos) { 1 } else { 0 }
                })
                .sum();
            if count < 4 {
                rolls.remove(&(*r, *c));
                rolls_removed_turn += 1;
            }
        }

        if rolls_removed_turn == 0 {
            break;
        }
        rolls_removed += rolls_removed_turn;
    }

    Ok(rolls_removed)
}

fn parse_input(input: &str) -> HashSet<(isize, isize)> {
    let mut grid = HashSet::new();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '@' {
                grid.insert((r as isize, c as isize));
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day04.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1367, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(9144, ans);
    }
}
