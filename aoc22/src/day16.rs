use bittle::{Bits, BitsMut};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    ops::BitAnd,
};

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete,
    multi::separated_list0,
    sequence::preceded,
};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u16> {
    let (collapsed_edges, flow_rates, aa_index) = parse_input(input);

    let mut observed = HashMap::new();
    max_relief(
        30,
        aa_index,
        0,
        States(0),
        &collapsed_edges,
        &flow_rates,
        &mut observed,
    );
    Ok(*observed.values().max().unwrap())
}

pub fn part02(input: &str) -> anyhow::Result<u16> {
    let (collapsed_edges, flow_rates, aa_index) = parse_input(input);

    let mut observed = HashMap::new();
    max_relief(
        26,
        aa_index,
        0,
        States(0),
        &collapsed_edges,
        &flow_rates,
        &mut observed,
    );

    // find the two highest paths that do not have overlapping states
    Ok(observed
        .iter()
        .tuple_combinations::<(_, _)>()
        .filter(|perms| *perms.0.0 & *perms.1.0 == 0)
        .map(|perms| perms.0.1 + perms.1.1)
        .max()
        .unwrap())
}

fn max_relief(
    time: u16,
    current: u16,
    relief: u16,
    states: States,
    edges: &HashMap<u16, Vec<(u16, u16)>>,
    flow_rates: &HashMap<u16, u16>,
    observed: &mut HashMap<States, u16>,
) {
    observed
        .entry(states)
        .and_modify(|e| *e = (*e).max(relief))
        .or_insert(relief);

    for (neighbor, distance) in edges.get(&current).unwrap() {
        let time_rem = time.saturating_sub(*distance + 1);
        if states.is_open(*neighbor) || time_rem == 0 {
            continue;
        }

        let mut states = states;
        states.open(*neighbor);
        max_relief(
            time_rem,
            *neighbor,
            relief + time_rem * flow_rates[neighbor],
            states,
            edges,
            flow_rates,
            observed,
        );
    }
}

fn distance(source: &String, dest: &String, edges: &HashMap<String, Vec<String>>) -> u16 {
    let mut queue = VecDeque::new();
    edges
        .get(source)
        .unwrap()
        .iter()
        .for_each(|v| queue.push_back((v, 1)));
    while let Some((v, distance)) = queue.pop_front() {
        if v == dest {
            return distance;
        }

        edges
            .get(v)
            .unwrap()
            .iter()
            .for_each(|u| queue.push_back((u, distance + 1)));
    }
    unreachable!("no path found between {} and {}", source, dest);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Valve {
    name: String,
    flow_rate: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct States(u16);

impl States {
    fn is_open(&self, valve: u16) -> bool {
        self.0.test_bit(valve as u32)
    }

    fn open(&mut self, valve: u16) {
        self.0.set_bit(valve as u32)
    }
}

impl BitAnd for States {
    type Output = u16;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.0 & rhs.0
    }
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (HashMap<u16, Vec<(u16, u16)>>, HashMap<u16, u16>, u16) {
    let lines: Vec<_> = input.lines().map(|l| parse_valve(l).unwrap().1).collect();
    let valves: Vec<_> = lines
        .iter()
        .map(|(valve, _)| valve)
        .filter(|&v| v.flow_rate > 0 || v.name == "AA")
        .cloned()
        .collect();
    // the open / close state of every valve is represented by a bitvec, this maps the name of each
    // valve into it's assigned bitvec index
    let valve_bit_indices: HashMap<String, u16> = valves
        .iter()
        .map(|v| v.name.clone())
        .enumerate()
        .map(|(i, v)| (v, i as u16))
        .collect();
    // valve -> [neighbor_valve]
    let edges: HashMap<String, Vec<String>> =
        lines.into_iter().map(|(v, ns)| (v.name, ns)).collect();
    // most of the valves in the input have 0 flow so instead of working with all of the valves, we
    // only work with the valves that have a flow rate > 0. this is a map between every single
    // valve in the system along with the distance it takes to get to that valve.
    // valve -> [(valve, distance), ...]
    let mut collapsed_edges: HashMap<u16, Vec<(u16, u16)>> = HashMap::new();
    valves.iter().permutations(2).for_each(|perms| {
        collapsed_edges
            .entry(valve_bit_indices[&perms[0].name])
            .or_default()
            .push((
                valve_bit_indices[&perms[1].name],
                distance(&perms[0].name, &perms[1].name, &edges),
            ));
    });
    let flow_rates: HashMap<u16, u16> = valves
        .iter()
        .map(|v| (*valve_bit_indices.get(&v.name).unwrap(), v.flow_rate))
        .collect();
    let aa_index = *valve_bit_indices.get("AA").unwrap();
    (collapsed_edges, flow_rates, aa_index)
}

fn parse_valve(input: &str) -> IResult<&str, (Valve, Vec<String>)> {
    let (input, name) = preceded(tag("Valve "), parse_name)(input)?;
    let (input, flow_rate) = preceded(tag(" has flow rate="), complete::u16)(input)?;
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    let (input, neighs) = separated_list0(tag(", "), parse_name)(input)?;
    Ok((input, (Valve { name, flow_rate }, neighs)))
}

fn parse_name(input: &str) -> IResult<&str, String> {
    let (input, name) = take(2usize)(input)?;
    Ok((input, name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day16.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day16.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(1651, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(2250, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(1707, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(3015, ans);
    }
}
