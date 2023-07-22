pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<i64> {
    Ok(iter_floor_deltas(input).sum())
}

pub fn part02(input: &str) -> anyhow::Result<i64> {
    Ok(iter_floor_deltas(input)
        .scan(0, |acc, delta| {
            if *acc == -1 {
                return None;
            }
            *acc += delta;
            Some(*acc)
        })
        .enumerate()
        .map(|(pos, _floor)| pos + 1)
        .last()
        .unwrap() as i64)
}

#[inline]
fn iter_floor_deltas(input: &str) -> impl Iterator<Item = i64> + '_ {
    input.trim_end().chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => unreachable!("unexpected char '{}'", c),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_part01(#[case] input: &str, #[case] expected: i64) {
        let ans = part01(input).unwrap();
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_part02(#[case] input: &str, #[case] expected: i64) {
        let ans = part02(input).unwrap();
        assert_eq!(expected, ans);
    }
}
