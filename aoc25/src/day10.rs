use std::collections::VecDeque;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let machines = parse_input(input);
    Ok(machines.into_iter().map(fewest_button_presses).sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

fn fewest_button_presses(machine: Machine) -> usize {
    let mut q = VecDeque::new();
    for btn in &machine.buttons {
        let mut new_lights = 0;
        for &b in btn {
            new_lights ^= 1 << b;
        }
        q.push_back((new_lights, 1));
    }

    while let Some((lights, button_presses)) = q.pop_front() {
        if lights == machine.lights {
            return button_presses;
        }

        for btn in &machine.buttons {
            let mut new_lights = lights;
            for &b in btn {
                new_lights ^= 1 << b;
            }
            q.push_back((new_lights, button_presses + 1));
        }
    }
    unreachable!("no solution found")
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let mut fields = line.split_whitespace();

        let light_str = fields.next().unwrap();
        let light_str = &light_str[1..light_str.len() - 1]; //trim []
        let mut lights = 0u16;
        // .rev() so that our bit manipulating is easier in the algo above
        for l in light_str.chars().rev() {
            lights <<= 1;
            if l == '#' {
                lights |= 1;
            }
        }

        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        for field in fields {
            let f = &field[1..field.len() - 1]; //trim () or {}
            let values: Vec<usize> = f
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            if field.starts_with('(') {
                buttons.push(values);
            } else if field.starts_with('{') {
                joltage = values;
            }
        }
        machines.push(Machine {
            lights,
            buttons,
            joltage,
        });
    }
    machines
}

struct Machine {
    lights: u16, // bitset of on/off
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field("lights", &format!("0b{:016b}", self.lights))
            .field("buttons", &self.buttons)
            .field("joltage", &self.joltage)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day10.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(547, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
