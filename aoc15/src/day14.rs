use std::str::FromStr;

use anyhow::Context;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

const TIME: u32 = 2503;

pub fn part01(input: &str) -> u32 {
    input
        .lines()
        .flat_map(Reindeer::from_str)
        .map(|r| r.distance_travelled(TIME))
        .max()
        .unwrap()
}

pub fn part02(input: &str) -> u32 {
    let reindeer: Vec<_> = input.lines().flat_map(Reindeer::from_str).collect();
    let mut scores = vec![0; reindeer.len()];

    for t in 1..=TIME {
        let mut max = 0;
        for r in reindeer.iter() {
            let d = r.distance_travelled(t);
            max = max.max(d);
        }
        for (i, r) in reindeer.iter().enumerate() {
            let d = r.distance_travelled(t);
            if d == max {
                scores[i] += 1;
            }
        }
    }
    *scores.iter().max().unwrap()
}

#[derive(Debug, PartialEq, Eq)]
struct Reindeer {
    speed: u32,
    fly_secs: u32,
    rest_secs: u32,
}

impl Reindeer {
    fn distance_travelled(&self, secs: u32) -> u32 {
        let distance_per_flight = self.speed * self.fly_secs;
        let cycle_secs = self.fly_secs + self.rest_secs;
        let cycles = secs / cycle_secs;
        let secs_last_cycle = secs % cycle_secs;
        let distance_last_cycle = secs_last_cycle.min(self.fly_secs) * self.speed;
        distance_per_flight * cycles + distance_last_cycle
    }
}

impl FromStr for Reindeer {
    type Err = anyhow::Error;

    // Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace();
        let mut words = words.skip(3);
        let speed: u32 = words.next().context("no speed")?.parse()?;
        let mut words = words.skip(2);
        let fly_secs: u32 = words.next().context("no fly seconds")?.parse()?;
        let mut words = words.skip(6);
        let rest_secs: u32 = words.next().context("no rest seconds")?.parse()?;
        Ok(Reindeer {
            speed,
            fly_secs,
            rest_secs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reindeer() {
        let comet = Reindeer {
            speed: 14,
            fly_secs: 10,
            rest_secs: 127,
        };
        assert_eq!(comet.distance_travelled(1), 14);
        assert_eq!(comet.distance_travelled(10), 140);
        assert_eq!(comet.distance_travelled(137), 140);
        assert_eq!(comet.distance_travelled(138), 154);
        assert_eq!(comet.distance_travelled(1000), 1120);
    }

    #[test]
    fn test_reindeer_parse() {
        const DESC: &str =
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let expected = Reindeer {
            speed: 14,
            fly_secs: 10,
            rest_secs: 127,
        };
        let ans: Reindeer = DESC.parse().unwrap();
        assert_eq!(expected, ans);
    }
}
