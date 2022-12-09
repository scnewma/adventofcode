use std::str::FromStr;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opp, me)| (opp.parse::<Move>().unwrap(), me.parse::<Move>().unwrap()))
        .map(|(opp, me)| {
            use GameResult::*;
            use Move::*;

            let res = match (&me, &opp) {
                (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
                (x, y) if x == y => Draw,
                _ => Loss,
            };
            me.score() + res.score()
        })
        .sum()
}

pub fn part02(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opp, result)| {
            (
                opp.parse::<Move>().unwrap(),
                result.parse::<GameResult>().unwrap(),
            )
        })
        .map(|(opp, result)| {
            use GameResult::*;
            use Move::*;

            let mymove = match (opp, &result) {
                (Rock, Win) => Paper,
                (Paper, Win) => Scissors,
                (Scissors, Win) => Rock,
                (Rock, Loss) => Scissors,
                (Paper, Loss) => Rock,
                (Scissors, Loss) => Paper,
                (m, Draw) => m,
            };
            mymove.score() + result.score()
        })
        .sum()
}

enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn score(&self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Loss => 0,
            GameResult::Draw => 3,
        }
    }
}

impl FromStr for GameResult {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err("unexpected result"),
        }
    }
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // X, Y, Z are only for part01, but they don't impact part02
        match input {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("unexpected move"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day02.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day02.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(15, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(11475, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(12, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(16862, ans);
    }
}
