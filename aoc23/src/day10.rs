use std::collections::{HashMap, HashSet};

use itertools::iproduct;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut grid: HashMap<(isize, isize), char> = HashMap::new();
    for (r, line) in input.lines().enumerate() {
        for (c, char) in line.char_indices() {
            grid.insert((r as isize, c as isize), char);
        }
    }

    let start = grid
        .iter()
        .find(|(_, ch)| **ch == 'S')
        .map(|(pos, _)| pos)
        .unwrap();

    let mut seen = HashSet::new();
    let mut stack = Vec::new();
    stack.push((start.0, start.1, vec![]));
    let len = loop {
        let current = stack.pop().unwrap();
        let (row, col, path) = current;
        let ch = grid[&(row, col)];

        if grid[&(row, col)] == 'S' && path.len() > 1 && path[1] != *path.last().unwrap() {
            break path.len();
        }

        if !seen.insert((row, col)) {
            continue;
        }

        // south
        if ch == 'S' || ch == '|' || ch == '7' || ch == 'F' {
            match grid.get(&(row + 1, col)) {
                Some('|') | Some('L') | Some('J') | Some('S') => {
                    let mut path = path.clone();
                    path.push((row, col));
                    stack.push((row + 1, col, path));
                }
                Some(_) | None => (),
            }
        }

        // north
        if ch == 'S' || ch == '|' || ch == 'L' || ch == 'J' {
            match grid.get(&(row - 1, col)) {
                Some('|') | Some('7') | Some('F') | Some('S') => {
                    let mut path = path.clone();
                    path.push((row, col));
                    stack.push((row - 1, col, path));
                }
                Some(_) | None => (),
            }
        }

        // east
        if ch == 'S' || ch == '-' || ch == 'L' || ch == 'F' {
            match grid.get(&(row, col + 1)) {
                Some('-') | Some('J') | Some('7') | Some('S') => {
                    let mut path = path.clone();
                    path.push((row, col));
                    stack.push((row, col + 1, path));
                }
                Some(_) | None => (),
            }
        }

        // west
        if ch == 'S' || ch == '-' || ch == 'J' || ch == '7' {
            match grid.get(&(row, col - 1)) {
                Some('-') | Some('L') | Some('F') | Some('S') => {
                    let mut path = path.clone();
                    path.push((row, col));
                    stack.push((row, col - 1, path));
                }
                Some(_) | None => (),
            }
        }
    };

    let mid = len / 2;
    Ok(mid)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut grid: HashMap<(isize, isize), char> = HashMap::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    // println!("h={height} w={width}");
    for (r, line) in input.lines().enumerate() {
        for (c, char) in line.char_indices() {
            grid.insert((r as isize, c as isize), char);
        }
    }

    let start = grid
        .iter()
        .find(|(_, ch)| **ch == 'S')
        .map(|(pos, _)| pos)
        .unwrap();

    let mut seen = HashSet::new();
    let mut stack = Vec::new();
    stack.push((start.0, start.1, vec![]));
    let path = loop {
        let current = stack.pop().unwrap();
        let (row, col, path) = current;
        let ch = grid[&(row, col)];

        if grid[&(row, col)] == 'S' && path.len() > 1 && path[1] != *path.last().unwrap() {
            break path;
        }

        if !seen.insert((row, col)) {
            continue;
        }

        // south
        if ch == 'S' || ch == '|' || ch == '7' || ch == 'F' {
            match grid.get(&(row + 1, col)) {
                Some('|') | Some('L') | Some('J') | Some('S') => {
                    let mut path = path.clone();
                    path.push((row, col));
                    stack.push((row + 1, col, path));
                }
                Some(_) | None => (),
            }
        }

        // north
        if ch == 'S' || ch == '|' || ch == 'L' || ch == 'J' {
            match grid.get(&(row - 1, col)) {
                Some('|') | Some('7') | Some('F') | Some('S') => {
                    let mut path = path.clone();
                    path.push((row, col));
                    stack.push((row - 1, col, path));
                }
                Some(_) | None => (),
            }
        }

        // east
        if ch == 'S' || ch == '-' || ch == 'L' || ch == 'F' {
            match grid.get(&(row, col + 1)) {
                Some('-') | Some('J') | Some('7') | Some('S') => {
                    let mut path = path.clone();
                    path.push((row, col));
                    stack.push((row, col + 1, path));
                }
                Some(_) | None => (),
            }
        }

        // west
        if ch == 'S' || ch == '-' || ch == 'J' || ch == '7' {
            match grid.get(&(row, col - 1)) {
                Some('-') | Some('L') | Some('F') | Some('S') => {
                    let mut path = path.clone();
                    path.push((row, col));
                    stack.push((row, col - 1, path));
                }
                Some(_) | None => (),
            }
        }
    };

    clean(&mut grid, width as isize, height as isize, &path);
    let scaled = scale(&grid, width as isize, height as isize);
    print_grid(&scaled, (width * 3) as isize, (height * 3) as isize);

    let mut inside = vec![];
    for (row, col) in iproduct!(0..height, 0..width) {
        let (row, col) = (row as isize, col as isize);
        if path.contains(&(row, col)) {
            continue;
        }

        if !is_outside(
            &scaled,
            &mut HashSet::new(),
            (height * 3) as isize,
            (width * 3) as isize,
            (row * 3) + 1,
            (col * 3) + 1,
        ) {
            inside.push((row, col));
        }
    }

    Ok(inside.len())
}

