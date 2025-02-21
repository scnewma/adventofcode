use arrayvec::ArrayVec;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete,
    sequence::{preceded, separated_pair},
};

use crate::SolveInfo;

// currently cannot run the samples on this day because we need to change the parameters if it's a
// sample
pub fn run(input: &str, is_sample: bool) -> anyhow::Result<SolveInfo> {
    if is_sample {
        Ok(SolveInfo {
            part01: part01_inner::<14, 10>(input).to_string(),
            part02: part02_inner::<14, 20>(input).to_string(),
        })
    } else {
        Ok(SolveInfo {
            part01: part01_inner::<28, 2000000>(input).to_string(),
            part02: part02_inner::<28, 4000000>(input).to_string(),
        })
    }
}

pub fn part01(input: &str) -> i32 {
    part01_inner::<28, 200000>(input)
}

pub fn part02(input: &str) -> u64 {
    part02_inner::<28, 4000000>(input)
}

const MAX_INTERVALS: usize = 101;

fn part01_inner<const SENSORS: usize, const ROW: i32>(input: &str) -> i32 {
    let sensors: ArrayVec<Sensor, SENSORS> =
        input.lines().map(|l| parse_sensor(l).unwrap().1).collect();
    let mut intervals: ArrayVec<_, MAX_INTERVALS> = ArrayVec::new();
    for sensor in sensors.iter() {
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

fn part02_inner<const SENSORS: usize, const MAX: u32>(input: &str) -> u64 {
    let sensors: ArrayVec<Sensor, SENSORS> =
        input.lines().map(|l| parse_sensor(l).unwrap().1).collect();
    for row in 0..=MAX {
        let mut intervals: ArrayVec<_, MAX_INTERVALS> = ArrayVec::new();
        for sensor in sensors.iter() {
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

fn insert_interval<const CAP: usize>(
    intervals: ArrayVec<(i32, i32), CAP>,
    mut new_interval: (i32, i32),
) -> ArrayVec<(i32, i32), CAP> {
    let mut merged = ArrayVec::new();
    let mut added = false;
    let mut idx = 0;
    for interval in intervals.iter() {
        idx += 1;
        // non-overlapping, starts after this interval
        if interval.1 < new_interval.0 {
            merged.push(*interval);

        // non-overlapping, ends before this starts, add both
        } else if interval.0 > new_interval.1 {
            merged.push(new_interval);
            merged.push(*interval);
            added = true;
            break;

        // overlapping
        } else {
            new_interval.0 = new_interval.0.min(interval.0);
            new_interval.1 = new_interval.1.max(interval.1);
        }
    }
    if !added {
        merged.push(new_interval);
    } else {
        for i in &intervals[idx..] {
            merged.push(*i);
        }
    }
    merged
}

#[derive(Debug, Clone)]
struct Sensor {
    pos: Point,
    beacon_dist: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // manhattan distance
    #[inline]
    fn distance(&self, o: &Point) -> u32 {
        self.x.abs_diff(o.x) + self.y.abs_diff(o.y)
    }
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, pos) = preceded(tag("Sensor at "), parse_point)(input)?;
    let (input, beacon) = preceded(tag(": closest beacon is at "), parse_point)(input)?;
    let beacon_dist = pos.distance(&beacon);
    Ok((input, Sensor { pos, beacon_dist }))
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

    const SAMPLE: &str = include_str!("../inputs/day15.sample.txt");
    const INPUT: &str = include_str!("../inputs/day15.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01_inner::<14, 10>(SAMPLE);
        assert_eq!(26, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01_inner::<28, 2000000>(INPUT);
        assert_eq!(5142231, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02_inner::<14, 20>(SAMPLE);
        assert_eq!(56000011, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02_inner::<28, 4000000>(INPUT);
        assert_eq!(10884459367718, ans);
    }
}
