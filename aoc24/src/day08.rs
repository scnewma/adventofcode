use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(solve::<false>(input))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(solve::<true>(input))
}

fn solve<const PART2: bool>(input: &str) -> usize {
    let (antennas, height, width) = parse_input(input);

    let mut antinodes = FxHashSet::default();
    for (_, locs) in antennas {
        for pair in locs.iter().combinations(2) {
            if PART2 {
                antinodes.insert(pair[0].clone());
                antinodes.insert(pair[1].clone());
            }

            let dr = pair[0].0.abs_diff(pair[1].0) as isize;
            let dc = pair[0].1.abs_diff(pair[1].1) as isize;

            let (top, bot) = if pair[0].0 < pair[1].0 {
                (pair[0], pair[1])
            } else {
                (pair[1], pair[0])
            };

            // add performs the insert given the location tuple (isize, isize) and the operations
            // to perform on the row, col.
            macro_rules! add {
                ($location:ident, $op_row:tt, $op_col:tt) => {
                    if !PART2 {
                        let row = $location.0 $op_row dr;
                        let col = $location.1 $op_col dc;
                        if row >= 0 && row < height && col >= 0 && col < width {
                            antinodes.insert((row, col));
                        }
                    } else {
                        (1..)
                            .map(|i| ($location.0 $op_row (dr * i), $location.1 $op_col (dc * i)))
                            .take_while(|&(r, c)| r >= 0 && r < height && c >= 0 && c < width)
                            .for_each(|pos| {
                                antinodes.insert(pos);
                            });
                    }
                };
            }

            if top.1 < bot.1 {
                // top-left -> bottom-right diagonal
                add!(top, -, -);
                add!(bot, +, +);
            } else {
                // bottom-left -> top-right digonal (or vertical)
                add!(top, -, +);
                add!(bot, +, -);
            }
        }
    }

    antinodes.len()
}

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
