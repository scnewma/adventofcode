use anyhow::Context;
use std::str::FromStr;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let sum = input
        .lines()
        .flat_map(Game::from_str)
        .filter(|game| {
            game.hands
                .iter()
                .all(|hand| hand.0 <= MAX_RED && hand.1 <= MAX_GREEN && hand.2 <= MAX_BLUE)
        })
        .map(|game| game.id)
        .sum();
    Ok(sum)
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let sum = input
        .lines()
        .flat_map(Game::from_str)
        .map(|game| {
            game.hands
                .into_iter()
                .reduce(|acc, e| Rgb(acc.0.max(e.0), acc.1.max(e.1), acc.2.max(e.2)))
                .unwrap()
        })
        .map(|hand| hand.0 * hand.1 * hand.2)
        .sum();
    Ok(sum)
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Rgb>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_no, game_info) = s
            .split_once(": ")
            .context("expected format Game #: <hands>")?;

        let game_no = game_no
            .strip_prefix("Game ")
            .context("missing \"Game \" prefix")?;

        let id: u32 = game_no.parse().context("parsing game number")?;

        let mut hands = Vec::new();
        for hand_info in game_info.split("; ") {
            let mut rgb = Rgb::default();
            for cube_info in hand_info.split(", ") {
                let (count, color) = cube_info
                    .split_once(' ')
                    .context("expected cube count to be in format \"<count> <color>\"")?;

                let count: u32 = count.parse().context("parsing cube count")?;

                match color {
                    "red" => rgb.0 = count,
                    "green" => rgb.1 = count,
                    "blue" => rgb.2 = count,
                    _ => unreachable!("unexpected color {color}"),
                }
            }
            hands.push(rgb);
        }

        Ok(Game { id, hands })
    }
}

#[derive(Debug, Default)]
struct Rgb(u32, u32, u32);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day02.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(2720, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(71535, ans);
    }
}
