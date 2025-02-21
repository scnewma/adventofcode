use std::collections::HashSet;

use anyhow::Context;

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const DELTAS: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];
const SOURCE: Point = Point { x: 500, y: 0 };

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let mut obstructions = parse_input(input)?;

    let max_y = obstructions.iter().map(|p| p.y).max().context("no max_y")?;

    let mut num_sand = 0;
    let mut sand = SOURCE;
    loop {
        let before = sand;
        for (dx, dy) in DELTAS {
            let next = Point {
                x: add(sand.x, dx),
                y: add(sand.y, dy),
            };
            if !obstructions.contains(&next) {
                sand = next;
                break;
            }
        }
        if sand == before {
            obstructions.insert(sand);
            num_sand += 1;
            sand = SOURCE;
        }
        if sand.y > max_y {
            break Ok(num_sand);
        }
    }
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let mut obstructions = parse_input(input)?;

    let max_y = obstructions.iter().map(|p| p.y).max().context("no max_y")?;
    let floor = max_y + 2;

    let mut num_sand = 0;
    let mut sand = SOURCE;
    let mut stk = Vec::new();
    loop {
        let before = sand;
        for (dx, dy) in DELTAS {
            let next = Point {
                x: add(sand.x, dx),
                y: add(sand.y, dy),
            };
            if next.y != floor && !obstructions.contains(&next) {
                sand = next;
                stk.push(sand);
                break;
            }
        }
        if sand == SOURCE {
            break Ok(num_sand + 1);
        }
        if sand == before {
            obstructions.insert(sand);
            num_sand += 1;
            stk.pop();
            sand = stk.pop().unwrap_or(SOURCE);
        }
    }
}

#[inline]
fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.unsigned_abs() as usize
    } else {
        u + i as usize
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> anyhow::Result<HashSet<Point>> {
    let mut map = HashSet::new();
    for line in input.lines() {
        let mut prev: Option<Point> = None;
        for point in line.split(" -> ") {
            let (left, right) = point.split_once(',').context("invalid point")?;
            let point = Point {
                x: left.parse()?,
                y: right.parse()?,
            };
            if let Some(prev) = prev {
                if prev.x == point.x {
                    // vertical
                    let (min, max) = (prev.y.min(point.y), prev.y.max(point.y));
                    for i in min..=max {
                        map.insert(Point { x: prev.x, y: i });
                    }
                } else {
                    // horizontal
                    let (min, max) = (prev.x.min(point.x), prev.x.max(point.x));
                    for i in min..=max {
                        map.insert(Point { x: i, y: prev.y });
                    }
                }
            }
            prev = Some(point);
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../inputs/day14.sample.txt");
    const INPUT: &str = include_str!("../inputs/day14.input.txt");

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
