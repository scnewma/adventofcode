use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let positions = parse_input(input);
    solve(&positions, |start, end| end - start)
}

pub fn part02(input: &str) -> i64 {
    let positions = parse_input(input);
    // adjusts the range to begin at 1 then calculates the sum of integers
    solve(&positions, |start, end| {
        let a = 1;
        let i = end - start; // adjustment for 1-based range
        let n = i;
        (n * (a + i)) / 2
    })
}

fn solve<F>(positions: &[i64], fuel_calc: F) -> i64
where
    F: Fn(i64, i64) -> i64,
{
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();
    let mut min_fuel = i64::MAX;
    (min_pos..max_pos).for_each(|desired_pos| {
        let mut fuel = 0;
        positions.iter().for_each(|crab_pos| {
            let start = desired_pos.min(*crab_pos);
            let end = desired_pos.max(*crab_pos);
            fuel += fuel_calc(start, end);
        });
        min_fuel = min_fuel.min(fuel);
    });
    min_fuel
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day07.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(364898, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(104149091, ans);
    }
}
