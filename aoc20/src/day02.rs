use anyhow::Context;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(parse_input(input)
        .filter_map(Result::ok)
        .filter(|&((lo, hi), letter, password)| {
            let count = password.bytes().filter(|&c| c == letter).count();
            lo <= count && count <= hi
        })
        .count())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(parse_input(input)
        .filter_map(Result::ok)
        .filter(|&((pos1, pos2), letter, password)| {
            let c1 = password.as_bytes().get(pos1 - 1).copied();
            let c2 = password.as_bytes().get(pos2 - 1).copied();
            c1.is_some_and(|c| c == letter) ^ c2.is_some_and(|c| c == letter)
        })
        .count())
}

fn parse_input(
    input: &str,
) -> impl Iterator<Item = anyhow::Result<((usize, usize), u8, &str)>> + '_ {
    input.lines().map(|line| {
        let mut it = line.split_whitespace();
        let (lhs, rhs) = it
            .next()
            .context("missing range")?
            .split_once('-')
            .context("malformed range")
            .and_then(|(lhs, rhs)| Ok((lhs.parse::<usize>()?, rhs.parse::<usize>()?)))?;

        let letter = it
            .next()
            .context("missing letter")?
            .as_bytes()
            .first()
            .copied()
            .context("no first char")?;

        let password = it.next().context("missing password")?;
        Ok(((lhs, rhs), letter, password))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day02.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(622, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(263, ans);
    }
}
