use bittle::BitsMut;
use itertools::{Itertools, iproduct};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, h, w) = parse_input(input);

    // for every row/col in grid
    Ok(iproduct!(0..h, 0..w)
        .filter(|&(r, c)| grid[r][c] == 'X') // optimization
        .map(|(r, c)| {
            // count how many directions XMAS exists in
            crate::DELTAS8
                .into_iter()
                .filter(|(dr, dc)| {
                    "XMAS".char_indices().all(|(i, ch)| {
                        match (
                            r.checked_add_signed(dr * i as isize),
                            c.checked_add_signed(dc * i as isize),
                        ) {
                            (Some(nr), Some(nc)) => nr < h && nc < w && grid[nr][nc] == ch,
                            _ => false,
                        }
                    })
                })
                .count()
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (grid, h, w) = parse_input(input);

    let mut ms_mask: u32 = 0;
    ms_mask.set_bit('M' as u32 - 'A' as u32);
    ms_mask.set_bit('S' as u32 - 'A' as u32);

    Ok(iproduct!(1..h - 1, 1..w - 1)
        .filter(|&(r, c)| grid[r][c] == 'A')
        .filter(|&(r, c)| {
            // check that both diagonals contain M and S
            [(-1, -1), (1, -1)].iter().all(|&(dr, dc)| {
                let mut mask = 0u32;

                // mul delta by 1/-1 to invert and get opposite corner on same diagonal
                [1, -1].iter().for_each(|sign| {
                    let nr = r.checked_add_signed(dr * sign).unwrap();
                    let nc = c.checked_add_signed(dc * sign).unwrap();
                    mask.set_bit(grid[nr][nc] as u32 - 'A' as u32);
                });
                mask & ms_mask == ms_mask
            })
        })
        .count())
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, usize, usize) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let h = grid.len();
    let w = grid[0].len();
    (grid, h, w)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day04.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(2521, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1912, ans);
    }
}
