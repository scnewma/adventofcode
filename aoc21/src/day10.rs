use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    input
        .lines()
        .map(process_line)
        // drop incomplete lines
        .filter_map(|line_type| match line_type {
            LineType::Invalid(c) => Some(c),
            _ => None,
        })
        // calculate score
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

pub fn part02(input: &str) -> i64 {
    let mut scores: Vec<i64> = input
        .lines()
        .map(process_line)
        // drop invalid lines
        .filter_map(|line_type| match line_type {
            LineType::Incomplete(ending) => Some(ending),
            _ => None,
        })
        // calculate score
        .map(|ending_chars| {
            ending_chars.iter().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[derive(PartialEq)]
enum LineType {
    Invalid(char),
    Incomplete(Vec<char>),
}

fn process_line(line: &str) -> LineType {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            closing => {
                if closing != pair(&stack.pop().unwrap()) {
                    return LineType::Invalid(c);
                }
            }
        }
    }

    let ending = stack.iter().rev().map(pair).collect();
    LineType::Incomplete(ending)
}

fn pair(c: &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day10.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(374061, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(2116639949, ans);
    }
}
