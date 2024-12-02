use std::collections::{HashSet, VecDeque};

use itertools::{iproduct, Itertools};
use polyfit_rs::polyfit_rs::polyfit;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let grid = parse_input(input);
    let size = grid.len();
    Ok(garden_plots_reached(
        &grid,
        (size as i32 / 2, size as i32 / 2),
        64,
    ))
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let grid = parse_input(input);
    let expanded = expand_grid(&grid, 5);
    let size = expanded.len();
    // 65 is the edge of the 1x1 grid from S
    // 196 is the edge of the 3x3 grid from S
    // 327 is the edge of the 5x5 grid from S
    let mut y_values = Vec::new();
    for n in [65, 196, 327] {
        let y = garden_plots_reached(&expanded, (size as i32 / 2, size as i32 / 2), n);
        y_values.push(y as f64);
    }
    // target is the furthest point we are going to reach.
    // assumption is that we reach the edge of a grid (not somewhere in the middle).
    let target = (26501365 - 64) / grid.len();
    let cos = polyfit(&[0f64, 1f64, 2f64], &y_values, 2).unwrap();
    // calculate the polynomial value at the target
    Ok(cos
        .iter()
        .map(|co| co.round())
        .enumerate()
        .fold(0, |acc, (i, co)| {
            acc + (co * (target as f64).powi(i as i32)) as u64
        }))
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    assert!(grid.len() == grid[0].len());
    grid
}

fn garden_plots_reached(grid: &[Vec<char>], start: (i32, i32), steps: usize) -> usize {
    let mut can_reach = HashSet::new();
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut queue = VecDeque::new();
    queue.push_back((start, steps));

    while let Some(((row, col), steps)) = queue.pop_front() {
        // if we are at an even step, we can always reach this plot because we can occilate between
        // a neighboring plot and this plot.
        // optimization to cache the plots we can reach inspired by hyper-neutrino
        if steps % 2 == 0 {
            can_reach.insert((row, col));
        }

        // out of steps
        if steps == 0 {
            continue;
        }

        for (dr, dc) in DELTAS {
            let (r, c) = (row + dr, col + dc);
            if r < 0
                || r >= grid.len() as i32
                || c < 0
                || c >= grid.len() as i32
                || grid[r as usize][c as usize] == '#'
                || seen.contains(&(r, c))
            {
                continue;
            }
            seen.insert((r, c));
            queue.push_back(((r, c), steps - 1));
        }
    }
    can_reach.len()
}

// turns a 1x1 to a NxN
fn expand_grid(grid: &[Vec<char>], n: usize) -> Vec<Vec<char>> {
    let size = grid.len();
    let mut expanded = vec![vec!['.'; size * n]; size * n];
    for i in 0..n * n {
        for (r, c) in iproduct!(0..size, 0..size) {
            let ch = if grid[r][c] == 'S' { '.' } else { grid[r][c] };
            expanded[((i / n) * size) + r][((i % n) * size) + c] = ch;
        }
    }
    expanded[(size * n) / 2][(size * n) / 2] = 'S';
    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day21.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3649, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(612941134797232, ans);
    }
}
