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

    let mut completed_paths = 0;
    let mut q = VecDeque::new();
    for v in graph.get("you").unwrap_or(&vec![]) {
        q.push_back(*v);
    }
    while let Some(v) = q.pop_front() {
        if v == "out" {
            completed_paths += 1;
            continue;
        }
        for u in graph.get(v).unwrap() {
            q.push_back(u);
        }
    }
    Ok(completed_paths)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut nodes = HashSet::new();
    let mut in_degree = HashMap::new();
    for line in input.lines() {
        let (src, dests) = line.split_once(": ").unwrap();
        nodes.insert(src);
        let dests: Vec<_> = dests.split_whitespace().collect();
        for dest in dests {
            graph.entry(src).or_default().push(dest);
            nodes.insert(dest);
            in_degree.entry(dest).and_modify(|n| *n += 1).or_insert(1);
        }
    }

    let mut queue = VecDeque::new();
    for v in nodes.iter() {
        if *in_degree.get(v).unwrap_or(&0) == 0 {
            queue.push_back(*v);
        }
    }

    let mut ways: HashMap<(&str, u8), usize> = HashMap::new();
    ways.insert(("svr", 0), 1);

    const MASKS: [u8; 4] = [
        0b00, // none seen
        0b01, // dac seen
        0b10, // fft seen
        0b11, // dac and fft seen
    ];

    // walk the graph topologically
    while let Some(v) = queue.pop_front() {
        for mask in MASKS {
            let count = *ways.get(&(v, mask)).unwrap_or(&0);
            if count == 0 {
                continue;
            }

            if let Some(children) = graph.get(v) {
                for child in children {
                    let mut next_mask = mask;
                    if *child == "dac" {
                        next_mask |= 0b01;
                    }
                    if *child == "fft" {
                        next_mask |= 0b10;
                    }
                    *ways.entry((child, next_mask)).or_insert(0) += count;
                }
            }
        }

        // advance topological walk
        if let Some(children) = graph.get(v) {
            for child in children {
                in_degree.entry(child).and_modify(|n| *n -= 1);
                if in_degree[child] == 0 {
                    queue.push_back(child);
                }
            }
        }
    }

    Ok(*ways.get(&("out", 0b11)).expect("no solution"))
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
        assert_eq!(372918445876116, ans);
    }
}
