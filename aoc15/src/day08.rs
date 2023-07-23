pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.chars().count() - line.length_codepoints())
        .sum()
}

pub fn part02(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let encoded = line.encode();
            encoded.chars().count() - line.chars().count()
        })
        .sum()
}

trait Codepoint {
    fn length_codepoints(&self) -> usize;
}

impl Codepoint for String {
    fn length_codepoints(&self) -> usize {
        self.as_str().length_codepoints()
    }
}

impl Codepoint for &str {
    fn length_codepoints(&self) -> usize {
        let mut len = 0;
        let mut chars = self.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('"') | Some('\\') => (),
                    Some('x') => {
                        chars.next();
                        chars.next();
                    }
                    Some(_) => unreachable!("unexpected escape code"),
                    None => unreachable!("Unexpected end of input string"),
                }
            }
            len += 1;
        }
        len - 2 // remove surrounding "
    }
}

trait Encode {
    fn encode(&self) -> String;
}

impl Encode for &str {
    fn encode(&self) -> String {
        let mut encoded = String::new();
        encoded.push('"');
        for c in self.chars() {
            match c {
                '"' => encoded.push_str("\\\""),
                '\\' => encoded.push_str("\\\\"),
                c => encoded.push(c),
            }
        }
        encoded.push('"');
        encoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("\"\"", 0)]
    #[case("\"abc\"", 3)]
    #[case("\"aaa\"aaa\"", 7)]
    #[case("\"\x27\"", 1)]
    fn test_examples(#[case] input: &str, #[case] expected: usize) {
        let ans = input.length_codepoints();
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case(r#""""#, r#""\"\"""#)]
    #[case(r#""abc""#, r#""\"abc\"""#)]
    #[case(r#""aaa\"aaa""#, r#""\"aaa\\\"aaa\"""#)]
    #[case(r#""\x27""#, r#""\"\\x27\"""#)]
    fn test_encode(#[case] input: &str, #[case] expected: &str) {
        let ans = input.encode();
        assert_eq!(expected, ans);
    }

    #[rstest]
    #[case("\"\"", 4)]
    fn test_part02(#[case] input: &str, #[case] expected: usize) {
        let ans = part02(input);
        assert_eq!(expected, ans);
    }
}
