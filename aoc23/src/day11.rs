use itertools::{iproduct, Itertools};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    Ok(solve(input, 1))
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    Ok(solve(input, 999_999))
}

fn solve(input: &str, expansion: usize) -> u64 {
    let cosmos = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut expanded_rows = vec![];
    let mut expanded_cols = vec![];
    for (r, row) in cosmos.iter().enumerate() {
        if row.iter().all(|ch| *ch == '.') {
            expanded_rows.push(r);
        }
    }
    for col in 0..cosmos[0].len() {
        let mut all_empty = true;
        for row in &cosmos {
            all_empty = all_empty && row[col] == '.'
        }
        if all_empty {
            expanded_cols.push(col);
        }
    }

    let galaxies = iproduct!(0..cosmos.len(), 0..cosmos[0].len())
        .filter(|(row, col)| cosmos[*row][*col] == '#')
        .collect_vec();

    let mut sum = 0u64;
    for (a, b) in galaxies.iter().tuple_combinations() {
        let mut growth = 0;
        for row in (a.0.min(b.0))..(a.0.max(b.0)) {
            if expanded_rows.contains(&row) {
                growth += expansion;
            }
        }
        for col in (a.1.min(b.1))..(a.1.max(b.1)) {
            if expanded_cols.contains(&col) {
                growth += expansion;
            }
        }
        // manhattan distance
        let distance = a.0.abs_diff(b.0) + a.1.abs_diff(b.1) + growth;
        sum += distance as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day11.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(9734203, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(568914596391, ans);
    }
}
