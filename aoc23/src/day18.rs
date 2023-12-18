use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let input: Vec<(&str, u64)> = input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            (
                words.next().unwrap(),
                words.next().unwrap().parse().unwrap(),
            )
        })
        .collect_vec();

    Ok(solve(input))
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let input: Vec<(&str, u64)> = input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            // skip part 1 inputs
            words.next().unwrap();
            words.next().unwrap();

            let hex = words.next().unwrap();
            let hex = &hex[2..hex.len() - 1]; // trim "(#" and ")"
            let dir = match hex.chars().last().unwrap() {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => unreachable!(),
            };
            let dist = u64::from_str_radix(&hex[0..5], 16).unwrap();
            (dir, dist)
        })
        .collect_vec();

    Ok(solve(input))
}

fn solve(input: Vec<(&str, u64)>) -> u64 {
    let points = calculate_points(&input);
    let perimeter = input.iter().map(|(_, dist)| dist).sum::<u64>();
    let inner_area = calculate_area(&points);
    // pick's theorem to account for the "half" of the perimeter that is part of the area
    inner_area + (perimeter / 2) + 1
}

fn calculate_points(input: &[(&str, u64)]) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    let mut current = (0, 0);
    for &(dir, dist) in input {
        let dist = dist as i64;
        points.push(current);
        match dir {
            "U" => current.0 -= dist,
            "D" => current.0 += dist,
            "L" => current.1 -= dist,
            "R" => current.1 += dist,
            _ => unreachable!(),
        }
    }
    points
}

// shoelace formula
fn calculate_area(points: &[(i64, i64)]) -> u64 {
    points
        .iter()
        .tuple_windows()
        .fold(0, |acc, (a, b)| acc + (a.0 * b.1) - (a.1 * b.0))
        .unsigned_abs()
        / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day18.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(70253, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(131265059885080, ans);
    }
}
