use std::collections::BinaryHeap;

use fxhash::{FxHashMap, FxHashSet};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, start, end) = parse_input(input);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        pos: start,
        dir: 1,
        cost: 0,
        path: Vec::new(),
    });
    let mut visited = FxHashSet::default();
    let mut min = usize::MAX;
    while let Some(State {
        pos,
        dir,
        cost,
        mut path,
    }) = heap.pop()
    {
        if !visited.insert((pos, dir)) || grid.get(&pos).is_none_or(|ch| *ch == '#') {
            continue;
        }

        path.push(pos);

        if pos == end {
            min = min.min(cost);
        }

        for (ndir, costd) in [
            (dir, 1),
            (dir.turn_clockwise(), 1001),
            (dir.turn_counter_clockwise(), 1001),
        ] {
            let npos = ndir.move_forward(pos);
            if grid[&npos] != '#' {
                heap.push(State::new(npos, ndir, cost + costd, path.clone()));
            }
        }
    }
    Ok(min)
}

// my initial solution was slow (~60s), reimplemented something similar to this solution for
// improved performance:
// https://old.reddit.com/r/adventofcode/comments/1hfboft/2024_day_16_solutions/m2cgw50/
pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (grid, start, end) = parse_input(input);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State::new(start, 1u8, 0, Vec::new()));

    let mut min = usize::MAX;
    let mut visited = FxHashMap::default();
    let mut best_seats = FxHashSet::default();
    while let Some(State {
        pos,
        dir,
        cost,
        mut path,
    }) = heap.pop()
    {
        if cost > min {
            break;
        }
        if visited
            .get(&(pos, dir))
            .is_some_and(|min_cost| *min_cost < cost)
        {
            continue;
        }
        visited.insert((pos, dir), cost);
        path.push(pos);
        if pos == end {
            min = cost;
            for path_pos in &path {
                best_seats.insert(*path_pos);
            }
        }

        for (ndir, costd) in [
            (dir, 1),
            (dir.turn_clockwise(), 1001),
            (dir.turn_counter_clockwise(), 1001),
        ] {
            let npos = ndir.move_forward(pos);
            if grid[&npos] != '#' {
                heap.push(State::new(npos, ndir, cost + costd, path.clone()));
            }
        }
    }
    Ok(best_seats.len())
}

fn parse_input(input: &str) -> (FxHashMap<Pos, char>, Pos, Pos) {
    let mut grid = FxHashMap::default();
    let mut start = (0isize, 0isize);
    let mut end = (0isize, 0isize);
    for (r, line) in input.lines().enumerate() {
        for (c, mut ch) in line.char_indices() {
            if ch == 'S' {
                start = (r as isize, c as isize);
                ch = '.';
            }
            if ch == 'E' {
                end = (r as isize, c as isize);
                ch = '.';
            }
            grid.insert((r as isize, c as isize), ch);
        }
    }
    (grid, start, end)
}

type Pos = (isize, isize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Pos,
    dir: u8,
    cost: usize,
    path: Vec<Pos>,
}

impl State {
    fn new(pos: Pos, dir: u8, cost: usize, path: Vec<Pos>) -> State {
        State {
            pos,
            dir,
            cost,
            path,
        }
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

trait Direction {
    fn move_forward(&self, pos: Pos) -> Pos;
    fn turn_clockwise(&self) -> Self;
    fn turn_counter_clockwise(&self) -> Self;
}

impl Direction for u8 {
    fn move_forward(&self, (r, c): Pos) -> Pos {
        const DELTAS: [Pos; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let (dr, dc) = DELTAS[*self as usize];
        (r + dr, c + dc)
    }

    fn turn_clockwise(&self) -> Self {
        (self + 1) % 4
    }

    fn turn_counter_clockwise(&self) -> Self {
        if *self == 0 {
            3
        } else {
            self - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day16.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(109496, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(551, ans);
    }
}
