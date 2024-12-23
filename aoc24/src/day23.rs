use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut graph = FxHashMap::<&str, Vec<&str>>::default();
    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        graph.entry(l).or_default().push(r);
        graph.entry(r).or_default().push(l);
    }

    let mut ans = 0;
    for (a, b, c) in graph.keys().tuple_combinations() {
        let yes = (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
            && graph[a].contains(b)
            && graph[a].contains(c)
            && graph[b].contains(c);
        if yes {
            ans += 1;
        }
    }

    Ok(ans)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day23.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1194, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
