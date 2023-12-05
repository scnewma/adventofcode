pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let mut sections = input.split("\n\n");
    let (_, seeds) = sections.next().unwrap().split_once(": ").unwrap();
    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mappings: Vec<_> = sections.map(parse_map).collect();
    let mut closest = u64::max_value();
    for seed in seeds {
        let mut lookup = seed;
        for map in mappings.iter() {
            for m in map {
                let (dst, src, amt) = m;
                if (*src..*src + amt).contains(&lookup) {
                    lookup = dst + (lookup - src);
                    break;
                }
            }
        }
        closest = closest.min(lookup);
    }
    Ok(closest)
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let mut sections = input.split("\n\n");
    let (_, seeds) = sections.next().unwrap().split_once(": ").unwrap();

    let nums: Vec<_> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut seeds = vec![];
    for i in (0..nums.len()).step_by(2) {
        let (src, amt) = (nums[i], nums[i + 1]);
        for i in 0..amt {
            seeds.push(src + i);
        }
    }

    let mappings: Vec<_> = sections.map(parse_map).collect();
    let mut closest = u64::max_value();
    for seed in seeds {
        let mut lookup = seed;
        for map in mappings.iter() {
            for m in map {
                let (dst, src, amt) = m;
                if (*src..*src + amt).contains(&lookup) {
                    lookup = dst + (lookup - src);
                    break;
                }
            }
        }
        closest = closest.min(lookup);
    }
    Ok(closest)
}

fn parse_map(s: &str) -> Vec<(u64, u64, u64)> {
    s.lines()
        .skip(1)
        .map(|line| {
            let r: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (r[0], r[1], r[2])
        })
        .collect()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const SAMPLE: &'static str = include_str!("../inputs/day05.sample.txt");
//     const INPUT: &'static str = include_str!("../inputs/day05.input.txt");

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
