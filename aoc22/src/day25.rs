use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input),
        part02: part02(input),
    })
}

pub fn part01(input: &str) -> String {
    input
        .lines()
        .map(isize::from_snafu)
        .sum::<isize>()
        .to_snafu()
}

// only making this a standalone fn so that the generated benchmarks can assume it's here
pub fn part02(_input: &str) -> String {
    "A Big Smoothie!".to_string()
}

trait Snafu {
    fn from_snafu(s: &str) -> Self;
    fn to_snafu(self) -> String;
}

impl Snafu for isize {
    fn from_snafu(s: &str) -> Self {
        s.chars()
            .rev()
            .map(|ch| match ch {
                '2' | '1' | '0' => ch.to_digit(10).unwrap() as isize,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            })
            .enumerate()
            .map(|(pos, d)| 5isize.pow(pos as u32) * d)
            .sum()
    }

    fn to_snafu(self) -> String {
        let mut pow = 0;
        while 5isize.pow(pow) < self {
            pow += 1;
        }

        let mut snafu = String::new();
        let mut rem = self;
        for pow in (0..=pow).rev() {
            let place = 5isize.pow(pow);
            if rem == 0 {
                snafu.push('0');
                continue;
            }

            // summary:
            // we greedily take as many of each place as long as taking that many wouldn't exceed
            // the capacity that we could fill with the rest of the remaining places (which is
            // place / 2 since half of the value digits are negative)

            if rem.abs().abs_diff(place * 2) <= place as usize / 2 {
                if rem < 0 {
                    snafu.push('=');
                    rem += place * 2;
                } else {
                    snafu.push('2');
                    rem -= place * 2;
                }
            } else if rem.abs().abs_diff(place) <= place as usize / 2 {
                if rem < 0 {
                    snafu.push('-');
                    rem += place;
                } else {
                    snafu.push('1');
                    rem -= place;
                }
            } else {
                // do not add leading zeros
                if !snafu.is_empty() {
                    snafu.push('0');
                }
            }
        }

        snafu
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const SAMPLE: &'static str = include_str!("../inputs/day25.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day25.input.txt");

    #[rstest]
    #[case("1=-0-2", 1747)]
    #[case("12111", 906)]
    #[case("2=0=", 198)]
    #[case("21", 11)]
    #[case("2=01", 201)]
    #[case("111", 31)]
    #[case("20012", 1257)]
    #[case("112", 32)]
    #[case("1=-1=", 353)]
    #[case("1-12", 107)]
    #[case("12", 7)]
    #[case("1=", 3)]
    #[case("122", 37)]
    fn test_from_snafu(#[case] s: &str, #[case] expected: isize) {
        let ans = isize::from_snafu(s);
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case(1747, "1=-0-2")]
    #[case(906, "12111")]
    #[case(198, "2=0=")]
    #[case(11, "21")]
    #[case(201, "2=01")]
    #[case(31, "111")]
    #[case(1257, "20012")]
    #[case(32, "112")]
    #[case(353, "1=-1=")]
    #[case(107, "1-12")]
    #[case(7, "12")]
    #[case(3, "1=")]
    #[case(37, "122")]
    fn test_to_snafu(#[case] i: isize, #[case] expected: String) {
        println!("num={i} expected=\"{expected}\"");
        assert_eq!(expected, i.to_snafu());
    }

    #[test]
    fn test_part_one_sample() {
        assert_eq!("2=-1=0", part01(SAMPLE));
    }

    #[test]
    fn test_part_one() {
        assert_eq!("20-=0=02=-21=00-02=2", part01(INPUT));
    }
}
