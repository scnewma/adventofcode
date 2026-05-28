use std::collections::{HashMap, HashSet, VecDeque};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (src, dests) = line.split_once(": ").unwrap();
        let dests: Vec<_> = dests.split_whitespace().collect();
        graph.insert(src, dests);
    }

    let mut completed_paths = HashSet::new();
    let mut q = VecDeque::new();
    for v in graph.get("you").unwrap() {
        q.push_back(vec![*v]);
    }
    while let Some(path) = q.pop_front() {
        let v = *path.last().unwrap();
        if v == "out" {
            completed_paths.insert(path);
            continue;
        }
        for u in graph.get(v).unwrap() {
            let mut next = path.clone();
            next.push(u);
            q.push_back(next);
        }
    }
    Ok(completed_paths.len())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day11.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(613, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
