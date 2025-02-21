use anyhow::Context;
use itertools::process_results;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<isize> {
    let monkeys: Vec<Monkey> =
        process_results(input.lines().map(Monkey::try_from), |it| it.collect())?;

    let monkeys_lookup: HashMap<String, Monkey> = monkeys
        .into_iter()
        .map(|monkey| (monkey.name.clone(), monkey))
        .collect();

    let results = evaluate_graph(monkeys_lookup)?;
    Ok(*results.get("root").unwrap())
}

pub fn part02(input: &str) -> anyhow::Result<isize> {
    let monkeys: Vec<Monkey> =
        process_results(input.lines().map(Monkey::try_from), |it| it.collect())?;
    let mut monkeys_lookup: HashMap<String, Monkey> = monkeys
        .into_iter()
        .map(|monkey| (monkey.name.clone(), monkey))
        .collect();

    // find the path from "root" to "humn"
    let path_to_humn = path_to_node(&monkeys_lookup, "humn")?;

    // we rewrite the graph from "humn" to "root" one node at a time so that "humn" is now the node
    // that needs to be solved for
    let mut current_monkey = "humn".to_string();
    for parent_name in path_to_humn.iter().rev() {
        let parent = monkeys_lookup.get(parent_name).unwrap().clone();
        monkeys_lookup
            .entry(current_monkey.clone())
            .and_modify(|m| {
                if let Job::Operation(lhs, op, rhs) = &parent.job {
                    // special case: root's new operation becomes "=" so we handle that here
                    if parent_name == "root" {
                        let other = if lhs == &current_monkey { rhs } else { lhs };
                        m.job = Job::Lookup(other.to_string());
                        return;
                    }
                    // division and subtraction are not commutative so they need to be handled
                    // differently
                    //
                    // Scenarios:
                    //   3
                    // ? / 4   lhs = parent * rhs
                    //
                    //    3    (non-commutative)
                    // 12 / ?  rhs = lhs / parent
                    //
                    //   3
                    // ? - 4   lhs = parent + rhs
                    //
                    //   3     (non-commutative)
                    // 7 - ?   rhs = lhs - parent
                    //
                    //   3
                    // ? + 2   lhs = parent - rhs
                    //
                    //   3
                    // 1 + ?   rhs = parent - lhs
                    //
                    //   8
                    // ? * 4   lhs = parent / rhs
                    //
                    //   8
                    // 2 * ?   rhs = parent / lhs
                    if &current_monkey == rhs && (*op == Op::Div || *op == Op::Sub) {
                        m.job = Job::Operation(lhs.to_string(), *op, parent_name.to_string());
                    } else {
                        let rhs = if lhs == &current_monkey { rhs } else { lhs };
                        m.job =
                            Job::Operation(parent_name.to_string(), op.invert(), rhs.to_string());
                    }
                } else {
                    // just double-checking that every node along the path should be an equation
                    // node
                    panic!("monkey in path to humn is static: {parent:?}");
                }
            });
        current_monkey = parent.name.clone();
    }
    // we don't need root anymore and instead of needing to rewrite it we just discard it
    monkeys_lookup.remove("root");

    let results = evaluate_graph(monkeys_lookup)?;
    Ok(*results.get("humn").unwrap())
}

// evaluate_graph evaluates all node in the graph in topological sorted order, the values for each
// node are added to scope
fn evaluate_graph(monkeys: HashMap<String, Monkey>) -> anyhow::Result<HashMap<String, isize>> {
    let mut queue = VecDeque::new();
    let mut in_degree = HashMap::new();
    let mut inedges: HashMap<&String, Vec<&String>> = HashMap::new();
    for (_, monkey) in monkeys.iter() {
        let depends_on = monkey.depends_on();

        match depends_on {
            Some(deps) => {
                for dep in deps {
                    inedges.entry(dep).or_default().push(&monkey.name);
                    in_degree
                        .entry(&monkey.name)
                        .and_modify(|cnt| *cnt += 1)
                        .or_insert(1);
                }
            }
            None => {
                queue.push_back(&monkey.name);
            }
        }
    }

    let mut scope = HashMap::new();
    while let Some(monkey_name) = queue.pop_front() {
        let monkey = monkeys.get(monkey_name).context("monkey not found")?;

        let value = monkey.eval(&scope)?;
        scope.insert(monkey.name.clone(), value);

        // update in degree for all connected verticies
        if let Some(inedges) = inedges.get(&monkey.name) {
            inedges.iter().for_each(|v| {
                in_degree.entry(v).and_modify(|cnt| *cnt -= 1);
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            });
        }
    }
    Ok(scope)
}

