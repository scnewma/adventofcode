use arrayvec::ArrayVec;
use bittle::{Bits, BitsMut};
use itertools::Itertools;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete,
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

use crate::SolveInfo;

pub fn run(input: &str, is_sample: bool) -> anyhow::Result<SolveInfo> {
    if is_sample {
        Ok(SolveInfo {
            part01: part01(input)?.to_string(),
            part02: part02_inner::<10>(input)?.to_string(),
        })
    } else {
        Ok(SolveInfo {
            part01: part01(input)?.to_string(),
            part02: part02_inner::<55>(input)?.to_string(),
        })
    }
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let lines: Vec<_> = input.lines().map(|l| parse_valve(l).unwrap().1).collect();
    let mut flow_rates = HashMap::new();
    let mut edges = HashMap::new();
    let valve_bit_indicies: HashMap<String, u32> = lines
        .iter()
        .map(|(v, _)| v.name.clone())
        .enumerate()
        .map(|(i, v)| (v, i as u32))
        .collect();
    for (valve, neighs) in lines {
        let ns: Vec<u32> = neighs
            .iter()
            .map(|name| *valve_bit_indicies.get(name).unwrap())
            .collect();

        edges.insert(*valve_bit_indicies.get(&valve.name).unwrap(), ns);
        flow_rates.insert(
            *valve_bit_indicies.get(&valve.name).unwrap(),
            valve.flow_rate,
        );
    }

    Ok(max_relief(
        0,
        *valve_bit_indicies.get(&"AA".to_string()).unwrap(),
        &mut HashMap::new(),
        &edges,
        &flow_rates,
        States(u64::max_value()),
    ))
}

const MAX_NEIGHBORS: usize = 5;

pub fn part02(input: &str) -> anyhow::Result<u64> {
    part02_inner::<55>(input)
}

fn part02_inner<const VALVES: usize>(input: &str) -> anyhow::Result<u64> {
    let lines: Vec<_> = input.lines().map(|l| parse_valve(l).unwrap().1).collect();
    let mut flow_rates = vec![0; lines.len()];
    let mut edges: ArrayVec<ArrayVec<_, MAX_NEIGHBORS>, VALVES> = ArrayVec::new();
    (0..lines.len()).for_each(|_| edges.push(ArrayVec::new()));
    let valve_bit_indicies: HashMap<String, u32> = lines
        .iter()
        .map(|(v, _)| v.name.clone())
        .enumerate()
        .map(|(i, v)| (v, i as u32))
        .collect();
    for (valve, neighs) in lines {
        let ns: ArrayVec<u32, MAX_NEIGHBORS> = neighs
            .iter()
            .map(|name| *valve_bit_indicies.get(name).unwrap())
            .collect();

        edges[*valve_bit_indicies.get(&valve.name).unwrap() as usize] = ns;
        flow_rates[*valve_bit_indicies.get(&valve.name).unwrap() as usize] = valve.flow_rate;
    }

    let pos_aa = *valve_bit_indicies.get(&"AA".to_string()).unwrap();

    Ok(max_relief_with_elephant(
        0,
        (pos_aa, pos_aa),
        &mut HashMap::new(),
        &edges,
        &flow_rates,
        States(u64::max_value()),
    ))
}

const TOTAL_TIME: u32 = 30;

fn max_relief(
    time: u32,
    current_valve: u32,
    cache: &mut HashMap<(u32, u32, States), u64>,
    edges: &HashMap<u32, Vec<u32>>,
    flow_rates: &HashMap<u32, u32>,
    states: States,
) -> u64 {
    if time == TOTAL_TIME {
        return 0;
    }
    if let Some(relief) = cache.get(&(current_valve, time, states)) {
        return *relief;
    }

    let mut relief = 0;

    // if this valve is open, fork the simulation to add another path where this valve is open
    // we also don't need to go down this path if the flow rate is 0 since it won't contribute
    let current_valve_flow_rate = *flow_rates.get(&current_valve).unwrap();
    if states.is_open(current_valve) && current_valve_flow_rate > 0 {
        let mut states = states.clone();
        states.close(current_valve);

        let total_relief = ((TOTAL_TIME - time - 1) * current_valve_flow_rate) as u64
            + max_relief(time + 1, current_valve, cache, edges, flow_rates, states);
        relief = relief.max(total_relief);
    }

    // we try all of the paths through the graph as if this valve were first not opened
    for neighbor in edges.get(&current_valve).unwrap() {
        relief = relief.max(max_relief(
            time + 1,
            *neighbor,
            cache,
            edges,
            flow_rates,
            states,
        ));
    }

    cache.insert((current_valve, time, states), relief);
    relief
}

