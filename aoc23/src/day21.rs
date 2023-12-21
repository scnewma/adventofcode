use std::collections::HashSet;

use itertools::{iproduct, Itertools};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let (height, width) = (grid.len(), grid[0].len());

    let mut positions = Vec::new();
    for (row, col) in iproduct!(0..height, 0..width) {
        if grid[row][col] == 'S' {
            positions.push((row as i32, col as i32));
        }
    }

    for _ in 0..64 {
        let mut next_positions = HashSet::new();
        while let Some((row, col)) = positions.pop() {
            for (dr, dc) in DELTAS {
                let (r, c) = (row + dr, col + dc);
                if r < 0
                    || r >= height as i32
                    || c < 0
                    || c >= width as i32
                    || grid[r as usize][c as usize] == '#'
                {
                    continue;
                }
                next_positions.insert((r, c));
            }
        }
        positions = next_positions.into_iter().collect_vec();
    }
    Ok(positions.len())
}

pub fn part02(_input: &str) -> anyhow::Result<i64> {
    Ok(0)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const SAMPLE: &'static str = include_str!("../inputs/day21.sample.txt");
//     const INPUT: &'static str = include_str!("../inputs/day21.input.txt");

//     #[test]
//     fn test_part_one_sample() {
//         let ans = part01(SAMPLE).unwrap();
//         assert_eq!(24000, ans);
//     }

//     #[test]
//     fn test_part_one() {
//         let ans = part01(INPUT).unwrap();
//         assert_eq!(69501, ans);
//     }

//     #[test]
//     fn test_part_two_sample() {
//         let ans = part02(SAMPLE).unwrap();
//         assert_eq!(45000, ans);
//     }

//     #[test]
//     fn test_part_two() {
//         let ans = part02(INPUT).unwrap();
//         assert_eq!(202346, ans);
//     }
// }
