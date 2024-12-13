use std::collections::VecDeque;

use fxhash::{FxHashMap, FxHashSet};
use itertools::iproduct;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, height, width) = parse_input(input);

    let mut sum = 0;
    let mut visited = FxHashSet::default();
    for (r, c) in iproduct!(0..height as isize, 0..width as isize) {
        if visited.contains(&(r, c)) {
            continue;
        }

        let region_ch = grid[&(r, c)];

        // checks if a grid[(row,col)] == region_ch
        macro_rules! is_same_region {
            ($row:expr, $col:expr) => {
                grid.get(&($row, $col)).is_some_and(|ch| *ch == region_ch)
            };
        }

        let mut q = VecDeque::new();
        q.push_back((r, c));

        let mut area = 0;
        let mut perimeter = 0;
        while let Some((r, c)) = q.pop_front() {
            if !is_same_region!(r, c) || !visited.insert((r, c)) {
                continue;
            }
            perimeter += [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .into_iter()
                .filter(|&(r, c)| !is_same_region!(r, c))
                .count();

            area += 1;
            q.push_back((r + 1, c));
            q.push_back((r - 1, c));
            q.push_back((r, c + 1));
            q.push_back((r, c - 1));
        }

        sum += area * perimeter;
    }

    Ok(sum)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (grid, height, width) = parse_input(input);

    #[derive(Hash, PartialEq, Eq)]
    enum Corner {
        TopLeft,
        TopRight,
        BotLeft,
        BotRight,
    }

    let mut sum = 0;
    let mut visited = FxHashSet::default();
    for (r, c) in iproduct!(0..height as isize, 0..width as isize) {
        if visited.contains(&(r, c)) {
            continue;
        }

        let region_ch = grid[&(r, c)];

        // checks if a grid[(row,col)] == region_ch
        macro_rules! is_same_region {
            ($row:expr, $col:expr) => {
                grid.get(&($row, $col)).is_some_and(|ch| *ch == region_ch)
            };
        }

        let mut q = VecDeque::new();
        q.push_back((r, c));

        let mut area = 0;
        let mut vertices = FxHashSet::default();

        while let Some((r, c)) = q.pop_front() {
            if !is_same_region!(r, c) || !visited.insert((r, c)) {
                continue;
            }
            let vertex_checks = [
                (Corner::TopLeft, (-1, -1)),
                (Corner::TopRight, (-1, 1)),
                (Corner::BotLeft, (1, 1)),
                (Corner::BotRight, (1, -1)),
            ];
            for (corner, (dr, dc)) in vertex_checks {
                let (nr, nc) = (r + dr, c + dc);
                let is_inner_corner =
                    is_same_region!(nr, c) && is_same_region!(r, nc) && !is_same_region!(nr, nc);
                let is_outer_corner = !is_same_region!(nr, c) && !is_same_region!(r, nc);
                if is_inner_corner || is_outer_corner {
                    vertices.insert((corner, r, c));
                }
            }
            area += 1;
            q.push_back((r + 1, c));
            q.push_back((r - 1, c));
            q.push_back((r, c + 1));
            q.push_back((r, c - 1));
        }

        sum += area * vertices.len();
    }

    Ok(sum)
}

fn parse_input(input: &str) -> (FxHashMap<(isize, isize), char>, usize, usize) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut grid = FxHashMap::default();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.char_indices() {
            grid.insert((r as isize, c as isize), ch);
        }
    }
    (grid, height, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day12.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1415378, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(862714, ans);
    }
}
