use std::collections::{BinaryHeap, VecDeque};

use fxhash::{FxHashMap, FxHashSet};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, start, end) = parse_input(input);
    let costs = shortest_path_costs(&grid, start, end);

    let min_cost = costs[&end];
    let mut worthwhile_cheats = 0;

    // check every non-wall position in the grid, if it's neighbor is a wall followed by an open
    // tile then we could cheat and move there directly.
    // we can calculate the distance with the costs map by taking the cost to get to this key and
    // adding the cost to get to the neighboring open tile taken from the overall shortest path
    // cost.
    for pos in grid.keys() {
        if grid[pos] == '#' || costs[pos] > min_cost {
            continue;
        }

        for (dr, dc) in crate::DELTAS4 {
            let neighbor = (pos.0 + dr, pos.1 + dc);
            let nneighbor = (pos.0 + dr * 2, pos.1 + dc * 2);
            if grid[&neighbor] == '#' && grid.get(&nneighbor).is_some_and(|ch| *ch == '.') {
                let neighbor_cost_to_end = min_cost - costs[&nneighbor];
                let cheated_cost = neighbor_cost_to_end + costs[pos] + 2;
                if cheated_cost < min_cost && min_cost - cheated_cost >= 100 {
                    worthwhile_cheats += 1;
                }
            }
        }
    }

    Ok(worthwhile_cheats)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (grid, start, end) = parse_input(input);
    let costs = shortest_path_costs(&grid, start, end);

    let min_cost = costs[&end];
    let mut worthwhile_cheats = 0;

    // check every non-wall position in the grid, check every cell within 20 moves to see if moving
    // directly to that cell (crow's path) would be faster.
    for pos in grid.keys() {
        if grid[pos] == '#' || costs[pos] > min_cost {
            continue;
        }

        let mut visited = FxHashSet::default();
        let mut q = VecDeque::new();
        q.push_back((*pos, 20));
        while let Some((cheat_pos, cheat_rem)) = q.pop_front() {
            if !visited.insert(cheat_pos) {
                continue;
            }
            if grid[&cheat_pos] == '.' {
                let cost_to_end = min_cost - costs[&cheat_pos];
                let cheated_cost = cost_to_end + costs[pos] + (20 - cheat_rem);
                if cheated_cost < min_cost && min_cost - cheated_cost >= 100 {
                    worthwhile_cheats += 1;
                }
            }

            if cheat_rem > 0 {
                for (dr, dc) in crate::DELTAS4 {
                    let neighbor = (cheat_pos.0 + dr, cheat_pos.1 + dc);
                    if !grid.contains_key(&neighbor) {
                        continue;
                    }

                    q.push_back((neighbor, cheat_rem - 1));
                }
            }
        }
    }

    Ok(worthwhile_cheats)
}

// dijkstra
fn shortest_path_costs(grid: &FxHashMap<Pos, char>, start: Pos, end: Pos) -> FxHashMap<Pos, usize> {
    let mut costs = FxHashMap::default();
    for pos in grid.keys() {
        if grid[pos] == '#' {
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
    costs
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
        assert_eq!(1518, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1032257, ans);
    }
}
