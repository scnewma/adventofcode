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

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let tiles = parse_input(input);

    let x_axis = CompressedAxis::new(tiles.iter().map(|(x, _)| *x));
    let y_axis = CompressedAxis::new(tiles.iter().map(|(_, y)| *y));

    let compressed = tiles
        .iter()
        .map(|(x, y)| (x_axis.compress(*x), y_axis.compress(*y)))
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

    // search every pair of points for a max area within the polygon
    let mut max_area = 0;
    for (a, b) in compressed.iter().tuple_combinations() {
        let min_x = a.0.min(b.0);
        let max_x = a.0.max(b.0);
        let min_y = a.1.min(b.1);
        let max_y = a.1.max(b.1);

        let w = x_axis.span_len(min_x, max_x);
        let h = y_axis.span_len(min_y, max_y);
        let possible_area = w * h;
        if possible_area < max_area {
            continue;
        }

        let is_inside_polygon =
            !iproduct!(min_x..=max_x, min_y..=max_y).any(|pt| outside.contains(&pt));
        if is_inside_polygon {
            max_area = possible_area;
        }
    }

    Ok(max_area as usize)
}

struct CompressedAxis {
    values: Vec<isize>,
    indexes: HashMap<isize, isize>,
}

impl CompressedAxis {
    fn new(values: impl IntoIterator<Item = isize>) -> Self {
        let mut values = values.into_iter().collect::<Vec<_>>();
        values.sort_unstable();
        values.dedup();

        let indexes = values
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i as isize))
            .collect();

        Self { values, indexes }
    }

    fn compress(&self, value: isize) -> isize {
        self.indexes[&value]
    }

    fn uncompress(&self, value: isize) -> isize {
        self.values[value as usize]
    }

    fn span_len(&self, a: isize, b: isize) -> isize {
        self.uncompress(a.max(b)) - self.uncompress(a.min(b)) + 1
    }
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
