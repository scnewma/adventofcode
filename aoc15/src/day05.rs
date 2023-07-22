use fxhash::FxHashMap;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    input.lines().filter(Nice::is_nice).count()
}

pub fn part02(input: &str) -> usize {
    input.lines().filter(Nice::is_nicer).count()
}

trait Nice {
    fn is_nice(&self) -> bool;

    // part 2
    fn is_nicer(&self) -> bool;
}

impl Nice for &str {
    fn is_nice(&self) -> bool {
        let mut vowels = 0;
        let mut has_repeat = false;
        let mut prev = ' ';
        for c in self.chars() {
            if "aeiou".contains(c) {
                vowels += 1;
            }

            match (prev, c) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
                (p, c) if p == c => has_repeat = true,
                _ => (),
            }

            prev = c;
        }
        vowels >= 3 && has_repeat
    }

    fn is_nicer(&self) -> bool {
        let (mut has_pair, mut has_repeat) = (false, false);

        let mut pairs = FxHashMap::default();
        let mut chars = self.char_indices();
        let (_, mut prev2) = chars.next().unwrap();
        let (i, mut prev) = chars.next().unwrap();
        pairs.insert((prev2, prev), i);
        for (i, c) in chars {
            match pairs.get(&(prev, c)) {
                Some(prev_i) => {
                    if *prev_i < i - 1 {
                        has_pair = true;
                    }
                }
                None => {
                    pairs.insert((prev, c), i);
                }
            }
            if prev2 == c {
                has_repeat = true
            }

            prev2 = prev;
            prev = c;
        }

        has_pair && has_repeat
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn test_is_nice(#[case] input: &str, #[case] expected: bool) {
        let ans = input.is_nice();
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("xxyxx", true)]
    #[case("aaa", false)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn test_is_nicer(#[case] input: &str, #[case] expected: bool) {
        let ans = input.is_nicer();
        assert_eq!(expected, ans);
    }
}
