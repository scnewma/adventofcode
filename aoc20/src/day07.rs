use fxhash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    // child -> [parent]
    let mut graph: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    graph.reserve(512);
    for (parent, child, _) in parse_input(input) {
        graph.entry(child).or_default().push(parent);
    }

    let mut queue = VecDeque::with_capacity(graph.len());
    queue.push_front("shiny gold");
    let mut seen = FxHashSet::default();
    seen.reserve(graph.len());
    let mut num_bags = 0;
    while let Some(curr) = queue.pop_front() {
        for &v in graph.get(curr).into_iter().flatten() {
            if !seen.insert(v) {
                continue;
            }
            num_bags += 1;
            queue.push_back(v);
        }
    }

    Ok(num_bags)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    fn count_bags<'a>(
        bag: &'a str,
        graph: &FxHashMap<&'a str, Vec<(&'a str, usize)>>,
        memo: &mut FxHashMap<&'a str, usize>,
    ) -> usize {
        if let Some(&count) = memo.get(bag) {
            return count;
        }

        let Some(children) = graph.get(bag) else {
            return 0;
        };

        let mut count = 0;
        for &(child, n) in children {
            count += n * (1 + count_bags(child, graph, memo));
        }
        memo.insert(bag, count);
        count
    }

    // parent -> [(child, count)]
    let mut graph: FxHashMap<&str, Vec<(&str, usize)>> = FxHashMap::default();
    graph.reserve(512);
    for (parent, child, n) in parse_input(input) {
        graph
            .entry(parent)
            .or_insert_with(|| Vec::with_capacity(4))
            .push((child, n));
    }

    let mut memo = FxHashMap::default();
    memo.reserve(graph.len());
    Ok(count_bags("shiny gold", &graph, &mut memo))
}

fn parse_input(input: &str) -> impl Iterator<Item = (&str, &str, usize)> {
    input.lines().flat_map(|line| {
        let (lhs, rhs) = line.split_once(" contain ").unwrap();
        let parent = lhs.strip_suffix(" bags").unwrap();

        rhs.split(", ").filter_map(move |mut child| {
            if child == "no other bags." {
                return None;
            }

            // trim prefix
            let n = (child.as_bytes()[0] - b'0') as usize;
            child = &child[2..];

            // trim suffix
            child = child.strip_suffix('.').unwrap_or(child);
            child = child
                .strip_suffix(" bags")
                .or_else(|| child.strip_suffix(" bag"))
                .unwrap();

            Some((parent, child, n))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day07.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(172, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(39645, ans);
    }
}
