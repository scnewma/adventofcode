use std::cmp::Ordering;

use itertools::process_results;

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<isize> {
    let file: Vec<isize> =
        process_results(input.lines().map(|line| line.parse::<isize>()), |it| {
            it.collect()
        })?;
    Ok(grove_coordinates(file, 1))
}

const DECRYPTION_KEY: isize = 811589153;

pub fn part02(input: &str) -> anyhow::Result<isize> {
    let file: Vec<isize> =
        process_results(input.lines().map(|line| line.parse::<isize>()), |it| {
            it.map(|n| n * DECRYPTION_KEY).collect()
        })?;
    Ok(grove_coordinates(file, 10))
}

fn grove_coordinates(file: Vec<isize>, rounds: usize) -> isize {
    let mut mixed: Vec<_> = file.clone().into_iter().enumerate().collect();
    for _ in 0..rounds {
        for (i, num) in file.iter().enumerate() {
            // find where this item currently is in the array, accounting for both i and num
            // because the nums could repeat
            let current_pos = mixed.iter().position(|&item| item == (i, *num)).unwrap();

            // from here we move this number to it's new home by removing it from the array and
            // re-inserting it at the new location. all new position calculations are performed on
            // the smaller array

            // verify that there wasn't some sort of bug in previous iterations of the loop
            assert_eq!(mixed.remove(current_pos), (i, *num));
            let next_pos = next_pos(current_pos, *num, &mixed);
            mixed.insert(next_pos, (i, *num));
        }
    }

    // sum up the 1000th, 2000th and 3000th position after 0
    let idx_zero = mixed.iter().position(|&item| item.1 == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|idx| {
            // how far into the array the coordinate would be
            let coord_offset = idx % mixed.len();
            // the final coordinate, adjusted to be coord_offset past zero
            let idx_num = (idx_zero + coord_offset) % mixed.len();
            mixed[idx_num].1
        })
        .sum()
}

fn next_pos<V>(pos: usize, amt: isize, vec: &Vec<V>) -> usize {
    let num_rotations = amt.abs() as usize % vec.len();
    let next_pos = match amt.cmp(&0) {
        Ordering::Equal => pos,
        Ordering::Less => {
            if num_rotations > pos {
                vec.len() - num_rotations + pos
            } else {
                pos - num_rotations
            }
        }
        Ordering::Greater => {
            if pos + num_rotations > vec.len() {
                (pos + num_rotations) % vec.len()
            } else {
                pos + num_rotations
            }
        }
    };
    next_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day20.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day20.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(3, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(7584, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(1623178306, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(4907679608191, ans);
    }
}
