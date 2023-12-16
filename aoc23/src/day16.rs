use fxhash::FxHashSet;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    Ok(energized(&grid, (0, 0, Direction::Right)))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let (width, height) = (grid[0].len() as i32, grid.len() as i32);
    let mut ans = 0;
    for col in 0..width {
        ans = ans.max(energized(&grid, (0, col, Direction::Down)));
        ans = ans.max(energized(&grid, (height - 1, col, Direction::Up)));
    }
    for row in 0..height {
        ans = ans.max(energized(&grid, (row, 0, Direction::Right)));
        ans = ans.max(energized(&grid, (row, width - 1, Direction::Left)));
    }
    Ok(ans)
}

fn energized(grid: &Vec<Vec<char>>, start: (i32, i32, Direction)) -> usize {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut energized = FxHashSet::default();
    let mut beams = Vec::new();
    beams.push(start);
    while let Some((row, col, dir)) = beams.pop() {
        if row < 0 || col < 0 || row >= height || col >= width {
            continue;
        }
        if !energized.insert((row, col, dir)) {
            continue;
        }

        let ch = grid[row as usize][col as usize];
        match (ch, &dir) {
            ('.', _)
            | ('|', Direction::Up)
            | ('|', Direction::Down)
            | ('-', Direction::Left)
            | ('-', Direction::Right) => beams.push((dir.next_row(row), dir.next_col(col), dir)),
            ('/', Direction::Right) => beams.push((row - 1, col, Direction::Up)),
            ('/', Direction::Left) => beams.push((row + 1, col, Direction::Down)),
            ('/', Direction::Up) => beams.push((row, col + 1, Direction::Right)),
            ('/', Direction::Down) => beams.push((row, col - 1, Direction::Left)),
            ('\\', Direction::Right) => beams.push((row + 1, col, Direction::Down)),
            ('\\', Direction::Left) => beams.push((row - 1, col, Direction::Up)),
            ('\\', Direction::Up) => beams.push((row, col - 1, Direction::Left)),
            ('\\', Direction::Down) => beams.push((row, col + 1, Direction::Right)),
            ('|', _) => {
                beams.push((row - 1, col, Direction::Up));
                beams.push((row + 1, col, Direction::Down));
            }
            ('-', _) => {
                beams.push((row, col - 1, Direction::Left));
                beams.push((row, col + 1, Direction::Right));
            }
            _ => unreachable!(),
        }
    }

    let positions: FxHashSet<_> = energized.into_iter().map(|(r, c, _)| (r, c)).collect();
    positions.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_row(&self, curr: i32) -> i32 {
        curr + match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }

    fn next_col(&self, curr: i32) -> i32 {
        curr + match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day16.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(7543, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(8231, ans);
    }
}
