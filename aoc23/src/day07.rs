use std::str::FromStr;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let mut buckets = vec![vec![]; 7];

    parse_input(input).for_each(|(raw, hand, bid)| {
        buckets[hand.bucket()].push((raw, bid));
    });

    Ok(calculate_winnings(&mut buckets, card_value))
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let mut buckets = vec![vec![]; 7];

    parse_input(input).for_each(|(raw, hand, bid)| {
        let bucket = best_hand(&hand);
        buckets[bucket].push((raw, bid));
    });

    Ok(calculate_winnings(&mut buckets, |card| match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }))
}

fn parse_input(input: &str) -> impl Iterator<Item = (&str, Hand, u64)> + '_ {
    input.lines().map(|line| {
        let mut tokens = line.split_whitespace();
        let handstr = tokens.next().unwrap();
        let bid: u64 = tokens.next().unwrap().parse().unwrap();
        let hand: Hand = handstr.parse().unwrap();
        (handstr, hand, bid)
    })
}

fn calculate_winnings<F>(buckets: &mut Vec<Vec<(&str, u64)>>, card_value: F) -> u64
where
    F: Fn(char) -> u32,
{
    use std::cmp::Ordering;

    let mut total_winnings = 0u64;
    let mut rank = 1u64;
    for bucket in buckets {
        bucket.sort_by(|(a, _), (b, _)| {
            for (a, b) in a.chars().zip(b.chars()) {
                let cmp = card_value(a).cmp(&card_value(b));
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        });

        for (_, bid) in bucket.iter() {
            let winnings = *bid * rank;
            total_winnings += winnings;
            rank += 1;
        }
    }
    total_winnings
}

struct Freq([u8; 15]);

impl Freq {
    fn max2(&self) -> (u8, u8) {
        let (mut m1, mut m2) = (0, 0);
        for f in self.0 {
            if f > m1 {
                m2 = m1;
                m1 = f;
            } else if f > m2 {
                m2 = f;
            }
        }
        (m1, m2)
    }
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
    fn next_joker(&self) -> Option<usize> {
        self.0
            .iter()
            .enumerate()
            .find(|(_, c)| **c == 'J')
            .map(|(i, _)| i)
    }

    fn freq(&self) -> Freq {
        let mut freq = [0; 15];
        for ch in self.0.iter() {
            freq[card_value(*ch) as usize] += 1;
        }
        Freq(freq)
    }

    fn bucket(&self) -> usize {
        match self.freq().max2() {
            (5, _) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            (1, _) => 0,
            _ => unreachable!(),
        }
    }
}

// brute forces the replacement of all jokers for different cards to select the best possible hand.
fn best_hand(hand: &Hand) -> usize {
    const REPLACEMENTS: [char; 12] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

    match hand.next_joker() {
        Some(i) => {
            let mut best = hand.bucket();
            for r in REPLACEMENTS {
                let mut h = hand.clone();
                h.0[i] = r;

                let v = best_hand(&h);
                best = best.max(v);
            }
            best
        }
        None => hand.bucket(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &'static str = include_str!("../inputs/day07.input.txt");

    #[rstest]
    #[case("AAAAA", 6)]
    #[case("AAAA2", 5)]
    #[case("AAA22", 4)]
    #[case("A2A2A", 4)]
    #[case("AAA21", 3)]
    #[case("AA221", 2)]
    #[case("AA321", 1)]
    #[case("AQ321", 0)]
    fn test_hand_bucket(#[case] h: &str, #[case] expected: usize) {
        let hand = Hand::from_str(h).unwrap();
        assert_eq!(expected, hand.bucket());
    }

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
