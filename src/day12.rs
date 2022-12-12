use anyhow::Context;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let map: ElevationMap = input.parse()?;
    map.shortest_path(map.start, map.end)
        .context("no shortest path!")
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let map: ElevationMap = input.parse()?;
    let starts: Vec<(i32, i32)> = map
        .grid
        .iter()
        .filter(|(_, el)| **el == 'a')
        .map(|(point, _)| *point)
        .collect();

    let mut shortest = u32::max_value();
    for start in starts {
        if let Some(path) = map.shortest_path(start, map.end) {
            shortest = shortest.min(path);
        }
    }

    Ok(shortest)
}

struct ElevationMap {
    // while all of the points are actually usizes, we use i32 so that we don't need to do bounds
    // checking in the algo
    grid: HashMap<(i32, i32), char>,
    start: (i32, i32),
    end: (i32, i32),
}

impl ElevationMap {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn shortest_path(&self, start: (i32, i32), end: (i32, i32)) -> Option<u32> {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        while let Some((point, steps)) = queue.pop_front() {
            if point == end {
                return Some(steps);
            }
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point);

            for (dr, dc) in Self::DELTAS {
                let neigh = (point.0 + dr, point.1 + dc);
                if let Some(elevation) = self.grid.get(&neigh) {
                    if self.grid[&point] as u8 + 1 >= *elevation as u8 {
                        queue.push_back((neigh, steps + 1));
                    }
                }
            }
        }
        None
    }
}

impl FromStr for ElevationMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();
        let (mut start, mut end) = ((0, 0), (0, 0));
        for (r, line) in s.lines().enumerate() {
            for (c, mut elevation) in line.chars().enumerate() {
                match elevation {
                    'S' => {
                        start = (r as i32, c as i32);
                        elevation = 'a';
                    }
                    'E' => {
                        end = (r as i32, c as i32);
                        elevation = 'z';
                    }
                    _ => {}
                }
                grid.insert((r as i32, c as i32), elevation);
            }
        }
        Ok(Self { grid, start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day12.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day12.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(31, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(394, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(29, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(388, ans);
    }
}
