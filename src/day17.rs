use std::{collections::HashMap, iter::Cycle, str::Chars};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input.trim_end())?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const BOT_GAP: usize = 3;

const ROCKS: [(usize, [u8; 4]); 5] = [
    // (height, mask)
    // room is only 7 wide so the leftmost bit MUST always be 1
    // * will need to handle this in the algorithm with an XOR or something
    (1, [0u8, 0u8, 0u8, 0b0011110u8]), // Horizonal Line
    (3, [0u8, 0b0001000u8, 0b0011100u8, 0b0001000u8]), // Plus
    (3, [0u8, 0b0000100u8, 0b0000100u8, 0b0011100u8]), // J
    (4, [0b0010000u8, 0b0010000u8, 0b0010000u8, 0b0010000u8]), // Vertical Line
    (2, [0u8, 0u8, 0b0011000u8, 0b0011000u8]), // Box
];

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let input = input.trim_end();
    let mut sim = Simulation::new(input);
    for i in 0..2022 {
        let rock = ROCKS[i % 5];
        sim.drop_rock(rock.1, rock.0);
    }
    Ok(sim.highest)
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let input = input.trim_end();
    let mut sim = Simulation::new(input);
    // divide the simulation grid height by the average height of the rocks to get a good guess as
    // to how many rocks we can simulate
    for i in 0..Simulation::GRID_HEIGHT / 3 {
        let rock = ROCKS[i % 5];
        sim.drop_rock(rock.1, rock.0);
    }

    let (pattern, offset_pattern) = sim.find_pattern();

    // run the simulation again to find how many rocks it took to get these patterns
    let mut sim = Simulation::new(input);
    let mut heights = HashMap::new();
    let mut offset_rocks = None;
    for i in 0..Simulation::GRID_HEIGHT / 3 {
        heights.insert(i, sim.highest);
        if offset_rocks.is_none()
            && &sim.grid[sim.grid.len() - offset_pattern.len()..] == offset_pattern
        {
            offset_rocks = Some(i);
        }
        if &sim.grid[sim.grid.len() - pattern.len() - offset_pattern.len()
            ..sim.grid.len() - offset_pattern.len()]
            == pattern
        {
            let offset_rocks = offset_rocks.expect("did not find offset pattern");
            let pattern_rocks = i - offset_rocks;
            const TOTAL_ROCKS: u64 = 1000000000000;
            let rocks = TOTAL_ROCKS - offset_rocks as u64;
            let pattern_repeat_count = rocks / pattern_rocks as u64;
            let remaining = rocks % pattern_rocks as u64 + offset_rocks as u64;
            let leftover =
                *heights.get(&(remaining as usize)).unwrap() as u64 - offset_pattern.len() as u64;
            return Ok(offset_pattern.len() as u64
                + pattern.len() as u64 * pattern_repeat_count
                + leftover);
        }

        let rock = ROCKS[i % 5];
        sim.drop_rock(rock.1, rock.0);
    }
    unreachable!("did not find solution")
}

struct Simulation<'a> {
    grid: Vec<u8>,
    jets: Cycle<Chars<'a>>,
    highest: usize,
}

impl<'a> Simulation<'a> {
    const GRID_HEIGHT: usize = 12000;

    fn new(jets: &'a str) -> Self {
        Simulation {
            grid: vec![0u8; Self::GRID_HEIGHT],
            jets: jets.chars().cycle(),
            highest: 0,
        }
    }

