use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<i64> {
    let input = parse_input(input)?;
    let mut prev = &input[0];
    let mut inc = 0;
    for n in input.iter().skip(1) {
        if n > prev {
            inc += 1;
        }
        prev = n;
    }
    Ok(inc)
}

pub fn part02(input: &str) -> anyhow::Result<i64> {
    let input = parse_input(input)?;
    let mut prev_win = input[0] + input[1] + input[2];
    let mut inc = 0;
    for i in 1..(input.len() - 2) {
        let sum = input[i] + input[i + 1] + input[i + 2];
        if sum > prev_win {
            inc += 1;
        }
        prev_win = sum
    }
    Ok(inc)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<i64>> {
    let mut nums = Vec::new();
    for line in input.lines() {
        let n: i64 = line.parse()?;
        nums.push(n);
    }
    Ok(nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day01.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1466, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1491, ans);
    }
}
