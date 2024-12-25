use std::str::FromStr;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let measurements = parse_input(input);
    measurements
        .iter()
        .flat_map(|m| &m.1)
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7)) // numbers: 1 | 7 | 4 | 8
        .count() as i64
}

pub fn part02(input: &str) -> i64 {
    let measurements = parse_input(input);
    measurements.iter().map(|d| d.decode()).sum()
}

struct Display(Vec<String>, Vec<String>);

impl Display {
    fn decode(&self) -> i64 {
        let char_signals: Vec<Vec<char>> = self.0.iter().map(|s| s.chars().collect()).collect();
        let one = char_signals.iter().find(|cs| cs.len() == 2).unwrap();
        let four = char_signals.iter().find(|cs| cs.len() == 4).unwrap();
        let seven = char_signals.iter().find(|cs| cs.len() == 3).unwrap();
        // remove known numbers from list
        let char_signals: Vec<Vec<char>> = char_signals
            .clone()
            .into_iter()
            .filter(|digit| !matches!(digit.len(), 2 | 3 | 4 | 7))
            .collect();

        // (top) can be determined by subtracting 1 from 7
        let top = seven.iter().find(|c| !one.contains(c)).unwrap();

        // (bot) subtract 7 and 4 from 9 (has all from 7 and 4)
        let nine: Vec<char> = char_signals
            .clone()
            .into_iter()
            .filter(|cs| cs.len() == 6)
            .find(|cs| seven.iter().all(|c| cs.contains(c)) && four.iter().all(|c| cs.contains(c)))
            .unwrap();
        let bot = nine
            .iter()
            .find(|c| !(seven.contains(c) || four.contains(c)))
            .unwrap();

        // (mid) if you filter 9 from the list. 0 is the only number left that contains all
        // sections from 1. you can determine mid by finding missing section
        let zero = char_signals
            .clone()
            .into_iter()
            .filter(|cs| cs.len() == 6)
            .filter(|cs| *cs != nine)
            .find(|cs| one.iter().all(|c| cs.contains(c)))
            .unwrap();
        let mid = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
            .iter()
            .find(|c| !zero.contains(c))
            .unwrap();

        // (top-left) subtract top-right, mid, and bot-right from 4
        let top_left = four
            .iter()
            .find(|c| !(one.contains(c) || mid == *c))
            .unwrap();

        // (bot-left) subtract 7 from 0, then subtract top-left, bot
        let bot_left = zero
            .iter()
            .find(|c| !(seven.contains(c) || *c == top_left || *c == bot))
            .unwrap();

        // (top-right) digit 2 has all of the following (top, mid, bot, bot-left, one digit from
        //   1). the digit that is shared with 1 is the top right
        let two = char_signals
            .into_iter()
            .filter(|cs| cs.len() == 5)
            .find(|cs| {
                cs.contains(top)
                    && cs.contains(mid)
                    && cs.contains(bot)
                    && cs.contains(bot_left)
                    && (cs.contains(&one[0]) || cs.contains(&one[1]))
            })
            .unwrap();
        let top_right = two.iter().find(|c| **c == one[0] || **c == one[1]).unwrap();

        // (bot-right) find remaining section
        let bot_right = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
            .iter()
            .find(|c| {
                !(*c == top
                    || *c == bot
                    || *c == top_left
                    || *c == top_right
                    || *c == mid
                    || *c == bot_left)
            })
            .unwrap();

        // [top, top_left, top_right, mid, bot_left, bot_right, bot],
        let masks = [
            (
                0,
                [
                    *top, *top_left, *top_right, ' ', *bot_left, *bot_right, *bot,
                ],
            ),
            (1, [' ', ' ', *top_right, ' ', ' ', *bot_right, ' ']),
            (2, [*top, ' ', *top_right, *mid, *bot_left, ' ', *bot]),
            (3, [*top, ' ', *top_right, *mid, ' ', *bot_right, *bot]),
            (4, [' ', *top_left, *top_right, *mid, ' ', *bot_right, ' ']),
            (5, [*top, *top_left, ' ', *mid, ' ', *bot_right, *bot]),
            (6, [*top, *top_left, ' ', *mid, *bot_left, *bot_right, *bot]),
            (7, [*top, ' ', *top_right, ' ', ' ', *bot_right, ' ']),
            (
                8,
                [
                    *top, *top_left, *top_right, *mid, *bot_left, *bot_right, *bot,
                ],
            ),
            (
                9,
                [*top, *top_left, *top_right, *mid, ' ', *bot_right, *bot],
            ),
        ];

        let mut digits = "".to_owned();
        for mask in &self.1 {
            let mut sections = [' '; 7];
            for c in mask.chars() {
                match c {
                    c if c == *top => sections[0] = c,
                    c if c == *top_left => sections[1] = c,
                    c if c == *top_right => sections[2] = c,
                    c if c == *mid => sections[3] = c,
                    c if c == *bot_left => sections[4] = c,
                    c if c == *bot_right => sections[5] = c,
                    c if c == *bot => sections[6] = c,
                    _ => panic!(""),
                }
            }

            for (digit, mask) in masks {
                if sections == mask {
                    digits.push_str(&format!("{}", digit));
                    break;
                }
            }
        }
        i64::from_str(&digits).unwrap()
    }
}

impl FromStr for Display {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sigpat, out) = s.trim().split_once(" | ").unwrap();
        let signal_patterns: Vec<String> = sigpat
            .split(' ')
            .map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort_by(|a, b| b.cmp(a));
                String::from_iter(chars)
            })
            .collect();
        let output_value: Vec<String> = out.split(' ').map(|s| s.to_owned()).collect();
        Ok(Display(signal_patterns, output_value))
    }
}

fn parse_input(input: &str) -> Vec<Display> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day08.input.txt");

    #[test]
    fn decode_test() {
        let s =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let m: Display = s.parse().unwrap();
        assert_eq!(5353, m.decode());
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(392, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(1004688, ans);
    }
}