    fn drop_rock(&mut self, mut rock: [u8; 4], rock_height: usize) {
        let mut y = self.grid.len() - 1 - self.highest - BOT_GAP;

        // OPTIMIZATION: you do not need to check for collisions with rocks for the first 3 moves
        // since we always spawn at least 3 units above the highest rock.
        for _ in 0..3 {
            let jet = self.jets.next().unwrap();
            match jet {
                '<' => {
                    // hits wall if leftmost (7th) bit is 1
                    // * only need to check bottom 2 rows as that is where the max width is
                    if (rock[2] >> 6) & 1 == 0 && (rock[3] >> 6) & 1 == 0 {
                        rock[0] = shl_unchecked(rock[0]);
                        rock[1] = shl_unchecked(rock[1]);
                        rock[2] = shl_unchecked(rock[2]);
                        rock[3] = shl_unchecked(rock[3]);
                    }
                }
                '>' => {
                    // hits wall if rightmost bit is 1
                    // * only need to check bottom 2 rows as that is where the max width is
                    if rock[2] & 1 == 0 && rock[3] & 1 == 0 {
                        rock[0] >>= 1;
                        rock[1] >>= 1;
                        rock[2] >>= 1;
                        rock[3] >>= 1;
                    }
                }
                _ => panic!(),
            }
            y += 1;
        }

        loop {
            // move left / right, if necessary
            let jet = self.jets.next().unwrap();
            match jet {
                '<' => {
                    // hits wall if leftmost (7th) bit is 1
                    // * only need to check bottom 2 rows as that is where the max width is
                    if (rock[2] >> 6) & 1 == 0
                        && (rock[3] >> 6) & 1 == 0
                        && self.grid[y] & shl_unchecked(rock[3]) == 0
                        && self.grid[y - 1] & shl_unchecked(rock[2]) == 0
                        && self.grid[y - 2] & shl_unchecked(rock[1]) == 0
                        && self.grid[y - 3] & shl_unchecked(rock[0]) == 0
                    {
                        rock[0] = shl_unchecked(rock[0]);
                        rock[1] = shl_unchecked(rock[1]);
                        rock[2] = shl_unchecked(rock[2]);
                        rock[3] = shl_unchecked(rock[3]);
                    }
                }
                '>' => {
                    // hits wall if rightmost bit is 1
                    // * only need to check bottom 2 rows as that is where the max width is
                    if rock[2] & 1 == 0
                        && rock[3] & 1 == 0
                        // check if hit rock
                        && self.grid[y] & rock[3] >> 1 == 0
                        && self.grid[y - 1] & rock[2] >> 1 == 0
                        && self.grid[y - 2] & rock[1] >> 1 == 0
                        && self.grid[y - 3] & rock[0] >> 1 == 0
                    {
                        rock[0] >>= 1;
                        rock[1] >>= 1;
                        rock[2] >>= 1;
                        rock[3] >>= 1;
                    }
                }
                _ => panic!(),
            }

            // check for if rock settles here
            if y == self.grid.len() - 1
                || (rock[3] & self.grid[y + 1] != 0 || rock[2] & self.grid[y] != 0)
            {
                self.grid[y - 3] |= rock[0];
                self.grid[y - 2] |= rock[1];
                self.grid[y - 1] |= rock[2];
                self.grid[y] |= rock[3];
                self.highest = self.highest.max(self.grid.len() - y - 1 + rock_height);
                break;
            }

            y += 1;
        }
    }

    fn find_pattern(&self) -> (&[u8], &[u8]) {
        for pattern_length in 1..Self::GRID_HEIGHT / 2 {
            for offset in 0..pattern_length {
                let bot =
                    &self.grid[self.grid.len() - pattern_length - offset..self.grid.len() - offset];
                let top = &self.grid[(self.grid.len() - pattern_length * 2) - offset
                    ..self.grid.len() - pattern_length - offset];
                if bot == top {
                    // println!("found pattern of length {pattern_length} @ {offset}");
                    return (bot, &self.grid[self.grid.len() - offset..self.grid.len()]);
                }
            }
        }
        panic!("no pattern found")
    }
}

#[inline]
fn shl_unchecked(line: u8) -> u8 {
    line << 1 & 0b01111111u8
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day17.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day17.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(3068, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3059, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(1514285714288, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1500874635587, ans);
    }
}
