use bittle::{Bits, BitsMut};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<isize> {
    let ops = parse_input(input);
    let (acc, _) = run_until_cycle(&ops);
    Ok(acc)
}

pub fn part02(input: &str) -> anyhow::Result<isize> {
    let mut ops = parse_input(input);

    for mut_idx in 0..ops.len() {
        if let Op::Acc = ops[mut_idx].0 {
            continue;
        }

        flip(&mut ops[mut_idx].0);
        let (acc, did_cycle) = run_until_cycle(&ops);
        flip(&mut ops[mut_idx].0);

        if !did_cycle {
            return Ok(acc);
        }
    }
    panic!("no solution found")
}

#[derive(Clone, Copy)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

fn run_until_cycle(ops: &[(Op, isize)]) -> (isize, bool) {
    let mut acc = 0;
    let mut ip = 0;
    let mut seen = vec![0u64; ops.len().div_ceil(u64::BITS as usize)];

    while ip < ops.len() && !seen.test_bit(ip as u32) {
        seen.set_bit(ip as u32);

        let (op, delta) = ops[ip];

        match op {
            Op::Nop => ip += 1,
            Op::Acc => {
                acc += delta;
                ip += 1;
            }
            Op::Jmp => {
                ip = ip.checked_add_signed(delta).expect("delta underflowed ip");
            }
        }
    }

    (acc, ip < ops.len())
}

fn flip(op: &mut Op) {
    *op = match op {
        Op::Nop => Op::Jmp,
        Op::Jmp => Op::Nop,
        Op::Acc => unreachable!(),
    };
}

fn parse_input(input: &str) -> Vec<(Op, isize)> {
    let mut ops = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let (op, delta) = line.split_once(' ').unwrap();
        let op = match op {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            _ => panic!("unknown op {op}"),
        };
        let delta = delta
            .strip_prefix('+')
            .unwrap_or(delta)
            .parse::<isize>()
            .unwrap();
        ops.push((op, delta));
    }
    ops
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day08.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1709, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1976, ans);
    }
}
