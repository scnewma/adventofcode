pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let first_digit = line
            .chars()
            .find(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap();
        sum += (first_digit * 10) + last_digit;
    }
    Ok(sum)
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = Vec::new();
        for (i, c) in line.char_indices() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap());
                continue;
            }

            let searches = [
                ("one", 1u32),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ];
            for (word, value) in searches {
                if i < word.len() - 1 {
                    continue;
                }

                if &line[i - (word.len() - 1)..=i] == word {
                    digits.push(value);
                }
            }
        }
        sum += (digits.first().unwrap() * 10) + digits.last().unwrap();
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day01.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(54951, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(55218, ans);
    }
}
