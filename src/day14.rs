// use std::collections::HashSet;

use anyhow::Context;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const DELTAS: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];
const SOURCE: Point = Point { x: 500, y: 0 };

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let (mut obstructions, _) = parse_input(input)?;

    // let max_y = obstructions.iter().map(|p| p.y).max().context("no max_y")?;
    let max_y = 157;

    let mut num_sand = 0;
    let mut sand = SOURCE;
    loop {
        let before = sand;
        for (dx, dy) in DELTAS {
            let next = Point {
                x: add(sand.x, dx),
                y: add(sand.y, dy),
            };
            if !obstructions[next.y][next.x] {
                sand = next;
                break;
            }
        }
        if sand == before {
            obstructions[sand.y][sand.x] = true;
            num_sand += 1;
            sand = SOURCE;
        }
        if sand.y > max_y {
            break Ok(num_sand);
        }
    }
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let (mut obstructions, max_y) = parse_input(input)?;
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
            if next.y < floor && !obstructions[next.y][next.x] {
                sand = next;
                stk.push(sand);
                break;
            }
        }
        if sand == SOURCE {
            break Ok(num_sand + 1);
        }
        if sand == before {
            obstructions[sand.y][sand.x] = true;
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

#[derive(Debug, Clone, Copy)]
struct Line(Point, Point);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> anyhow::Result<([[bool; 1000]; 200], usize)> {
    // let mut obstructions = HashSet::new();
    let mut obstructions = [[false; 1000]; 200];
    let mut max_y = 0;
    for line in input.lines() {
        let mut prev: Option<Point> = None;
        for point in line.split(" -> ") {
            let (left, right) = point.split_once(',').context("invalid point")?;
            let point = Point {
                x: left.parse()?,
                y: right.parse()?,
            };
            max_y = max_y.max(point.y);
            if let Some(prev) = prev {
                if prev.x == point.x {
                    // vertical
                    let (min, max) = (prev.y.min(point.y), prev.y.max(point.y));
                    for i in min..=max {
                        obstructions[i][prev.x] = true;
                    }
                } else {
                    // horizontal
                    let (min, max) = (prev.x.min(point.x), prev.x.max(point.x));
                    for i in min..=max {
                        obstructions[prev.y][i] = true;
                    }
                }
            }
            prev = Some(point);
        }
    }
    Ok((obstructions, max_y))
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
