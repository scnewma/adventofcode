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
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut grid = FxHashMap::default();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.char_indices() {
            grid.insert((r as isize, c as isize), ch);
        }
    }

    let mut sum = 0;
    let mut visited = FxHashSet::default();
    for (r, c) in iproduct!(0..height, 0..width) {
        let r = r as isize;
        let c = c as isize;

        let region_ch = grid[&(r, c)];

        let mut q = VecDeque::new();
        q.push_back((r, c));

        let mut area = 0;
        let mut perimeter = 0;
        while let Some((r, c)) = q.pop_front() {
            if grid.get(&(r, c)).is_none_or(|ch| *ch != region_ch) {
                continue;
            }
            if !visited.insert((r, c)) {
                continue;
            }
            perimeter += [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .iter()
                .filter(|&&(r, c)| grid.get(&(r, c)).is_none_or(|ch| *ch != region_ch))
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
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut grid = FxHashMap::default();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.char_indices() {
            grid.insert((r as isize, c as isize), ch);
        }
    }

    let mut sum = 0;
    let mut visited = FxHashSet::default();
    for (r, c) in iproduct!(0..height, 0..width) {
        if visited.contains(&(r as isize, c as isize)) {
            continue;
        }
        let r = r as isize;
        let c = c as isize;

        let region_ch = grid[&(r, c)];

        let mut q = VecDeque::new();
        q.push_back((r, c));

        let mut area = 0;
        let mut points = FxHashSet::default();
        let mut min_r = 0;
        let mut min_c = 0;
        let mut max_r = height as isize;
        let mut max_c = width as isize;
        while let Some((r, c)) = q.pop_front() {
            if grid.get(&(r, c)).is_none_or(|ch| *ch != region_ch) {
                continue;
            }
            if !visited.insert((r, c)) {
                continue;
            }
            points.insert((r, c));
            min_r = min_r.min(r);
            min_c = min_c.min(c);
            max_r = max_r.max(r);
            max_c = max_c.max(c);

            area += 1;
            q.push_back((r + 1, c));
            q.push_back((r - 1, c));
            q.push_back((r, c + 1));
            q.push_back((r, c - 1));
        }

        // sets of vertices (edges = count of vertices)
        let mut tlv = FxHashSet::default();
        let mut trv = FxHashSet::default();
        let mut blv = FxHashSet::default();
        let mut brv = FxHashSet::default();
        for (r, c) in iproduct!(min_r - 1..=max_r + 1, min_c - 1..=max_c + 1) {
            if points.contains(&(r, c)) {
                let top_left_corner = points.contains(&(r + 1, c + 1))
                    && !points.contains(&(r + 1, c))
                    && !points.contains(&(r, c + 1));
                if top_left_corner {
                    tlv.insert((r + 1, c + 1));
                }
                let top_right_corner = points.contains(&(r + 1, c - 1))
                    && !points.contains(&(r + 1, c))
                    && !points.contains(&(r, c - 1));
                if top_right_corner {
                    trv.insert((r + 1, c - 1));
                }
                let bot_left_corner = points.contains(&(r - 1, c + 1))
                    && !points.contains(&(r - 1, c))
                    && !points.contains(&(r, c + 1));
                if bot_left_corner {
                    blv.insert((r - 1, c + 1));
                }
                let bot_right_corner = points.contains(&(r - 1, c - 1))
                    && !points.contains(&(r - 1, c))
                    && !points.contains(&(r, c - 1));
                if bot_right_corner {
                    brv.insert((r - 1, c - 1));
                }
                continue;
            }
            let top_left_corner = points.contains(&(r + 1, c + 1))
                && points.contains(&(r + 1, c)) == points.contains(&(r, c + 1));
            if top_left_corner {
                tlv.insert((r + 1, c + 1));
            }
            let top_right_corner = points.contains(&(r + 1, c - 1))
                && points.contains(&(r + 1, c)) == points.contains(&(r, c - 1));
            if top_right_corner {
                trv.insert((r + 1, c - 1));
            }
            let bot_left_corner = points.contains(&(r - 1, c + 1))
                && points.contains(&(r - 1, c)) == points.contains(&(r, c + 1));
            if bot_left_corner {
                blv.insert((r - 1, c + 1));
            }
            let bot_right_corner = points.contains(&(r - 1, c - 1))
                && points.contains(&(r - 1, c)) == points.contains(&(r, c - 1));
            if bot_right_corner {
                brv.insert((r - 1, c - 1));
            }
        }

        let num_sides = tlv.len() + trv.len() + blv.len() + brv.len();
        sum += area * num_sides;
    }

    Ok(sum)
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
