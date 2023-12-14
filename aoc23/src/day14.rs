use std::collections::HashMap;

use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut grid = parse_input(input);
    tilt_north(&mut grid);
    Ok(calculate_load(&grid))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut grid = parse_input(input);

    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut i = 0;
    let cycle_len = loop {
        spin_cycle(&mut grid);

        // detect cycle
        if let Some(j) = cache.get(&grid) {
            break i - j;
        }

        cache.insert(grid.clone(), i);
        i += 1;
    };

    // i+1 to account for the last spin cycle
    let rem = (1000000000 - (i + 1)) % cycle_len;
    for _ in 0..rem {
        spin_cycle(&mut grid);
    }
    Ok(calculate_load(&grid))
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == 'O' {
                total += grid.len() - row;
            }
        }
    }
    total
}

fn spin_cycle(grid: &mut Vec<Vec<char>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == '.' {
                for i in row + 1..grid.len() {
                    match grid[i][col] {
                        'O' => {
                            grid[row][col] = 'O';
                            grid[i][col] = '.';
                            break;
                        }
                        '#' => break,
                        _ => (),
                    }
                }
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let mut tilted = rotate(&rotate(grid));
    tilt_north(&mut tilted);
    *grid = rotate(&rotate(&tilted));
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    let mut rotated = rotate_left(grid.clone());
    tilt_north(&mut rotated);
    *grid = rotate(&rotated);
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    let mut rotated = rotate(grid);
    tilt_north(&mut rotated);
    *grid = rotate_left(rotated);
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut t = vec![Vec::with_capacity(matrix.len()); matrix.len()];
    for r in matrix {
        for i in 0..r.len() {
            t[i].push(r[i].clone());
        }
    }
    t
}

fn rotate<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
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
    transpose(&r)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day14.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(107053, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(88371, ans);
    }
}
