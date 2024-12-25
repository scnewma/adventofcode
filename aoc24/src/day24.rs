use bittle::Bits;
use fxhash::FxHashMap;
use itertools::Itertools;

use crate::topsort::TopSort;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?,
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

    Ok(decode(&wires, 'z'))
}

// This doesn't actually solve the input, it's just code that I used to manually solve the problem.
// Conceptually, this is a binary adder problem. I noticed that many of the z outputs (which are
// the bits of the final sum) were not XORs as they should be:
//
// ❯ cat aoc24/inputs/day24.input.txt | grep -v XOR | grep z
// cdc OR stq -> z21
// x12 AND y12 -> z12
// jbr AND wcs -> z33
// mgw OR bds -> z45 (OK - final carry)
//
// So, I created a graphviz and manually debugged what was wrong with those binary adders. Turns
// out that the last bit (z45) was correct since it was the carry-out of the 44th bit.
//
// After fixing 3/4 of those, I set all of the bits in the input numbers (x and y) to 1 to force
// carry through the chain. I then checked the result against the expected result and found the
// last incorrect adder in the chain.
//
// Helpful Details:
//   https://www.electronics-tutorials.ws/combination/comb_7.html
//
//   Half adder:
//   SUM = A XOR B
//   CARRY = A AND B
//
//   Full adder:
//   SUM = (A XOR B) XOR Cin
//   CARRY-OUT = A AND B OR Cin(A XOR B)
pub fn part02(input: &str) -> anyhow::Result<String> {
    let swaps = FxHashMap::from_iter([
        ("nhn", "z21"),
        ("z21", "nhn"),
        ("vdc", "z12"),
        ("z12", "vdc"),
        ("gst", "z33"),
        ("z33", "gst"),
        ("khg", "tvb"),
        ("tvb", "khg"),
    ]);

    let (wires, gates) = input.split_once("\n\n").unwrap();
    // for the purposes of verifying the binary adder, we set all the input wires to 1 so we can
    // determine where the binary adder is broken
    let mut wires: FxHashMap<&str, bool> = wires
        .lines()
        .map(|s| {
            let (wire, _value) = s.split_once(": ").unwrap();
            // let v = value.parse::<u8>().unwrap();
            (wire, true)
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

    // let mut file = File::create("./aoc24/src/day24.dot")?;

    let mut topsort = TopSort::default();
    let mut ops: FxHashMap<&str, (&str, &str, &str)> = FxHashMap::default();
    // writeln!(&mut file, "digraph G {{")?;
    for gate in &gates {
        let out = swaps.get(gate.3).cloned().unwrap_or(gate.3);

        topsort.add_dependency(gate.0, out);
        topsort.add_dependency(gate.2, out);
        assert!(
            !ops.contains_key(out),
            "duplicate output wire :: out = {out}"
        );
        ops.insert(out, (gate.0, gate.1, gate.2));
        // let color = match gate.1 {
        //     "AND" => "green",
        //     "OR" => "blue",
        //     "XOR" => "red",
        //     _ => unreachable!(),
        // };
        // writeln!(&mut file, "{} [color = {}]", out, color)?;
        // writeln!(&mut file, "{} -> {};", gate.0, out)?;
        // writeln!(&mut file, "{} -> {};", gate.2, out)?;
    }
    // writeln!(&mut file, "}}")?;

    let x = decode(&wires, 'x');
    let y = decode(&wires, 'y');

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

    let z = decode(&wires, 'z');

    let xz = x + y;
    for i in 0..64u32 {
        if xz.test_bit(i) == z.test_bit(i) {
            continue;
        }
        println!("bit #{i} incorrect");
    }

    println!("x = {x}, y = {y}");
    println!("ez = {:b}", x + y);
    println!(" z = {z:b}");

    Ok(swaps.keys().sorted().join(","))
}

fn decode(wires: &FxHashMap<&str, bool>, starting_with: char) -> usize {
    let mut n = 0;
    for (_, value) in wires
        .iter()
        .filter(|(wire, _)| wire.starts_with(starting_with))
        .sorted()
        .rev()
    {
        n <<= 1;
        n |= *value as usize;
    }
    n
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
        assert_eq!("gst,khg,nhn,tvb,vdc,z12,z21,z33", ans);
    }
}
