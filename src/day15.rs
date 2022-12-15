use nom::{
    bytes::complete::tag,
    character::complete,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::SolveInfo;

// currently cannot run the samples on this day because we need to change the parameters if it's a
// sample
pub fn run(input: &str, is_sample: bool) -> anyhow::Result<SolveInfo> {
    if is_sample {
        Ok(SolveInfo {
            part01: part01_inner::<10>(input).to_string(),
            part02: part02_inner::<20>(input).to_string(),
        })
    } else {
        Ok(SolveInfo {
            part01: part01_inner::<2000000>(input).to_string(),
            part02: part02_inner::<4000000>(input).to_string(),
        })
    }
}

pub fn part01(input: &str) -> i32 {
    part01_inner::<200000>(input)
}

pub fn part02(input: &str) -> u64 {
    part02_inner::<4000000>(input)
}

fn part01_inner<const ROW: i32>(input: &str) -> i32 {
    // change to 2000000 for input
    let sensors: Vec<Sensor> = input.lines().map(|l| parse_sensor(l).unwrap().1).collect();
    let mut intervals = Vec::new();
    let mut beacon_locations = Vec::new();
    for sensor in sensors.iter() {
        if sensor.beacon.y == ROW {
            beacon_locations.push(sensor.beacon.x);
        }

        let dist = sensor.pos.distance(&Point {
            x: sensor.pos.x,
            y: ROW,
        });
        if dist <= sensor.beacon_dist {
            let delta = sensor.beacon_dist as i32 - dist as i32;
            let new_interval = (sensor.pos.x - delta, sensor.pos.x + delta);
            intervals = insert_interval(intervals, new_interval);
        }
    }
    let mut count = 0;
    for interval in intervals {
        count += interval.1 - interval.0 + 1;
    }
    count - 1
}

fn part02_inner<const MAX: u32>(input: &str) -> u64 {
    let sensors: Vec<Sensor> = input.lines().map(|l| parse_sensor(l).unwrap().1).collect();
    for row in 0..=MAX {
        let mut intervals = Vec::new();
        for sensor in sensors.iter() {
            if sensor.beacon.y == row as i32 {
                intervals = insert_interval(intervals, (sensor.beacon.x, sensor.beacon.x));
            }

            let dist = sensor.pos.distance(&Point {
                x: sensor.pos.x,
                y: row as i32,
            });
            if dist <= sensor.beacon_dist {
                let delta = sensor.beacon_dist as i32 - dist as i32;
                let new_interval = (sensor.pos.x - delta, sensor.pos.x + delta);
                intervals = insert_interval(intervals, new_interval);
            }
        }

        // if there is more than one interval left, we found the distress beacon, it's in between
        // the two intervals
        if intervals.len() > 1 {
            let x = intervals[0].1 + 1;
            return x as u64 * 4000000 + row as u64;
        }
    }
    panic!("distress beacon not found!")
}

fn insert_interval(
    mut intervals: Vec<(i32, i32)>,
    mut new_interval: (i32, i32),
) -> Vec<(i32, i32)> {
    intervals.sort();
    let mut merged = Vec::new();
    let mut added = false;
    for interval in intervals.into_iter() {
        if added {
            merged.push(interval);
        }

        // non-overlapping, starts after this interval
        if interval.1 < new_interval.0 {
            merged.push(interval);

        // non-overlapping, ends before this starts, add both
        } else if interval.0 > new_interval.1 {
            merged.push(new_interval);
            merged.push(interval);
            added = true;

        // overlapping
        } else {
            new_interval.0 = new_interval.0.min(interval.0);
            new_interval.1 = new_interval.1.max(interval.1);
        }
    }
    if !added {
        merged.push(new_interval);
    }
    merged
}

#[derive(Debug, Clone)]
struct Sensor {
    pos: Point,
    beacon: Point,
    beacon_dist: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // manhattan distance
    fn distance(&self, o: &Point) -> u32 {
        self.x.abs_diff(o.x) + self.y.abs_diff(o.y)
    }
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, pos) = preceded(tag("Sensor at "), parse_point)(input)?;
    let (input, beacon) = preceded(tag(": closest beacon is at "), parse_point)(input)?;
    let beacon_dist = pos.distance(&beacon);
    Ok((
        input,
        Sensor {
            pos,
            beacon,
            beacon_dist,
        },
    ))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)?;
    Ok((input, Point { x, y }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day15.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day15.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01_inner::<10>(SAMPLE);
        assert_eq!(26, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01_inner::<2000000>(INPUT);
        assert_eq!(5142231, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02_inner::<20>(SAMPLE);
        assert_eq!(56000011, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02_inner::<4000000>(INPUT);
        assert_eq!(10884459367718, ans);
    }
}
