use itertools::{Either, iproduct};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut locks = Vec::<[usize; 5]>::new();
    let mut keys = Vec::<[usize; 5]>::new();
    for schematic in input.split("\n\n") {
        let (locks_or_keys, lines) = if schematic.starts_with('#') {
            (&mut locks, Either::Left(schematic.lines()))
        } else {
            (&mut keys, Either::Right(schematic.lines().rev()))
        };

        let mut heights = [0usize; 5];
        lines
            .skip(1)
            .flat_map(|line| line.char_indices())
            .for_each(|(col, ch)| heights[col] += (ch == '#') as usize);

        locks_or_keys.push(heights);
    }

    Ok(iproduct!(locks, keys)
        .filter(|(lock, key)| lock.iter().enumerate().all(|(i, lh)| lh + key[i] <= 5))
        .count())
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
