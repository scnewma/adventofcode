use std::collections::HashMap;

use anyhow::Context;
use nom::{bytes::complete::tag, character::complete, sequence::preceded, Finish, IResult};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let blueprints = parse_input(input)?;
    for blueprint in blueprints {
        let cracked = crack_geodes(
            Resources {
                time: 24,
                ore_robots: 1,
                ..Default::default()
            },
            &blueprint,
            &mut HashMap::new(),
        );
        println!("{} = {cracked}", blueprint.id);
    }
    Ok(0)
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    Ok(0)
}

fn crack_geodes(r: Resources, bp: &Blueprint, cache: &mut HashMap<Resources, u32>) -> u32 {
    // println!("t={} ore={} clay={} obsidian={} oreR={} clayR={} obsidianR={} geodeR={} cracked_geodes={cracked_geodes}", r.time, r.ore, r.clay, r.obsidian, r.ore_robots, r.clay_robots, r.obsidian_robots, r.geode_robots);
    if r.time == 0 {
        return 0;
    }
    if let Some(cracked) = cache.get(&r) {
        return *cracked;
    }

    // collect resources with current robots
    // let mut next_resources = r.clone();
    // next_resources.time -= 1;
    let mut cracked_geodes = 0;

    // options:
    // * build 0..O ore robots
    // * build 0..C clay robots (competes with ore robots)
    // * build 0..B obsidian robots (competes with ore / clay)
    // * build 0..G geode crackers (competes with ore / clay / geode)

    let buildable_ore_robots = r.ore / bp.ore;
    // println!("buildable_ore_robots={buildable_ore_robots}");
    for ore_robots_to_build in 0..=buildable_ore_robots {
        let next_resources_ore = r.ore - ore_robots_to_build * bp.ore;
        // next_resources.ore -= ore_robots_to_build * bp.ore;

        let buildable_clay_robots = next_resources_ore / bp.clay;
        // println!("buildable_clay_robots={buildable_clay_robots}");
        for clay_robots_to_build in 0..=buildable_clay_robots {
            // println!("clay_robots_to_build={clay_robots_to_build}");
            // next_resources.ore -= clay_robots_to_build * bp.clay;
            let next_resources_ore = next_resources_ore - clay_robots_to_build * bp.clay;

            let buildable_obsidian_robots =
                (next_resources_ore / bp.obsidian.0).min(r.clay / bp.obsidian.1);
            // println!("buildable_obsidian_robots={buildable_obsidian_robots}");
            for obsidian_robots_to_build in 0..=buildable_obsidian_robots {
                // next_resources.ore -= obsidian_robots_to_build * bp.obsidian.0;
                // next_resources.clay -= obsidian_robots_to_build * bp.obsidian.1;
                let next_resources_ore =
                    next_resources_ore - obsidian_robots_to_build * bp.obsidian.0;
                let next_resources_clay = r.clay - obsidian_robots_to_build * bp.obsidian.1;

                let buildable_geode_robots =
                    (next_resources_ore / bp.geode.0).min(r.obsidian / bp.geode.1);
                // println!("buildable_geode_robots={buildable_geode_robots}");
                for geode_robots_to_build in 0..=buildable_geode_robots {
                    let next_resources_ore =
                        next_resources_ore - geode_robots_to_build * bp.geode.0;
                    let next_resources_obsidian = r.obsidian - geode_robots_to_build * bp.geode.1;
                    // next_resources.ore -= geode_robots_to_build * bp.geode.0;
                    // next_resources.obsidian -= geode_robots_to_build * bp.geode.1;

                    // at the end of this minute, we will get resources from our robots, reflect
                    // that for the next minute
                    let next_resources = Resources {
                        time: r.time - 1,
                        ore: next_resources_ore + r.ore_robots,
                        ore_robots: r.ore_robots + ore_robots_to_build,
                        clay: next_resources_clay + r.clay_robots,
                        clay_robots: r.clay_robots + clay_robots_to_build,
                        obsidian: next_resources_obsidian + r.obsidian_robots,
                        obsidian_robots: r.obsidian_robots + obsidian_robots_to_build,
                        geode_robots: r.geode_robots + geode_robots_to_build,
                    };

                    let path_cracked_geodes = crack_geodes(next_resources, bp, cache);
                    cracked_geodes = cracked_geodes.max(path_cracked_geodes);
                }
            }
        }
    }

    cracked_geodes += r.geode_robots;

    // println!("CACHE PUT t={} ore={} clay={} obsidian={} oreR={} clayR={} obsidianR={} geodeR={} cracked_geodes={cracked_geodes}", r.time, r.ore, r.clay, r.obsidian, r.ore_robots, r.clay_robots, r.obsidian_robots, r.geode_robots);
    cache.insert(r, cracked_geodes);
    cracked_geodes
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Resources {
    time: u32,
    ore: u32,
    ore_robots: u32,
    clay: u32,
    clay_robots: u32,
    obsidian: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    // ore, clay
    obsidian: (u32, u32),
    // ore, obsidian
    geode: (u32, u32),
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Blueprint>> {
    let mut blueprints = Vec::new();
    for line in input.lines() {
        let (_, blueprint) = parse_blueprint(line).finish().unwrap();
        blueprints.push(blueprint);
    }
    Ok(blueprints)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = preceded(tag("Blueprint "), complete::u32)(input)?;
    let (input, ore_robot_cost) = preceded(tag(": Each ore robot costs "), complete::u32)(input)?;
    let (input, clay_robot_cost) =
        preceded(tag(" ore. Each clay robot costs "), complete::u32)(input)?;
    let (input, obsidian_robot_ore) =
        preceded(tag(" ore. Each obsidian robot costs "), complete::u32)(input)?;
    let (input, obsidian_robot_clay) = preceded(tag(" ore and "), complete::u32)(input)?;
    let (input, geode_robot_ore) =
        preceded(tag(" clay. Each geode robot costs "), complete::u32)(input)?;
    let (input, geode_robot_obsidian) = preceded(tag(" ore and "), complete::u32)(input)?;
    Ok((
        input,
        Blueprint {
            id,
            ore: ore_robot_cost,
            clay: clay_robot_cost,
            obsidian: (obsidian_robot_ore, obsidian_robot_clay),
            geode: (geode_robot_ore, geode_robot_obsidian),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day19.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day19.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
