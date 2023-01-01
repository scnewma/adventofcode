use bittle::Bits;
use itertools::iproduct;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (valley, entrance, exit) = parse_input(input);
    let h = valley.0.len();
    let (steps, _) = shortest_path(valley, (0, entrance), (h - 1, exit));
    Ok(steps)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (valley, entrance, exit) = parse_input(input);
    let h = valley.0.len();
    let start = (0, entrance);
    let end = (h - 1, exit);
    let (first_trip, valley) = shortest_path(valley, start, end);
    let (second_trip, valley) = shortest_path(valley, end, start);
    let (third_trip, _) = shortest_path(valley, start, end);
    // + 1 for each "stop" otherwise you arrive and leave a destination in the same step
    Ok(first_trip + second_trip + 1 + third_trip + 1)
}

fn shortest_path(valley: Valley, start: (usize, usize), end: (usize, usize)) -> (usize, Valley) {
    let (h, w) = (valley.0.len(), valley.0[0].len());

    // we don't need to recalculate the valley state for every iteration of the bfs since the
    // valley state is independent of the player position. this vec is a cache of every valley
    // state we have seen with index = step
    let mut valley_states = vec![valley];

    // bfs queue
    let mut queue = VecDeque::new();
    queue.push_back((start, 1));

    // typical bfs visited pattern to not get into an infinite loop. it's slightly modified to be
    // based on the current position + the current step since the overall grid changes every step
    let mut visited = HashSet::new();
    while let Some((pos, step)) = queue.pop_front() {
        if !visited.insert((pos, step)) {
            continue;
        }
        if step == valley_states.len() {
            // need to generate a new state
            valley_states.push(valley_states[step - 1].tick());
        }
        // the valley state we will be comparing to is the next step we are going to take
        let valley = &valley_states[step];

        // check if we can exit the valley
        if pos == end {
            return (step - 1, valley.clone());
        }

        let (row, col) = pos;
        // simulate stepping in each available direction
        if row > 0 && valley.0[row - 1][col] == 0 {
            queue.push_back(((row - 1, col), step + 1));
        }
        if row < h - 1 && valley.0[row + 1][col] == 0 {
            queue.push_back(((row + 1, col), step + 1));
        }
        if col > 0 && valley.0[row][col - 1] == 0 {
            queue.push_back(((row, col - 1), step + 1));
        }
        if col < w - 1 && valley.0[row][col + 1] == 0 {
            queue.push_back(((row, col + 1), step + 1));
        }

        // wait here
        if valley.0[row][col] == 0 {
            queue.push_back((pos, step + 1));
        }
    }
    unreachable!("no result found")
}

const DELTAS: [(isize, isize, Direction); 4] = [
    (1, 0, Direction::Up),
    (-1, 0, Direction::Down),
    (0, 1, Direction::Left),
    (0, -1, Direction::Right),
];

// the valley is a grid of u8, each u8 is a bitmask that describes the state of that location in
// the grid. the state could be open, wall, or 1-4 blizzards. this allows us to represent the 4
// blizzards in a single mask
//   i.e. 0b00001011
//      represents 3 blizzards (right, up, left)
#[derive(Clone, PartialEq, Eq)]
struct Valley(Vec<Vec<u8>>);

impl Valley {
    fn tick(&self) -> Valley {
        // ASSUMPTION: making an assumption that the blizzards cannot occupy the gaps in the walls
        // at the top / bottom, but that may be a bad assumption
        let (w, h) = (self.0[0].len(), self.0.len());
        let mut grid = vec![vec![0u8; w]; h];
        for (r, c) in iproduct!(0..h, 0..w) {
            // copy perimeter directly
            if r == 0 || r == h - 1 || c == 0 || c == w - 1 {
                grid[r][c] = self.0[r][c];
                continue;
            }

            let mut mask = 0;

            for (dr, dc, dir) in DELTAS {
                // we don't need to worry about the row/col going negative here since we
                // already handled row=0 and col=0 above (the walls)
                let mut nr = r.checked_add_signed(dr).unwrap();
                if nr == 0 {
                    nr = h - 2;
                } else if nr == h - 1 {
                    nr = 1;
                }

                let mut nc = c.checked_add_signed(dc).unwrap();
                if nc == 0 {
                    nc = w - 2;
                } else if nc == w - 1 {
                    nc = 1;
                }

                if self.0[nr][nc].is_moving(dir) {
                    mask.add_movement(dir);
                }
            }
            grid[r][c] = mask;
        }
        Valley(grid)
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.0.len() {
            for c in 0..self.0[0].len() {
                write!(f, "{}", self.0[r][c].symbol())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(s: &str) -> (Valley, usize, usize) {
    let w = s.lines().next().unwrap().len();
    let h = s.lines().count();

    let entrance = s
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == '.')
        .unwrap();
    let exit = s
        .lines()
        .last()
        .unwrap()
        .chars()
        .position(|c| c == '.')
        .unwrap();

    let mut tiles = vec![vec![0u8; w]; h];
    for (r, line) in s.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            tiles[r][c] = match ch {
                '#' => WALL,
                '.' => 0,
                '>' => BZR,
                '<' => BZL,
                '^' => BZU,
                'v' => BZD,
                _ => unreachable!(),
            };
        }
    }
    (Valley(tiles), entrance, exit)
}

const WALL: u8 = 0xFF;
const BZR: u8 = 1;
const BZU: u8 = 1 << 1;
const BZD: u8 = 1 << 2;
const BZL: u8 = 1 << 3;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait Blizzard {
    fn is_moving(&self, dir: Direction) -> bool;
    fn add_movement(&mut self, dir: Direction);
    fn symbol(&self) -> char;
}

impl Blizzard for u8 {
    fn is_moving(&self, dir: Direction) -> bool {
        match dir {
            Direction::Up => self >> 1 & 1 == 1,
            Direction::Down => self >> 2 & 1 == 1,
            Direction::Left => self >> 3 & 1 == 1,
            Direction::Right => self & 1 == 1,
        }
    }

    fn add_movement(&mut self, dir: Direction) {
        match dir {
            Direction::Up => *self |= BZU,
            Direction::Down => *self |= BZD,
            Direction::Left => *self |= BZL,
            Direction::Right => *self |= BZR,
        }
    }

    fn symbol(&self) -> char {
        match *self {
            0 => '.',
            WALL => '#',
            BZR => '>',
            BZL => '<',
            BZU => '^',
            BZD => 'v',
            _ => char::from_digit(self.count_ones(), 10).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day24.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day24.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(18, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(240, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(54, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(717, ans);
    }
}
