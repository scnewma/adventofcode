use std::str::FromStr;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

const LENGTH: u32 = 1000;
const FULL_SIZE: usize = LENGTH as usize * LENGTH as usize;

pub fn part01(input: &str) -> u32 {
    let mut grid = [0u8; FULL_SIZE];

    input
        .lines()
        .flat_map(Instr::from_str)
        .for_each(|i| i.execute_v1(&mut grid));

    grid.into_iter().filter(|n| *n == 1).count() as u32
}

pub fn part02(input: &str) -> u32 {
    let mut grid = [0u32; FULL_SIZE];

    input
        .lines()
        .flat_map(Instr::from_str)
        .for_each(|i| i.execute_v2(&mut grid));

    grid.iter().sum()
}

struct Point(usize, usize);

struct Instr {
    tl: Point,
    br: Point,
    action: Action,
}

enum Action {
    On,
    Off,
    Toggle,
}

impl Instr {
    // i was previously using bittle and a bit array, but this is significantly faster
    fn execute_v1(self, grid: &mut [u8; FULL_SIZE]) {
        for x in self.tl.0..=self.br.0 {
            for y in self.tl.1..=self.br.1 {
                let i = x * LENGTH as usize + y;
                match self.action {
                    Action::On => grid[i] = 1,
                    Action::Off => grid[i] = 0,
                    Action::Toggle => {
                        grid[i] ^= 1;
                    }
                }
            }
        }
    }

    fn execute_v2(self, grid: &mut [u32; FULL_SIZE]) {
        for x in self.tl.0..=self.br.0 {
            for y in self.tl.1..=self.br.1 {
                let i = x * LENGTH as usize + y;
                match self.action {
                    Action::On => grid[i] += 1,
                    Action::Off => grid[i] = grid[i].saturating_sub(1),
                    Action::Toggle => grid[i] += 2,
                }
            }
        }
    }
}

impl FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let action = match words.next().unwrap() {
            "turn" => match words.next().unwrap() {
                "on" => Action::On,
                "off" => Action::Off,
                word => unreachable!("unknown action: turn {}", word),
            },
            "toggle" => Action::Toggle,
            word => unreachable!("unknown action: {}", word),
        };

        let tl = words.next().unwrap();
        words.next().unwrap(); // skip "through"
        let br = words.next().unwrap();

        let tl: Point = tl.parse().unwrap();
        let br: Point = br.parse().unwrap();

        Ok(Instr { tl, br, action })
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Point(x.parse().unwrap(), y.parse().unwrap()))
    }
}
