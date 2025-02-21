use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    let part01 = part01(input)?.to_string();
    let part02 = part02(input).to_string();

    Ok(SolveInfo { part01, part02 })
}

pub fn part01(input: &str) -> anyhow::Result<i64> {
    let (report, width) = parse_input(input);
    let mut gamma: u16 = 0;
    let mut epsilon: u16 = 0;

    for i in (0..width).rev() {
        let mut ones = 0;
        let mut zeros = 0;

        for num in &report {
            let bit = (num >> i) & 1;
            if bit == 1 {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        if ones > zeros {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    Ok(gamma as i64 * epsilon as i64)
}

pub fn part02(input: &str) -> i64 {
    let (report, width) = parse_input(input);
    oxygen_generator_rating(&report, width) as i64 * co2_scrubber_rating(&report, width) as i64
}

fn parse_input(input: &str) -> (Vec<u16>, usize) {
    let report: Vec<u16> = input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).expect("failed to parse line"))
        .collect();
    let bit_width = input.lines().take(1).next().unwrap().len();
    (report, bit_width)
}

fn find_rating(report: &[u16], width: usize, cond: Cond) -> u16 {
    let mut report = report.to_owned();
    for i in (0..width).rev() {
        let mut ones = 0;
        let mut zeros = 0;

        for num in report.iter() {
            let bit = (num >> i) & 1;
            if bit == 1 {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        match cond {
            Cond::MostCommon => {
                if ones >= zeros {
                    report.retain(|num| (num >> i) & 1 == 1);
                } else {
                    report.retain(|num| (num >> i) & 1 == 0);
                }
            }
            Cond::LeastCommon => {
                if ones < zeros {
                    report.retain(|num| (num >> i) & 1 == 1);
                } else {
                    report.retain(|num| (num >> i) & 1 == 0);
                }
            }
        }

        if report.len() == 1 {
            return report[0];
        }
    }
    panic!("not found")
}

enum Cond {
    MostCommon,
    LeastCommon,
}

fn oxygen_generator_rating(report: &[u16], width: usize) -> u16 {
    find_rating(report, width, Cond::MostCommon)
}

fn co2_scrubber_rating(report: &[u16], width: usize) -> u16 {
    find_rating(report, width, Cond::LeastCommon)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day03.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3009600, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(6940518, ans);
    }
}
