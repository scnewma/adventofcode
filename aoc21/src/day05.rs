use core::fmt;
use std::{cmp::Ordering, str::FromStr};

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let lines: Vec<Line> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut grid = Grid::new();

    lines.iter().filter(|l| !l.is_diag()).for_each(|l| {
        // println!("Covering {:?}", l);
        grid.cover(l);
        // println!("{}", grid);
    });
    grid.overlapping()
}

pub fn part02(input: &str) -> i64 {
    let lines: Vec<Line> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut grid = Grid::new();

    lines.iter().for_each(|l| {
        // println!("Covering {:?}", l);
        grid.cover(l);
        // println!("{}", grid);
    });
    grid.overlapping()
}

const SIZE: usize = 1000;

#[derive(Debug)]
struct Grid(Vec<Vec<i64>>);

impl Grid {
    fn new() -> Self {
        let mut rows = Vec::with_capacity(SIZE);
        (0..SIZE).for_each(|x| {
            let mut cols = Vec::<i64>::with_capacity(SIZE);
            (0..SIZE).for_each(|y| cols.insert(y, 0));
            rows.insert(x, cols);
        });
        Self(rows)
    }

    fn cover(&mut self, line: &Line) {
        let mut pos = (line.0.0, line.0.1);
        let end = (line.1.0, line.1.1);

        // mark the starting position
        self.0[pos.0][pos.1] += 1;

        loop {
            // adjust the current position towards the ending pos
            match pos.0.cmp(&end.0) {
                Ordering::Less => pos.0 += 1,
                Ordering::Greater => pos.0 -= 1,
                Ordering::Equal => {}
            }
            match pos.1.cmp(&end.1) {
                Ordering::Less => pos.1 += 1,
                Ordering::Greater => pos.1 -= 1,
                Ordering::Equal => {}
            }

            // mark current position
            self.0[pos.0][pos.1] += 1;

            // if current == end we are done
            if pos == end {
                break;
            }
        }
    }

    fn overlapping(&self) -> i64 {
        let mut count = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if self.0[x][y] > 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let cell = self.0[x][y];
                if cell == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Line(Point, Point);

impl Line {
    fn is_horz(&self) -> bool {
        self.0.1 == self.1.1
    }

    fn is_vert(&self) -> bool {
        self.0.0 == self.1.0
    }

    fn is_diag(&self) -> bool {
        !self.is_horz() && !self.is_vert()
    }
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let start: Point = parts.next().unwrap().parse().unwrap();
        parts.next(); // skip ->
        let end: Point = parts.next().unwrap().parse().unwrap();
        Ok(Line(start, end))
    }
}

#[derive(Debug)]
struct Point(usize, usize);

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').expect("point not in x,y format");
        Ok(Point(
            x.parse().expect("x not i64"),
            y.parse().expect("y not i64"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day05.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(6397, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(22335, ans);
    }
}
