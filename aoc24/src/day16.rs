use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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

    // let mut q = VecDeque::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        pos: start,
        dir: Dir::East,
        cost: 0,
        path: vec![],
    });
    // q.push_back((start, Dir::East, 0, vec![]));
    let mut visited = HashSet::new();
    let mut min = usize::MAX;
    // while let Some((pos, dir, cost, path)) = q.pop_front() {
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
        path.push((pos, dir));

        if pos == end {
            min = min.min(cost);
            // println!("--- cost: {cost}");
            // for r in 0..15 {
            //     for c in 0..15 {
            //         if let Some((_, dir)) = path.iter().find(|(pos, _dir)| *pos == (r, c)) {
            //             print!(
            //                 "{}",
            //                 match dir {
            //                     Dir::North => '^',
            //                     Dir::South => 'v',
            //                     Dir::East => '>',
            //                     Dir::West => '<',
            //                 }
            //             )
            //         } else {
            //             print!("{}", grid[&(r, c)]);
            //         }
            //     }
            //     println!();
            // }
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
        // q.push_back((pos, dir.turn_clockwise(), cost + 1000, path.clone()));
        // q.push_back((pos, dir.turn_counter_clockwise(), cost + 1000, path));
    }
    Ok(min)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (isize, isize),
    dir: Dir,
    cost: usize,
    path: Vec<((isize, isize), Dir)>,
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
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
