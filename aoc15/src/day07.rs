use std::{collections::VecDeque, str::FromStr};

use anyhow::Context;
use fxhash::FxHashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult, Parser,
};

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> u16 {
    let wires: FxHashMap<_, _> = input
        .lines()
        .flat_map(Wire::from_str)
        .map(|w| (w.name.clone(), w))
        .collect();
    let res = eval(&wires).unwrap();
    *res.get("a").unwrap()
}

pub fn part02(input: &str) -> u16 {
    let wire_a = part01(input);

    let mut wires: FxHashMap<_, _> = input
        .lines()
        .flat_map(Wire::from_str)
        .map(|w| (w.name.clone(), w))
        .collect();

    wires
        .entry("b".to_string())
        .and_modify(|w| w.input = Input::Signal(Expr::Number(wire_a)));

    let res = eval(&wires).unwrap();
    *res.get("a").unwrap()
}

// evaluate the graph topologically
fn eval(wires: &FxHashMap<String, Wire>) -> anyhow::Result<FxHashMap<&str, u16>> {
    let mut queue = VecDeque::new();
    let mut in_degree = FxHashMap::<&str, u32>::default();
    let mut in_edges = FxHashMap::<&str, Vec<&str>>::default();
    for (_, wire) in wires.iter() {
        let depends_on = wire.depends_on();

        if depends_on.is_empty() {
            queue.push_back(wire.name.as_str());
            continue;
        }

        for dep in depends_on {
            in_edges.entry(dep).or_default().push(&wire.name);
            in_degree
                .entry(wire.name.as_str())
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
    }

    let mut scope = FxHashMap::default();
    while let Some(name) = queue.pop_front() {
        let wire = wires
            .get(name)
            .with_context(|| format!("wire '{}' not found", name))?;

        let value = wire.signal(&scope);
        scope.insert(name, value);

        if let Some(in_edges) = in_edges.get(name) {
            in_edges.iter().for_each(|v| {
                in_degree.entry(v).and_modify(|n| *n -= 1);
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            });
        }
    }

    Ok(scope)
}

#[derive(Debug, PartialEq, Eq)]
struct Wire {
    name: String,
    input: Input,
}

#[derive(Debug, PartialEq, Eq)]
enum Input {
    Signal(Expr),
    Op(Op),
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    And(Expr, Expr),
    Or(Expr, Expr),
    Not(String),
    LShift(String, u32),
    RShift(String, u32),
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Number(u16),
    Var(String),
}

impl Expr {
    fn eval(&self, scope: &FxHashMap<&str, u16>) -> u16 {
        match self {
            Expr::Number(n) => *n,
            Expr::Var(name) => *scope.get(name.as_str()).unwrap(),
        }
    }
}

impl Wire {
    fn signal(&self, scope: &FxHashMap<&str, u16>) -> u16 {
        match &self.input {
            Input::Signal(o) => o.eval(scope),
            Input::Op(op) => match op {
                Op::And(l, r) => l.eval(scope) & r.eval(scope),
                Op::Or(l, r) => l.eval(scope) | r.eval(scope),
                Op::Not(i) => !*scope.get(i.as_str()).unwrap(),
                Op::LShift(i, n) => scope.get(i.as_str()).unwrap() << n,
                Op::RShift(i, n) => scope.get(i.as_str()).unwrap() >> n,
            },
        }
    }

    fn depends_on(&self) -> Vec<&str> {
        let mut deps = vec![];

        macro_rules! maybe_add_expr {
            ( $( $e:ident),* ) => {
                $(
                    if let Expr::Var(name) = $e {
                        deps.push(name.as_str());
                    }
                )*
            };
        }

        match &self.input {
            Input::Signal(o) => maybe_add_expr!(o),
            Input::Op(op) => match op {
                Op::And(lhs, rhs) => {
                    maybe_add_expr!(lhs, rhs);
                }
                Op::Or(lhs, rhs) => {
                    maybe_add_expr!(lhs, rhs);
                }
                Op::Not(o) => deps.push(o.as_str()),
                Op::LShift(o, _) => deps.push(o.as_str()),
                Op::RShift(o, _) => deps.push(o.as_str()),
            },
        }
        deps
    }
}

impl FromStr for Wire {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_wire(s).finish() {
            Ok((_, wire)) => Ok(wire),
            Err(err) => anyhow::bail!("remaining input: {}", err.input.to_string()),
        }
    }
}

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    let (rem, (input, name)) = separated_pair(parse_input, tag(" -> "), parse_name)(input)?;
    Ok((rem, Wire { name, input }))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    alt((parse_operation, parse_signal))(input)
}

