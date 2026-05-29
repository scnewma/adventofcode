pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(iter_seats(input).fold(0, |acc, seat_id| acc.max(seat_id)))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (min, max, sum) = iter_seats(input).fold((usize::MAX, 0, 0), |(min, max, sum), seat_id| {
        (min.min(seat_id), max.max(seat_id), sum + seat_id)
    });
    let expected_sum = (min + max) * (max - min + 1) / 2;
    Ok(expected_sum - sum)
}

fn iter_seats(input: &str) -> impl Iterator<Item = usize> {
    input.lines().map(|boarding_pass| {
        let row = locate(&boarding_pass[..7], 127);
        let col = locate(&boarding_pass[7..], 7);
        row * 8 + col
    })
}

fn locate(path: &str, max: usize) -> usize {
    let (mut lo, mut hi) = (0, max);
    for c in path.chars() {
        match c {
            'F' | 'L' => hi = lo + (hi - lo) / 2,
            'B' | 'R' => lo = hi - (hi - lo) / 2,
            _ => unreachable!("character {c} unknown"),
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day05.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(816, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(539, ans);
    }
}
