use std::{cmp::Ordering, str::FromStr};

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    run_simulations(input).into_iter().max().unwrap() as i64
}

pub fn part02(input: &str) -> i64 {
    run_simulations(input).iter().len() as i64
}

fn run_simulations(input: &str) -> Vec<i32> {
    let target_area: Rect = input.parse().unwrap();
    let mut results = Vec::new();
    let ymin = -target_area.br.1.abs();
    let ymax = target_area.br.1.abs();
    for xv in 0..=target_area.br.0 {
        for yv in ymin..ymax {
            if let Some(ymax) = simulate(&target_area, xv, yv) {
                results.push(ymax);
            }
        }
    }
    results
}

fn simulate(target_area: &Rect, mut xv: i32, mut yv: i32) -> Option<i32> {
    let mut pos = (0, 0);
    let mut ymax = 0;
    loop {
        pos.0 += xv;
        pos.1 += yv;
        ymax = ymax.max(pos.1);

        if target_area.contains(pos) {
            return Some(ymax);
        }

        // went down past bottom of target area
        if yv < 0 && pos.1 < target_area.br.1 {
            return None;
        }

        // drag
        match xv.cmp(&0) {
            Ordering::Greater => xv -= 1,
            Ordering::Less => xv += 1,
            Ordering::Equal => (),
        }

        // gravity
        yv -= 1;
    }
}

struct Rect {
    tl: (i32, i32),
    br: (i32, i32),
}

impl Rect {
    fn contains(&self, pos: (i32, i32)) -> bool {
        pos.0 >= self.tl.0 && pos.0 <= self.br.0 && pos.1 >= self.br.1 && pos.1 <= self.tl.1
    }
}

impl FromStr for Rect {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // strip "target area: "
        let coords = &input[13..].trim();

        // split "150..193, y=-136..-86" into "150..193" and "y=-136..-86"
        let (xrange, yrange) = coords.split_once(", ").unwrap();

        // extract numbers from "150..193"
        let parse_coord = |s: &str| -> (i32, i32) {
            s[2..] // trim x= or y=
                .split_once("..")
                .map(|(xc, yc)| (xc.parse::<i32>().unwrap(), yc.parse::<i32>().unwrap()))
                .unwrap()
        };

        let xrange = parse_coord(xrange);
        let yrange = parse_coord(yrange);

        Ok(Rect {
            tl: (xrange.0, yrange.1),
            br: (xrange.1, yrange.0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day17.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(9180, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(3767, ans);
    }
}
