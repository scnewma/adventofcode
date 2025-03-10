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

fn calculate_load(grid: &[Vec<char>]) -> usize {
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

fn spin_cycle(orig: &mut Vec<Vec<char>>) {
    tilt_north(orig);

    // tilt west
    let mut grid = rotate(orig);
    tilt_north(&mut grid);

    // tilt south
    let mut grid = rotate(&grid);
    tilt_north(&mut grid);

    // tilt east
    let mut grid = rotate(&grid);
    tilt_north(&mut grid);

    *orig = rotate(&grid);
}

fn tilt_north(grid: &mut [Vec<char>]) {
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

fn rotate<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut r = transpose(matrix);
    for col in &mut r {
        col.reverse();
    }
    r
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day14.input.txt");

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
