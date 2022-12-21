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
    let mut mixed: Vec<_> = file.clone().into_iter().enumerate().collect();
    for (i, num) in file.iter().enumerate() {
        let current_pos = mixed.iter().position(|&item| item == (i, *num)).unwrap();
        // verify that there wasn't some sort of bug in previous iterations of the loop
        assert_eq!(mixed.remove(current_pos), (i, *num));
        let next_pos = next_pos(current_pos, *num, &mixed);
        mixed.insert(next_pos, (i, *num));
    }

    let idx_zero = mixed.iter().position(|&item| item.1 == 0).unwrap();
    Ok([1000, 2000, 3000]
        .iter()
        .map(|idx| {
            let n_after_zero = idx % mixed.len();
            let final_idx = (idx_zero + n_after_zero) % mixed.len();
            let n = mixed[final_idx].1;
            n
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    Ok(0)
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

    // #[test]
    // fn test_part_two_sample() {
    //     let ans = part02(SAMPLE).unwrap();
    //     assert_eq!(0, ans);
    // }

    // #[test]
    // fn test_part_two() {
    //     let ans = part02(INPUT).unwrap();
    //     assert_eq!(0, ans);
    // }
}