fn path_to_node(monkeys: &HashMap<String, Monkey>, node: &str) -> anyhow::Result<Vec<String>> {
    let mut path_to_node = vec![];

    let mut stk = Vec::new();
    let root = monkeys.get(&"root".to_string()).unwrap().clone();
    let root_deps = root.depends_on().unwrap();
    stk.push((root_deps[0].clone(), vec!["root".to_string()]));
    stk.push((root_deps[1].clone(), vec!["root".to_string()]));
    while let Some((monkey_name, path)) = stk.pop() {
        if monkey_name == node {
            path_to_node = path;
            break;
        }

        let monkey = monkeys.get(&monkey_name).context("monkey not found")?;
        if let Some(deps) = monkey.depends_on() {
            let mut path = path.clone();
            path.push(monkey_name);
            stk.push((deps[0].clone(), path.clone()));
            stk.push((deps[1].clone(), path.clone()));
        }
    }
    Ok(path_to_node)
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    job: Job,
}

impl Monkey {
    fn depends_on(&self) -> Option<Vec<&String>> {
        match &self.job {
            Job::Number(_) => None,
            Job::Lookup(other) => Some(vec![other]),
            Job::Operation(lhs, _, rhs) => Some(vec![lhs, rhs]),
        }
    }

    fn eval(&self, scope: &HashMap<String, isize>) -> anyhow::Result<isize> {
        match &self.job {
            Job::Number(n) => Ok(*n),
            Job::Lookup(other) => Ok(*scope.get(other).context("not found")?),
            Job::Operation(lhs, op, rhs) => {
                let lhsv = scope
                    .get(lhs)
                    .context(format!("{}: {} not found in scope", self.name, lhs))?;
                let rhsv = scope
                    .get(rhs)
                    .context(format!("{}: {} not found in scope", self.name, rhs))?;

                Ok(match op {
                    Op::Add => lhsv + rhsv,
                    Op::Sub => lhsv - rhsv,
                    Op::Mul => lhsv * rhsv,
                    Op::Div => {
                        // verify that we don't have any bugs due to expecting floats
                        assert!(
                            lhsv % rhsv == 0,
                            "monkey={} lhs={} rhs={} lhsv={} rhsv={}",
                            self.name,
                            lhs,
                            rhs,
                            lhsv,
                            rhsv
                        );
                        lhsv / rhsv
                    }
                })
            }
        }
    }
}

impl TryFrom<&str> for Monkey {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (name, job) = s.split_once(": ").context("malformed")?;
        let job = match job.parse::<isize>() {
            Ok(num) => Job::Number(num),
            Err(_) => {
                let mut words = job.split_whitespace();
                Job::Operation(
                    words.next().context("malformed: no lhs")?.to_string(),
                    words.next().context("malformed: no op")?.parse()?,
                    words.next().context("malformed: no rhs")?.to_string(),
                )
            }
        };
        Ok(Monkey {
            name: name.to_string(),
            job,
        })
    }
}

#[derive(Debug, Clone)]
enum Job {
    Number(isize),
    Lookup(String),
    Operation(String, Op, String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn invert(&self) -> Op {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "*",
                Op::Div => "/",
            }
        )
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => anyhow::bail!("invalid op {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../inputs/day21.sample.txt");
    const INPUT: &str = include_str!("../inputs/day21.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(152, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(142707821472432, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(301, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(3587647562851, ans);
    }
}
