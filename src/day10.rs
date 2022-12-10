use anyhow::Context;
use std::fmt::Write;
use std::str::FromStr;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input)?,
    })
}

pub fn part01(input: &str) -> i32 {
    let mut sum = 0;
    let mut last_cycle: i32 = -20;
    let mut crt = Crt::new(|cycle, x| {
        if cycle as i32 == last_cycle + 40 {
            let signal = cycle as i32 * x;
            sum += signal;
            last_cycle = cycle as i32;
        }
    });
    input
        .lines()
        .flat_map(Instruction::from_str)
        .for_each(|i| crt.process(i));
    sum
}

pub fn part02(input: &str) -> anyhow::Result<String> {
    let mut crt = Crt::new(|_, _| ());
    input
        .lines()
        .flat_map(Instruction::from_str)
        .for_each(|i| crt.process(i));
    let mut s = String::new();
    for (i, pixel) in crt.screen.iter().enumerate() {
        if i > 0 && i % W == 0 {
            writeln!(&mut s)?;
        }
        write!(&mut s, "{}", if *pixel == 1 { "#" } else { "." })?;
    }
    Ok(s)
}

const W: usize = 40;
const H: usize = 6;

struct Crt<F>
where
    F: FnMut(usize, i32),
{
    cycle: usize,
    x: i32,
    hook: F,
    screen: [u8; W * H],
}

impl<F> Crt<F>
where
    F: FnMut(usize, i32),
{
    fn new(hook: F) -> Self {
        Self {
            cycle: 0,
            x: 1,
            hook,
            screen: [0; W * H],
        }
    }

    fn process(&mut self, i: Instruction) {
        use Instruction::*;
        match i {
            Noop => {
                self.tick();
            }
            AddX(v) => {
                self.tick();
                self.tick();
                self.x += v;
            }
        }
    }

    fn tick(&mut self) {
        let w = self.cycle % W;
        if (self.x - 1..=self.x + 1).contains(&(w as i32)) {
            self.screen[self.cycle] = 1;
        }
        self.cycle += 1;
        (self.hook)(self.cycle, self.x);
    }
}

enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        match words.next().context("no instruction")? {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::AddX(
                words.next().context("no value for addx")?.parse()?,
            )),
            i => unreachable!("unexpected instruction {}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day10.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day10.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE);
        assert_eq!(13140, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(13180, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let expect: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        .trim();
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(expect, ans);
    }

    #[test]
    fn test_part_two() {
        let expect: &str = "
####.####.####..##..#..#...##..##..###..
#.......#.#....#..#.#..#....#.#..#.#..#.
###....#..###..#....####....#.#..#.###..
#.....#...#....#....#..#....#.####.#..#.
#....#....#....#..#.#..#.#..#.#..#.#..#.
####.####.#.....##..#..#..##..#..#.###..
"
        .trim();
        let ans = part02(INPUT).unwrap();
        assert_eq!(expect, ans);
    }
}
