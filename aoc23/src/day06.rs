use anyhow::Context;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let mut lines = input.lines();

    let mut get_numbers = || -> Vec<u64> {
        let (_, rest) = lines.next().unwrap().split_once(':').unwrap();
        let ns: Vec<u64> = rest
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        ns
    };

    let times = get_numbers();
    let distances = get_numbers();

    let ans = (0..times.len())
        .map(|i| winning_strategies(times[i], distances[i]))
        .product();
    Ok(ans)
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let mut lines = input.lines();

    let mut get_number = || -> anyhow::Result<u64> {
        let n: u64 = lines
            .next()
            .context("no line")?
            .split_once(':')
            .map(|(_, rest)| rest)
            .context("wrong format")?
            .replace(' ', "")
            .parse()?;
        Ok(n)
    };
    let time = get_number()?;
    let distance = get_number()?;

    Ok(winning_strategies(time, distance))
}

fn winning_strategies(time: u64, min_distance: u64) -> u64 {
    let mut winning_strategies = 0;
    for t in 1..time {
        let left = time - t;
        let traveled = left * t;
        if traveled > min_distance {
            winning_strategies += 1;
        }
    }
    winning_strategies
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day06.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(2344708, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(30125202, ans);
    }
}
