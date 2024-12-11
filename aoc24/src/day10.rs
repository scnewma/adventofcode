use std::collections::VecDeque;

use arrayvec::ArrayVec;
use fxhash::{FxHashMap, FxHashSet};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, trailheads) = parse_input(input);

    let mut res = 0;
    let mut q = VecDeque::new();
    let mut reached = FxHashSet::default();
    for (tr, tc) in trailheads {
        q.clear();
        reached.clear();

        q.push_front((tr + 1, tc, 0));
        q.push_front((tr - 1, tc, 0));
        q.push_front((tr, tc + 1, 0));
        q.push_front((tr, tc - 1, 0));

        while let Some((r, c, prev)) = q.pop_front() {
            if grid.get(&(r, c)).is_none_or(|h| *h != prev + 1) {
                continue;
            }
            if grid[&(r, c)] == 9 {
                reached.insert((r, c));
                continue;
            }

            q.push_front((r + 1, c, prev + 1));
            q.push_front((r - 1, c, prev + 1));
            q.push_front((r, c + 1, prev + 1));
            q.push_front((r, c - 1, prev + 1));
        }
        res += reached.len();
    }

    Ok(res)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (grid, trailheads) = parse_input(input);

    let mut res = 0;
    let mut q = VecDeque::new();
    let mut paths = FxHashSet::default();
    for (tr, tc) in trailheads {
        q.clear();
        paths.clear();

        let mut path = ArrayVec::<(isize, isize), 10>::new();
        path.push((tr, tc));

        q.push_front((tr + 1, tc, 0, path.clone()));
        q.push_front((tr - 1, tc, 0, path.clone()));
        q.push_front((tr, tc + 1, 0, path.clone()));
        q.push_front((tr, tc - 1, 0, path));

        while let Some((r, c, prev, mut path)) = q.pop_front() {
            if grid.get(&(r, c)).is_none_or(|h| *h != prev + 1) {
                continue;
            }

            path.push((r, c));

            if grid[&(r, c)] == 9 {
                paths.insert(path);
                continue;
            }

            q.push_front((r + 1, c, prev + 1, path.clone()));
            q.push_front((r - 1, c, prev + 1, path.clone()));
            q.push_front((r, c + 1, prev + 1, path.clone()));
            q.push_front((r, c - 1, prev + 1, path));
        }
        res += paths.len();
    }

    Ok(res)
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (FxHashMap<(isize, isize), u32>, FxHashSet<(isize, isize)>) {
    let mut grid = FxHashMap::default();
    let mut trailheads = FxHashSet::default();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.char_indices() {
            let r = r as isize;
            let c = c as isize;
            let d = ch.to_digit(10).unwrap();
            if d == 0 {
                trailheads.insert((r, c));
            }
            grid.insert((r, c), d);
        }
    }
    (grid, trailheads)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day10.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(550, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1255, ans);
    }
}
