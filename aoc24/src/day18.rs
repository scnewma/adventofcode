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
    let mut bytes = iter_bytes(input);

    let mut fallen = FxHashSet::default();
    let mut path = shortest_path(&fallen).unwrap();
    loop {
        let byte = bytes.next().unwrap();
        fallen.insert(byte);

        // if we blocked the current path, we try and find a new path
        if path.contains(&byte) {
            match shortest_path(&fallen) {
                Some(p) => path = p,
                None => break format!("{},{}", byte.0, byte.1),
            }
        }
    }
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

fn iter_bytes(input: &str) -> impl Iterator<Item = (isize, isize)> + '_ {
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
