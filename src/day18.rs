use std::collections::HashSet;

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let mut cubes: HashSet<(u32, u32, u32)> = input
        .lines()
        .map(|line| {
            let mut pos = line.splitn(3, ',');
            (
                pos.next().unwrap().parse::<u32>().unwrap(),
                pos.next().unwrap().parse::<u32>().unwrap(),
                pos.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    let mut surface_area = 0;
    for cube in &cubes {
        let mut surface_area_cube = 6;
        for (dx, dy, dz) in [
            (1i32, 0i32, 0i32),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            if let (Some(x), Some(y), Some(z)) = (
                cube.0.checked_add_signed(dx),
                cube.1.checked_add_signed(dy),
                cube.2.checked_add_signed(dz),
            ) {
                let neigh = (x, y, z);
                if cubes.contains(&neigh) {
                    surface_area_cube -= 1;
                }
            }
        }
        surface_area += surface_area_cube;
    }
    Ok(surface_area)
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day18.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day18.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(64, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