fn parse_signal(input: &str) -> IResult<&str, Input> {
    let (input, expr) = parse_expr(input)?;
    Ok((input, Input::Signal(expr)))
}

fn parse_operation(input: &str) -> IResult<&str, Input> {
    alt((parse_not, parse_infix))(input).map(|(i, op)| (i, Input::Op(op)))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((complete::u16.map(Expr::Number), parse_name.map(Expr::Var)))(input)
}

fn parse_name(input: &str) -> IResult<&str, String> {
    alpha1(input).map(|(i, s)| (i, s.to_string()))
}

fn parse_not(input: &str) -> IResult<&str, Op> {
    preceded(tag("NOT "), parse_name)(input).map(|(i, s)| (i, Op::Not(s)))
}

fn parse_infix(input: &str) -> IResult<&str, Op> {
    let and = tuple((parse_expr, tag(" AND "), parse_expr)).map(|(l, _, r)| Op::And(l, r));
    let or = tuple((parse_expr, tag(" OR "), parse_expr)).map(|(l, _, r)| Op::Or(l, r));
    let lshift =
        tuple((parse_name, tag(" LSHIFT "), complete::u32)).map(|(l, _, n)| Op::LShift(l, n));
    let rshift =
        tuple((parse_name, tag(" RSHIFT "), complete::u32)).map(|(l, _, n)| Op::RShift(l, n));

    alt((and, or, lshift, rshift))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("123 -> x", Wire{ name: "x".to_string(), input: Input::Signal(Expr::Number(123)) })]
    #[case("lx -> x",
        Wire{ name: "x".to_string(), input: Input::Signal(Expr::Var("lx".to_string())) })]
    #[case(
        "x AND y -> d",
        Wire{
            name: "d".to_string(),
            input: Input::Op(Op::And(Expr::Var("x".to_string()), Expr::Var("y".to_string())))
        })]
    #[case(
        "x OR y -> e",
        Wire{
            name: "e".to_string(),
            input: Input::Op(Op::Or(Expr::Var("x".to_string()), Expr::Var("y".to_string())))
        })]
    #[case(
        "1 AND y -> d",
        Wire{
            name: "d".to_string(),
            input: Input::Op(Op::And(Expr::Number(1), Expr::Var("y".to_string())))
        })]
    #[case(
        "x OR 1 -> e",
        Wire{
            name: "e".to_string(),
            input: Input::Op(Op::Or(Expr::Var("x".to_string()), Expr::Number(1)))
        })]
    #[case(
        "x LSHIFT 2 -> f",
        Wire{
            name: "f".to_string(),
            input: Input::Op(Op::LShift("x".to_string(), 2))
        })]
    #[case(
        "y RSHIFT 2 -> g",
        Wire{
            name: "g".to_string(),
            input: Input::Op(Op::RShift("y".to_string(), 2))
        })]
    #[case(
        "NOT x -> h",
        Wire{
            name: "h".to_string(),
            input: Input::Op(Op::Not("x".to_string()))
        })]
    #[case(
        "NOT dq -> dr",
        Wire{
            name: "dr".to_string(),
            input: Input::Op(Op::Not("dq".to_string()))
        })]
    fn test_parse_wire(#[case] input: &str, #[case] expected: Wire) {
        let ans: Wire = input.parse().context("parse error").unwrap();
        assert_eq!(expected, ans);
    }
}
