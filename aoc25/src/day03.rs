pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    solve(input, 2)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    solve(input, 12)
}

fn solve(input: &str, n_turn_on: usize) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .map(|line| max_joltage(&line.chars().collect::<Vec<char>>(), n_turn_on))
        .sum())
}

fn max_joltage(battery_bank: &[char], n_turn_on: usize) -> usize {
    let mut joltage = 0;
    let mut start = 0;
    for reserved in (0..n_turn_on).rev() {
        let mut battery_no = '0';
        for (i, battery) in battery_bank
            .iter()
            .enumerate()
            .take(battery_bank.len() - reserved)
            .skip(start)
        {
            if *battery > battery_no {
                battery_no = *battery;
                start = i + 1;
            }
        }
        joltage = joltage * 10 + battery_no.to_digit(10).unwrap() as usize;
    }
    joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day03.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(17109, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(169347417057382, ans);
    }
}
