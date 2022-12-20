use std::{cmp::Ordering, collections::HashMap};

use itertools::process_results;

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<i32> {
    let file: Vec<i32> = process_results(input.lines().map(|line| line.parse::<i32>()), |it| {
        it.collect()
    })?;
    let mut zero_pair = (0, 0);
    let mut positions = HashMap::new();
    for (i, num) in file.iter().enumerate() {
        positions.insert((i, *num), i);
        if *num == 0 {
            zero_pair = (i, *num);
        }
    }
    let mut mixed: Vec<_> = file.clone().into_iter().enumerate().collect();
    for (i, num) in file.iter().enumerate() {
        // println!(
        //     "current={:?} i={i} num={num}",
        //     mixed.iter().map(|(_, n)| n).collect::<Vec<_>>()
        // );
        let current_pos = *positions.get(&(i, *num)).unwrap();
        let max_rotation = num.abs() as usize % file.len();
        // println!("  current_pos={current_pos} max_rotation={max_rotation}");
        match num.cmp(&0) {
            Ordering::Equal => (),
            Ordering::Less => {
                for j in 0..max_rotation {
                    let j_idx = index_rotate_left(current_pos, j, file.len());
                    let next_idx = index_rotate_left(current_pos, j + 1, file.len());
                    // println!("  mixed[{}] = mixed[{}]", j_idx, next_idx);
                    mixed[j_idx] = mixed[next_idx];
                    positions.insert(mixed[j_idx], j_idx);
                }
                let new_pos = index_rotate_left(current_pos, max_rotation, file.len());
                mixed[new_pos] = (i, *num);
                positions.insert((i, *num), new_pos);
            }
            Ordering::Greater => {
                for j in 0..max_rotation {
                    let j_idx = (current_pos + j) % file.len();
                    let next_idx = (current_pos + j + 1) % file.len();
                    mixed[j_idx] = mixed[next_idx];
                    positions.insert(mixed[j_idx], j_idx);
                }
                let new_pos = (current_pos + max_rotation) % file.len();
                mixed[new_pos] = (i, *num);
                positions.insert((i, *num), new_pos);
            }
        }
    }

    let idx_zero = *positions.get(&zero_pair).unwrap();
    Ok([1000, 2000, 3000]
        .iter()
        .map(|idx| {
            let n_after_zero = idx % mixed.len();
            let final_idx = (idx_zero + n_after_zero) % mixed.len();
            mixed[final_idx].1
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    Ok(0)
}

fn index_rotate_left(pos: usize, amt: usize, max: usize) -> usize {
    if amt > pos {
        max - (amt - pos)
    } else {
        pos - amt
    }
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

    // #[test]
    // fn test_part_one() {
    //     let ans = part01(INPUT).unwrap();
    //     assert_eq!(1766, ans);
    // }

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
