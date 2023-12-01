pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<i64> {
    Ok(0)
}

pub fn part02(input: &str) -> anyhow::Result<i64> {
    Ok(0)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const SAMPLE: &'static str = include_str!("../inputs/day04.sample.txt");
//     const INPUT: &'static str = include_str!("../inputs/day04.input.txt");

//     #[test]
//     fn test_part_one_sample() {
//         let ans = part01(SAMPLE).unwrap();
//         assert_eq!(24000, ans);
//     }

//     #[test]
//     fn test_part_one() {
//         let ans = part01(INPUT).unwrap();
//         assert_eq!(69501, ans);
//     }

//     #[test]
//     fn test_part_two_sample() {
//         let ans = part02(SAMPLE).unwrap();
//         assert_eq!(45000, ans);
//     }

//     #[test]
//     fn test_part_two() {
//         let ans = part02(INPUT).unwrap();
//         assert_eq!(202346, ans);
//     }
// }
