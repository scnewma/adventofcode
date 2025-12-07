use num::Integer;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut dial = 50isize;
    let mut nzeros = 0usize;

    for (dir, dist) in parse_input(input) {
        match dir {
            Direction::Left => {
                dial = (dial.checked_sub_unsigned(dist).unwrap() + 100) % 100;
            }
            Direction::Right => {
                dial = dial.checked_add_unsigned(dist).unwrap() % 100;
            }
        }
        if dial == 0 {
            nzeros += 1;
        }
    }

    Ok(nzeros)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut dial = 50isize;
    let mut nzeros = 0usize;

    for (dir, dist) in parse_input(input) {
        let (full_rotations, dist) = dist.div_mod_floor(&100);
        nzeros += full_rotations;

        let new_dial = match dir {
            Direction::Left => {
                let mut new_dial = dial.checked_sub_unsigned(dist).unwrap();
                if new_dial < 0 {
                    new_dial = 100 - new_dial.abs();
                    if dial != 0 {
                        nzeros += 1;
                    }
                }
                new_dial
            }
            Direction::Right => {
                let new_dial = dial.checked_add_unsigned(dist).unwrap() % 100;
                if dial > new_dial && new_dial != 0 {
                    nzeros += 1;
                }
                new_dial
            }
        };

        if new_dial == 0 {
            nzeros += 1;
        }

        dial = new_dial;
    }

    Ok(nzeros)
}

enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, usize)> + '_ {
    input.lines().map(|line| {
        let (dir, dist) = line.split_at(1);
        let dist = dist.trim().parse::<usize>().unwrap();
        let direction = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        };
        (direction, dist)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day01.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1076, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(6379, ans);
    }
}
