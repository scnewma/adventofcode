pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<i64> {
    Ok(parse_input(input)
        .map(|nums| predict(nums, Prediction::End))
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<i64> {
    Ok(parse_input(input)
        .map(|nums| predict(nums, Prediction::Start))
        .sum())
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    })
}

enum Prediction {
    Start,
    End,
}

fn predict(nums: Vec<i64>, prediction: Prediction) -> i64 {
    let mut stk: Vec<Vec<i64>> = Vec::new();
    stk.push(nums);
    while !stk.last().unwrap().iter().all(|n| *n == 0) {
        stk.push(compute_deltas(stk.last().unwrap()));
    }

    stk.pop(); // remove 0s

    let mut delta = 0;
    while let Some(nums) = stk.pop() {
        match prediction {
            Prediction::Start => delta = nums.first().unwrap() - delta,
            Prediction::End => delta += nums.last().unwrap(),
        }
    }
    delta
}

fn compute_deltas(nums: &[i64]) -> Vec<i64> {
    let mut hist = vec![];
    for i in 1..nums.len() {
        hist.push(nums[i] - nums[i - 1]);
    }
    hist
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day09.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1904165718, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(964, ans);
    }
}
