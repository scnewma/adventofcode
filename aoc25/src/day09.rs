use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut tiles = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse::<isize>()?, y.parse::<isize>()?);
        tiles.push((x, y));
    }

    let mut max_area = 0;
    for combo in tiles.into_iter().combinations(2) {
        let (a, b) = (combo[0], combo[1]);

        let area = (((a.0 - b.0).abs()+1) * ((a.1 - b.1).abs()+1)) as usize;
        max_area = max_area.max(area);
    }
    Ok(max_area)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day09.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(4771532800, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
