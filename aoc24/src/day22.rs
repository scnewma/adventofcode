use fxhash::{FxHashMap, FxHashSet};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let secret_numbers = input.lines().map(|line| line.parse::<usize>().unwrap());
    Ok(secret_numbers
        .map(|secret_number| {
            (0..2000).fold(secret_number, |secret_number, _| {
                next_secret_number(secret_number)
            })
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let secret_numbers = input.lines().map(|line| line.parse::<usize>().unwrap());

    let mut change_seq_prices = FxHashMap::default();

    let mut seen = FxHashSet::default();
    for mut secret_number in secret_numbers {
        seen.clear();

        let mut price_changes = (0isize, 0isize, 0isize, 0isize);
        let mut prev_price = secret_number % 10;
        for i in 0..2000 {
            secret_number = next_secret_number(secret_number);

            let price = secret_number % 10;
            let change = price as isize - prev_price as isize;
            price_changes = (price_changes.1, price_changes.2, price_changes.3, change);

            if i > 3 && seen.insert(price_changes) {
                change_seq_prices
                    .entry(price_changes)
                    .and_modify(|e| *e += price)
                    .or_insert(price);
            }
            prev_price = price;
        }
    }

    Ok(*change_seq_prices.values().max().unwrap())
}

#[inline]
fn next_secret_number(mut secret_number: usize) -> usize {
    secret_number = secret_number ^ ((secret_number * 64) % 16777216);
    secret_number = secret_number ^ ((secret_number / 32) % 16777216);
    secret_number ^ ((secret_number * 2048) % 16777216)
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
