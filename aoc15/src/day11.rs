use std::collections::HashMap;

use fxhash::FxHashMap;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> String {
    next_valid_password(input.trim())
}

pub fn part02(input: &str) -> String {
    next_valid_password(&next_valid_password(input.trim()))
}

fn next_valid_password(input: &str) -> String {
    let mut prev = String::from(input);
    loop {
        let next = increment(&prev);
        if meets_reqs(&next) {
            return next;
        }
        prev = next;
    }
}

fn increment(old: &str) -> String {
    if old.is_empty() {
        return String::from("a");
    }

    let mut next = String::new();
    let mut chars = old.chars().rev();
    loop {
        let c = next_char(chars.next().unwrap());
        next.push(c);
        if c != 'a' {
            break;
        }
    }
    next.extend(chars);
    next.chars().rev().collect()
}

fn meets_reqs(password: &str) -> bool {
    let mut has_three_straight = false;
    let mut pairs: HashMap<char, Vec<usize>, _> = FxHashMap::default();
    let chars: Vec<_> = password.chars().collect();
    for i in 0..chars.len() {
        let curr = chars[i];
        if curr == 'i' || curr == 'o' || curr == 'l' {
            return false;
        }

        if i > 0 {
            let prev = chars[i - 1];
            if curr == prev {
                pairs.entry(curr).or_default().push(i - 1);
            }
        }

        if i > 1 {
            let cur = curr as u32;
            let prev2 = chars[i - 2] as u32;
            let prev = chars[i - 1] as u32;
            if (cur - 2 == prev - 1) && (cur - 2 == prev2) {
                has_three_straight = true;
            }
        }
    }

    let mut has_two_pair = pairs.len() >= 2;
    if !has_two_pair {
        for (_, indicies) in pairs {
            let first = indicies.first().unwrap();
            let last = indicies.last().unwrap();
            if last - first > 1 {
                has_two_pair = true;
                break;
            }
        }
    }

    has_three_straight && has_two_pair
}

fn next_char(c: char) -> char {
    match c {
        'z' => 'a',
        _ => std::char::from_u32(c as u32 + 1).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("xx", "xy")]
    #[case("xy", "xz")]
    #[case("xz", "ya")]
    #[case("ya", "yb")]
    #[case("azz", "baa")]
    #[case("hxbxwxba", "hxbxwxbb")]
    fn test_increament(#[case] input: &str, #[case] expected: &str) {
        let ans = increment(input);
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    fn test_meets(#[case] input: &str, #[case] expected: bool) {
        let ans = meets_reqs(input);
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_next_valid(#[case] input: &str, #[case] expected: &str) {
        let ans = next_valid_password(input);
        assert_eq!(expected, ans);
    }
}
