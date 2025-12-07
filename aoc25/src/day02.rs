pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut result = 0;
    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<usize>()?;
        let end = end.parse::<usize>()?;

        for id in start..=end {
            let s = id.to_string();
            let (l, r) = s.split_at(s.len() / 2);
            if l == r {
                result += id;
            }
        }
    }
    Ok(result)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day02.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(23039913998, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
