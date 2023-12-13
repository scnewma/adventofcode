use itertools::{iproduct, Itertools};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let patterns = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();
    let mut total = 0;
    for pattern in patterns {
        for row in &pattern {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
        let reflections = lines_of_reflection(&pattern, "PART 1");
        assert!(reflections.len() == 1);
        match reflections[0] {
            LineOfReflection::Horizontal(n) => {
                total += n;
            }
            LineOfReflection::Vertical(n) => {
                total += n * 100;
            }
        }
    }
    Ok(total)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    println!("part02");
    let patterns = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let mut total = 0;
    for pattern in patterns {
        let orig_reflections = lines_of_reflection(&pattern, "ORIGINAL");
        assert!(orig_reflections.len() == 1);
        let orig_reflection = orig_reflections[0];

        let mut found = false;
        'permutations: for (r, c) in iproduct!(0..pattern.len(), 0..pattern[0].len()) {
            let mut perm = pattern.clone();
            perm[r][c] = if perm[r][c] == '#' { '.' } else { '#' };

            let reflections = lines_of_reflection(&perm, "SMUDGED");
            for reflection in reflections {
                if reflection != orig_reflection {
                    total += match reflection {
                        LineOfReflection::Horizontal(n) => n,
                        LineOfReflection::Vertical(n) => 100 * n,
                    };
                    found = true;
                    break 'permutations;
                }
            }
        }

        assert!(found);

        // TODO: there is a bug in the below, but i'm not sure what it is. it should be much more
        // efficient though...
        //
        // let orig_reflections = lines_of_reflection(&pattern, "ORIGINAL");
        // assert!(orig_reflections.len() == 1);
        // let orig_reflection = orig_reflections[0];

        // let before = total;
        // for i in 0..pattern.len() {
        //     for j in i + 1..pattern.len() {
        //         let a = String::from_iter(&pattern[i]);
        //         let b = String::from_iter(&pattern[j]);
        //         if edit_distance(&a, &b) != 1 {
        //             continue;
        //         }

        //         let mut updated = pattern.clone();
        //         updated[i] = pattern[j].clone();
        //         let reflections = lines_of_reflection(&updated, "ROW MUTATION 1");
        //         for reflection in reflections {
        //             if reflection != orig_reflection {
        //                 total += match reflection {
        //                     LineOfReflection::Horizontal(n) => n,
        //                     LineOfReflection::Vertical(n) => 100 * n,
        //                 };
        //                 continue 'patterns;
        //             }
        //         }

        //         let mut updated = pattern.clone();
        //         updated[j] = pattern[i].clone();
        //         let reflections = lines_of_reflection(&updated, "ROW MUTATION 2");
        //         for reflection in reflections {
        //             if reflection != orig_reflection {
        //                 total += match reflection {
        //                     LineOfReflection::Horizontal(n) => n,
        //                     LineOfReflection::Vertical(n) => 100 * n,
        //                 };
        //                 continue 'patterns;
        //             }
        //         }
        //     }
        // }

        // let pattern = transpose(&pattern);
        // for i in 0..pattern.len() {
        //     for j in i + 1..pattern.len() {
        //         let a = String::from_iter(&pattern[i]);
        //         let b = String::from_iter(&pattern[j]);
        //         if edit_distance(&a, &b) != 1 {
        //             continue;
        //         }

        //         let mut updated = pattern.clone();
        //         updated[i] = pattern[j].clone();
        //         let reflections = lines_of_reflection(&updated, "COL MUTATION 1");
        //         for reflection in reflections {
        //             if reflection != orig_reflection {
        //                 total += match reflection {
        //                     LineOfReflection::Horizontal(n) => n,
        //                     LineOfReflection::Vertical(n) => 100 * n,
        //                 };
        //                 continue 'patterns;
        //             }
        //         }

        //         let mut updated = pattern.clone();
        //         updated[j] = pattern[i].clone();
        //         let reflections = lines_of_reflection(&updated, "COL MUTATION 2");
        //         for reflection in reflections {
        //             if reflection != orig_reflection {
        //                 total += match reflection {
        //                     LineOfReflection::Horizontal(n) => n,
        //                     LineOfReflection::Vertical(n) => 100 * n,
        //                 };
        //                 continue 'patterns;
        //             }
        //         }
        //     }
        // }
        // if before == total {
        //     println!("no reflection");
        // }
    }
    Ok(total)
}

fn print_pattern(pattern: &Vec<Vec<char>>) {
    for row in pattern {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineOfReflection {
    Horizontal(usize),
    Vertical(usize),
}

fn lines_of_reflection(pattern: &Vec<Vec<char>>, label: &str) -> Vec<LineOfReflection> {
    println!("--- {label}");
    print_pattern(pattern);
    let mut reflections = Vec::new();
    reflections.append(
        &mut reflects_horizontal(pattern)
            .into_iter()
            .inspect(|n| println!("reflects horizontal at {}", n))
            .map(LineOfReflection::Horizontal)
            .collect_vec(),
    );
    reflections.append(
        &mut reflects_vertical(pattern)
            .into_iter()
            .inspect(|n| println!("reflects vertical at {}", n))
            .map(LineOfReflection::Vertical)
            .collect_vec(),
    );
    reflections
}

fn reflects_vertical(pattern: &Vec<Vec<char>>) -> Vec<usize> {
    let mut reflections = vec![];
    'outer: for row in 0..pattern.len() - 1 {
        let mut top = row as i32;
        let mut bot = row + 1;
        while top >= 0 && bot < pattern.len() {
            if pattern[top as usize] != pattern[bot] {
                continue 'outer;
            }
            top -= 1;
            bot += 1;
        }
        reflections.push(row + 1);
    }
    reflections
}

fn reflects_horizontal(pattern: &Vec<Vec<char>>) -> Vec<usize> {
    let pattern = transpose(pattern);
    reflects_vertical(&pattern)
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut t = vec![Vec::with_capacity(matrix.len()); matrix[0].len()];
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

    const INPUT: &'static str = include_str!("../inputs/day13.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(43614, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(36771, ans);
    }
}
