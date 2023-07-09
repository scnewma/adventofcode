use fxhash::FxHashSet;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut visited = input
        .trim_end()
        .chars()
        .scan((0, 0), |santa, c| {
            *santa = santa.next_location(c);
            Some(*santa)
        })
        .collect::<FxHashSet<_>>();
    visited.insert((0, 0));

    Ok(visited.len())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut visited = FxHashSet::default();
    visited.insert((0, 0));
    let mut santa = (0, 0);
    let mut robo = (0, 0);
    for (i, c) in input.trim_end().char_indices() {
        visited.insert(santa);
        visited.insert(robo);
        if i % 2 == 0 {
            santa = santa.next_location(c);
        } else {
            robo = robo.next_location(c);
        }
    }
    visited.insert(santa);
    visited.insert(robo);

    Ok(visited.len())
}

trait GridNavigation {
    fn next_location(self, c: char) -> Self;
}

impl GridNavigation for (i32, i32) {
    #[inline]
    fn next_location(self, c: char) -> Self {
        let (x, y) = self;
        match c {
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_part01(#[case] input: &str, #[case] expected: usize) {
        let ans = part01(input).unwrap();
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_part02(#[case] input: &str, #[case] expected: usize) {
        let ans = part02(input).unwrap();
        assert_eq!(expected, ans);
    }
}
