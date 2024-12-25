use std::collections::HashMap;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let (template, rules) = parse_input(input);
    solve(10, &template, &rules)
}

pub fn part02(input: &str) -> i64 {
    let (template, rules) = parse_input(input);
    solve(40, &template, &rules)
}

fn solve(steps: usize, template: &[char], rules: &HashMap<(char, char), char>) -> i64 {
    // aggregate all pairs into a single map entry with count as a value; there is no need to
    // iterate all pairs via a string since all input pairs of the same type produce the same two
    // output pairs
    let mut pairs: HashMap<(char, char), i64> = HashMap::new();

    // seed with initial template
    template.windows(2).for_each(|pair| {
        *pairs.entry((pair[0], pair[1])).or_default() += 1;
    });

    (1..=steps).for_each(|_| {
        let mut step_pairs: HashMap<(char, char), i64> = HashMap::new();
        for (pair, count) in pairs.iter() {
            // a pair (A, B) produces two new pairs (A, x) and (x, B) where x is the resulting char
            // in the insertion rules for the pair (A, B)
            let c = rules.get(pair).unwrap();
            *step_pairs.entry((pair.0, *c)).or_default() += count;
            *step_pairs.entry((*c, pair.1)).or_default() += count;
        }

        pairs = step_pairs;
    });

    // count of individual characters
    let mut counts: HashMap<char, i64> = HashMap::new();
    for ((_, r), count) in pairs.iter() {
        // only count the right char since each char is in two different pairs
        *counts.entry(*r).or_default() += count;
    }
    // add the head of the initial template to the count. this isn't included since we only counted
    // thi right side of each pair. the head of the template will remain stable throughout the
    // mutation
    *counts.entry(*template.iter().next().unwrap()).or_default() += 1;

    // solution is count of most common element - count of least common element
    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();
    max - min
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template.chars().collect();

    let rules = rules
        .lines()
        .flat_map(|s| s.split_once(" -> "))
        .map(|(l, r)| {
            let mut lcs = l.chars();
            (
                (lcs.next().unwrap(), lcs.next().unwrap()),
                r.chars().next().unwrap(),
            )
        })
        .collect();

    (template, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day14.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(3247, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(4110568157153, ans);
    }
}
