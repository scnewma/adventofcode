pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut matrix = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    for line in &lines[..lines.len() - 1] {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        matrix.push(nums);
    }
    let mut sum = 0;
    for (col, op) in lines.last().unwrap().split_whitespace().enumerate() {
        let mut solution = if op == "*" { 1 } else { 0 };
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
    let mut m: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let ops = m.last().unwrap().clone();
    m.truncate(m.len() - 1);

    let mut sum = 0;
    let mut start = 0;
    let mut more = true;
    while more {
        let next = ops
            .iter()
            .enumerate()
            .skip(start + 1)
            .find(|(_, ch)| **ch != ' ')
            .map(|(i, _)| i);
        more = next.is_some();
        let next = next.unwrap_or(ops.len());

        let mut solution = if ops[start] == '*' { 1 } else { 0 };
        for col in start..next {
            let mut num = 0;
            for row in &m {
                if let Some(n) = row[col].to_digit(10) {
                    num = num * 10 + n as usize
                }
            }
            if num == 0 {
                // blank column
                continue;
            }

            match ops[start] {
                '*' => solution *= num,
                '+' => solution += num,
                _ => unreachable!(),
            }
        }
        sum += solution;

        start = next;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day06.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(4449991244405, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(9348430857627, ans);
    }
}
