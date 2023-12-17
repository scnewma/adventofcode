use std::collections::BinaryHeap;

use fxhash::FxHashSet;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let get_neighbors = |state: &State| {
        let mut neighbors = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        // not allowed to backtrack
        neighbors.retain(|&dir| dir != state.run_direction.opposite());
        // not allowed to travel in the same direction for more than 3 steps
        if state.run_len == 3 {
            neighbors.retain(|&dir| dir != state.run_direction);
        }
        neighbors
    };

    Ok(shortest_path(
        grid,
        vec![State::start(Direction::Right)],
        get_neighbors,
        |_| true,
    ))
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let get_neighbors = |state: &State| {
        if state.run_len < 4 {
            // needs to move a minimum of four blocks in that direction before it can turn
            vec![state.run_direction]
        } else if state.run_len <= 10 {
            let mut ns = vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ];
            // not allowed to backtrack
            ns.retain(|&dir| dir != state.run_direction.opposite());
            // can move a maximum of ten consecutive blocks without turning
            if state.run_len == 10 {
                ns.retain(|&dir| dir != state.run_direction);
            }
            ns
        } else {
            unreachable!("run_len should be at most 10");
        }
    };

    Ok(shortest_path(
        grid,
        vec![
            State::start(Direction::Right),
            State::start(Direction::Down),
        ],
        get_neighbors,
        // must have moved in the same direction for at least four blocks to be considered
        // valid
        |state| state.run_len >= 4 && state.run_len <= 10,
    ))
}

fn shortest_path(
    grid: Vec<Vec<u32>>,
    starts: Vec<State>,
    get_neighbors: impl Fn(&State) -> Vec<Direction>,
    is_valid_path: impl Fn(&State) -> bool,
) -> u32 {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    for start in starts {
        heap.push(start);
    }

    let mut visited = FxHashSet::default();
    while let Some(state) = heap.pop() {
        // cannot include heat_loss in the visited set because it would mean there is effectively 0
        // caching
        if !visited.insert((state.pos, state.run_direction, state.run_len)) {
            continue;
        }

        if state.pos == (grid.len() as i32 - 1, grid[0].len() as i32 - 1) {
            if !is_valid_path(&state) {
                continue;
            }

            return state.heat_loss;
        }

        for next_dir in get_neighbors(&state) {
            let (nr, nc) = (
                next_dir.next_row(state.pos.0),
                next_dir.next_col(state.pos.1),
            );
            if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
                continue;
            }

            let d = state.heat_loss + grid[nr as usize][nc as usize];
            heap.push(State {
                pos: (nr, nc),
                heat_loss: d,
                run_direction: next_dir,
                run_len: if next_dir == state.run_direction {
                    state.run_len + 1
                } else {
                    1
                },
            });
        }
    }

    unreachable!("no solution found");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (i32, i32),
    heat_loss: u32,
    run_direction: Direction,
    run_len: u32,
}

impl State {
    fn start(run_direction: Direction) -> Self {
        Self {
            pos: (0, 0),
            heat_loss: 0,
            run_direction,
            run_len: 0,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_row(&self, row: i32) -> i32 {
        match self {
            Direction::Up => row - 1,
            Direction::Down => row + 1,
            _ => row,
        }
    }

    fn next_col(&self, col: i32) -> i32 {
        match self {
            Direction::Left => col - 1,
            Direction::Right => col + 1,
            _ => col,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day17.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(694, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(829, ans);
    }
}
