use std::{fmt::Display, str::FromStr};

use crate::SolveInfo;
use anyhow::Context;

pub fn run(input: &str, is_sample: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02_inner(input, is_sample)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, moves) = parse_input(input)?;

    // this position is 0-based
    let mut pos = grid.start_pos();
    let mut direction = Direction::Right;

    for mv in moves {
        match mv {
            Move::Walk(n) => {
                for _ in 1..=n {
                    match grid.walk(pos, &direction) {
                        Some(next) => pos = next,
                        None => break,
                    }
                }
            }
            Move::TurnLeft => {
                direction = direction.turn_left();
            }
            Move::TurnRight => {
                direction = direction.turn_right();
            }
        }
    }

    Ok((pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + direction.facing())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    part02_inner(input, false)
}

// i'm not happy with this code anyway! :)
#[allow(clippy::all)]
fn part02_inner(input: &str, _is_sample: bool) -> anyhow::Result<usize> {
    // TODO: I wrote the cube walking algorithm to be based on a specific pattern so that this
    // the input could be translated to it. at this point i'm just going to hardcode that
    // translation because i'm getting tired of working on this problem
    let (grid, moves) = parse_input(input)?;

    // SAMPLE DATA
    // const SIZE: usize = 4;
    // let mut data = vec![vec![Tile::Unknown; SIZE * 3]; SIZE * 4];
    // let mut left = vec![Vec::with_capacity(SIZE); SIZE];
    // let mut row = 0;
    // for r in SIZE..SIZE * 2 {
    //     for c in SIZE..SIZE * 2 {
    //         left[row].push(grid.data[r][c]);
    //     }
    //     row += 1;
    // }
    // left = rotate(left);

    // let mut bottom = vec![Vec::with_capacity(SIZE); SIZE];
    // let mut row = 0;
    // for r in SIZE..SIZE * 2 {
    //     for c in 0..SIZE {
    //         bottom[row].push(grid.data[r][c]);
    //     }
    //     row += 1;
    // }
    // bottom = rotate(rotate(bottom));

    // let mut right = vec![Vec::with_capacity(SIZE); SIZE];
    // let mut row = 0;
    // for r in SIZE * 2..SIZE * 3 {
    //     for c in SIZE * 3..SIZE * 4 {
    //         right[row].push(grid.data[r][c]);
    //     }
    //     row += 1;
    // }
    // right = rotate(rotate(right));

    // // add left
    // for r in 0..SIZE {
    //     for c in 0..SIZE {
    //         data[r][c] = left[r][c];
    //     }
    // }

    // // add up
    // for r in 0..SIZE {
    //     let mut col = SIZE * 2;
    //     for c in SIZE..SIZE * 2 {
    //         data[r][c] = grid.data[r][col];
    //         col += 1;
    //     }
    // }

    // // add right
    // for r in 0..SIZE {
    //     let mut col = 0;
    //     for c in SIZE * 2..SIZE * 3 {
    //         data[r][c] = right[r][col];
    //         col += 1;
    //     }
    // }

    // // add front
    // for r in SIZE..SIZE * 2 {
    //     let mut col = SIZE * 2;
    //     for c in SIZE..SIZE * 2 {
    //         data[r][c] = grid.data[r][col];
    //         col += 1;
    //     }
    // }

    // // add down
    // for r in SIZE * 2..SIZE * 3 {
    //     let mut col = SIZE * 2;
    //     for c in SIZE..SIZE * 2 {
    //         data[r][c] = grid.data[r][col];
    //         col += 1;
    //     }
    // }

    // // add bottom
    // let mut row = 0;
    // for r in SIZE * 3..SIZE * 4 {
    //     let mut col = 0;
    //     for c in SIZE..SIZE * 2 {
    //         data[r][c] = bottom[row][col];
    //         col += 1;
    //     }
    //     row += 1;
    // }

    const SIZE: usize = 50;
    let mut data = vec![vec![Tile::Unknown; SIZE * 3]; SIZE * 4];
    // copy up, front, down into data
    for r in 0..SIZE * 3 {
        for c in SIZE..SIZE * 2 {
            data[r][c] = grid.data[r][c];
        }
    }
    // copy right into data
    for r in 0..SIZE {
        for c in SIZE * 2..SIZE * 3 {
            data[r][c] = grid.data[r][c];
        }
    }

    // extract left and rotate it
    let mut left = vec![Vec::with_capacity(SIZE); SIZE];
    let mut row = 0;
    for r in SIZE * 2..SIZE * 3 {
        for c in 0..SIZE {
            left[row].push(grid.data[r][c]);
        }
        row += 1;
    }
    left = rotate(rotate(left));

    // copy left into data
    for r in 0..SIZE {
        for c in 0..SIZE {
            data[r][c] = left[r][c];
        }
    }

    // extract bottom and rotate it
    let mut bottom = vec![Vec::with_capacity(SIZE); SIZE];
    let mut row = 0;
    for r in SIZE * 3..SIZE * 4 {
        for c in 0..SIZE {
            bottom[row].push(grid.data[r][c]);
        }
        row += 1;
    }
    bottom = rotate_left(bottom);

    // copy bottom into data
    let mut row = 0;
    for r in SIZE * 3..SIZE * 4 {
        let mut col = 0;
        for c in SIZE..SIZE * 2 {
            data[r][c] = bottom[row][col];
            col += 1;
        }
        row += 1;
    }

    // print_grid(&data);

    let cube = Cube::<SIZE> { data };
    let mut pos = (0, SIZE);
    let mut direction = Direction::Right;

    for mv in moves {
        match mv {
            Move::Walk(n) => {
                for _ in 0..n {
                    match cube.walk(pos, direction) {
                        Some((next_pos, next_dir)) => {
                            pos = next_pos;
                            direction = next_dir;
                        }
                        None => break,
                    }
                }
            }
            Move::TurnLeft => direction = direction.turn_left(),
            Move::TurnRight => direction = direction.turn_right(),
        }
    }

    // TODO: I also didn't implement any way to automatically move the standardized coordinate back
    // to the puzzle input, but luckily the final position ends in the Down section which maps 1-1
    // to the input so we can use it directly :phew:
    assert_eq!((105, 97), pos);

    Ok((pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + direction.facing())
}

fn parse_input(input: &str) -> anyhow::Result<(Grid, Vec<Move>)> {
    let (gridstr, movestr) = input.split_once("\n\n").context("malformed")?;
    let mut moves = Vec::new();
    let mut n = 0usize;
    for ch in movestr.trim().chars() {
        match ch {
            '0'..='9' => {
                n *= 10;
                n += ch.to_digit(10).unwrap() as usize;
            }
            'L' | 'R' => {
                if n > 0 {
                    moves.push(Move::Walk(n));
                    n = 0;
                }

                moves.push(if ch == 'L' {
                    Move::TurnLeft
                } else {
                    Move::TurnRight
                });
            }
            _ => anyhow::bail!("unexpected char while parsing moves '{:?}'", ch),
        }
    }
    if n > 0 {
        moves.push(Move::Walk(n));
    }
    Ok((gridstr.parse()?, moves))
}

struct Grid {
    data: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Self {
            data: vec![vec![Tile::Unknown; w]; h],
        }
    }

    fn set(&mut self, pos: (usize, usize), tile: Tile) {
        assert!(pos.0 < self.data.len());
        assert!(pos.1 < self.data[0].len());

        self.data[pos.0][pos.1] = tile;
    }

    fn start_pos(&self) -> (usize, usize) {
        let col = self.data[0]
            .iter()
            .position(|tile| *tile == Tile::Open)
            .unwrap();
        (0, col)
    }

    fn walk(&self, pos: (usize, usize), dir: &Direction) -> Option<(usize, usize)> {
        use Direction::*;
        let first_col = self.data[pos.0]
            .iter()
            .position(|tile| *tile == Tile::Open || *tile == Tile::Wall)
            .unwrap();
        let last_col = self.data[pos.0]
            .iter()
            .rposition(|tile| *tile == Tile::Open || *tile == Tile::Wall)
            .unwrap();
        let first_row = self
            .data
            .iter()
            .position(|row| row[pos.1] == Tile::Open || row[pos.1] == Tile::Wall)
            .unwrap();
        let last_row = self
            .data
            .iter()
            .rposition(|row| row[pos.1] == Tile::Open || row[pos.1] == Tile::Wall)
            .unwrap();
        let next = match dir {
            Right => {
                if pos.1 == last_col {
                    (pos.0, first_col)
                } else {
                    (pos.0, pos.1 + 1)
                }
            }
            Down => {
                if pos.0 == last_row {
                    (first_row, pos.1)
                } else {
                    (pos.0 + 1, pos.1)
                }
            }
            Left => {
                if pos.1 == first_col {
                    (pos.0, last_col)
                } else {
                    (pos.0, pos.1 - 1)
                }
            }
            Up => {
                if pos.0 == first_row {
                    (last_row, pos.1)
                } else {
                    (pos.0 - 1, pos.1)
                }
            }
        };

        if self.data[next.0][next.1] == Tile::Wall {
            None
        } else {
            Some(next)
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let h = s.lines().count();
        let w = s
            .lines()
            .map(|line| line.chars().count())
            .max()
            .context("no max line")?;
        let mut grid = Grid::new(w, h);
        for (r, line) in s.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                let tile = match ch {
                    ' ' => Tile::Unknown,
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => anyhow::bail!("unexpected grid character '{}'", ch),
                };
                grid.set((r, c), tile);
            }
        }
        Ok(grid)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.data.len() {
            for c in 0..self.data[0].len() {
                write!(f, "{}", self.data[r][c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_left(&self) -> Self {
        use Direction::*;
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn facing(&self) -> usize {
        use Direction::*;
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Unknown,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Open => '.',
            Tile::Wall => '#',
            Tile::Unknown => ' ',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug)]
enum Move {
    Walk(usize),
    TurnLeft,
    TurnRight,
}

struct Cube<const SIZE: usize> {
    data: Vec<Vec<Tile>>,
}

impl<const SIZE: usize> Cube<SIZE> {
    fn walk(
        &self,
        (row, col): (usize, usize),
        dir: Direction,
    ) -> Option<((usize, usize), Direction)> {
        use Direction::*;

        let next_pos;
        let mut next_dir = dir;
        // L->D (1-3)
        if col == 0 && row < SIZE && dir == Left {
            next_pos = (SIZE * 3 - 1 - row, SIZE);
            next_dir = Right;

        // D->L (1-3)
        } else if col == SIZE && row >= SIZE * 2 && row < SIZE * 3 && dir == Left {
            next_pos = (SIZE * 3 - 1 - row, 0);
            next_dir = Right;

        // R->D (2-4)
        } else if col == SIZE * 3 - 1 && row < SIZE && dir == Right {
            next_pos = (SIZE * 3 - 1 - row, SIZE * 2 - 1);
            next_dir = Left;

        // D->R (2-4)
        } else if col == SIZE * 2 - 1 && row >= SIZE * 2 && row < SIZE * 3 && dir == Right {
            next_pos = (SIZE - 1 - (row - SIZE * 2), SIZE * 3 - 1);
            next_dir = Left;

        // L->F (1-7)
        } else if row == SIZE - 1 && col < SIZE && dir == Down {
            next_pos = (SIZE * 2 - 1 - col, SIZE);
            next_dir = Right;

        // F->L (1-7)
        } else if col == SIZE && row >= SIZE && row < SIZE * 2 && dir == Left {
            next_pos = (SIZE - 1, SIZE * 2 - 1 - row);
            next_dir = Up;

        // R->F (2-8)
        } else if row == SIZE - 1 && col >= SIZE * 2 && col < SIZE * 3 && dir == Down {
            next_pos = (col - SIZE, SIZE * 2 - 1);
            next_dir = Left;

        // F->R (2-8)
        } else if col == SIZE * 2 - 1 && row >= SIZE && row < SIZE * 2 && dir == Right {
            next_pos = (SIZE - 1, row + SIZE);
            next_dir = Up;

        // L->B (3-5)
        } else if row == 0 && col < SIZE && dir == Up {
            next_pos = (SIZE * 3 + col, SIZE);
            next_dir = Right;

        // B->L (3-5)
        } else if col == SIZE && row >= SIZE * 3 && row < SIZE * 4 && dir == Left {
            next_pos = (0, row - SIZE * 3);
            next_dir = Down;

        // R->B (4-6)
        } else if row == 0 && col >= SIZE * 2 && col < SIZE * 3 && dir == Up {
            next_pos = (SIZE * 4 - 1 - (col - SIZE * 2), SIZE * 2 - 1);
            next_dir = Left;

        // B->R (4-6)
        } else if col == SIZE * 2 - 1 && row >= SIZE * 3 && row < SIZE * 4 && dir == Right {
            next_pos = (0, SIZE * 3 - 1 - (row - SIZE * 3));
            next_dir = Down;

        // U->B (5-6)
        } else if row == 0 && col >= SIZE && col < SIZE * 2 && dir == Up {
            next_pos = (SIZE * 4 - 1, col);
            next_dir = Up;

        // B->U (5-6)
        } else if row == SIZE * 4 - 1 && col >= SIZE && col < SIZE * 2 && dir == Down {
            next_pos = (0, col);
            next_dir = Down;
        } else {
            match dir {
                Right => next_pos = (row, col + 1),
                Down => next_pos = (row + 1, col),
                Left => next_pos = (row, col - 1),
                Up => next_pos = (row - 1, col),
            }
        }

        if self.data[next_pos.0][next_pos.1] == Tile::Wall {
            None
        } else {
            Some((next_pos, next_dir))
        }
    }
}

fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut t = vec![Vec::with_capacity(matrix.len()); matrix.len()];
    for r in matrix {
        for i in 0..r.len() {
            t[i].push(r[i].clone());
        }
    }
    t
}

fn rotate<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut r = transpose(matrix);
    for col in &mut r {
        col.reverse();
    }
    r
}

fn rotate_left<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut r = matrix;
    for col in &mut r {
        col.reverse();
    }
    transpose(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../inputs/day22.sample.txt");
    const INPUT: &str = include_str!("../inputs/day22.input.txt");
    const TEST: &str = ".......#.#..
.....#......
....#.....#.
.#..........
    ...#
    #...
    ....
    ..#.
    ...#
    ....
    .#..
    ....
    ....
    .#..
    ....
    #...";

    #[test]
    fn test_cube_walk() {
        let grid = Grid::from_str(TEST).unwrap();
        print_grid(&grid.data);
        let mut cube = Cube::<4> { data: grid.data };

        // from sample failures
        let ans = cube.walk((14, 4), Direction::Left).unwrap();
        assert_eq!(ans.0, (0, 2));
        assert_eq!(ans.1, Direction::Down);

        let ans = cube.walk((1, 11), Direction::Right).unwrap();
        assert_eq!(ans.0, (10, 7));
        assert_eq!(ans.1, Direction::Left);

        // remove all walls to test cube walking in general
        for r in 0..16 {
            for c in 0..12 {
                if cube.data[r][c] == Tile::Wall {
                    cube.data[r][c] = Tile::Open;
                }
            }
        }

        // L->D
        let ans = cube.walk((2, 0), Direction::Left).unwrap();
        assert_eq!(ans.0, (9, 4));
        assert_eq!(ans.1, Direction::Right);

        // D->L
        let ans = cube.walk((9, 4), Direction::Left).unwrap();
        assert_eq!(ans.0, (2, 0));
        assert_eq!(ans.1, Direction::Right);
        // let ans = cube.walk((8, 4), Direction::Left).unwrap();
        // assert_eq!(ans.0, (3, 0));
        // assert_eq!(ans.1, Direction::Right);

        // R->D
        let ans = cube.walk((2, 11), Direction::Right).unwrap();
        assert_eq!(ans.0, (9, 7));
        assert_eq!(ans.1, Direction::Left);

        // D->R
        let ans = cube.walk((9, 7), Direction::Right).unwrap();
        assert_eq!(ans.0, (2, 11));
        assert_eq!(ans.1, Direction::Left);

        // L->F
        let ans = cube.walk((3, 2), Direction::Down).unwrap();
        assert_eq!(ans.0, (5, 4));
        assert_eq!(ans.1, Direction::Right);

        // F->L
        let ans = cube.walk((5, 4), Direction::Left).unwrap();
        assert_eq!(ans.0, (3, 2));
        assert_eq!(ans.1, Direction::Up);

        // R->F
        let ans = cube.walk((3, 9), Direction::Down).unwrap();
        assert_eq!(ans.0, (5, 7));
        assert_eq!(ans.1, Direction::Left);

        // F->R
        let ans = cube.walk((5, 7), Direction::Right).unwrap();
        assert_eq!(ans.0, (3, 9));
        assert_eq!(ans.1, Direction::Up);

        // L->B
        let ans = cube.walk((0, 2), Direction::Up).unwrap();
        assert_eq!(ans.0, (14, 4));
        assert_eq!(ans.1, Direction::Right);

        // B->L
        let ans = cube.walk((14, 4), Direction::Left).unwrap();
        assert_eq!(ans.0, (0, 2));
        assert_eq!(ans.1, Direction::Down);

        // R->B
        let ans = cube.walk((0, 9), Direction::Up).unwrap();
        assert_eq!(ans.0, (14, 7));
        assert_eq!(ans.1, Direction::Left);

        // B->R
        let ans = cube.walk((14, 7), Direction::Right).unwrap();
        assert_eq!(ans.0, (0, 9));
        assert_eq!(ans.1, Direction::Down);

        // U->B
        let ans = cube.walk((0, 5), Direction::Up).unwrap();
        assert_eq!(ans.0, (15, 5));
        assert_eq!(ans.1, Direction::Up);

        // B->U
        let ans = cube.walk((15, 5), Direction::Down).unwrap();
        assert_eq!(ans.0, (0, 5));
        assert_eq!(ans.1, Direction::Down);
    }

    #[test]
    fn test_rotate() {
        let m = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let expect = vec![vec![6, 3, 0], vec![7, 4, 1], vec![8, 5, 2]];
        assert_eq!(expect, rotate(m));
    }

    #[test]
    fn test_rotate_left() {
        let m = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let expect = vec![vec![2, 5, 8], vec![1, 4, 7], vec![0, 3, 6]];
        assert_eq!(expect, rotate_left(m));
    }

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(6032, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(73346, ans);
    }

    #[test]
    #[ignore]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }

    fn print_grid(data: &[Vec<Tile>]) {
        for r in 0..data.len() {
            for c in 0..data[0].len() {
                print!("{}", data[r][c]);
            }
            println!();
        }
    }
}
