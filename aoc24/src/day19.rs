pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (patterns, towels) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();

    Ok(towels.lines().filter(|t| is_possible(&patterns, t)).count())
}

fn is_possible(patterns: &[&str], towel: &str) -> bool {
    for pattern in patterns {
        if towel == *pattern {
            return true;
        }

        if towel.starts_with(pattern) && is_possible(patterns, &towel[pattern.len()..]) {
            return true;
        }
    }
    false
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day19.input.txt");

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
