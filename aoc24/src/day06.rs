use fxhash::{FxHashMap, FxHashSet};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (grid, start_pos) = parse_input(input);
    let mut visited = FxHashSet::default();
    simulate(&grid, start_pos, |pos, _| {
        visited.insert(pos);
        true
    });
    Ok(visited.len())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (grid, start_pos) = parse_input(input);

    // only need to try and put an obstacle in positions that the guard would visit on the
    // pre-modified grid
    let mut initial_visited = FxHashSet::default();
    simulate(&grid, start_pos, |pos, _| {
        initial_visited.insert(pos);
        true
    });

    // can't put an obstacle in the start position
    initial_visited.remove(&start_pos);

    Ok(initial_visited
        .par_iter()
        .filter(|obstacle_pos| {
            let mut grid = grid.clone();
            grid.insert(**obstacle_pos, '#');

            let mut looped = false;
            let mut visited = FxHashSet::default();
            simulate(&grid, start_pos, |pos, dir| {
                let new = visited.insert((pos, dir));
                if !new {
                    looped = true;
                }
                new
            });
            looped
        })
        .count())
}

fn simulate<F>(grid: &Grid, start_pos: Pos, mut on_visit: F)
where
    F: FnMut(Pos, u8) -> bool,
{
    let mut pos = start_pos;
    let mut dir = 0;

    while on_visit(pos, dir) {
        let (row, col) = pos;
        let next = match dir {
            0 => (row - 1, col), // north
            1 => (row, col + 1), // east
            2 => (row + 1, col), // south
            3 => (row, col - 1), // west
            _ => unreachable!(),
        };

        match grid.get(&next) {
            Some('#') => dir = (dir + 1) % 4,
            Some('.') => pos = next,
            None => break,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> (Grid, Pos) {
    let mut grid = FxHashMap::default();
    let mut pos = None;
    for (row, line) in input.lines().enumerate() {
        for (col, mut ch) in line.char_indices() {
            if ch == '^' {
                pos = Some((row as isize, col as isize));
                ch = '.';
            }
            grid.insert((row as isize, col as isize), ch);
        }
    }
    (grid, pos.unwrap())
}

type Grid = FxHashMap<Pos, char>;
type Pos = (isize, isize);

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
