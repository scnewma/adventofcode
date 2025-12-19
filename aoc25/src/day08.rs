use itertools::Itertools;
use ordered_float::NotNan;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    // example vs normal input
    let n_shortest_connections = if input.lines().count() == 20 {
        10
    } else {
        1000
    };

    // parse input
    let mut boxes = Vec::new();
    for line in input.lines() {
        let nums: Vec<usize> = line.splitn(3, ',').map(|s| s.parse().unwrap()).collect();
        boxes.push((nums[0], nums[1], nums[2])); // x, y, z
    }

    // sort all pairs by distance
    let mut pairs = HashMap::new(); // stores pairs by distance for later lookup
    let mut heap = BinaryHeap::new();
    for combo in boxes.clone().into_iter().combinations(2) {
        let (a, b) = (combo[0], combo[1]);
        let dx = a.0 as f64 - b.0 as f64;
        let dy = a.1 as f64 - b.1 as f64;
        let dz = a.2 as f64 - b.2 as f64;

        let d = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
        let d = NotNan::new(d).unwrap();
        heap.push(Reverse(d));
        assert!(pairs.insert(d, (a, b)).is_none());
    }

    // use union-find to connect the shortest pairs
    let mut uf = QuickUnionUf::<UnionBySize>::new(boxes.len());
    for _ in 0..n_shortest_connections {
        if let Some(Reverse(d)) = heap.pop() {
            let (a, b) = pairs.get(&d).unwrap();
            let ai = boxes.iter().position(|x| x == a).unwrap();
            let bi = boxes.iter().position(|x| x == b).unwrap();
            uf.union(ai, bi);
        }
    }

    // built circuits based on where union-find algo put them
    let mut circuits = HashMap::new();
    (0..boxes.len()).for_each(|i| {
        _ = circuits
            .entry(uf.find(i))
            .and_modify(|pairs: &mut Vec<_>| pairs.push(&boxes[i]))
            .or_insert(vec![&boxes[i]])
    });

    // product of top3 circuit sizes
    Ok(circuits
        .values()
        .map(|v| v.len())
        .sorted_by_key(|l| Reverse(*l))
        .take(3)
        .product::<usize>())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day08.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(54600, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
