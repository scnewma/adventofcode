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

pub fn part01(input: &str) -> anyhow::Result<u32> {
    let cubes = parse_input(input);
    let mut surface_area = 0;
    for cube in &cubes {
        let mut surface_area_cube = 6;
        for (dx, dy, dz) in [
            (1i32, 0i32, 0i32),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            if let (Some(x), Some(y), Some(z)) = (
                cube.0.checked_add_signed(dx),
                cube.1.checked_add_signed(dy),
                cube.2.checked_add_signed(dz),
            ) {
                let neigh = (x, y, z);
                if cubes.contains(&neigh) {
                    surface_area_cube -= 1;
                }
            }
        }
        surface_area += surface_area_cube;
    }
    Ok(surface_area)
}

// largest x, y, or z was 19
const SIZE: u32 = 20;
const DELTAS: [(i32, i32, i32); 6] = [
    (1i32, 0i32, 0i32),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

pub fn part02(input: &str) -> anyhow::Result<u32> {
    let cubes = parse_input(input);
    let mut surface_area = 0;
    for cube in &cubes {
        let mut surface_area_cube = 6;
        for (dx, dy, dz) in DELTAS {
            if let (Some(x), Some(y), Some(z)) = (
                cube.0.checked_add_signed(dx),
                cube.1.checked_add_signed(dy),
                cube.2.checked_add_signed(dz),
            ) {
                let neigh = (x, y, z);
                if cubes.contains(&neigh) {
                    surface_area_cube -= 1;
                }
            }
        }
        surface_area += surface_area_cube;
    }

    // TODO: there is probably a much for efficient way to calculate an index in a 3D grid, but I
    // can't think of it immediately
    let mut indices = HashMap::new();
    let mut partitions: PartitionVec<(u32, u32, u32)> =
        PartitionVec::with_capacity((SIZE * SIZE * SIZE) as usize + 2);
    let mut idx = 0;
    // create initial sets of self only
    for (x, y, z) in iproduct!(0..SIZE, 0..SIZE, 0..SIZE) {
        indices.insert((x, y, z), idx);
        partitions.push((x, y, z));
        idx += 1;
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
                let me_index = indices.get(&(x, y, z)).unwrap();
                partitions.union(*me_index, ESCAPE_NODE);
            }
            for delta in DELTAS {
                if let Some(n) = get_neighbor(&(x, y, z), &delta) {
                    if !cubes.contains(&n) {
                        let idx1 = indices.get(&(x, y, z));
                        let idx2 = indices.get(&n);
                        if let (Some(idx1), Some(idx2)) = (idx1, idx2) {
                            partitions.union(*idx1, *idx2);
                        }
                    }
                }
            }
        } else {
            let me_index = indices.get(&(x, y, z)).unwrap();
            partitions.union(*me_index, LAVA_NODE);
        }
    }

    let mut trapped_surface_area = 0;
    let mut visited = HashSet::new();
    for (x, y, z) in iproduct!(0..SIZE, 0..SIZE, 0..SIZE) {
        let idx = *indices.get(&(x, y, z)).unwrap();
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

                let ni = indices.get(&n);
                if ni.is_none() {
                    continue;
                }
                let ni = *ni.unwrap();

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

    // 3338 - too high
    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1980, ans);
    }
}
