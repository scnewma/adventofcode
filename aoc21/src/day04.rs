use crate::SolveInfo;

use std::{num::ParseIntError, str::FromStr};

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let mut game: Game = input.parse().unwrap();
    for call in &game.calls {
        for board in &mut game.boards {
            board.mark(*call);
            if board.winner() {
                return board.score(*call) as i64;
            }
        }
    }
    panic!("no winning board found!")
}

pub fn part02(input: &str) -> i64 {
    let mut game: Game = input.parse().unwrap();

    for call in &game.calls {
        for board in &mut game.boards {
            board.mark(*call);
        }
        if game.boards.len() == 1 {
            // even though it's the last board, we need the final score after this board has won
            if game.boards[0].winner() {
                return game.boards[0].score(*call) as i64;
            }
        } else {
            game.boards.retain(|b| !b.winner());
        }
    }
    panic!("02: no losing boards found")
}

struct Game {
    calls: Vec<i32>,
    boards: Vec<Board>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks: Vec<_> = s.split("\n\n").collect();
        let calls = chunks[0].split(',').map(|s| s.parse().unwrap()).collect();
        let boards = chunks[1..].iter().map(|s| s.parse().unwrap()).collect();

        Ok(Self { calls, boards })
    }
}

#[derive(Debug)]
struct Cell {
    num: i32,
    called: bool,
}

impl FromStr for Cell {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cell {
            num: s.parse()?,
            called: false,
        })
    }
}

#[derive(Debug)]
struct Board(Vec<Cell>);

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<_> = s
            .split_whitespace() // splits on both " " and \n
            .map(|s| s.parse::<Cell>().unwrap())
            .collect();
        Ok(Board(cells))
    }
}

impl Board {
    fn mark(&mut self, num: i32) {
        for cell in self.0.iter_mut() {
            if cell.num == num {
                cell.called = true;
            }
        }
    }

    fn winner(&self) -> bool {
        self.horz_win() || self.vert_win()
    }

    fn horz_win(&self) -> bool {
        // for each row #
        //   row * 5: determines starting position in full array for this row
        //   .take(5): rows are 5 cells long
        //   .all(...): check if all cells are marked
        (0..5).any(|row| ((row * 5)..).take(5).all(|idx| self.0[idx].called))
    }

    fn vert_win(&self) -> bool {
        // for each col #
        //   step_by(5): produce iterator that skips through final list 5 at a time (0, 5, 10, etc.)
        //   .take(5): cols are 5 cells long
        //   .all(...): check if all cells are marked
        (0..5).any(|col| (col..).step_by(5).take(5).all(|idx| self.0[idx].called))
    }

    fn score(&self, winning_number: i32) -> i32 {
        self.sum_unmarked() * winning_number
    }

    fn sum_unmarked(&self) -> i32 {
        self.0
            .iter()
            .filter(|cell| !cell.called)
            .map(|cell| cell.num)
            .sum()
    }
}

// experiment packing the mark bit into the u8 directly.
// -- individual testing works but didn't work with existing algo for some reason
// struct C(u8);

// impl C {
//     fn value(&self) -> u8 {
//         self.0 & 0b01111111
//     }

//     fn mark(&mut self) {
//         self.0 |= 0b10000000
//     }

//     fn marked(&self) -> bool {
//         self.0 & 0b10000000 == 1
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day04.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(44088, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(23670, ans);
    }
}
