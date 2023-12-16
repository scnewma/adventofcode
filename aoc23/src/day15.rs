use std::collections::VecDeque;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(input.trim().split(',').map(hash).sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut hashmap = vec![VecDeque::<(&str, usize)>::default(); 256];
    for word in input.trim().split(',') {
        match word.split_once('=') {
            Some((label, rest)) => {
                let v: usize = rest.parse().unwrap();
                let bx = &mut hashmap[hash(label)];

                bx.iter_mut()
                    .find(|(k, _)| *k == label)
                    .map(|(_, val)| *val = v)
                    .or_else(|| {
                        bx.push_back((label, v));
                        None
                    });
            }
            None => {
                assert!(word.ends_with('-'));
                let label = &word[..word.len() - 1];
                let bx = &mut hashmap[hash(label)];

                bx.iter()
                    .position(|(k, _)| k == &label)
                    .and_then(|i| bx.remove(i));
            }
        }
    }

    let focusing_power = hashmap
        .iter()
        .enumerate()
        .map(|(i, bx)| {
            bx.iter()
                .enumerate()
                .map(|(j, (_, v))| (i + 1) * (j + 1) * v)
                .sum::<usize>()
        })
        .sum();
    Ok(focusing_power)
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, ch| ((acc + ch as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day15.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(513214, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(258826, ans);
    }
}
