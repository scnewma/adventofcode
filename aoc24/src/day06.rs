use bittle::{Bits, BitsMut};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const WIDTH: usize = 130;
const HEIGHT: usize = 130;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, start_pos) = parse_input(input);
    let mut visited = [0u64; (WIDTH * HEIGHT) / 64];
    GuardIter::new(&grid, start_pos, None).for_each(|(pos, _)| visited.set_bit(index(pos)));
    Ok(visited.count_ones() as usize)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (grid, start_pos) = parse_input(input);

    // only need to try and put an obstacle in positions that the guard would visit on the
    // pre-modified grid
    let mut initial_visited = [0u64; (WIDTH * HEIGHT) / 64];
    GuardIter::new(&grid, start_pos, None).for_each(|(pos, _)| initial_visited.set_bit(index(pos)));
    initial_visited.clear_bit(index(start_pos));

    Ok(initial_visited
        .iter_ones()
        .filter(|obstacle_pos| {
            let r = obstacle_pos / WIDTH as u32;
            let c = obstacle_pos % WIDTH as u32;
            // basically a 3d matrix -- row of cells where each cell stores a bit for each
            // direction
            let mut visited = [0u64; (WIDTH * HEIGHT * 4) / 64];
            GuardIter::new(&grid, start_pos, Some((r as isize, c as isize)))
                // if we find a pos+dir that's been seen before, we've looped
                .any(|(pos, dir)| {
                    let looped = visited.test_bit(index3(pos, dir));
                    visited.set_bit(index3(pos, dir));
                    looped
                })
        })
        .count())
}

fn index(pos: Pos) -> u32 {
    (pos.0 as u32 * WIDTH as u32) + pos.1 as u32
}

fn index3(pos: Pos, dir: u8) -> u32 {
    (pos.0 as u32 * WIDTH as u32 * 4) + (pos.1 as u32 * 4) + dir as u32
}

struct GuardIter<'a> {
    grid: &'a Grid,
    pos: Pos,
    dir: u8,
    obstacle: Option<Pos>,
}

impl<'a> GuardIter<'a> {
    fn new(grid: &'a Grid, start_pos: Pos, obstacle: Option<Pos>) -> Self {
        Self {
            grid,
            // start one tile south to "cleanly" handle the visit of the start_pos
            pos: (start_pos.0 + 1, start_pos.1),
            dir: 0,
            obstacle,
        }
    }
}

impl Iterator for GuardIter<'_> {
    type Item = (Pos, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let (row, col) = self.pos;
        let next = match self.dir {
            0 => (row - 1, col), // north
            1 => (row, col + 1), // east
            2 => (row + 1, col), // south
            3 => (row, col - 1), // west
            _ => unreachable!(),
        };

        let tile = if self.obstacle.is_some_and(|o| o == next) {
            Some('#')
        } else {
            if next.0 < 0 || (next.0 as usize) >= HEIGHT || next.1 < 0 || (next.1 as usize) >= WIDTH
            {
                None
            } else {
                Some(self.grid[next.0 as usize][next.1 as usize])
            }
        };

        tile.inspect(|&t| match t {
            '#' => self.dir = (self.dir + 1) % 4,
            '.' => self.pos = next,
            _ => unreachable!(),
        })
        .map(|_| (self.pos, self.dir))
    }
}

type Grid = [[char; WIDTH]; HEIGHT];
type Pos = (isize, isize);

fn parse_input(input: &str) -> (Grid, Pos) {
    let mut grid = [['.'; WIDTH]; HEIGHT];
    let mut pos = None;
    for (row, line) in input.lines().enumerate() {
        for (col, mut ch) in line.char_indices() {
            if ch == '^' {
                pos = Some((row as isize, col as isize));
                ch = '.';
            }
            grid[row][col] = ch;
        }
    }
    (grid, pos.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day06.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(4665, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1688, ans);
    }
}
