use itertools::{iproduct, Itertools};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (mut bricks, size) = parse_input(input);
    bricks.sort_by(|a, b| a.0.z.cmp(&b.0.z));

    drop_bricks(&mut bricks, size);

    let mut safe_to_disentigrate = 0;
    for i in 0..bricks.len() {
        let mut new = bricks.clone();
        new.remove(i);
        if drop_bricks(&mut new, size) == 0 {
            safe_to_disentigrate += 1;
        }
    }

    Ok(safe_to_disentigrate)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (mut bricks, size) = parse_input(input);
    bricks.sort_by(|a, b| a.0.z.cmp(&b.0.z));

    drop_bricks(&mut bricks, size);

    let mut fallen = 0;
    for i in 0..bricks.len() {
        let mut new = bricks.clone();
        new.remove(i);
        fallen += drop_bricks(&mut new, size);
    }

    Ok(fallen)
}

fn drop_bricks(bricks: &mut [(Point, Point)], size: usize) -> usize {
    let mut fallen = 0;
    // iterate bottom to top dropping bricks to fill the space
    // build a heightmap as we go to track z at each x,y
    let mut heightmap = vec![vec![0; size]; size];
    for brick in bricks.iter_mut() {
        // find supporting brick
        let mut max_z = 0;
        for (x, y) in iproduct!(brick.0.x..=brick.1.x, brick.0.y..=brick.1.y) {
            max_z = max_z.max(heightmap[x][y]);
        }
        let z = max_z + 1;
        if brick.0.z != z {
            fallen += 1;
        }
        let z_diff = brick.1.z - brick.0.z;
        brick.0.z = z;
        brick.1.z = z + z_diff;
        // increase the heightmap
        for (x, y) in iproduct!(brick.0.x..=brick.1.x, brick.0.y..=brick.1.y) {
            heightmap[x][y] = brick.1.z;
        }
    }
    fallen
}

fn parse_input(input: &str) -> (Vec<(Point, Point)>, usize) {
    let bricks = input
        .lines()
        .map(|line| {
            line.split('~')
                .map(|s| {
                    let (x, y, z) = s
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Point { x, y, z }
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    // check assumptions
    bricks.iter().for_each(|(p1, p2)| {
        // brick points are sorted
        assert!(p1.x <= p2.x);
        assert!(p1.y <= p2.y);
        assert!(p1.z <= p2.z);
    });

    let max_x = bricks.iter().map(|(_, p2)| p2.x).max().unwrap();
    let max_y = bricks.iter().map(|(_, p2)| p2.y).max().unwrap();
    assert!(max_x == max_y);
    let size = max_x + 1;

    (bricks, size)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../inputs/day22.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(424, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(55483, ans);
    }
}
