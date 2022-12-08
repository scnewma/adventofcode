use crate::SolveInfo;
use itertools::{iproduct, Either};
use take_until::TakeUntilExt;

pub(crate) fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

fn part01(input: &str) -> usize {
    #[derive(Clone)]
    struct Tree {
        height: u32,
        visible: bool,
    }
    let mut grid: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| Tree {
                    height: ch.to_digit(10).unwrap(),
                    visible: false,
                })
                .collect()
        })
        .collect();
    let (height, width) = (grid.len(), grid[0].len());

    for (dr, dc) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        // tree heights is used as a memo for what we have seen previously in our grid iteration.
        // it tracks the greatest seen height in this row / col. if any tree in this row / col is
        // greater than the current tree's height then it isn't visible from the edge.
        let mut tree_heights = grid.clone();

        // if we are checking to the right or bottom then we need to iterate bottom-right to
        // top-left instead of top-left to bottom-right because the memo needs to be built in that
        // order to be useful
        let should_reverse = dr == 1 || dc == 1;
        let points = if should_reverse {
            Either::Left(iproduct!((0..height).rev(), (0..width).rev()))
        } else {
            Either::Right(iproduct!(0..height, 0..width))
        };
        for (row, col) in points {
            let mut tree = grid[row].get_mut(col).unwrap();
            tree.visible = tree.visible
                || !(1..height - 1).contains(&row)
                || !(1..width - 1).contains(&col)
                || tree_heights[add(row, dr)][col].height < tree.height
                || tree_heights[row][add(col, dc)].height < tree.height;

            let mut max_height = tree.height;
            if (1..height - 1).contains(&row) {
                max_height = max_height.max(tree_heights[add(row, dr)][col].height);
            }
            if (1..width - 1).contains(&col) {
                max_height = max_height.max(tree_heights[row][add(col, dc)].height);
            }
            tree_heights[row][col].height = max_height;
        }
    }

    // count visible trees
    grid.into_iter()
        .flatten()
        .filter(|tree| tree.visible)
        .count()
}

fn part02(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    // brute force solution, i'm sure there is probably a memo solution
    let mut max = 0;
    for (row, col) in iproduct!(0..grid.len(), 0..grid[0].len()) {
        let left = (0..col)
            .rev()
            .take_until(|ncol| grid[row][col] <= grid[row][*ncol])
            .count();
        let right = (col + 1..grid[0].len())
            .take_until(|ncol| grid[row][col] <= grid[row][*ncol])
            .count();
        let top = (0..row)
            .rev()
            .take_until(|nrow| grid[row][col] <= grid[*nrow][col])
            .count();
        let bot = (row + 1..grid.len())
            .take_until(|nrow| grid[row][col] <= grid[*nrow][col])
            .count();

        max = max.max(left * right * top * bot);
    }
    max
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.unsigned_abs() as usize
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
