pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut matrix = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    for line in &lines[..lines.len()-1] {
        let nums:Vec <_> = line.split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        matrix.push(nums);
    }
    let mut sum = 0;
    for (col, op) in lines.last().unwrap().split_whitespace().enumerate() {
        let mut solution = if op == "*" { 1 } else {0};
        for row in &matrix {
            match op {
                "*" => solution *= row[col],
                "+" => solution += row[col],
                _ => unreachable!(),
            }
        }
        sum += solution;
    }
    Ok(sum)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day06.input.txt");

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
