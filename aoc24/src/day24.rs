use fxhash::FxHashMap;
use itertools::Itertools;

use crate::topsort::TopSort;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (wires, gates) = input.split_once("\n\n").unwrap();
    let mut wires: FxHashMap<&str, bool> = wires
        .lines()
        .map(|s| {
            let (wire, value) = s.split_once(": ").unwrap();
            let v = value.parse::<u8>().unwrap();
            (wire, v == 1)
        })
        .collect();
    let gates: Vec<(&str, &str, &str, &str)> = gates
        .lines()
        .map(|s| {
            let mut it = s.split_whitespace();
            let lhs = it.next().unwrap();
            let op = it.next().unwrap();
            let rhs = it.next().unwrap();
            it.next(); // skip ->
            let out = it.next().unwrap();
            (lhs, op, rhs, out)
        })
        .collect();

    let mut topsort = TopSort::default();
    let mut ops: FxHashMap<&str, (&str, &str, &str)> = FxHashMap::default();
    for gate in &gates {
        topsort.add_dependency(gate.0, gate.3);
        topsort.add_dependency(gate.2, gate.3);
        assert!(!ops.contains_key(gate.3));
        ops.insert(gate.3, (gate.0, gate.1, gate.2));
    }

    for wire in topsort {
        if !ops.contains_key(wire) {
            continue;
        }
        let (lhs, op, rhs) = ops[wire];
        let res = match op {
            "AND" => wires[lhs] && wires[rhs],
            "OR" => wires[lhs] || wires[rhs],
            "XOR" => wires[lhs] != wires[rhs],
            _ => unreachable!("invalid op {op}"),
        };
        wires.insert(wire, res);
    }

    let mut res = 0;
    for (_, value) in wires
        .into_iter()
        .filter(|(wire, _)| wire.starts_with('z'))
        .sorted()
        .rev()
    {
        res <<= 1;
        res |= value as usize;
    }
    Ok(res)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day24.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(51410244478064, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
