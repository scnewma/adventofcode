use arrayvec::ArrayVec;
use fxhash::FxHashMap;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const MAX_WIDTH: usize = 30;
type VecPages = ArrayVec<usize, MAX_WIDTH>;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (updates, ordering) = parse_input(input);

    Ok(updates
        .into_iter()
        .map(|update| {
            let sorted = topsort(&update, &ordering);
            if update == sorted {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (updates, ordering) = parse_input(input);

    Ok(updates
        .into_iter()
        .map(|update| {
            let sorted = topsort(&update, &ordering);
            if update != sorted {
                sorted[sorted.len() / 2]
            } else {
                0
            }
        })
        .sum())
}

fn topsort(update: &[usize], ordering: &FxHashMap<usize, VecPages>) -> VecPages {
    let mut ts = crate::topsort::ArrayTopSort::<usize, MAX_WIDTH>::new();
    for n in update {
        if let Some(depends_on) = ordering.get(n) {
            for v in depends_on {
                if update.contains(v) {
                    ts.add_dependency(*v, *n);
                }
            }
        }
    }
    ts.collect()
}

fn parse_input(input: &str) -> (Vec<VecPages>, FxHashMap<usize, VecPages>) {
    let (orders, updates) = input.split_once("\n\n").unwrap();
    let mut ordering = FxHashMap::<usize, VecPages>::default();
    orders
        .lines()
        .map(|s| {
            s.split_once('|')
                .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                .unwrap()
        })
        .for_each(|(l, r)| ordering.entry(r).or_default().push(l));
    let updates = updates
        .trim()
        .lines()
        .map(|s| s.split(',').map(|s| s.parse::<usize>().unwrap()).collect())
        .collect_vec();
    (updates, ordering)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day05.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(5509, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(4407, ans);
    }
}
