use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;
use nom::{
    Finish, IResult, branch::alt, bytes::complete::tag, multi::separated_list0, sequence::delimited,
};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(Element::from_str)
        .tuples()
        .enumerate()
        .filter(|(_idx, (l, r))| l.cmp(r) == Ordering::Less)
        .map(|(idx, _)| idx + 1)
        .sum()
}

pub fn part02(input: &str) -> usize {
    let divider_packets = ["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];

    input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(Element::from_str)
        .chain(divider_packets.iter().cloned())
        .sorted()
        .enumerate()
        .filter(|(_idx, pkt)| pkt == &divider_packets[0] || pkt == &divider_packets[1])
        .map(|(idx, _pkt)| idx + 1)
        .product()
}

#[derive(Debug, Clone)]
enum Element {
    Number(u32),
    List(Vec<Element>),
}

impl Element {
    fn singleton(el: Element) -> Element {
        Element::List(vec![el])
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        use Element::*;

        match (self, other) {
            (Number(l), Number(r)) => l.cmp(r),
            (Number(l), r) => Element::singleton(Number(*l)).cmp(r),
            (l, Number(r)) => l.cmp(&Element::singleton(Number(*r))),
            (List(l), List(r)) => {
                let min = l.len().min(r.len());
                for i in 0..min {
                    match l[i].cmp(&r[i]) {
                        Ordering::Equal => (),
                        non_eq => return non_eq,
                    }
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Element {}

impl FromStr for Element {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_element(s).finish() {
            Ok((_, root)) => Ok(root),
            Err(err) => anyhow::bail!(err.input.to_string()),
        }
    }
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    alt((parse_list_element, parse_number_element))(input)
}

fn parse_list_element(input: &str) -> IResult<&str, Element> {
    let (rest, elements) =
        delimited(tag("["), separated_list0(tag(","), parse_element), tag("]"))(input)?;
    Ok((rest, Element::List(elements)))
}

fn parse_number_element(input: &str) -> IResult<&str, Element> {
    let (rest, n) = nom::character::complete::u32(input)?;
    Ok((rest, Element::Number(n)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day13.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day13.input.txt");

    #[test]
    fn test_element_cmp_1() {
        let left: Element = "[1,1,3,1,1]".parse().unwrap();
        let right: Element = "[1,1,5,1,1]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_element_cmp_2() {
        let left: Element = "[[1],[2,3,4]]".parse().unwrap();
        let right: Element = "[[1],4]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_element_cmp_3() {
        let left: Element = "[9]".parse().unwrap();
        let right: Element = "[[8,7,6]]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_element_cmp_4() {
        let left: Element = "[[4,4],4,4]".parse().unwrap();
        let right: Element = "[[4,4],4,4,4]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_element_cmp_4_2() {
        let left: Element = "[[4,4],4,4,4]".parse().unwrap();
        let right: Element = "[[4,4],4,4,4]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Equal);
    }

    #[test]
    fn test_element_cmp_4_3() {
        let left: Element = "[[4,4],4,4,4,4]".parse().unwrap();
        let right: Element = "[[4,4],4,4,4]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_element_cmp_5() {
        let left: Element = "[7,7,7,7]".parse().unwrap();
        let right: Element = "[7,7,7]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_element_cmp_6() {
        let left: Element = "[]".parse().unwrap();
        let right: Element = "[3]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_element_cmp_7() {
        let left: Element = "[[[]]]".parse().unwrap();
        let right: Element = "[[]]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_element_cmp_8() {
        let left: Element = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse().unwrap();
        let right: Element = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse().unwrap();

        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_element_cmp_9_1() {
        let left: Element = "[[]]".parse().unwrap();
        let right: Element = "[[],[]]".parse().unwrap();
        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_element_cmp_9_2() {
        let left: Element = "[[],[]]".parse().unwrap();
        let right: Element = "[[]]".parse().unwrap();
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_element_cmp_9_3() {
        let left: Element = "[[]]".parse().unwrap();
        let right: Element = "[[]]".parse().unwrap();
        assert_eq!(left.cmp(&right), Ordering::Equal);
    }

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(13, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(6428, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE);
        assert_eq!(140, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(22464, ans);
    }
}
