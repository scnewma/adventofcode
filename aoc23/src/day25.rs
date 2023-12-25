use std::collections::{HashSet, VecDeque};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut edges = HashSet::new();
    let mut nodes = HashSet::new();
    for line in input.lines() {
        let (lhs, rhs) = line.split_once(": ").unwrap();
        nodes.insert(lhs.to_string());
        rhs.split_whitespace().for_each(|s| {
            nodes.insert(s.to_string());
            let mut v = vec![lhs.to_string(), s.to_string()];
            v.sort();
            edges.insert((v[0].clone(), v[1].clone()));
        });
    }

    // print the dot graph
    // println!("graph {{");
    // for (n1, n2) in edges.iter() {
    //     println!("  {} -- {};", n1, n2);
    // }
    // println!("}}");

    // sample
    // for e in [("jqt", "nvd"), ("hfx", "pzl"), ("bvb", "cmg")] {
    for e in [("gsk", "ncg"), ("gmr", "ntx"), ("mrd", "rjs")] {
        edges.remove(&(e.0.to_string(), e.1.to_string()));
    }

    let mut queue = VecDeque::new();
    queue.push_back(nodes.iter().next().unwrap().as_str());

    let mut visited = HashSet::new();

    while let Some(node) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }

        for edge in edges.iter() {
            if edge.0 == node || edge.1 == node {
                queue.push_back(if edge.0 == node { &edge.1 } else { &edge.0 });
            }
        }
    }

    Ok(visited.len() * (nodes.len() - visited.len()))
}

// needs to stay here because of benchmarks
pub fn part02(_input: &str) -> anyhow::Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day25.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(580800, ans);
    }
}
