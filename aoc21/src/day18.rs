use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let complete = input
        .lines()
        .flat_map(Number::from_str)
        .reduce(|acc, next| {
            let mut n = acc.add(next);
            n.reduce();
            n
        })
        .unwrap();
    Ok(complete.magnitude())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .flat_map(Number::from_str)
        .permutations(2)
        .map(|pair| {
            pair.into_iter()
                .reduce(|acc, next| {
                    let mut n = acc.add(next);
                    n.reduce();
                    n
                })
                .unwrap()
        })
        .map(|n| n.magnitude())
        .max()
        .unwrap())
}

#[derive(Debug, Clone)]
enum Number {
    Literal(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn add(self, other: Number) -> Number {
        Number::Pair(Box::new(self), Box::new(other))
    }

    fn magnitude(&self) -> usize {
        match self {
            Number::Literal(n) => *n,
            Number::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn reduce(&mut self) {
        loop {
            let mut exploded = false;
            // explode any numbers of depth 4
            while self.maybe_explode(0).is_some() {
                exploded = true;
            }

            let split = self.split();

            if !exploded && !split {
                break;
            }
        }
    }

    fn maybe_explode(&mut self, depth: usize) -> Option<(bool, usize, usize)> {
        if let Number::Pair(l, r) = self {
            if depth == 4 {
                let Number::Pair(l, r) = self else {
                    unreachable!("unexpected structure at depth 4: {self:?}");
                };
                let Number::Literal(l) = **l else {
                    unreachable!("unexpected structure at depth 4: {self:?}");
                };
                let Number::Literal(r) = **r else {
                    unreachable!("unexpected structure at depth 4: {self:?}");
                };
                return Some((true, l, r));
            }

            if let Some((deleteme, dl, mut dr)) = l.maybe_explode(depth + 1) {
                if dr > 0 && r.explode_add_left(dr) {
                    dr = 0;
                }
                if deleteme {
                    *l = Box::new(Number::Literal(0));
                }
                return Some((false, dl, dr));
            }

            if let Some((deleteme, mut dl, dr)) = r.maybe_explode(depth + 1) {
                if dl > 0 && l.explode_add_right(dl) {
                    dl = 0;
                }
                if deleteme {
                    *r = Box::new(Number::Literal(0));
                }
                return Some((false, dl, dr));
            }
        }

        None
    }

    fn explode_add_left(&mut self, d: usize) -> bool {
        match self {
            Number::Literal(n) => {
                *n += d;
                true
            }
            Number::Pair(l, _) => l.explode_add_left(d),
        }
    }

    fn explode_add_right(&mut self, d: usize) -> bool {
        match self {
            Number::Literal(n) => {
                *n += d;
                true
            }
            Number::Pair(_, r) => r.explode_add_right(d),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Literal(n) => {
                if *n >= 10 {
                    *self = Number::Pair(
                        Box::new(Number::Literal(*n / 2)),
                        Box::new(Number::Literal(n.div_ceil(2))),
                    );
                    true
                } else {
                    false
                }
            }
            // only leftmost number splits
            Number::Pair(l, r) => l.split() || r.split(),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Literal(n) => write!(f, "{}", n),
            Number::Pair(l, r) => {
                write!(f, "[")?;
                write!(f, "{}", l)?;
                write!(f, ",")?;
                write!(f, "{}", r)?;
                write!(f, "]")
            }
        }
    }
}

impl FromStr for Number {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_number(s))
    }
}

fn parse_number(s: &str) -> Number {
    let (_, n) = parse_number_impl(s);
    n
}

fn parse_number_impl(s: &str) -> (usize, Number) {
    match s.chars().next().unwrap() {
        '[' => {
            let mut s = &s[1..]; // skip [
            let (skipl, left) = parse_number_impl(s);
            s = &s[skipl + 1..]; // skip left Number + ,
            let (skipr, right) = parse_number_impl(s);
            (
                3 + skipl + skipr, // 3 for [,]
                Number::Pair(Box::new(left), Box::new(right)),
            )
        }
        '0'..='9' => {
            let end = s.find([',', ']']).unwrap();
            let n: usize = s[..end].parse().unwrap();
            (end, Number::Literal(n))
        }
        ch => unreachable!("got unexpected char {ch}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = include_str!("../inputs/day18.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3305, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(4563, ans);
    }

    #[rstest]
    #[case("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
    #[case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
    #[case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
    #[case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    )]
    #[case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
    fn test_explode(#[case] s: &str, #[case] expected: &str) {
        let mut number = parse_number(s);
        let res = number.maybe_explode(0);
        dbg!(res);
        assert_eq!(expected, number.to_string());
    }

    #[rstest]
    #[case("[10,0]", "[[5,5],0]")]
    #[case("[11,0]", "[[5,6],0]")]
    #[case("[11,10]", "[[5,6],10]")]
    #[case("[[1,[2,20]],1]", "[[1,[2,[10,10]]],1]")]
    fn test_split(#[case] s: &str, #[case] expected: &str) {
        let mut number = parse_number(s);
        number.split();
        assert_eq!(expected, number.to_string());
    }

    #[test]
    fn test_reduce() {
        let left = parse_number("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = parse_number("[1,1]");
        let mut num = left.add(right);
        num.reduce();
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", num.to_string());
    }

    #[rstest]
    #[case("[9,1]", 29)]
    #[case("[1,9]", 21)]
    #[case("[[9,1],[1,9]]", 129)]
    #[case("[[1,2],[[3,4],5]]", 143)]
    #[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    #[case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    fn test_magnitude(#[case] number: &str, #[case] expected: usize) {
        let number = parse_number(number);
        assert_eq!(expected, number.magnitude());
    }
}
