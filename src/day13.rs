use std::{cmp::Ordering, str::FromStr};

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            let left = lines.next().unwrap();
            let right = lines.next().unwrap();
            (
                left.parse::<Element>().unwrap(),
                right.parse::<Element>().unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (l, r))| l.cmp(r) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part02(input: &str) -> usize {
    let mut packets: Vec<Element> = input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(Element::from_str)
        .collect();
    let divider_packets: (Element, Element) = ("[[2]]".parse().unwrap(), "[[6]]".parse().unwrap());
    packets.push(divider_packets.0.clone());
    packets.push(divider_packets.1.clone());
    packets.sort();
    packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| p == &divider_packets.0 || p == &divider_packets.1)
        .map(|(i, _)| i + 1)
        .product()
}

#[derive(Debug, Clone)]
enum Element {
    Number(u32),
    List(Vec<Element>),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        use Element::*;
        let res = match (self, other) {
            (Number(l), Number(r)) => l.cmp(r),
            (List(l), List(r)) => {
                for i in 0..l.len().max(r.len()) {
                    if i >= l.len() && i >= r.len() {
                        return Ordering::Equal;
                    } else if i >= l.len() {
                        return Ordering::Less;
                    } else if i >= r.len() {
                        return Ordering::Greater;
                    } else {
                        let ord = l[i].cmp(&r[i]);
                        if ord == Ordering::Less || ord == Ordering::Greater {
                            return ord;
                        }
                    }
                }
                Ordering::Equal
            }
            (Number(l), r) => Element::List(vec![Number(*l)]).cmp(r),
            (l, Number(r)) => l.cmp(&Element::List(vec![Number(*r)])),
        };
        res
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
        let mut stk: Vec<Vec<Element>> = Vec::new();
        let mut root: Option<Element> = None;
        let mut num_pending: Option<String> = None;
        for ch in s.chars() {
            match ch {
                '[' => {
                    stk.push(Vec::new());
                }
                ']' => {
                    if let Some(num) = num_pending {
                        let last_idx = stk.len() - 1;
                        stk[last_idx].push(Element::Number(num.parse().unwrap()));
                        num_pending = None
                    }

                    let els = stk.pop().unwrap();
                    if stk.len() == 0 {
                        root.replace(Element::List(els));
                    } else {
                        let last_idx = stk.len() - 1;
                        stk[last_idx].push(Element::List(els));
                    }
                }
                ',' => {
                    if let Some(num) = num_pending {
                        let last_idx = stk.len() - 1;
                        stk[last_idx].push(Element::Number(num.parse().unwrap()));
                        num_pending = None
                    }
                }
                '0'..='9' => {
                    if num_pending == None {
                        num_pending = Some(String::from(ch));
                    } else {
                        num_pending = num_pending.map(|mut s| {
                            s.push(ch);
                            s
                        });
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(root.unwrap())
    }
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
