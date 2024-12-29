use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (moves1, moves2) = parse_input(input);

    let points1: FxHashSet<_> = PositionsIter::new(moves1).collect();
    Ok(PositionsIter::new(moves2)
        .filter(|position| points1.contains(position))
        .map(|position| position.0.unsigned_abs() + position.1.unsigned_abs())
        .min()
        .unwrap())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (moves1, moves2) = parse_input(input);

    let mut points1 = FxHashMap::default();
    for (steps, position) in PositionsIter::new(moves1).enumerate() {
        points1.entry(position).or_insert(steps + 1);
    }

    let mut min_steps = usize::MAX;
    for (steps, position) in PositionsIter::new(moves2).enumerate() {
        if let Some(steps1) = points1.get(&position) {
            min_steps = min_steps.min(steps + steps1 + 1);
        }
    }

    Ok(min_steps)
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .collect_tuple()
        .unwrap()
}

struct PositionsIter<'a> {
    moves: Vec<&'a str>,
    mi: usize,
    delta: (isize, isize),
    rem: usize,
    position: (isize, isize),
}

impl<'a> PositionsIter<'a> {
    fn new(moves: Vec<&'a str>) -> Self {
        PositionsIter {
            moves,
            mi: 0,
            delta: (0, 0),
            rem: 0,
            position: (0, 0),
        }
    }
}

impl Iterator for PositionsIter<'_> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem == 0 {
            if self.mi == self.moves.len() {
                return None;
            }

            let mv = self.moves[self.mi];
            self.mi += 1;
            self.delta = match &mv[..1] {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => unreachable!(),
            };
            self.rem = mv[1..].parse().unwrap();
        }

        self.position = (
            self.position.0 + self.delta.0,
            self.position.1 + self.delta.1,
        );
        self.rem -= 1;
        Some(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day03.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(266, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(19242, ans);
    }
}
