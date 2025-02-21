pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let mass: usize = line.parse().unwrap();
            mass / 3 - 2
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    fn fuel_req(mass: usize) -> usize {
        let fuel = (mass / 3).saturating_sub(2);
        if fuel == 0 { 0 } else { fuel + fuel_req(fuel) }
    }

    Ok(input
        .lines()
        .map(|line| {
            let mass: usize = line.parse().unwrap();
            fuel_req(mass)
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day01.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3464458, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(5193796, ans);
    }
}
