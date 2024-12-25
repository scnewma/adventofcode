use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let initial_pop = parse_input(input).unwrap();
    simulate(&initial_pop, 80)
}

pub fn part02(input: &str) -> i64 {
    let initial_pop = parse_input(input).unwrap();
    simulate(&initial_pop, 256)
}

fn simulate(inital_pop: &[i64], days: i64) -> i64 {
    // pop represents the population of lanternfish by grouping lanternfish into groups by age
    let mut pop = [0; 9];

    // add initial population to aged population array
    inital_pop.iter().for_each(|i| pop[*i as usize] += 1);

    // simulate days
    (0..days).for_each(|_| {
        pop.rotate_left(1);
        pop[6] += pop[8];
    });

    pop.iter().sum()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<i64>> {
    Ok(input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day06.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(393019, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(1757714216975, ans);
    }
}
