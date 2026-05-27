use std::{
    collections::VecDeque,
    ops::{Add, Div, Mul, Sub},
};

use num::integer::gcd;

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
    let machines = parse_input(input);
    Ok(machines
        .into_iter()
        .map(fewest_button_presses_joltage)
        .sum())
}

fn fewest_button_presses_joltage(machine: Machine) -> usize {
    // convert to equations
    let mut equations = Vec::new();
    for (slot_index, j) in machine.joltage.iter().enumerate() {
        let mut eq = vec![Fraction::zero(); machine.buttons.len() + 1];
        for (btn_idx, btn) in machine.buttons.iter().enumerate() {
            if btn.contains(&slot_index) {
                eq[btn_idx] = Fraction::one();
            }
        }
        eq[machine.buttons.len()] = Fraction::new(*j as i128, 1);
        equations.push(eq);
    }

    // rref
    let mut pivot_row = 0;
    let mut pivots = Vec::new();
    // exclude rhs from loop
    for col in 0..machine.buttons.len() {
        let found = (pivot_row..equations.len()).find(|row| !equations[*row][col].is_zero());
        if found.is_none() {
            continue;
        }
        let found = found.unwrap();

        equations.swap(pivot_row, found);

        let pivot_value = equations[pivot_row][col];
        for c in 0..equations[pivot_row].len() {
            equations[pivot_row][c] = equations[pivot_row][c] / pivot_value;
        }

        for row in 0..equations.len() {
            if row == pivot_row {
                continue;
            }

            let factor = equations[row][col];
            if factor.is_zero() {
                continue;
            }

            for c in 0..equations[0].len() {
                equations[row][c] = equations[row][c] - factor * equations[pivot_row][c];
            }
        }

        pivots.push((pivot_row, col));
        pivot_row += 1;

        if pivot_row == equations.len() {
            break;
        }
    }

    // check for inconsistency
    for eq in &equations {
        let all_zero = eq[..machine.buttons.len()].iter().all(Fraction::is_zero);
        let rhs_nonzero = !eq[machine.buttons.len()].is_zero();
        if all_zero && rhs_nonzero {
            unreachable!("no solution");
        }
    }

    let pivot_cols: Vec<usize> = pivots.iter().map(|&(_, col)| col).collect();
    let free_cols: Vec<usize> = (0..machine.buttons.len())
        .filter(|col| !pivot_cols.contains(col))
        .collect();

    let mut solution = vec![Fraction::zero(); machine.buttons.len()];
    let mut best = None;
    enumerate_free_variables(
        0,
        &free_cols,
        &machine,
        &equations,
        &pivots,
        &mut solution,
        &mut best,
    );

    best.expect("no solution found")
}

fn enumerate_free_variables(
    free_index: usize,
    free_cols: &[usize],
    machine: &Machine,
    equations: &[Vec<Fraction>],
    pivots: &[(usize, usize)],
    solution: &mut [Fraction],
    best: &mut Option<usize>,
) {
    if free_index == free_cols.len() {
        let rhs_col = machine.buttons.len();

        for &(row, pivot_col) in pivots {
            let mut value = equations[row][rhs_col];
            for &free_col in free_cols {
                value = value - equations[row][free_col] * solution[free_col];
            }
            solution[pivot_col] = value;
        }

        let mut presses = 0;
        for value in solution.iter() {
            let Some(count) = value.as_nonnegative_usize() else {
                return;
            };
            presses += count;
        }

        if best.is_none_or(|best| presses < best) {
            *best = Some(presses);
        }
        return;
    }

    let free_col = free_cols[free_index];
    let max_count = machine.buttons[free_col]
        .iter()
        .map(|&slot| machine.joltage[slot])
        .min()
        .unwrap_or(0);

    for count in 0..=max_count {
        solution[free_col] = Fraction::new(count as i128, 1);
        enumerate_free_variables(
            free_index + 1,
            free_cols,
            machine,
            equations,
            pivots,
            solution,
            best,
        );
    }

    solution[free_col] = Fraction::zero();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Fraction {
    num: i128,
    den: i128,
}

impl Fraction {
    fn new(mut num: i128, mut den: i128) -> Fraction {
        if den == 0 {
            panic!("divide by zero");
        }

        if den < 0 {
            num = -num;
            den = -den;
        }
        if num == 0 {
            den = 1;
        }
        let n = gcd(num.abs(), den);
        num /= n;
        den /= n;
        Self { num, den }
    }

    fn zero() -> Fraction {
        Self { num: 0, den: 1 }
    }

    fn one() -> Fraction {
        Self { num: 1, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn as_nonnegative_usize(&self) -> Option<usize> {
        if self.den == 1 && self.num >= 0 {
            Some(self.num as usize)
        } else {
            None
        }
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den, self.den * rhs.num)
    }
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
        assert_eq!(21111, ans);
    }
}
