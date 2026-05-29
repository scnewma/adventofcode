pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(solve::<true>(input))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(solve::<false>(input))
}

pub fn solve<const ANYONE: bool>(input: &str) -> usize {
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut group_mask = if ANYONE { 0u32 } else { u32::MAX };
        for line in group.lines() {
            let mut person_mask = 0u32;
            for c in line.chars() {
                person_mask |= 1 << (c as u32 - 'a' as u32);
            }

            if ANYONE {
                group_mask |= person_mask;
            } else {
                group_mask &= person_mask;
            }
        }
        sum += group_mask.count_ones();
    }
    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day06.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(6612, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(3268, ans);
    }
}
