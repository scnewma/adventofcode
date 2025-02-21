use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    solve(input, 4)
}

pub fn part02(input: &str) -> usize {
    solve(input, 14)
}

fn solve(input: &str, num_distinct: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut counts = [0; 26];
    let mut left = 0;
    let mut right = 0;
    while right < chars.len() {
        counts[chars[right] as usize - 'a' as usize] += 1;
        if right - left > num_distinct - 1 {
            counts[chars[left] as usize - 'a' as usize] -= 1;
            left += 1;
        }
        if counts.iter().filter(|cnt| **cnt == 1).count() == num_distinct {
            break;
        }
        right += 1;
    }

    // + 1 because right is an index and the problem wants total characters processed
    right + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../inputs/day06.sample.txt");
    const INPUT: &str = include_str!("../inputs/day06.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(7, ans);
    }

    #[test]
    fn test_part_one_sample2() {
        let ans = part01("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(5, ans);
    }

    #[test]
    fn test_part_one_sample3() {
        let ans = part01("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(6, ans);
    }

    #[test]
    fn test_part_one_sample4() {
        let ans = part01("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(10, ans);
    }

    #[test]
    fn test_part_one_sample5() {
        let ans = part01("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(11, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(1876, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(19, ans);
    }

    #[test]
    fn test_part_two_sample2() {
        let ans = part02("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(23, ans);
    }

    #[test]
    fn test_part_two_sample3() {
        let ans = part02("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(23, ans);
    }

    #[test]
    fn test_part_two_sample4() {
        let ans = part02("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(29, ans);
    }

    #[test]
    fn test_part_two_sample5() {
        let ans = part02("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(26, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(2202, ans);
    }
}
