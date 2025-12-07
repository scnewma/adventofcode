use itertools::iproduct;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const DELTAS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let grid: PointMap = input.parse()?;
    let part_numbers = find_parts(&grid);

    let mut sum = 0;
    for part in part_numbers {
        let mut adjsym = false;
        for c in part.range.0..=part.range.1 {
            for delta in DELTAS {
                let nr = part.row + delta.0;
                let nc = c + delta.1;

                adjsym = adjsym
                    || grid
                        .points
                        .get(&(nr, nc))
                        .is_some_and(|v| !(*v).is_ascii_digit() && *v != '.');
            }
        }
        if adjsym {
            sum += part.value;
        }
    }

    Ok(sum)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let grid: PointMap = input.parse()?;
    let part_numbers = find_parts(&grid);

    let mut sum = 0;
    for (r, c) in iproduct!(0..grid.height as isize, 0..grid.width as isize) {
        if grid.points[&(r, c)] != '*' {
            continue;
        }

        let mut connected_parts = HashSet::new();
        for delta in DELTAS {
            let nr = r + delta.0;
            let nc = c + delta.1;

            if let Some(part) = part_at(&part_numbers, (nr, nc)) {
                connected_parts.insert(part);
            }
        }
        if connected_parts.len() == 2 {
            let ratio: usize = connected_parts.iter().map(|part| part.value).product();
            sum += ratio;
        }
    }
    Ok(sum)
}

fn find_parts(grid: &PointMap) -> Vec<PartNo> {
    let mut parts = Vec::new();
    let mut seen = HashSet::new();
    for (r, c) in iproduct!(0..grid.height as isize, 0..grid.width as isize) {
        if !seen.insert((r, c)) {
            continue;
        }

        if grid.points[&(r, c)].is_ascii_digit() {
            let mut end = c;
            while end < grid.width as isize {
                seen.insert((r, end));
                if !grid.points[&(r, end)].is_ascii_digit() {
                    break;
                }
                end += 1;
            }

            let range = (c, end - 1);

            let mut value = 0usize;
            for col in range.0..=range.1 {
                let d = grid.points.get(&(r, col)).unwrap().to_digit(10).unwrap();
                value = value * 10 + d as usize;
            }

            parts.push(PartNo {
                row: r,
                range,
                value,
            });
        }
    }
    parts
}

fn part_at(part_numbers: &[PartNo], (r, c): (isize, isize)) -> Option<PartNo> {
    for partno in part_numbers {
        if partno.row == r && (partno.range.0..=partno.range.1).contains(&c) {
            return Some(partno.clone());
        }
    }
    None
}

struct PointMap {
    width: usize,
    height: usize,
    points: HashMap<(isize, isize), char>,
}

impl FromStr for PointMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashMap::new();
        let height = s.lines().count();
        let mut width = 0;
        for (r, line) in s.lines().enumerate() {
            width = line.len();
            for (c, char) in line.char_indices() {
                points.insert((r as isize, c as isize), char);
            }
        }
        Ok(PointMap {
            width,
            height,
            points,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct PartNo {
    row: isize,
    range: (isize, isize),
    value: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day03.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(527144, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(81463996, ans);
    }
}
