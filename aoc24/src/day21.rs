use std::collections::{HashSet, VecDeque};

use fxhash::FxHashMap;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let codes: Vec<&str> = input.lines().collect();

    let numerical_grid = FxHashMap::from_iter([
        ('7', (0isize, 0isize)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);
    let not_valid = (0, 3);
    let mut numerical_paths = FxHashMap::default();
    for pair in ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        .iter()
        .permutations(2)
    {
        let (ay, ax) = numerical_grid[pair[0]];
        let (by, bx) = numerical_grid[pair[1]];
        let horiz = if ax - bx < 0 {
            ">".repeat(ax.abs_diff(bx))
        } else {
            "<".repeat(ax.abs_diff(bx))
        };
        let vert = if ay - by < 0 {
            "v".repeat(ay.abs_diff(by))
        } else {
            "^".repeat(ay.abs_diff(by))
        };

        let paths: HashSet<String> = [horiz.clone() + &vert, vert + &horiz]
            .into_iter()
            .filter(|path| {
                let (mut x, mut y) = (ax, ay);
                for symbol in path.chars() {
                    match symbol {
                        '^' => y -= 1,
                        'v' => y += 1,
                        '<' => x -= 1,
                        '>' => x += 1,
                        _ => unreachable!(),
                    }
                    if (x, y) == not_valid {
                        return false;
                    }
                }
                true
            })
            .collect();

        numerical_paths.insert((*pair[0], *pair[1]), paths);
    }
    for sym in ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
        numerical_paths.insert((sym, sym), HashSet::from_iter(["".to_string()]));
    }

    let mut dir_paths = FxHashMap::default();
    dir_paths.insert(('^', 'A'), vec![">"]);
    dir_paths.insert(('^', 'v'), vec!["v"]);
    dir_paths.insert(('^', '>'), vec![">v", "v>"]);
    dir_paths.insert(('^', '<'), vec!["v<"]);
    dir_paths.insert(('^', '^'), vec![""]);

    dir_paths.insert(('A', '^'), vec!["<"]);
    dir_paths.insert(('A', '>'), vec!["v"]);
    dir_paths.insert(('A', 'v'), vec!["v<", "<v"]);
    dir_paths.insert(('A', '<'), vec!["v<<"]);
    dir_paths.insert(('A', 'A'), vec![""]);

    dir_paths.insert(('>', 'A'), vec!["^"]);
    dir_paths.insert(('>', 'v'), vec!["<"]);
    dir_paths.insert(('>', '^'), vec!["^<", "<^"]);
    dir_paths.insert(('>', '<'), vec!["<<"]);
    dir_paths.insert(('>', '>'), vec![""]);

    dir_paths.insert(('<', 'A'), vec![">>^"]);
    dir_paths.insert(('<', 'v'), vec![">"]);
    dir_paths.insert(('<', '^'), vec![">^"]);
    dir_paths.insert(('<', '>'), vec![">>"]);
    dir_paths.insert(('<', '<'), vec![""]);

    dir_paths.insert(('v', '^'), vec!["^"]);
    dir_paths.insert(('v', '>'), vec![">"]);
    dir_paths.insert(('v', '<'), vec!["<"]);
    dir_paths.insert(('v', 'A'), vec![">^", "^>"]);
    dir_paths.insert(('v', 'v'), vec![""]);

    let mut ans = 0;
    for code in codes {
        let mut nkp = 'A';
        let mut dkp_codes = vec!["".to_string()];
        for sym in code.chars() {
            let paths = &numerical_paths[&(nkp, sym)];
            let mut extended = Vec::new();
            for path in paths {
                for code in &dkp_codes {
                    extended.push(code.clone() + path + "A");
                }
            }
            dkp_codes = extended;
            nkp = sym;
        }

        for _ in 0..2 {
            let mut next_dkp_codes = Vec::new();

            for dkp_code in dkp_codes {
                let mut dkp = 'A';
                let mut next_codes = vec!["".to_string()];
                for sym in dkp_code.chars() {
                    let paths = &dir_paths[&(dkp, sym)];
                    let mut extended = Vec::new();
                    for path in paths {
                        for code in &next_codes {
                            extended.push(code.clone() + path + "A");
                        }
                    }
                    dkp = sym;
                    next_codes = extended;
                }
                next_dkp_codes.extend(next_codes);
            }
            dkp_codes = next_dkp_codes;
        }

        let complexity = code[..3].parse::<usize>().unwrap()
            * dkp_codes.iter().map(|code| code.len()).min().unwrap();
        ans += complexity;
    }
    Ok(ans)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day21.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(237342, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
