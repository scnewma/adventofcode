use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(solve(input, false))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(solve(input, true))
}

#[inline]
fn solve(input: &str, enable_concat: bool) -> usize {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let (test, nums) = line.split_once(':').unwrap();
            let test: usize = test.parse().unwrap();
            let nums: Vec<usize> = nums
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (test, nums)
        })
        .filter(|(test, nums)| can_solve(enable_concat, nums, 0, *test, 0))
        .map(|(test, _)| test)
        .sum()
}

fn can_solve(enable_concat: bool, nums: &[usize], index: usize, total: usize, curr: usize) -> bool {
    if index == nums.len() {
        return curr == total;
    }

    if curr > total {
        return false;
    }

    can_solve(enable_concat, nums, index + 1, total, curr + nums[index])
        || (curr != 0 && can_solve(enable_concat, nums, index + 1, total, curr * nums[index]))
        || (enable_concat
            && curr != 0
            && can_solve(
                enable_concat,
                nums,
                index + 1,
                total,
                format!("{}{}", curr, nums[index]).parse().unwrap(),
            ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day07.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(932137732557, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(661823605105500, ans);
    }
}
