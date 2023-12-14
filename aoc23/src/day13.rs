use itertools::{iproduct, Itertools};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let patterns = parse_input(input);

    Ok(patterns
        .into_iter()
        .map(|pattern| {
            let reflections = lines_of_reflection(&pattern);
            assert!(reflections.len() == 1);
            reflections[0].value()
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let patterns = parse_input(input);

    Ok(patterns
        .into_iter()
        .map(|mut pattern| {
            let orig_reflections = lines_of_reflection(&pattern);
            assert!(orig_reflections.len() == 1);
            let orig_reflection = orig_reflections[0];

            for (r, c) in iproduct!(0..pattern.len(), 0..pattern[0].len()) {
                pattern[r][c] = if pattern[r][c] == '#' { '.' } else { '#' };

                let reflections = lines_of_reflection(&pattern);
                for reflection in reflections {
                    if reflection != orig_reflection {
                        return reflection.value();
                    }
                }

                // reset
                pattern[r][c] = if pattern[r][c] == '#' { '.' } else { '#' };
            }

            unreachable!("no alternative reflection found");
        })
        .sum())
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineOfReflection {
    Horizontal(usize),
    Vertical(usize),
}

impl LineOfReflection {
    fn value(&self) -> usize {
        match self {
            LineOfReflection::Horizontal(n) => *n,
            LineOfReflection::Vertical(n) => *n * 100,
        }
    }
}

fn lines_of_reflection(pattern: &Vec<Vec<char>>) -> Vec<LineOfReflection> {
    let mut reflections = Vec::new();
    reflections.append(
        &mut reflects_horizontal(pattern)
            .into_iter()
            .map(LineOfReflection::Horizontal)
            .collect_vec(),
    );
    reflections.append(
        &mut reflects_vertical(pattern)
            .into_iter()
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
