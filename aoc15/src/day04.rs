use md5::{Digest, Md5};
use std::io::Write;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    let mut buf = Default::default();
    let mut hasher = Md5::new();
    for i in 0.. {
        hasher.update(input);
        write!(&mut hasher, "{}", i).unwrap();
        hasher.finalize_into_reset(&mut buf);
        if buf[0] | buf[1] | (buf[2] >> 4) == 0 {
            return i;
        }
    }
    unreachable!("no solution found")
}

pub fn part02(input: &str) -> usize {
    let mut buf = Default::default();
    let mut hasher = Md5::new();
    hasher.update(input);
    for i in 0.. {
        let mut hasher = hasher.clone();
        write!(&mut hasher, "{}", i).unwrap();
        hasher.finalize_into_reset(&mut buf);
        if buf[0] | buf[1] | buf[2] == 0 {
            return i;
        }
    }
    unreachable!("no solution found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    #[case("iwrupvqb", 346386)]
    fn test_part01(#[case] input: &str, #[case] expected: usize) {
        let ans = part01(input);
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case("iwrupvqb", 9958218)]
    fn test_part02(#[case] input: &str, #[case] expected: usize) {
        let ans = part02(input);
        assert_eq!(expected, ans);
    }
}
