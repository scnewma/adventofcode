pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let secret_numbers = input.lines().map(|line| line.parse::<usize>().unwrap());
    let mut sum = 0;
    for mut secret_number in secret_numbers {
        for _ in 0..2000 {
            let a = secret_number * 64;
            secret_number ^= a;
            secret_number %= 16777216;

            let b = secret_number / 32;
            secret_number ^= b;
            secret_number %= 16777216;

            let c = secret_number * 2048;
            secret_number ^= c;
            secret_number %= 16777216;
        }
        sum += secret_number;
    }
    Ok(sum)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day22.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
