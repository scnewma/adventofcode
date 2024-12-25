use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut locks = HashSet::<[usize; 5]>::new();
    let mut keys = HashSet::<[usize; 5]>::new();
    for schematic in input.split("\n\n") {
        let mut schem = schematic.to_string();
        let mut heights = [0usize; 5];
        let is_lock = schematic.starts_with('#');
        if !is_lock {
            schem = schematic.lines().rev().join("\n");
        }

        for line in schem.lines().skip(1) {
            for (i, ch) in line.char_indices() {
                if ch == '#' {
                    heights[i] += 1;
                }
            }
        }

        if is_lock {
            locks.insert(heights);
        } else {
            keys.insert(heights);
        }
    }

    let mut ans = 0;
    for lock in &locks {
        for key in &keys {
            if lock.iter().enumerate().all(|(i, lh)| lh + key[i] <= 5) {
                ans += 1;
            }
        }
    }

    Ok(ans)
}

pub fn part02(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day25.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3107, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
