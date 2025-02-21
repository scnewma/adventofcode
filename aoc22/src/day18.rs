use itertools::iproduct;
use partitions::PartitionVec;
use std::collections::{HashSet, VecDeque};

use crate::SolveInfo;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

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

// To make code easier to write this one dimension of the 3D cube, it is 1 + max(dimension) of the
// input
const SIZE: u32 = 20;
const VOL: u32 = SIZE * SIZE * SIZE;

// Summary of algorithm:
// Each position in the 3D cube is put in one of three categories of disjoint sets:
//   * Escaped Air: Air block that is not "trapped"
//   * Lava Droplet: All lava droplets (puzzle input)
//   * Trapped Air: Basically everything that wasn't in one of the first two categories
//
// After we have categorized all of the positions in the 3D cube, we then calculate the surface
// area of the trapped air. There are many different disjoint sets of trapped air (because they are
// trapped in different locations). The total surface area of all pockets of trapped air is then
// subtracted from the total surface area (i.e. part01).
pub fn part02(input: &str) -> anyhow::Result<u32> {
    let cubes = parse_input(input);
    let surface_area = part01_inner(&cubes);

    // partitions is a disjoint set of every position within the 3D cube
    let mut partitions: PartitionVec<(u32, u32, u32)> =
        PartitionVec::with_capacity(VOL as usize + 2);
    // create initial sets of self only
    for (x, y, z) in iproduct!(0..SIZE, 0..SIZE, 0..SIZE) {
        partitions.push((x, y, z));
    }

    // "fake" node that represents the escape node
    const ESCAPE_NODE: usize = (SIZE * SIZE * SIZE) as usize;
    partitions.push((u32::MAX, u32::MAX, u32::MAX));

    // "fake" node to union all lava droplets to
    const LAVA_NODE: usize = (SIZE * SIZE * SIZE) as usize + 1;
    partitions.push((u32::MAX, u32::MAX, u32::MAX));

    for pos in iproduct!(0..SIZE, 0..SIZE, 0..SIZE) {
        if cubes.contains(&pos) {
            // if this is a lava droplet, we put in in the set with the other lava droplets so that
            // we can exclude these positions later
            partitions.union(safe_index(&pos).unwrap(), LAVA_NODE);
            continue;
        }

        // every air block on the outer edges of the cube are not trapped so we union them with
        // the escape node to ensure they are not treated as trapped
        if pos.0 == 0 || pos.1 == 0 || pos.2 == 0 {
            partitions.union(safe_index(&pos).unwrap(), ESCAPE_NODE);
        }

        // this location is an air block, union it with any neighboring air blocks
        for neighbor in neighbors(&pos) {
            if !cubes.contains(&neighbor) {
                if let (Some(pi), Some(ni)) = (safe_index(&pos), safe_index(&neighbor)) {
                    partitions.union(pi, ni);
                }
            }
        }
    }

    let mut trapped_surface_area = 0;
    let mut visited = HashSet::new();
    for (x, y, z) in iproduct!(0..SIZE, 0..SIZE, 0..SIZE) {
        // we are looking for trapped air, so we only need to check air that isn't in the same set
        // as lava droplets or escaped air
        let idx = safe_index(&(x, y, z)).unwrap();
        if partitions.same_set(idx, ESCAPE_NODE) || partitions.same_set(idx, LAVA_NODE) {
            continue;
        }

        // expand the volume of trapped air to find the outer surface area of this pocket of air
        let mut queue = VecDeque::new();
        queue.push_back((x, y, z));
        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            for neighbor in neighbors(&pos) {
                if let Some(ni) = safe_index(&neighbor) {
                    if partitions.same_set(ni, LAVA_NODE) {
                        // if this neighbor is a lava droplet then we found an edge to our air pocket
                        trapped_surface_area += 1;
                    } else {
                        // this neighbor is an air block, we'll check that later
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    Ok(surface_area - trapped_surface_area)
}

// calculates an array index for a 3D point
// returns None if the provided position is outside of the bounds of the cube
fn safe_index((x, y, z): &(u32, u32, u32)) -> Option<usize> {
    let idx = z + y * SIZE + x * SIZE * SIZE;
    if idx > VOL { None } else { Some(idx as usize) }
}

fn neighbors(pos: &(u32, u32, u32)) -> Neighbors {
    Neighbors {
        pos: *pos,
        deltas_index: 0,
        max_pos: SIZE,
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

const DELTAS: [(i32, i32, i32); 6] = [
    (1i32, 0i32, 0i32),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

struct Neighbors {
    pos: (u32, u32, u32),
    deltas_index: usize,
    max_pos: u32,
}

impl Iterator for Neighbors {
    type Item = (u32, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // we don't have any more neighbors to calculate
            if self.deltas_index >= DELTAS.len() {
                break None;
            }
            let delta = DELTAS[self.deltas_index];
            self.deltas_index += 1;

            // we clamp the return positions of this iterator to be within 0..max_pos
            if let (Some(x), Some(y), Some(z)) = (
                // checked_add_signed requires Rust 1.66, but yay for new features!
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

    const SAMPLE: &str = include_str!("../inputs/day18.sample.txt");
    const INPUT: &str = include_str!("../inputs/day18.input.txt");

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
