pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(count_arboreal_stops(input, 3, 1))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| count_arboreal_stops(input, dx, dy))
        .product())
}

fn count_arboreal_stops(input: &str, dx: usize, dy: usize) -> usize {
    let width = input.lines().next().unwrap().len();
    let mut trees = 0;
    let mut col = 0;
    for line in input.lines().step_by(dy).skip(1) {
        col += dx;
        if line.as_bytes()[col % width] == b'#' {
            trees += 1
        }
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day03.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(216, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(6708199680, ans);
    }
}
