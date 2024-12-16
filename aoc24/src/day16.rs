use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use fxhash::{FxHashMap, FxHashSet};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut grid = HashMap::new();
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

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        pos: start,
        dir: Dir::East,
        cost: 0,
        path: FxHashMap::default(),
    });
    let mut visited = HashSet::new();
    let mut min = usize::MAX;
    while let Some(State {
        pos,
        dir,
        cost,
        path,
    }) = heap.pop()
    {
        if !visited.insert((pos, dir)) || grid.get(&pos).is_none_or(|ch| *ch == '#') {
            continue;
        }

        let mut path = path.clone();
        path.insert((pos, dir), 0);

        if pos == end {
            min = min.min(cost);
        }

        // moves
        heap.push(State {
            pos: dir.move_forward(pos),
            dir,
            cost: cost + 1,
            path: path.clone(),
        });
        heap.push(State {
            pos,
            dir: dir.turn_clockwise(),
            cost: cost + 1000,
            path: path.clone(),
        });
        heap.push(State {
            pos,
            dir: dir.turn_counter_clockwise(),
            cost: cost + 1000,
            path: path.clone(),
        });
    }
    Ok(min)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let min = part01(input)?;

    let mut grid = FxHashMap::default();
    // let height = input.lines().count();
    // let width = input.lines().next().unwrap().len();
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

    // TODO: if we implemented dfs directly instead of via binary heap then we wouldn't need to do
    // some many clones of the path, which should be MUCH faster
    // Not sure what exactly is incorrect here though...
    // fn dfs(
    //     min: usize,
    //     grid: &FxHashMap<(isize, isize), char>,
    //     height: isize,
    //     width: isize,
    //     pos: (isize, isize),
    //     dir: Dir,
    //     cost: usize,
    //     end_pos: (isize, isize),
    //     seats: &mut FxHashSet<(isize, isize)>,
    //     current: &mut FxHashSet<(isize, isize)>,
    //     min_costs: &mut FxHashMap<((isize, isize), Dir), usize>,
    // ) {
    //     if cost > min {
    //         return;
    //     }
    //     if cost == min && pos == end_pos {
    //         for &pos in current.iter() {
    //             seats.insert(pos);
    //         }
    //         return;
    //     }

    //     for (pos, dir, dcost) in [
    //         (dir.move_forward(pos), dir, 0),
    //         (pos, dir.turn_clockwise(), 1000),
    //         (pos, dir.turn_counter_clockwise(), 1000),
    //     ] {
    //         if grid.get(&pos).is_some_and(|ch| *ch != '#') && !current.contains(&pos) {
    //             current.insert(pos);
    //             dfs(
    //                 min,
    //                 grid,
    //                 height,
    //                 width,
    //                 pos,
    //                 dir,
    //                 cost + dcost + 1,
    //                 end_pos,
    //                 seats,
    //                 current,
    //                 min_costs,
    //             );
    //             current.remove(&pos);
    //         }
    //         // if grid.get(&pos).is_none_or(|ch| *ch == '#')
    //         //     || current.contains(&pos)
    //         //     || min_costs
    //         //         .get(&(pos, dir))
    //         //         .is_some_and(|c| cost + dcost > *c)
    //         // {
    //         //     continue;
    //         // }
    //     }
    // }

    // let mut seats = FxHashSet::default();
    // let mut current = FxHashSet::default();
    // current.insert(start);
    // let mut min_costs = FxHashMap::default();
    // min_costs.insert((start, Dir::East), 0);
    // dfs(
    //     min,
    //     &grid,
    //     height as isize,
    //     width as isize,
    //     start,
    //     Dir::East,
    //     0,
    //     end,
    //     &mut seats,
    //     &mut current,
    //     &mut min_costs,
    // );
    // Ok(seats.len())

    let mut heap: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    heap.push(Reverse(State {
        pos: start,
        dir: Dir::East,
        cost: 0,
        path: FxHashMap::default(),
    }));
    let mut min_costs = FxHashMap::default();
    let mut best_seats = FxHashSet::default();
    while let Some(Reverse(State {
        pos,
        dir,
        cost,
        mut path,
    })) = heap.pop()
    {
        if cost > min
            || min_costs
                .get(&(pos, dir))
                .is_some_and(|min_cost| cost > *min_cost)
            || path.contains_key(&(pos, dir))
            || grid.get(&pos).is_none_or(|ch| *ch == '#')
        {
            continue;
        }

        path.insert((pos, dir), cost);
        min_costs.insert((pos, dir), cost);

        if pos == end && cost == min {
            for ((pos, _), _) in &path {
                best_seats.insert(*pos);
            }
            continue;
        }

        // moves
        if cost + 1 <= min {
            heap.push(Reverse(State {
                pos: dir.move_forward(pos),
                dir,
                cost: cost + 1,
                path: path.clone(),
            }));
        }
        if cost + 1000 <= min {
            heap.push(Reverse(State {
                pos,
                dir: dir.turn_clockwise(),
                cost: cost + 1000,
                path: path.clone(),
            }));
            heap.push(Reverse(State {
                pos,
                dir: dir.turn_counter_clockwise(),
                cost: cost + 1000,
                path,
            }));
        }
    }
    Ok(best_seats.len())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: (isize, isize),
    dir: Dir,
    cost: usize,
    path: FxHashMap<((isize, isize), Dir), usize>,
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn move_forward(&self, (r, c): (isize, isize)) -> (isize, isize) {
        match self {
            Dir::North => (r - 1, c),
            Dir::South => (r + 1, c),
            Dir::East => (r, c + 1),
            Dir::West => (r, c - 1),
        }
    }

    fn turn_clockwise(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::East => Dir::South,
            Dir::West => Dir::North,
        }
    }

    fn turn_counter_clockwise(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
            Dir::West => Dir::South,
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

    #[ignore]
    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(551, ans);
    }
}
