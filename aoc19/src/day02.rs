use itertools::iproduct;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut memory = parse_input(input);
    memory[1] = 12;
    memory[2] = 2;

    Ok(eval_intcode(&mut memory))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    const EXPECT: usize = 19690720;

    let memory = parse_input(input);

    Ok(iproduct!(0..100, 0..100)
        .find(|&(noun, verb)| {
            let mut memory = memory.clone();
            memory[1] = noun;
            memory[2] = verb;

            eval_intcode(&mut memory) == EXPECT
        })
        .map(|(noun, verb)| 100 * noun + verb)
        .unwrap())
}

fn eval_intcode(memory: &mut [usize]) -> usize {
    let mut ip = 0;
    while memory[ip] != 99 {
        let lptr = memory[ip + 1];
        let rptr = memory[ip + 2];
        let optr = memory[ip + 3];

        match memory[ip] {
            1 => memory[optr] = memory[lptr] + memory[rptr],
            2 => memory[optr] = memory[lptr] * memory[rptr],
            op => unreachable!("invalid op code {op}"),
        }
        ip += 4;
    }

    return memory[0];
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day02.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3267740, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(7870, ans);
    }
}
