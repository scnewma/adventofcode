use arrayvec::ArrayVec;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const PLAYERS: usize = 2;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut positions: ArrayVec<usize, PLAYERS> = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
        .collect();

    let mut scores = ArrayVec::<_, PLAYERS>::from_iter([0usize; 2]);

    let mut dice = (1..=100).cycle();
    let mut rolls = 0;

    'outer: loop {
        for pid in 0..PLAYERS {
            let (r1, r2, r3) = dice.next_tuple().unwrap();
            let total_roll = r1 + r2 + r3;
            rolls += 3;
            let next_position = (positions[pid] + total_roll - 1) % 10 + 1;
            scores[pid] += next_position;
            positions[pid] = next_position;

            if scores[pid] >= 1000 {
                break 'outer;
            }
        }
    }

    let min_score = scores.iter().min().unwrap();
    Ok(min_score * rolls)
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
        assert_eq!(556206, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
