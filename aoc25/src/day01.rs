pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut dial = 50isize;
    let mut nzeros = 0;
    for line in input.lines() {
        let (dir, dist) = line.split_at(1);
        // println!("dir: {}, dist: {}, dial: {}", dir, dist, dial);
        let dist = dist.parse::<usize>()? % 100;
        match dir {
            "L" => {
                dial = dial.checked_sub_unsigned(dist).unwrap();
                if dial < 0 {
                    dial = 100 - dial.abs();
                }
            }
            "R" => dial = dial.checked_add_unsigned(dist).unwrap() % 100,
            _ => anyhow::bail!("invalid direction"),
        }
        nzeros += if dial == 0 { 1 } else { 0 };
    }
    Ok(nzeros)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day01.input.txt");

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
