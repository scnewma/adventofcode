pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
// pub mod day11;
// pub mod day12;

pub struct SolveInfo {
    pub part01: String,
    pub part02: String,
}

// useful when iterating over a grid and checking cardinal directions of a cell
pub const DELTAS4: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// useful when iterating over a grid and checking all 8 neighbors of a cell
pub const DELTAS8: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
