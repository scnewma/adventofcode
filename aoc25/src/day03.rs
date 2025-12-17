pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let mut sum = 0;
    for battery_bank in input.lines() {
        let batteries: Vec<char> = battery_bank.chars().collect();
        let mut l = '0';
        let mut li = 0;
        for i in 0..batteries.len() - 1 {
            if batteries[i] > l {
                l = batteries[i];
                li = i;
            }
        }
        let mut r = '0';
        for i in li + 1..batteries.len() {
            if batteries[i] > r {
                r = batteries[i];
            }
        }

        let joltage = l.to_digit(10).unwrap() * 10 + r.to_digit(10).unwrap();
        sum += joltage;
    }
    Ok(sum)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day03.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
