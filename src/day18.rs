use itertools::iproduct;
use partitions::PartitionVec;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const DELTAS: [(i32, i32, i32); 6] = [
    (1i32, 0i32, 0i32),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let cubes = parse_input(input);
    Ok(part01_inner(&cubes))
}

fn part01_inner(cubes: &HashSet<(u32, u32, u32)>) -> u32 {
    let mut surface_area = 0;
    for cube in cubes {
        // the surface area that this cube adds to the total surface area is 6 - the number of
        // adjacent neighbors
        let surface_area_cube = 6 - neighbors(cube).filter(|n| cubes.contains(n)).count() as u32;
        surface_area += surface_area_cube;
    }
    surface_area
}

// largest x, y, or z was 19
const SIZE: u32 = 20;
const VOL: u32 = SIZE * SIZE * SIZE;

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let cubes = parse_input(input);
    let surface_area = part01_inner(&cubes);

    let mut partitions: PartitionVec<(u32, u32, u32)> =
        PartitionVec::with_capacity(VOL as usize + 2);
    // create initial sets of self only
    for (x, y, z) in iproduct!(0..SIZE, 0..SIZE, 0..SIZE) {
        partitions.push((x, y, z));
    }

    // "fake" node that represents the escape node
    const ESCAPE_NODE: usize = (SIZE * SIZE * SIZE) as usize;
    partitions.push((u32::max_value(), u32::max_value(), u32::max_value()));

    // "fake" node to union all lava droplets to
    const LAVA_NODE: usize = (SIZE * SIZE * SIZE) as usize + 1;
    partitions.push((u32::max_value(), u32::max_value(), u32::max_value()));

    for (x, y, z) in iproduct!(0..SIZE, 0..SIZE, 0..SIZE) {
        // if this location is not a cube then it's an air block, union it with any
        // neighboring air blocks
        if !cubes.contains(&(x, y, z)) {
            if x == 0 || y == 0 || z == 0 {
                let me_index = index(&(x, y, z));
                partitions.union(me_index, ESCAPE_NODE);
            }
            for delta in DELTAS {
                if let Some(n) = get_neighbor(&(x, y, z), &delta) {
                    if !cubes.contains(&n) {
                        let idx1 = index(&(x, y, z));
                        let idx2 = index(&n);
                        if idx1 > VOL as usize || idx2 > VOL as usize {
                            continue;
                        }
                        partitions.union(idx1, idx2);
                    }
                }
            }
        } else {
            let me_index = index(&(x, y, z));
            partitions.union(me_index, LAVA_NODE);
        }
    }

    let mut trapped_surface_area = 0;
    let mut visited = HashSet::new();
    for (x, y, z) in iproduct!(0..SIZE, 0..SIZE, 0..SIZE) {
        let idx = index(&(x, y, z));
        if partitions.same_set(idx, ESCAPE_NODE) || partitions.same_set(idx, LAVA_NODE) {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back((x, y, z));
        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            for delta in DELTAS {
                let n = get_neighbor(&pos, &delta);
                if n.is_none() {
                    continue;
                }
                let n = n.unwrap();

                let ni = index(&n);
                if ni > VOL as usize {
                    continue;
                }

                if partitions.same_set(ni, LAVA_NODE) {
                    trapped_surface_area += 1;
                } else {
                    queue.push_back(n);
                }
            }
        }
    }

    Ok(surface_area - trapped_surface_area)
}

fn index((x, y, z): &(u32, u32, u32)) -> usize {
    (z + y * SIZE + x * SIZE * SIZE) as usize
}

fn neighbors(pos: &(u32, u32, u32)) -> Neighbors {
    Neighbors {
        pos: *pos,
        deltas_index: 0,
        max_pos: SIZE,
    }
}

fn get_neighbor(pos: &(u32, u32, u32), delta: &(i32, i32, i32)) -> Option<(u32, u32, u32)> {
    if let (Some(x), Some(y), Some(z)) = (
        pos.0.checked_add_signed(delta.0),
        pos.1.checked_add_signed(delta.1),
        pos.2.checked_add_signed(delta.2),
    ) {
        Some((x, y, z))
    } else {
        None
    }
}

fn parse_input(input: &str) -> HashSet<(u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            let mut pos = line.splitn(3, ',');
            (
                pos.next().unwrap().parse::<u32>().unwrap(),
                pos.next().unwrap().parse::<u32>().unwrap(),
                pos.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

struct Neighbors {
    pos: (u32, u32, u32),
    deltas_index: usize,
    max_pos: u32,
}

impl Iterator for Neighbors {
    type Item = (u32, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.deltas_index >= DELTAS.len() {
                break None;
            }
            let delta = DELTAS[self.deltas_index];
            self.deltas_index += 1;

            if let (Some(x), Some(y), Some(z)) = (
                self.pos.0.checked_add_signed(delta.0),
                self.pos.1.checked_add_signed(delta.1),
                self.pos.2.checked_add_signed(delta.2),
            ) {
                if x < self.max_pos && y < self.max_pos && z < self.max_pos {
                    break Some((x, y, z));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day18.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day18.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(64, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3346, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(58, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1980, ans);
    }
}
