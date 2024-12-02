use std::str::FromStr;

use anyhow::Context;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let (workflows, parts) = parse_input(input);
    Ok(parts
        .into_iter()
        .map(|part| {
            let mut next_part = "in".to_string();

            while next_part != "A" && next_part != "R" {
                let workflow = workflows.iter().find(|w| w.name == *next_part).unwrap();
                next_part = workflow.eval(&part);
            }

            if next_part == "A" {
                part.x + part.m + part.a + part.s
            } else {
                0
            }
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
    let (workflows, _) = parse_input(input);

    let mut accepted = Vec::new();
    find_accepted_ranges(&workflows, "in", &Ranges::default(), &mut accepted);

    Ok(accepted
        .into_iter()
        .map(|r| {
            (r.x.1 - r.x.0) as u64
                * (r.m.1 - r.m.0) as u64
                * (r.a.1 - r.a.0) as u64
                * (r.s.1 - r.s.0) as u64
        })
        .sum())
}

fn find_accepted_ranges(
    workflows: &[Workflow],
    workflow: &str,
    ranges: &Ranges,
    accepted: &mut Vec<Ranges>,
) {
    if workflow == "A" {
        accepted.push(*ranges);
        return;
    } else if workflow == "R" {
        return;
    }

    let workflow = workflows
        .iter()
        .find(|w| w.name == *workflow)
        .with_context(|| format!("workflow {} not found", workflow))
        .unwrap();

    let mut ranges = *ranges;
    for expr in &workflow.exprs {
        match expr {
            Expr::Gt(category, bound, next) => {
                // expr is true
                let mut gt_ranges = ranges;
                gt_ranges.modify(category, |r| r.0 = r.0.max(*bound + 1));
                find_accepted_ranges(workflows, next, &gt_ranges, accepted);

                // expr is false
                ranges.modify(category, |r| r.1 = r.1.min(*bound + 1));
            }
            Expr::Lt(category, bound, next) => {
                // expr is true
                let mut lt_ranges = ranges;
                lt_ranges.modify(category, |r| r.1 = r.1.min(*bound));
                find_accepted_ranges(workflows, next, &lt_ranges, accepted);

                // expr is false
                ranges.modify(category, |r| r.0 = r.0.max(*bound));
            }
            Expr::Goto(next) => find_accepted_ranges(workflows, next, &ranges, accepted),
        }
    }
}

// note: ranges are [min, max)
#[derive(Debug, Clone, Copy)]
struct Ranges {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl Ranges {
    fn modify<F>(&mut self, category: &str, f: F)
    where
        F: FnOnce(&mut (u32, u32)),
    {
        match category {
            "x" => f(&mut self.x),
            "m" => f(&mut self.m),
            "a" => f(&mut self.a),
            "s" => f(&mut self.s),
            _ => unreachable!(),
        }
    }
}

impl Default for Ranges {
    fn default() -> Self {
        let r = (1, 4001);
        Self {
            x: r,
            m: r,
            a: r,
            s: r,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let parts: Vec<_> = parts.lines().flat_map(Part::from_str).collect();
    let workflows: Vec<_> = workflows
        .lines()
        .flat_map(Workflow::from_str)
        .collect::<Vec<_>>();
    (workflows, parts)
}

#[derive(Debug)]
struct Workflow {
    name: String,
    exprs: Vec<Expr>,
}

impl Workflow {
    fn eval(&self, part: &Part) -> String {
        for expr in &self.exprs {
            match expr {
                Expr::Gt(category, v, goto) => {
                    if part.get(category) > *v {
                        return goto.to_string();
                    }
                }
                Expr::Lt(category, v, goto) => {
                    if part.get(category) < *v {
                        return goto.to_string();
                    }
                }
                Expr::Goto(goto) => {
                    return goto.to_string();
                }
            }
        }
        unreachable!()
    }
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name_end = s.find('{').unwrap();
        let name = s[..name_end].to_string();
        let exprs = s[name_end + 1..s.len() - 1] // trim "{" and "}"
            .split(',')
            .flat_map(Expr::from_str)
            .collect::<Vec<_>>();
        Ok(Workflow { name, exprs })
    }
}

#[derive(Debug)]
enum Expr {
    Gt(String, u32, String),
    Lt(String, u32, String),
    Goto(String),
}

impl FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((expr, res)) => {
                let op_start = expr.find(['>', '<']).unwrap();
                let category = &expr[..op_start];
                let op = &expr[op_start..op_start + 1];
                let val = expr[op_start + 1..].parse()?;
                Ok(match op {
                    ">" => Expr::Gt(category.to_string(), val, res.to_string()),
                    "<" => Expr::Lt(category.to_string(), val, res.to_string()),
                    _ => unreachable!(),
                })
            }
            None => Ok(Expr::Goto(s.to_string())),
        }
    }
}

#[derive(Debug, Default)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get(&self, category: &str) -> u32 {
        match category {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len() - 1]; // trim "{" and "}"
        let mut part = Part::default();
        for category_info in s.split(',') {
            let (category, value) = category_info.split_once('=').unwrap();
            let value = value.parse()?;
            match category {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => unreachable!(),
            }
        }
        Ok(part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day19.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(353553, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(124615747767410, ans);
    }
}
