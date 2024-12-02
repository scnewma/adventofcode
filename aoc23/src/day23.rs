use std::collections::BinaryHeap;

use fxhash::FxHashSet;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = (0, grid[0].iter().position(|&c| c == '.').unwrap());
    let end = (
        grid.len() - 1,
        grid[grid.len() - 1].iter().position(|&c| c == '.').unwrap(),
    );
    Ok(longest_path(&grid, start, end))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    println!("part02");
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| match ch {
                    '^' | '>' | '<' | 'v' => '.',
                    ch => ch,
                })
                .collect()
        })
        .collect();
    let start = (0, grid[0].iter().position(|&c| c == '.').unwrap());
    let end = (
        grid.len() - 1,
        grid[grid.len() - 1].iter().position(|&c| c == '.').unwrap(),
    );
    Ok(longest_path_dfs(
        &grid,
        &mut FxHashSet::default(),
        start,
        end,
    ))
}

fn longest_path(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push((0, start, vec![]));

    let mut distances = vec![vec![0; grid[0].len()]; grid.len()];

    while let Some((dist, (row, col), path)) = queue.pop() {
        distances[row][col] = distances[row][col].max(dist);
        let deltas = match grid[row][col] {
            '>' => vec![(0, 1)],
            '<' => vec![(0, -1)],
            'v' => vec![(1, 0)],
            '^' => vec![(-1, 0)],
            _ => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        };

        for (dr, dc) in deltas {
            let (nr, nc) = (row as i32 + dr, col as i32 + dc);

            if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == '#' || path.contains(&(nr, nc)) {
                continue;
            }

            let mut path = path.clone();
            path.push((row, col));
            queue.push((dist + 1, (nr, nc), path));
        }
    }
    distances[end.0][end.1]
}

fn longest_path_dfs(
    grid: &Vec<Vec<char>>,
    visited: &mut FxHashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    if start == end {
        return visited.len();
    }

    let (row, col) = start;
    let mut longest_path = 0;
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nr, nc) = (row as i32 + dr, col as i32 + dc);
        if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
            continue;
        }

        let (nr, nc) = (nr as usize, nc as usize);
        if grid[nr][nc] == '#' {
            continue;
        }

        if !visited.insert((nr, nc)) {
            continue;
        }
        longest_path = longest_path.max(longest_path_dfs(grid, visited, (nr, nc), end));
        visited.remove(&(nr, nc));
    }
    longest_path
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day23.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(2314, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(6874, ans);
    }
}
