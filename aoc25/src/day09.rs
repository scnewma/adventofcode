use std::collections::{HashMap, HashSet, VecDeque};

use itertools::{Itertools, iproduct};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let tiles = parse_input(input);

    let mut max_area = 0;
    for combo in tiles.into_iter().combinations(2) {
        let (a, b) = (combo[0], combo[1]);

        let area = (((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)) as usize;
        max_area = max_area.max(area);
    }
    Ok(max_area)
}

// pub fn part02(input: &str) -> anyhow::Result<usize> {
//     let red_tiles = parse_input(input);

//     let mut max_area = 0;
//     for (a, b) in red_tiles.iter().tuple_combinations() {
//         let area = Area {
//             min_x: a.0.min(b.0) as usize,
//             max_x: a.0.max(b.0) as usize,
//             min_y: a.1.min(b.1) as usize,
//             max_y: a.1.max(b.1) as usize,
//         };

//         let invalid = red_tiles
//             .iter()
//             .any(|tile| area.contains(tile.0 as usize, tile.1 as usize));
//         if !invalid {
//             println!("a={a:?} b={b:?} size={}", area.size());
//             print_tiles(
//                 &red_tiles,
//                 &(a.0 as usize, a.1 as usize),
//                 &(b.0 as usize, b.1 as usize),
//             );

//             max_area = max_area.max(area.size());
//         }
//     }
//     Ok(max_area)
// }

// fn print_tiles(tiles: &[(isize, isize)], highlight1: &(usize, usize), highlight2: &(usize, usize)) {
//     let max_x = tiles.iter().map(|(x, _)| *x as usize).max().unwrap() + 2;
//     let max_y = tiles.iter().map(|(_, y)| *y as usize).max().unwrap() + 1;
//     let lookup: HashSet<(usize, usize)> = tiles
//         .iter()
//         .map(|(x, y)| (*x as usize, *y as usize))
//         .collect();

//     for y in 0..=max_y {
//         for x in 0..=max_x {
//             if &(x, y) == highlight1 || &(x, y) == highlight2 {
//                 print!("\x1b[1mX\x1b[0m");
//             } else {
//                 print!("{}", if lookup.contains(&(x, y)) { "O" } else { "." })
//             }
//         }
//         println!()
//     }
// }

// #[derive(Debug)]
// struct Area {
//     min_x: usize,
//     max_x: usize,
//     min_y: usize,
//     max_y: usize,
// }

// impl Area {
//     fn contains(&self, x: usize, y: usize) -> bool {
//         // does not count == as contains
//         self.min_x < x && x < self.max_x && self.min_y < y && y < self.max_y
//     }

//     fn size(&self) -> usize {
//         let w = self.max_x - self.min_x + 1;
//         let h = self.max_y - self.min_y + 1;
//         w * h
//     }
// }

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let tiles = parse_input(input);

    // compress coordinates
    let sorted_xs = {
        let mut xs: Vec<isize> = tiles.iter().map(|(x, _)| *x).collect();
        xs.sort_unstable();
        xs.dedup();
        xs
    };
    let xs: HashMap<isize, isize> = sorted_xs
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i as isize))
        .collect();

    let sorted_ys = {
        let mut ys: Vec<isize> = tiles.iter().map(|(_, y)| *y).collect();
        ys.sort_unstable();
        ys.dedup();
        ys
    };
    let ys: HashMap<isize, isize> = sorted_ys
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i as isize))
        .collect();

    let uncompress =
        |original_vs: &Vec<isize>, compressed_v: isize| original_vs[compressed_v as usize];

    // map original coordinates to compressed coordinates
    let compressed = tiles
        .iter()
        .map(|(x, y)| (xs[x], ys[y]))
        .collect::<Vec<(isize, isize)>>();

    // fill the outline
    let mut outline = HashSet::new();
    for (curr, dest) in compressed.iter().circular_tuple_windows() {
        for x in curr.0.min(dest.0)..=curr.0.max(dest.0) {
            for y in curr.1.min(dest.1)..=curr.1.max(dest.1) {
                outline.insert((x, y));
            }
        }
    }

    // flood fill to find outside
    let min_x = outline.iter().map(|p| p.0).min().unwrap() - 1;
    let max_x = outline.iter().map(|p| p.0).max().unwrap() + 1;
    let min_y = outline.iter().map(|p| p.1).min().unwrap() - 1;
    let max_y = outline.iter().map(|p| p.1).max().unwrap() + 1;
    let mut outside = HashSet::new();
    let mut q = VecDeque::new();
    q.push_front((min_x, min_y));
    outside.insert((min_x, min_y));
    while let Some((x, y)) = q.pop_front() {
        for next in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            let (nx, ny) = next;
            if nx < min_x || nx > max_x || ny < min_y || ny > max_y {
                continue;
            }
            if outline.contains(&next) || outside.contains(&next) {
                continue;
            }

            outside.insert(next);
            q.push_back(next);
        }
    }

    let mut max_area = 0;
    for (a, b) in compressed.iter().tuple_combinations() {
        let min_x = a.0.min(b.0);
        let max_x = a.0.max(b.0);
        let min_y = a.1.min(b.1);
        let max_y = a.1.max(b.1);

        let w = uncompress(&sorted_xs, max_x) - uncompress(&sorted_xs, min_x) + 1;
        let h = uncompress(&sorted_ys, max_y) - uncompress(&sorted_ys, min_y) + 1;
        let possible_area = w * h;
        if possible_area < max_area {
            continue;
        }

        let is_disjoint = !iproduct!(min_x..=max_x, min_y..=max_y).any(|pt| outside.contains(&pt));
        if is_disjoint {
            max_area = possible_area;
        }
    }

    Ok(max_area as usize)
}

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    let mut tiles = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap());
        tiles.push((x, y));
    }
    tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day09.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(4771532800, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1544362560, ans);
    }
}
