use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (antennas, height, width) = parse_input(input);
    Ok(antennas
        .values()
        .fold(FxHashSet::<(isize, isize)>::default(), |mut acc, locs| {
            locs.iter()
                .permutations(2)
                .map(|pair| (2 * pair[0].0 - pair[1].0, 2 * pair[0].1 - pair[1].1))
                .filter(|&(r, c)| r >= 0 && r < height && c >= 0 && c < width)
                .for_each(|pos| {
                    acc.insert(pos);
                });
            acc
        })
        .len())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (antennas, height, width) = parse_input(input);
    let mut antinodes = FxHashSet::default();
    for locs in antennas.values() {
        for pair in locs.iter().permutations(2) {
            let dr = pair[0].0 - pair[1].0;
            let dc = pair[0].1 - pair[1].1;
            (0..)
                .map(|i| (pair[0].0 + (dr * i), pair[0].1 + (dc * i)))
                .take_while(|&(r, c)| r >= 0 && r < height && c >= 0 && c < width)
                .for_each(|pos| {
                    antinodes.insert(pos);
                });
        }
    }
    Ok(antinodes.len())
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (FxHashMap<char, FxHashSet<(isize, isize)>>, isize, isize) {
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;

    let mut antennas = FxHashMap::<char, FxHashSet<(isize, isize)>>::default();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.char_indices() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_default()
                    .insert((r as isize, c as isize));
            }
        }
    }
    (antennas, height, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day08.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(413, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1417, ans);
    }
}
