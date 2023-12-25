use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<u64> {
    let hailstones = input.lines().flat_map(Hailstone::from_str).collect_vec();
    // let rect = Rect {
    //     tl: (7.0, 27.0),
    //     br: (27.0, 7.0),
    // };
    let rect = Rect {
        tl: (200000000000000.0, 400000000000000.0),
        br: (400000000000000.0, 200000000000000.0),
    };

    let mut ans = 0;
    'combinations: for (h1, h2) in hailstones.iter().tuple_combinations() {
        let (b1, b2) = (h1.b(), h2.b());
        let intercept_x = (b2 - b1) / (h1.m() - h2.m());
        if intercept_x.is_infinite() {
            continue;
        }

        let intercept_y = h1.m() * intercept_x + b1;

        // take a step in the direction of the velocity, if the distance to the intercept
        // increases, you are moving away from it (i.e. it is in the past)
        for h in [h1, h2] {
            let d = (h.x - intercept_x).abs() + (h.y - intercept_y).abs();
            let dn = ((h.x + h.vx) - intercept_x).abs() + ((h.y + h.vy) - intercept_y).abs();
            if dn > d {
                continue 'combinations;
            }
        }

        if rect.contains(intercept_x, intercept_y) {
            ans += 1;
        }
    }
    Ok(ans)
}

pub fn part02(input: &str) -> anyhow::Result<i64> {
    let hailstones = input
        .lines()
        .flat_map(Hailstone::from_str)
        // you only need 3 points to define a plane
        .take(3)
        .collect_vec();

    // print a mathematica script
    let mut equations = vec![];
    for (t, h) in hailstones.iter().enumerate() {
        equations.push(format!("t{t} >= 0"));
        equations.push(format!(
            "{x} + {vx} t{t} == rx + vx t{t}",
            t = t,
            x = h.x,
            vx = h.vx,
        ));
        equations.push(format!(
            "{y} + {vy} t{t} == ry + vy t{t}",
            t = t,
            y = h.y,
            vy = h.vy,
        ));
        equations.push(format!(
            "{z} + {vz} t{t} == rz + vz t{t}",
            t = t,
            z = h.z,
            vz = h.vz,
        ));
    }
    println!(
        "Solve[{{{}}}, {{rx, ry, rz, vx, vy, vz}}]",
        equations.join(", ")
    );

    Ok(0)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    tl: (f64, f64),
    br: (f64, f64),
}

impl Rect {
    fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.tl.0 && x <= self.br.0 && y <= self.tl.1 && y >= self.br.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hailstone {
    fn m(&self) -> f64 {
        self.vy / self.vx
    }

    fn b(&self) -> f64 {
        self.y - (self.m() * self.x)
    }
}

impl FromStr for Hailstone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" @ ").context("malformed")?;
        let (x, y, z) = position
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .context("malformed")?;
        let (vx, vy, vz) = velocity
            .split(", ")
            .map(|s| {
                s.trim()
                    .parse()
                    .with_context(|| format!("parse failure '{s}'"))
                    .unwrap()
            })
            .collect_tuple()
            .context("malformed")?;
        Ok(Hailstone {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const SAMPLE: &'static str = include_str!("../inputs/day24.sample.txt");
//     const INPUT: &'static str = include_str!("../inputs/day24.input.txt");

//     #[test]
//     fn test_part_one_sample() {
//         let ans = part01(SAMPLE).unwrap();
//         assert_eq!(24000, ans);
//     }

//     #[test]
//     fn test_part_one() {
//         let ans = part01(INPUT).unwrap();
//         assert_eq!(69501, ans);
//     }

//     #[test]
//     fn test_part_two_sample() {
//         let ans = part02(SAMPLE).unwrap();
//         assert_eq!(45000, ans);
//     }

//     #[test]
//     fn test_part_two() {
//         let ans = part02(INPUT).unwrap();
//         assert_eq!(202346, ans);
//     }
// }
