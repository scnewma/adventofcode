use num::integer::Integer;
use std::collections::HashMap;

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
        .filter(|n| n.chars().last().unwrap() == 'A')
        .cloned()
        .map(|n| navigate(moves, &nodes, n, |node| node.chars().last().unwrap() == 'Z'))
        .fold(1u64, |acc, n| acc.lcm(&n)))
}

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (moves, grid) = input.split_once("\n\n").unwrap();
    let mut nodes = HashMap::new();
    for line in grid.lines() {
        let (node, lr) = line.split_once(" = ").unwrap();
        let lr = &lr[1..lr.len() - 1]; // trim "(" ")"
        let (l, r) = lr.split_once(", ").unwrap();
        nodes.insert(node, (l, r));
    }
    (moves, nodes)
}

fn navigate<F>(moves: &str, nodes: &HashMap<&str, (&str, &str)>, start: &str, is_end: F) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut steps = 0;
    let mut current = start;
    let mut moves = moves.chars().cycle();
    while !is_end(current) {
        let lr = nodes.get(current).unwrap();
        let m = moves.next().unwrap();
        current = match m {
            'L' => lr.0,
            'R' => lr.1,
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day08.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(20777, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(13_289_612_809_129, ans);
    }
}