const TOTAL_TIME_WITH_ELEPHANT: u32 = 26;

fn max_relief_with_elephant(
    time: u32,
    current: (u32, u32),
    cache: &mut HashMap<((u32, u32), u32, States), u64>,
    edges: &[ArrayVec<u32, MAX_NEIGHBORS>],
    flow_rates: &[u32],
    states: States,
) -> u64 {
    if time == TOTAL_TIME_WITH_ELEPHANT {
        return 0;
    }
    if let Some(relief) = cache.get(&(current, time, states)) {
        return *relief;
    }

    let mut relief = 0;

    let mut my_options: ArrayVec<_, { MAX_NEIGHBORS + 1 }> = ArrayVec::new();
    if states.is_open(current.0) && flow_rates[current.0 as usize] > 0 {
        my_options.push(Turn::Open(current.0));
    }
    if time < TOTAL_TIME_WITH_ELEPHANT - 1 {
        for neighbor in &edges[current.0 as usize] {
            my_options.push(Turn::Move(*neighbor));
        }
    }

    let mut elephant_options: ArrayVec<_, { MAX_NEIGHBORS + 1 }> = ArrayVec::new();
    if states.is_open(current.1) && flow_rates[current.1 as usize] > 0 {
        elephant_options.push(Turn::Open(current.1));
    }
    if time < TOTAL_TIME_WITH_ELEPHANT - 1 {
        for neighbor in &edges[current.1 as usize] {
            elephant_options.push(Turn::Move(*neighbor));
        }
    }

    for (my_move, elephant_move) in my_options.iter().cartesian_product(elephant_options.iter()) {
        match (my_move, elephant_move) {
            // invalid for both me and elephant to open the valve
            (Turn::Open(v1), Turn::Open(v2)) if v1 == v2 => continue,
            _ => (),
        }

        let mut next_pos = current.clone();
        let mut states = states.clone();
        let mut total_relief = 0;
        match my_move {
            Turn::Open(valve) => {
                states.close(*valve);
                total_relief +=
                    ((TOTAL_TIME_WITH_ELEPHANT - time - 1) * flow_rates[current.0 as usize]) as u64;
            }
            Turn::Move(valve) => {
                next_pos.0 = *valve;
            }
        }
        match elephant_move {
            Turn::Open(valve) => {
                states.close(*valve);
                total_relief +=
                    ((TOTAL_TIME_WITH_ELEPHANT - time - 1) * flow_rates[current.1 as usize]) as u64;
            }
            Turn::Move(valve) => {
                next_pos.1 = *valve;
            }
        }
        total_relief +=
            max_relief_with_elephant(time + 1, next_pos, cache, edges, flow_rates, states);

        relief = relief.max(total_relief);
    }

    cache.insert((current, time, states), relief);
    relief
}

#[derive(Debug)]
enum Turn {
    Open(u32),
    Move(u32),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow_rate: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct States(u64);

impl States {
    fn is_open(&self, valve: u32) -> bool {
        self.0.test_bit(valve)
    }

    fn close(&mut self, valve: u32) {
        self.0.clear_bit(valve)
    }
}

fn parse_valve(input: &str) -> IResult<&str, (Valve, Vec<String>)> {
    let (input, name) = preceded(tag("Valve "), parse_name)(input)?;
    let (input, flow_rate) = preceded(tag(" has flow rate="), complete::u32)(input)?;
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
        let ans = part02_inner::<10>(SAMPLE).unwrap();
        assert_eq!(1707, ans);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let ans = part02_inner::<55>(INPUT).unwrap();
        assert_eq!(3015, ans);
    }
}
