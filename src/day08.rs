use std::collections::HashMap;

use crate::SolveInfo;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

fn part01(input: &str) -> usize {
    // build a point map
    let mut trees = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, tree) in line.chars().enumerate() {
            trees.insert((row, col), tree.to_digit(10).unwrap());
            width = col;
        }
        height = row;
    }

    let mut visible = HashMap::new();

    // top-left to bottom-right
    let deltas: [(i32, i32); 2] = [(-1, 0), (0, -1)];
    for (dr, dc) in deltas.into_iter() {
        let mut tree_heights = trees.clone();
        for row in 0..=height {
            for col in 0..=width {
                let point = (row, col);
                let h = *trees.get(&point).unwrap();
                let is_visible = *visible.get(&point).unwrap_or(&false);
                let is_visible = is_visible
                    || row == 0
                    || col == 0
                    || *tree_heights.get(&(add(row, dr), col)).unwrap() < h
                    || *tree_heights.get(&(row, add(col, dc))).unwrap() < h;
                visible.insert(point, is_visible);

                let maxh = [
                    Some(h),
                    if row == 0 {
                        None
                    } else {
                        tree_heights.get(&(add(row, dr), col)).cloned()
                    },
                    if col == 0 {
                        None
                    } else {
                        tree_heights.get(&(row, add(col, dc))).cloned()
                    },
                ]
                .into_iter()
                .filter_map(|o| o)
                .max()
                .unwrap();
                tree_heights.insert(point, maxh);
            }
        }
    }

    // bottom-right to top-left
    let deltas: [(i32, i32); 2] = [(1, 0), (0, 1)];
    for (dr, dc) in deltas.into_iter() {
        let mut tree_heights = trees.clone();
        for row in (0..=height).rev() {
            for col in (0..=width).rev() {
                let point = (row, col);
                let h = *trees.get(&point).unwrap();
                let is_visible = *visible.get(&point).unwrap_or(&false);
                let is_visible = is_visible
                    || row == height
                    || col == width
                    || *tree_heights.get(&(add(row, dr), col)).unwrap() < h
                    || *tree_heights.get(&(row, add(col, dc))).unwrap() < h;
                visible.insert(point, is_visible);

                let maxh = [
                    Some(h),
                    tree_heights.get(&(add(row, dr), col)).cloned(),
                    tree_heights.get(&(row, add(col, dc))).cloned(),
                ]
                .into_iter()
                .filter_map(|o| o)
                .max()
                .unwrap();
                tree_heights.insert(point, maxh);
            }
        }
    }

    visible.into_iter().filter(|(_, vis)| *vis).count()
}

fn part02(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut max = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let mut left = 0;
            for i in (0..col).rev() {
                left += 1;
                if grid[row][col] <= grid[row][i] {
                    break;
                }
            }
            let mut right = 0;
            for i in col + 1..grid[0].len() {
                right += 1;
                if grid[row][col] <= grid[row][i] {
                    break;
                }
            }
            let mut top = 0;
            for i in (0..row).rev() {
                top += 1;
                if grid[row][col] <= grid[i][col] {
                    break;
                }
            }
            let mut bot = 0;
            for i in row + 1..grid.len() {
                bot += 1;
                if grid[row][col] <= grid[i][col] {
                    break;
                }
            }

            max = max.max(left * right * top * bot);
        }
    }
    max
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.abs() as usize
    } else {
        u + i as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/8.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/8.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(21, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(1785, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(8, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(345168, ans);
    }
}
