use fxhash::FxHashMap;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (patterns, towels) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();

    Ok(towels
        .lines()
        .filter(|t| count_possible(&patterns, t, &mut FxHashMap::default()) > 0)
        .count())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (patterns, towels) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();

    Ok(towels
        .lines()
        .map(|t| count_possible(&patterns, t, &mut FxHashMap::default()))
        .sum())
}

fn count_possible<'a>(
    patterns: &[&str],
    towel: &'a str,
    cache: &mut FxHashMap<&'a str, usize>,
) -> usize {
    if let Some(count) = cache.get(towel) {
        return *count;
    }

    let mut count = 0;
    for pattern in patterns {
        if towel == *pattern {
            count += 1;
        }

        if towel.ends_with(pattern) {
            count += count_possible(patterns, &towel[..towel.len() - pattern.len()], cache);
        }
    }
    cache.insert(towel, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day19.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(296, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(619970556776002, ans);
    }
}
