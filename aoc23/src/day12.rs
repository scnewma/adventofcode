use std::collections::HashMap;

use arrayvec::ArrayVec;

use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let input: Vec<(&str, ArrayVec<u32, 30>)> = input
        .lines()
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(l, r)| (l, r.split(',').map(|s| s.parse().unwrap()).collect()))
        .collect_vec();

    let mut total = 0;
    for (row, summary) in input {
        total += permutations(row, &summary, &mut HashMap::new());
    }
    Ok(total)
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let input: Vec<(String, Vec<u32>)> = input
        .lines()
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(l, r)| (l, r.split(',').map(|s| s.parse().unwrap()).collect_vec()))
        .map(|(mask, summary)| {
            let mask = (0..5).fold(String::new(), |mut acc, i| {
                acc.push_str(mask);
                if i != 4 {
                    acc.push('?');
                }
                acc
            });
            (mask, summary.repeat(5))
        })
        .collect_vec();

    Ok(input
        .iter()
        .map(|(row, summary)| permutations(row, summary, &mut HashMap::new()))
        .sum())
}

fn permutations<'a>(
    mask: &'a str,
    summary: &'a [u32],
    cache: &mut HashMap<(&'a str, &'a [u32]), u64>,
) -> u64 {
    let key = (mask, summary);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    match (
        mask.is_empty(),
        summary.is_empty(),
        mask.chars().contains(&'#'),
    ) {
        // out of both inputs or out of groups and no more broken springs
        (true, true, _) | (_, true, false) => return 1,
        // out of input but still have groups or out of groups but still have broken springs
        (true, false, _) | (_, true, true) => return 0,
        _ => (),
    }

    let mut total = 0;
    let spring = mask.chars().next().unwrap();
    // if '?' treat current position as a '.'
    if spring == '?' || spring == '.' {
        total += permutations(&mask[1..], summary, cache);
    }
    // if '?' treat current position as a '#'
    if spring == '?' || spring == '#' {
        let n = summary[0] as usize;
        if n <= mask.len()
            && !&mask[..n].contains('.')
            && (n == mask.len() || mask.chars().nth(n).unwrap() != '#')
        {
            // we can skip forward to the end of this group
            // +1 because a group must be followed by a '.'
            let start = if n == mask.len() { n } else { n + 1 };
            total += permutations(&mask[start..], &summary[1..], cache);
        }
    }
    cache.insert(key, total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day12.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(7460, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(6720660274964, ans);
    }
}
