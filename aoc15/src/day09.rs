use anyhow::Context;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1},
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> u32 {
    let mut min = u32::MAX;
    all_path_distances(input, |cost| min = min.min(cost));
    min
}

pub fn part02(input: &str) -> u32 {
    let mut max = 0;
    all_path_distances(input, |cost| max = max.max(cost));
    max
}

fn all_path_distances<F>(input: &str, f: F)
where
    F: FnMut(u32),
{
    let graph = parse_graph(input);
    let mut cities = FxHashSet::default();
    for ((from, to), _) in graph.iter() {
        cities.insert(*from);
        cities.insert(*to);
    }
    let k = cities.len();
    cities
        .into_iter()
        .permutations(k)
        .map(|cities| path_distance(&graph, &cities))
        .for_each(f)
}

fn path_distance(graph: &FxHashMap<(&str, &str), u32>, path: &[&str]) -> u32 {
    let mut d = 0;
    let mut cities = path.into_iter();
    let mut prev = cities.next().unwrap();
    for city in cities {
        let cost = *graph
            .get(&(prev, city))
            .with_context(|| format!("no cost found for {} to {}", prev, city))
            .unwrap();
        d += cost;
        prev = city;
    }
    d
}

fn parse_graph(input: &str) -> FxHashMap<(&str, &str), u32> {
    let mut graph = FxHashMap::default();
    for line in input.lines() {
        let (_, (from, to, d)) = parse_line(line).unwrap();
        graph.insert((from, to), d);
        graph.insert((to, from), d);
    }
    graph
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str, u32)> {
    let (input, (from, to)) = separated_pair(alpha1, tag(" to "), alpha1)(input)?;
    let (input, d) = preceded(tag(" = "), complete::u32)(input)?;
    Ok((input, (from, to, d)))
}
