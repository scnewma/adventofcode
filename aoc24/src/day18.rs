use std::collections::VecDeque;

use fxhash::FxHashSet;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input),
    })
}

// const SIZE: isize = 6;
// const SIMULATE_N: usize = 12;
const SIZE: isize = 70;
const SIMULATE_N: usize = 1024;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let bytes: FxHashSet<(isize, isize)> = iter_bytes(input).take(SIMULATE_N).collect();
    // -1 b/c we are counting steps needed, not path length
    Ok(shortest_path(&bytes).unwrap().len() - 1)
}

pub fn part02(input: &str) -> String {
    let mut fallen: FxHashSet<(isize, isize)> = iter_bytes(input).collect();
    let mut bytes = iter_bytes(input).rev();

    let mut last_byte = (0, 0);
    while shortest_path(&fallen).is_none() {
        last_byte = bytes.next().unwrap();
        fallen.remove(&last_byte);
    }
    format!("{},{}", last_byte.0, last_byte.1)
}

fn shortest_path(bytes: &FxHashSet<(isize, isize)>) -> Option<FxHashSet<(isize, isize)>> {
    let end = (SIZE, SIZE);

    let mut q = VecDeque::new();
    q.push_back((0, 0, FxHashSet::default()));
    let mut visited = FxHashSet::default();
    while let Some((r, c, mut steps)) = q.pop_front() {
        if !visited.insert((r, c))
            || bytes.contains(&(r, c))
            || !(0..=SIZE).contains(&r)
            || !(0..=SIZE).contains(&c)
        {
            continue;
        }

        steps.insert((r, c));
        if (r, c) == end {
            return Some(steps);
        }

        for (dr, dc) in crate::DELTAS4 {
            q.push_back((r + dr, c + dc, steps.clone()));
        }
    }
    None
}

fn iter_bytes(input: &str) -> impl DoubleEndedIterator<Item = (isize, isize)> + '_ {
    input.lines().map(|s| {
        let (r, c) = s.split_once(',').unwrap();
        let r: isize = r.parse().unwrap();
        let c: isize = c.parse().unwrap();
        (r, c)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day18.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(324, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!("46,23", ans);
    }
}
