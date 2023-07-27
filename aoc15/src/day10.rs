use std::fmt::Write;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    solve(input, 40)
}

pub fn part02(input: &str) -> usize {
    solve(input, 50)
}

fn solve(input: &str, n: usize) -> usize {
    let mut encoded = input.trim().to_string();
    for _ in 0..n {
        encoded = look_and_say(encoded.as_str());
    }
    encoded.len()
}

fn look_and_say(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    if input.is_empty() {
        return encoded;
    }

    let mut digits = input.chars();
    let mut prev = digits.next().unwrap();
    let mut n = 1;
    for digit in digits {
        if digit == prev {
            n += 1;
        } else {
            write!(&mut encoded, "{}", n).unwrap();
            encoded.push(prev);
            n = 1;
            prev = digit;
        }
    }
    write!(&mut encoded, "{}{}", n, prev).unwrap();
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_look_and_say(#[case] input: &str, #[case] expected: &str) {
        let ans = look_and_say(input);
        assert_eq!(expected, ans);
    }
}
