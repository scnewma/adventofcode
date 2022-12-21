use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete, sequence::preceded, Finish, IResult};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: "IGNORED".to_string(),
        // part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let blueprints = parse_input(input)?;
    Ok(blueprints
        .par_iter()
        .map(|bp| {
            let cracked = crack_geodes(
                Resources {
                    time: 24,
                    ore_robots: 1,
                    ..Default::default()
                },
                bp,
                &mut HashMap::new(),
            );
            println!("{} = {cracked}", bp.id);
            bp.id * cracked
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let blueprints = parse_input(input)?;
    Ok(blueprints
        .par_iter()
        .take(3)
        .map(|bp| {
            println!("blueprint #{}", bp.id);
            let cracked = crack_geodes(
                Resources {
                    time: 32,
                    ore_robots: 1,
                    ..Default::default()
                },
                bp,
                &mut HashMap::new(),
            );
            println!("{} = {cracked}", bp.id);
            cracked
        })
        .product())
}

fn crack_geodes(mut r: Resources, bp: &Blueprint, cache: &mut HashMap<Resources, u32>) -> u32 {
    if r.time == 0 {
        return 0;
    }
    if let Some(cracked) = cache.get(&r) {
        return *cracked;
    }

    let original_resources = r;

    let mut cracked = 0;

    r.time -= 1;
    // simulate if we built a geode robot
    if r.ore >= bp.geode.0 && r.obsidian >= bp.geode.1 {
        let mut r = r;
        r.ore += r.ore_robots;
        r.clay += r.clay_robots;
        r.obsidian += r.obsidian_robots;

        r.ore -= bp.geode.0;
        r.obsidian -= bp.geode.1;
        r.geode_robots += 1;
        cracked = cracked.max(crack_geodes(r, bp, cache));
    }

    // simulate if we built an obsidian robot
    let need_obsidian_robots = r.obsidian_robots < bp.geode.1;
    if r.ore >= bp.obsidian.0 && r.clay >= bp.obsidian.1 && need_obsidian_robots {
        let mut r = r;
        r.ore += r.ore_robots;
        r.clay += r.clay_robots;
        r.obsidian += r.obsidian_robots;

        r.ore -= bp.obsidian.0;
        r.clay -= bp.obsidian.1;
        r.obsidian_robots += 1;
        cracked = cracked.max(crack_geodes(r, bp, cache));
    }

    // simulate if we built an clay robot
    let need_clay_robots = r.clay_robots < bp.obsidian.1;
    if r.ore >= bp.clay && need_clay_robots {
        let mut r = r;
        r.ore += r.ore_robots;
        r.clay += r.clay_robots;
        r.obsidian += r.obsidian_robots;

        r.ore -= bp.clay;
        r.clay_robots += 1;
        cracked = cracked.max(crack_geodes(r, bp, cache));
    }

    // simulate if we built an ore robot
    let need_ore_robots = r.ore_robots < bp.ore.max(bp.clay).max(bp.obsidian.0).max(bp.geode.0);
    if r.ore >= bp.ore && need_ore_robots {
        let mut r = r;
        r.ore += r.ore_robots;
        r.clay += r.clay_robots;
        r.obsidian += r.obsidian_robots;

        r.ore -= bp.ore;
        r.ore_robots += 1;
        cracked = cracked.max(crack_geodes(r, bp, cache));
    }

    // simulate if you didn't build any robots
    r.ore += r.ore_robots;
    r.clay += r.clay_robots;
    r.obsidian += r.obsidian_robots;
    cracked = cracked.max(crack_geodes(r, bp, cache));

    cracked += r.geode_robots;
    cache.insert(original_resources, cracked);
    cracked
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Resources {
    time: u8,
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
    #[ignore]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(33, ans);
    }

    #[test]
    #[ignore]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1766, ans);
    }

    #[test]
    #[ignore]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
