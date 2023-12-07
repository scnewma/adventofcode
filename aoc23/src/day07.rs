use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    const NAMES: [&str; 7] = [
        "five of a kind",
        "four of a kind",
        "full house",
        "three of a kind",
        "two pair",
        "one pair",
        "high card",
    ];
    let mut buckets = vec![vec![]; 7];

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let hand = tokens.next().unwrap();
        let bid: u64 = tokens.next().unwrap().parse().unwrap();

        let freq = freq(hand);
        match max_freq(&freq, hand) {
            5 => buckets[6].push((hand, bid)),
            4 => buckets[5].push((hand, bid)),
            3 => {
                if freq.values().contains(&2) {
                    // full house
                    buckets[4].push((hand, bid));
                } else {
                    // three of a kind
                    buckets[3].push((hand, bid));
                }
            }
            2 => {
                if freq.values().filter(|n| **n == 2).count() == 2 {
                    // two pair
                    buckets[2].push((hand, bid));
                } else {
                    // one pair
                    buckets[1].push((hand, bid));
                }
            }
            1 => buckets[0].push((hand, bid)),
            _ => unreachable!(),
        }
    }

    let mut total_winnings = 0u64;
    let mut rank = 1u64;
    for bucket in &mut buckets {
        bucket.sort_by(|(a, _), (b, _)| {
            for (a, b) in a.chars().zip(b.chars()) {
                let av = card_value(a);
                let bv = card_value(b);
                match av.cmp(&bv) {
                    std::cmp::Ordering::Equal => continue,
                    res => return res,
                }
            }
            std::cmp::Ordering::Equal
        });

        for (_, bid) in bucket {
            let winnings = *bid * rank;
            total_winnings += winnings;
            rank += 1;
        }
    }

    Ok(total_winnings)
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let mut buckets = vec![vec![]; 7];

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let handstr = tokens.next().unwrap();
        let hand: Hand = handstr.parse().unwrap();
        let bid: u64 = tokens.next().unwrap().parse().unwrap();

        let bucket = best_hand(&hand);

        buckets[bucket].push((handstr, bid));
    }

    let mut total_winnings = 0u64;
    let mut rank = 1u64;
    for bucket in &mut buckets {
        bucket.sort_by(|(a, _), (b, _)| {
            for (a, b) in a.chars().zip(b.chars()) {
                let av = card_value2(a);
                let bv = card_value2(b);
                match av.cmp(&bv) {
                    std::cmp::Ordering::Equal => continue,
                    res => return res,
                }
            }
            std::cmp::Ordering::Equal
        });

        for (_, bid) in bucket {
            let winnings = *bid * rank;
            total_winnings += winnings;
            rank += 1;
        }
    }

    Ok(total_winnings)
}

#[derive(Clone)]
struct Hand([char; 5]);

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let h: Vec<char> = s.chars().collect();
        Ok(Hand([h[0], h[1], h[2], h[3], h[4]]))
    }
}

impl Hand {
    fn has_joker(&self) -> bool {
        self.0.iter().any(|c| *c == 'J')
    }

    fn next_joker(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .find(|(_, c)| **c == 'J')
            .map(|(i, _)| i)
            .unwrap()
    }

    fn freq(&self) -> HashMap<char, u32> {
        let mut freq: HashMap<char, u32> = HashMap::new();
        for ch in self.0.iter() {
            let e = freq.entry(*ch).or_default();
            *e += 1;
        }
        freq
    }

    fn max_freq(&self, freq: &HashMap<char, u32>) -> u32 {
        *freq.values().max().unwrap()
    }

    fn bucket(&self) -> usize {
        let freq = self.freq();
        match self.max_freq(&freq) {
            5 => 6,
            4 => 5,
            3 => {
                if freq.values().contains(&2) {
                    // full house
                    4
                } else {
                    // three of a kind
                    3
                }
            }
            2 => {
                if freq.values().filter(|n| **n == 2).count() == 2 {
                    // two pair
                    2
                } else {
                    // one pair
                    1
                }
            }
            1 => 0,
            _ => unreachable!(),
        }
    }
}

fn best_hand(hand: &Hand) -> usize {
    if !hand.has_joker() {
        return hand.bucket();
    }

    const REPLACEMENTS: [char; 12] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    let mut best = hand.bucket();
    let i = hand.next_joker();
    for r in REPLACEMENTS {
        let mut h = hand.clone();
        h.0[i] = r;

        let v = best_hand(&h);
        best = best.max(v);
    }
    best
}

fn hand_bucket(hand: &Hand) -> usize {
    let freq = hand.freq();
    match hand.max_freq(&freq) {
        5 => 6,
        4 => 5,
        3 => {
            if freq.values().contains(&2) {
                // full house
                4
            } else {
                // three of a kind
                3
            }
        }
        2 => {
            if freq.values().filter(|n| **n == 2).count() == 2 {
                // two pair
                2
            } else {
                // one pair
                1
            }
        }
        1 => 0,
        _ => unreachable!(),
    }
}

fn card_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn card_value2(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn freq(hand: &str) -> HashMap<char, u32> {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for ch in hand.chars() {
        let e = freq.entry(ch).or_default();
        *e += 1;
    }
    freq
}

fn max_freq(freq: &HashMap<char, u32>, hand: &str) -> u32 {
    *freq.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day07.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(250453939, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(248652697, ans);
    }
}
