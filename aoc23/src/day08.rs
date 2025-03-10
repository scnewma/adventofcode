use fxhash::FxHashMap;
use num::integer::Integer;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let (moves, nodes) = parse_input(input);
    Ok(navigate(moves, &nodes, "AAA", |node| node == "ZZZ"))
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let (moves, nodes) = parse_input(input);

    Ok(nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .cloned()
        .map(|n| navigate(moves, &nodes, n, |node| node.ends_with('Z')))
        .fold(1u64, |acc, n| acc.lcm(&n)))
}

fn parse_input(input: &str) -> (&str, FxHashMap<&str, (&str, &str)>) {
    let (moves, grid) = input.split_once("\n\n").unwrap();
    let mut nodes = FxHashMap::default();
    for line in grid.lines() {
        let (node, lr) = line.split_once(" = ").unwrap();
        let lr = &lr[1..lr.len() - 1]; // trim "(" ")"
        let (l, r) = lr.split_once(", ").unwrap();
        nodes.insert(node, (l, r));
    }
    (moves, nodes)
}

fn navigate<F>(moves: &str, nodes: &FxHashMap<&str, (&str, &str)>, start: &str, is_end: F) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut steps = 0;
    let mut current = start;
    let mut moves = moves.chars().cycle();
    while !is_end(current) {
        current = match moves.next().unwrap() {
            'L' => nodes[current].0,
            'R' => nodes[current].1,
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day08.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(20777, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(13289612809129, ans);
    }
}
