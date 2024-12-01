use fxhash::FxHashMap;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut right = Vec::new();
    let left = parse_input(input, |i| right.push(i));
    right.sort();
    Ok(left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut h = FxHashMap::default();
    let left = parse_input(input, |i| {
        h.entry(i).and_modify(|e| *e += 1).or_insert(1);
    });
    Ok(left.into_iter().map(|l| l * h.get(&l).unwrap_or(&0)).sum())
}

fn parse_input<F>(input: &str, mut on_right: F) -> Vec<usize>
where
    F: FnMut(usize),
{
    let mut left: Vec<usize> = Vec::new();
    input.lines().map(str::split_whitespace).for_each(|mut it| {
        left.push(it.next().unwrap().parse().unwrap());
        on_right(it.next().unwrap().parse().unwrap());
    });
    left.sort();
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day01.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1879048, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(21024792, ans);
    }
}
