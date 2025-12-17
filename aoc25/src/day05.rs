use std::ops::RangeInclusive;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (ranges, ingredients) = parse_input(input);
    let merged = merge_ranges(ranges);

    Ok(ingredients
        .into_iter()
        .filter(|i| merged.iter().any(|r| r.contains(i)))
        .count())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(ranges);

    Ok(merged
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum())
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged = vec![ranges.first().unwrap().clone()];
    for range in &ranges[1..] {
        let last = merged.last().unwrap().clone();
        if last.contains(range.start()) {
            let new = RangeInclusive::new(*last.start(), *last.end().max(range.end()));
            let len = merged.len();
            merged[len - 1] = new;
        } else {
            merged.push(range.clone());
        }
    }
    merged
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ingredients: Vec<usize> = ingredients.lines().map(|l| l.parse().unwrap()).collect();
    let ranges: Vec<_> = ranges
        .lines()
        .map(|ln| {
            let (l, r) = ln.split_once('-').unwrap();
            let l: usize = l.parse().unwrap();
            let r: usize = r.parse().unwrap();
            assert!(l <= r, "line: {ln}");
            l..=r
        })
        .collect();
    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day05.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(664, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(350780324308385, ans);
    }
}
