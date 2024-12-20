use std::collections::{BinaryHeap, VecDeque};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, start, end) = parse_input(input);

    let mut costs = FxHashMap::default();
    for pos in grid.keys() {
        if grid[&pos] == '#' {
            continue;
        }
        costs.insert(*pos, usize::MAX);
    }
    costs.insert(start, 0);

    let mut heap = BinaryHeap::<State>::new();
    heap.push(State::new(start, 0));
    while let Some(State { pos, cost }) = heap.pop() {
        if cost > costs[&pos] {
            continue;
        }

        costs.insert(pos, cost);

        if pos == end {
            continue;
        }

        for (dr, dc) in crate::DELTAS4 {
            let npos = (pos.0 + dr, pos.1 + dc);
            if grid.get(&npos).is_none_or(|ch| *ch == '#') {
                continue;
            }

            heap.push(State::new(npos, cost + 1));
        }
    }

    let min_cost = costs[&end];
    let mut costs_saved = Vec::new();

    for pos in grid.keys() {
        if grid[&pos] == '#' || costs[&pos] > min_cost {
            continue;
        }

        for (dr, dc) in crate::DELTAS4 {
            let neighbor = (pos.0 + dr, pos.1 + dc);
            let nneighbor = (pos.0 + dr * 2, pos.1 + dc * 2);
            if grid[&neighbor] == '#' && grid.get(&nneighbor).is_some_and(|ch| *ch == '.') {
                let neighbor_cost_to_end = min_cost - costs[&nneighbor];
                let cheated_cost = neighbor_cost_to_end + costs[&pos] + 2;
                if cheated_cost < min_cost {
                    costs_saved.push(min_cost - cheated_cost);
                }
            }
        }
    }

    Ok(costs_saved.iter().filter(|cost| **cost >= 100).count())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Pos,
    cost: usize,
}

impl State {
    fn new(pos: Pos, cost: usize) -> State {
        State { pos, cost }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Pos = (isize, isize);

fn parse_input(input: &str) -> (FxHashMap<Pos, char>, Pos, Pos) {
    let mut grid = FxHashMap::default();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (r, line) in input.lines().enumerate() {
        for (c, mut ch) in line.char_indices() {
            if ch == 'S' {
                ch = '.';
                start = (r as isize, c as isize);
            } else if ch == 'E' {
                ch = '.';
                end = (r as isize, c as isize);
            }
            grid.insert((r as isize, c as isize), ch);
        }
    }
    (grid, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day20.input.txt");

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
