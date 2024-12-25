use std::collections::HashSet;
use std::fmt::Write;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: format!("\n{}", part02(input)),
    })
}

pub fn part01(input: &str) -> i64 {
    let (points, folds) = parse_input(input);
    execute_folds(&points, &folds[0..1]).len() as i64
}

pub fn part02(input: &str) -> String {
    let (points, folds) = parse_input(input);
    let grid = execute_folds(&points, &folds);
    stringify_grid(&grid)
}

enum Fold {
    X(usize),
    Y(usize),
}

fn execute_folds(points: &[(usize, usize)], folds: &[Fold]) -> HashSet<(usize, usize)> {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut grid = HashSet::new();
    for (x, y) in points {
        grid.insert((*x, *y));
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    for fold in folds {
        grid = match fold {
            Fold::X(idx) => {
                let mut new_grid = HashSet::new();

                for y in 0..=max_y {
                    for x in 0..*idx {
                        let left = grid.get(&(x, y));
                        let right = grid.get(&(2 * idx - x, y));

                        match (left, right) {
                            (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => {
                                new_grid.insert((x, y));
                            }
                            (None, None) => continue,
                        }
                    }
                }

                new_grid
            }
            Fold::Y(idx) => {
                let mut new_grid = HashSet::new();

                for y in 0..*idx {
                    for x in 0..=max_x {
                        let top = grid.get(&(x, y));
                        let bot = grid.get(&(x, 2 * idx - y));

                        match (top, bot) {
                            (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => {
                                new_grid.insert((x, y));
                            }
                            (None, None) => continue,
                        }
                    }
                }

                new_grid
            }
        };
    }

    grid
}

fn stringify_grid(grid: &HashSet<(usize, usize)>) -> String {
    let mut s = String::new();
    let (max_x, max_y) = grid.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    });

    for y in 0..=max_y {
        for x in 0..=max_x {
            let point = grid.get(&(x, y)).map_or(" ", |_| "#");
            write!(&mut s, "{}", point).unwrap();
        }
        if y != max_y {
            writeln!(&mut s).unwrap();
        }
    }
    s
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Fold>) {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points: Vec<(usize, usize)> = points
        .lines()
        .flat_map(|s| s.split_once(','))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let folds = folds
        .lines()
        .flat_map(|s| s.strip_prefix("fold along "))
        .flat_map(|s| s.split_once('='))
        .map(|(axis, idx)| {
            let idx = idx.parse().unwrap();
            match axis {
                "x" => Fold::X(idx),
                "y" => Fold::Y(idx),
                _ => unreachable!(),
            }
        })
        .collect();

    (points, folds)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day13.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(610, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        let expected = r#"
###  #### ####   ## #  # ###  #### ####
#  #    # #       # #  # #  # #       #
#  #   #  ###     # #### #  # ###    # 
###   #   #       # #  # ###  #     #  
#    #    #    #  # #  # # #  #    #   
#    #### #     ##  #  # #  # #    ####"#;
        assert_eq!(expected.trim_start(), ans);
    }
}
