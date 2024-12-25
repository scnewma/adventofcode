use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<i64> {
    let commands = parse_input(input)?;
    let mut hor = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(n) => hor += n,
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n,
        }
    }
    Ok(hor * depth)
}

pub fn part02(input: &str) -> anyhow::Result<i64> {
    let commands = parse_input(input)?;
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(n) => {
                hor += n;
                depth += aim * n;
            }
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n,
        }
    }
    Ok(hor * depth)
}

enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Command>> {
    let mut commands = Vec::new();
    for line in input.lines() {
        let (cmd, units) = line.split_once(' ').unwrap();
        let units: i64 = units.parse()?;

        let command = match cmd {
            "forward" => Command::Forward(units),
            "up" => Command::Up(units),
            "down" => Command::Down(units),
            _ => panic!("unknown command {}", cmd),
        };
        commands.push(command);
    }
    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day02.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(2187380, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(2086357770, ans);
    }
}
