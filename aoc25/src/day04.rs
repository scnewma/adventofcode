use std::collections::HashMap;

use itertools::iproduct;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut grid = HashMap::new();
    let h = input.lines().count() as i32;
    let w = input.lines().next().unwrap().chars().count() as i32;
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid.insert((r as i32, c as i32), ch);
        }
    }

    const DELTAS: &[(i32, i32)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut accessible_rolls = 0;
    for (r, c) in iproduct!(0..h as i32, 0..w as i32) {
        if grid[&(r, c)] != '@' {
            continue;
        }

        let mut count = 0;
        for (dr, dc) in DELTAS {
            let (nr, nc) = (r + dr, c + dc);
            if nr < 0 || nr > h - 1 || nc < 0 || nc > w - 1 {
                continue;
            }
            if grid[&(nr, nc)] == '@' {
                count += 1;
            }
        }
        if count < 4 {
            accessible_rolls += 1;
        }
    }
    Ok(accessible_rolls)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day04.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
