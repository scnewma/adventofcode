use std::collections::HashSet;
use std::collections::VecDeque;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let grid = parse_input(input);
    let low_points = find_low_points(&grid);
    // calc risk level
    low_points
        .iter()
        .map(|point| grid[point.0][point.1] + 1)
        .sum::<u32>() as i64
}

pub fn part02(input: &str) -> i64 {
    let grid = parse_input(input);
    let low_points = find_low_points(&grid);

    let mut basin_sizes: Vec<i64> = low_points.iter().map(|p| basin_size(&grid, *p)).collect();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn nleft(&self) -> Point {
        Point(self.0 - 1, self.1)
    }

    fn nright(&self) -> Point {
        Point(self.0 + 1, self.1)
    }

    fn ntop(&self) -> Point {
        Point(self.0, self.1 - 1)
    }

    fn nbottom(&self) -> Point {
        Point(self.0, self.1 + 1)
    }
}

fn basin_size(grid: &[Vec<u32>], p: Point) -> i64 {
    let mut size = 0;

    let max_x = grid.len() - 1;
    let max_y = grid[0].len() - 1;

    let mut deq = VecDeque::from([p]);
    let mut seen: HashSet<Point> = HashSet::new();
    while let Some(point) = deq.pop_front() {
        if !seen.insert(point) || grid[point.0][point.1] == 9 {
            continue;
        }
        size += 1;

        // enqueue proper neighbors based on where we are in the grid
        let Point(x, y) = point;
        if x < max_x {
            deq.push_back(point.nright());
        }

        if x > 0 {
            deq.push_back(point.nleft());
        }

        if y < max_y {
            deq.push_back(point.nbottom());
        }

        if y > 0 {
            deq.push_back(point.ntop());
        }
    }

    size
}

fn find_low_points(input: &[Vec<u32>]) -> Vec<Point> {
    let mut low_points = Vec::new();
    let max_y = input.len() - 1;
    for y in 0..input.len() {
        let max_x = input[y].len() - 1;
        for x in 0..input[y].len() {
            let val = input[y][x];

            let low_horz = match x {
                // check right only
                0 => val < input[y][x + 1],
                // check left only
                x if x == max_x => val < input[y][x - 1],
                // check left and right
                _ => (val < input[y][x - 1]) && (val < input[y][x + 1]),
            };
            let low_vert = match y {
                // check bottom only
                0 => val < input[y + 1][x],
                // check top only
                y if y == max_y => val < input[y - 1][x],
                // check top and bottom
                _ => (val < input[y + 1][x]) && (val < input[y - 1][x]),
            };

            if low_horz && low_vert {
                low_points.push(Point(y, x));
            }
        }
    }

    low_points
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day09.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(532, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(1110780, ans);
    }
}