fn clean(
    grid: &mut HashMap<(isize, isize), char>,
    width: isize,
    height: isize,
    path: &[(isize, isize)],
) {
    for (row, col) in iproduct!(0..height, 0..width) {
        if !path.contains(&(row, col)) {
            grid.insert((row, col), '.');
        }
    }
}

fn scale(
    grid: &HashMap<(isize, isize), char>,
    width: isize,
    height: isize,
) -> HashMap<(isize, isize), char> {
    let mut s_pos = (0, 0);
    let mut scaled = HashMap::new();
    for (row, col) in iproduct!(0..height, 0..width) {
        let mask = match grid[&(row, col)] {
            '|' => ".x..x..x.",
            '-' => "...xxx...",
            'L' => ".x..xx...",
            'J' => ".x.xx....",
            '7' => "...xx..x.",
            'F' => "....xx.x.",
            // we fill in 'S' later
            'S' => {
                s_pos = ((row * 3) + 1, (col * 3) + 1);
                "....x...."
            }
            _ => ".........",
        };

        for i in 0..9 {
            let sr = row * 3 + (i / 3);
            let sc = col * 3 + (i % 3);
            scaled.insert((sr, sc), mask.chars().nth(i as usize).unwrap());
        }
    }

    // populate the 'S'
    for (dr, dc) in [(2, 0), (-2, 0), (0, 2), (0, -2)] {
        if let Some('x') = scaled.get(&(s_pos.0 + dr, s_pos.1 + dc)) {
            scaled.insert((s_pos.0 + (dr / 2), s_pos.1 + (dc / 2)), 'x');
        }
    }

    scaled
}

fn grid_str(grid: &HashMap<(isize, isize), char>, width: isize, height: isize) -> String {
    let mut s = String::new();
    for row in 0..height {
        for col in 0..width {
            s.push(grid[&(row, col)]);
        }
        s.push('\n');
    }
    s.trim_end().to_string()
}

fn print_grid(grid: &HashMap<(isize, isize), char>, width: isize, height: isize) {
    println!("{}", grid_str(grid, width, height));
}

fn is_outside(
    grid: &HashMap<(isize, isize), char>,
    // curr_path: Vec<(isize, isize)>,
    seen: &mut HashSet<(isize, isize)>,
    height: isize,
    width: isize,
    row: isize,
    col: isize,
) -> bool {
    if !seen.insert((row, col)) {
        return false;
    }
    // println!("{row},{col} path={curr_path:?}");
    match grid.get(&(row, col)) {
        Some('x') => {
            // println!("  wall");
            return false;
        }
        None => {
            // println!("  off grid");
            return false;
        }
        Some(_) => (),
    }

    row == 0
        || row == height - 1
        || col == 0
        || col == width - 1
        || is_outside(grid, seen, height, width, row + 1, col)
        || is_outside(grid, seen, height, width, row - 1, col)
        || is_outside(grid, seen, height, width, row, col + 1)
        || is_outside(grid, seen, height, width, row, col - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day10.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(6773, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(493, ans);
    }
}
