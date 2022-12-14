use std::collections::HashMap;

use anyhow::Context;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let mut map: HashMap<(usize, usize), Obstruction> = HashMap::new();
    for Line(p1, p2) in parse_input(input)? {
        if p1.0 == p2.0 {
            // vertical
            let (min, max) = (p1.1.min(p2.1), p1.1.max(p2.1));
            for i in min..=max {
                map.insert((p1.0, i), Obstruction::Rock);
            }
        } else {
            // horizontal
            let (min, max) = (p1.0.min(p2.0), p1.0.max(p2.0));
            for i in min..=max {
                map.insert((i, p1.1), Obstruction::Rock);
            }
        }
    }

    let max_y = *map
        .iter()
        .map(|((_x, y), _obs)| y)
        .max()
        .context("no max_y")?;

    let mut num_sand = 0;
    let mut falling = (500usize, 0usize);
    loop {
        let (sand_x, sand_y) = falling;
        // sand fell into void
        if sand_y > max_y {
            return Ok(num_sand);
        }

        // check down
        match map.get(&(sand_x, sand_y + 1)) {
            None => {
                falling.1 += 1;
                continue;
            }
            Some(_) => (),
        }
        // check down-left
        match map.get(&(sand_x - 1, sand_y + 1)) {
            None => {
                falling = (sand_x - 1, sand_y + 1);
                continue;
            }
            Some(_) => (),
        }
        // check down-right
        match map.get(&(sand_x + 1, sand_y + 1)) {
            None => {
                falling = (sand_x + 1, sand_y + 1);
                continue;
            }
            Some(_) => (),
        }

        // has settled at source
        if (sand_x, sand_y) == (500, 0) {
            return Ok(num_sand);
        }
        // has settled below source, reset and run again
        map.insert((sand_x, sand_y), Obstruction::Sand);
        num_sand += 1;
        falling = (500, 0);
    }
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let mut map: HashMap<(usize, usize), Obstruction> = HashMap::new();
    for Line(p1, p2) in parse_input(input)? {
        if p1.0 == p2.0 {
            // vertical
            let (min, max) = (p1.1.min(p2.1), p1.1.max(p2.1));
            for i in min..=max {
                map.insert((p1.0, i), Obstruction::Rock);
            }
        } else {
            // horizontal
            let (min, max) = (p1.0.min(p2.0), p1.0.max(p2.0));
            for i in min..=max {
                map.insert((i, p1.1), Obstruction::Rock);
            }
        }
    }

    let max_y = *map
        .iter()
        .map(|((_x, y), _obs)| y)
        .max()
        .context("no max_y")?;

    let floor = max_y + 2;

    let mut map = ObstructionMap { grid: map, floor };

    let mut num_sand = 0;
    let mut falling = (500usize, 0usize);
    loop {
        let (sand_x, sand_y) = falling;
        // check down
        match map.get(&(sand_x, sand_y + 1)) {
            None => {
                falling.1 += 1;
                continue;
            }
            Some(_) => (),
        }
        // check down-left
        match map.get(&(sand_x - 1, sand_y + 1)) {
            None => {
                falling = (sand_x - 1, sand_y + 1);
                continue;
            }
            Some(_) => (),
        }
        // check down-right
        match map.get(&(sand_x + 1, sand_y + 1)) {
            None => {
                falling = (sand_x + 1, sand_y + 1);
                continue;
            }
            Some(_) => (),
        }

        num_sand += 1;
        // has settled at source
        if (sand_x, sand_y) == (500, 0) {
            return Ok(num_sand);
        }
        // has settled below source, reset and run again
        map.insert((sand_x, sand_y), Obstruction::Sand);
        falling = (500, 0);
    }
}

struct ObstructionMap {
    grid: HashMap<(usize, usize), Obstruction>,
    floor: usize,
}

impl ObstructionMap {
    fn get(&self, point: &(usize, usize)) -> Option<Obstruction> {
        if point.1 == self.floor {
            Some(Obstruction::Rock)
        } else {
            self.grid.get(point).cloned()
        }
    }

    fn insert(&mut self, point: (usize, usize), obs: Obstruction) {
        self.grid.insert(point, obs);
    }
}

#[derive(Debug, Clone)]
enum Obstruction {
    Sand,
    Rock,
}

#[derive(Debug, Clone, Copy)]
struct Line(Point, Point);

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

fn parse_input(input: &str) -> anyhow::Result<Vec<Line>> {
    let mut lines = Vec::new();
    for line in input.lines() {
        let mut prev: Option<Point> = None;
        for point in line.split(" -> ") {
            let (left, right) = point.split_once(',').context("invalid point")?;
            let point = Point(left.parse()?, right.parse()?);
            if let Some(prev) = prev {
                lines.push(Line(prev, point));
            }
            prev = Some(point);
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day14.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day14.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(24, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(696, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(93, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(23610, ans);
    }
}
