use bittle::{Bits, BitsMut};
use std::{cmp::Ordering, collections::HashSet};

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    let mut combos = HashSet::new();
    combinations(150, &parse_input(input), 0, 0, &mut combos);
    combos.len()
}

pub fn part02(input: &str) -> usize {
    let mut combos = HashSet::new();
    combinations(150, &parse_input(input), 0, 0, &mut combos);
    combos
        .into_iter()
        .map(|c| c.count_ones())
        .fold((u32::max_value(), 0), |(min, n), e| match e.cmp(&min) {
            Ordering::Less => (e, 1),
            Ordering::Equal => (e, n + 1),
            Ordering::Greater => (min, n),
        })
        .1
}

fn combinations(
    rem: usize,
    ctrs: &[usize],
    ctrs_i: usize,
    used: usize,
    combos: &mut HashSet<usize>,
) {
    if rem == 0 {
        combos.insert(used);
        return;
    }
    for i in ctrs_i..ctrs.len() {
        let s = ctrs[i];
        let i = i as u32;
        if !used.test_bit(i) && rem >= s {
            let mut used = used;
            used.set_bit(i);
            combinations(rem - s, ctrs, i as usize, used, combos);
        }
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut ctrs: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    ctrs.sort();
    assert!(ctrs.len() < 64); // length of usize for "used" bit
    ctrs
}
