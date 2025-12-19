use std::collections::HashSet;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut tachyons = HashSet::new();
    let mut splitters = HashSet::new();
    let max_r = input.lines().count();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            match ch {
                'S' => tachyons.insert((r, c)),
                '^' => splitters.insert((r, c)),
                _ => false,
            };
        }
    }

    let mut splits = 0;
    for _ in 0..max_r {
        let mut next = HashSet::new();
        for (r, c) in tachyons {
            if splitters.contains(&(r + 1, c)) {
                next.insert((r + 1, c - 1));
                next.insert((r + 1, c + 1));

                splits += 1;
            } else {
                next.insert((r + 1, c));
            }
        }
        tachyons = next;
    }
    Ok(splits)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day07.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
