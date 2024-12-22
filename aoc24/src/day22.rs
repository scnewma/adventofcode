use std::collections::VecDeque;

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let secret_numbers = input.lines().map(|line| line.parse::<usize>().unwrap());
    let mut sum = 0;
    for mut secret_number in secret_numbers {
        for _ in 0..2000 {
            let a = secret_number * 64;
            secret_number ^= a;
            secret_number %= 16777216;

            let b = secret_number / 32;
            secret_number ^= b;
            secret_number %= 16777216;

            let c = secret_number * 2048;
            secret_number ^= c;
            secret_number %= 16777216;
        }
        sum += secret_number;
    }
    Ok(sum)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let secret_numbers = input.lines().map(|line| line.parse::<usize>().unwrap());

    let mut change_seq_prices = FxHashMap::<(isize, isize, isize, isize), usize>::default();

    for (_, mut secret_number) in secret_numbers.enumerate() {
        let mut first_sell = FxHashMap::<(isize, isize, isize, isize), usize>::default();
        let mut price_changes = VecDeque::new();
        let mut prev_price = secret_number % 10;
        for _ in 0..2000 {
            let a = secret_number * 64;
            secret_number ^= a;
            secret_number %= 16777216;

            let b = secret_number / 32;
            secret_number ^= b;
            secret_number %= 16777216;

            let c = secret_number * 2048;
            secret_number ^= c;
            secret_number %= 16777216;

            let price = secret_number % 10;
            price_changes.push_back(price as isize - prev_price as isize);
            if price_changes.len() > 4 {
                price_changes.pop_front();
            }
            if let Some((a, b, c, d)) = price_changes.iter().collect_tuple() {
                if !first_sell.contains_key(&(*a, *b, *c, *d)) {
                    first_sell.insert((*a, *b, *c, *d), price);
                }
            }
            prev_price = price;
        }
        for (k, v) in first_sell {
            change_seq_prices
                .entry(k)
                .and_modify(|e| *e += v)
                .or_insert(v);
        }
    }

    return Ok(*change_seq_prices.values().max().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day22.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(13584398738, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1612, ans);
    }
}
