use std::collections::{HashMap, HashSet};

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut sim = Simulation::new(parse_input(input));

    for _round in 1..=10 {
        if sim.simulate_round() == 0 {
            break;
        }
    }
    Ok(sim.count_empty_ground())
}

pub fn part02(input: &str) -> anyhow::Result<i64> {
    let mut sim = Simulation::new(parse_input(input));
    Ok((1..)
        .into_iter()
        .find(|_round| sim.simulate_round() == 0)
        .unwrap())
}

struct Simulation {
    elves: HashSet<(isize, isize)>,
    moves: [Move; 4],
}

impl Simulation {
    fn new(elves: HashSet<(isize, isize)>) -> Simulation {
        Simulation {
            elves,
            moves: [Move::North, Move::South, Move::West, Move::East],
        }
    }

    fn simulate_round(&mut self) -> u32 {
        // propose moves
        let mut proposed = HashMap::new();
        for elf in self.elves.iter() {
            let neighbors = Neighbors::new(elf, &self.elves);
            if neighbors.is_empty() {
                // elf isn't going to move
                continue;
            }

            for mve in self.moves.iter() {
                if neighbors.is_open(mve) {
                    let propose = mve.update(elf);
                    proposed
                        .entry(propose)
                        .and_modify(|e: &mut Vec<(isize, isize)>| (*e).push(*elf))
                        .or_insert(vec![*elf]);
                    break;
                }
            }
        }

        // perform moves
        let mut elves_moved = 0;
        for (newpos, elves_want) in proposed {
            if elves_want.len() > 1 {
                continue;
            }
            elves_moved += 1;
            // move elf from old position to new position
            self.elves.remove(&elves_want[0]);
            self.elves.insert(newpos);
        }

        self.moves.rotate_left(1);

        elves_moved
    }

    fn count_empty_ground(&self) -> usize {
        let (mut minrow, mut maxrow) = (isize::max_value(), isize::min_value());
        let (mut mincol, mut maxcol) = (isize::max_value(), isize::min_value());
        for pos in &self.elves {
            minrow = minrow.min(pos.0);
            maxrow = maxrow.max(pos.0);
            mincol = mincol.min(pos.1);
            maxcol = maxcol.max(pos.1);
        }
        let area = (maxrow - minrow + 1) * (maxcol - mincol + 1);
        area as usize - self.elves.len()
    }
}

fn parse_input(input: &str) -> HashSet<(isize, isize)> {
    let mut elves = HashSet::new();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert((r as isize, c as isize));
            }
        }
    }
    elves
}

fn print_grid(grid: &HashSet<(isize, isize)>) {
    let (mut minrow, mut maxrow) = (isize::max_value(), isize::min_value());
    let (mut mincol, mut maxcol) = (isize::max_value(), isize::min_value());
    for pos in grid {
        minrow = minrow.min(pos.0);
        maxrow = maxrow.max(pos.0);
        mincol = mincol.min(pos.1);
        maxcol = maxcol.max(pos.1);
    }
    for r in minrow..=maxrow {
        for c in mincol..=maxcol {
            let ch = if grid.contains(&(r, c)) { "#" } else { "." };
            print!("{}", ch);
        }
        println!();
    }
}

// NW, N, NE, E, SE, S, SW, W
#[derive(Debug)]
struct Neighbors(u8);

impl Neighbors {
    fn new(pos: &(isize, isize), locations: &HashSet<(isize, isize)>) -> Neighbors {
        let mut mask = 0;
        let mut bit = 7;
        for (dr, dc) in [
            (-1, -1), // NW
            (-1, 0),  // N
            (-1, 1),  // NE
            (0, 1),   // E
            (1, 1),   // SE
            (1, 0),   // S
            (1, -1),  // SW
            (0, -1),  // W
        ] {
            if locations.contains(&(pos.0 + dr, pos.1 + dc)) {
                mask |= 1 << bit;
            }
            bit -= 1;
        }
        Neighbors(mask)
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn is_open(&self, m: &Move) -> bool {
        match m {
            Move::North => self.north_open(),
            Move::South => self.south_open(),
            Move::West => self.west_open(),
            Move::East => self.east_open(),
        }
    }

    fn north_open(&self) -> bool {
        self.0 & 0b11100000 == 0
    }

    fn south_open(&self) -> bool {
        self.0 & 0b00001110 == 0
    }

    fn east_open(&self) -> bool {
        self.0 & 0b00111000 == 0
    }

    fn west_open(&self) -> bool {
        self.0 & 0b10000011 == 0
    }
}

#[derive(Debug)]
enum Move {
    North,
    South,
    West,
    East,
}

impl Move {
    fn update(&self, pos: &(isize, isize)) -> (isize, isize) {
        match self {
            Move::North => (pos.0 - 1, pos.1),
            Move::South => (pos.0 + 1, pos.1),
            Move::West => (pos.0, pos.1 - 1),
            Move::East => (pos.0, pos.1 + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day23.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day23.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(110, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(4056, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(20, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(999, ans);
    }
}
