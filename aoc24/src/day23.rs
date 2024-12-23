use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use string_interner::{symbol::SymbolU32, StringInterner};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?,
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut graph = FxHashMap::<&str, Vec<&str>>::default();
    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        graph.entry(l).or_default().push(r);
        graph.entry(r).or_default().push(l);
    }

    let mut cliques = FxHashSet::default();
    for node in graph.keys().filter(|node| node.starts_with('t')) {
        for v in &graph[node] {
            for u in &graph[v] {
                if graph[node].contains(u) {
                    let mut clique = [node, v, u];
                    clique.sort();
                    cliques.insert(clique);
                }
            }
        }
    }

    Ok(cliques.len())
}

pub fn part02(input: &str) -> anyhow::Result<String> {
    // performance optimization: intern the strings
    let mut graph = FxHashMap::<SymbolU32, FxHashSet<SymbolU32>>::default();
    let mut interner = StringInterner::default();
    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        let l = interner.get_or_intern(l);
        let r = interner.get_or_intern(r);
        graph.entry(l).or_default().insert(r);
        graph.entry(r).or_default().insert(l);
    }

    let mut vertices = graph.keys().cloned().collect();
    let largest_network = maximal_clique_pivot(
        &graph,
        &mut FxHashSet::default(),
        &mut vertices,
        &mut FxHashSet::default(),
    );
    Ok(largest_network
        .into_iter()
        .map(|sym| interner.resolve(sym).unwrap())
        .sorted()
        .join(","))
}

// bron kerbasch
fn maximal_clique_pivot(
    graph: &FxHashMap<SymbolU32, FxHashSet<SymbolU32>>,
    r: &mut FxHashSet<SymbolU32>,
    p: &mut FxHashSet<SymbolU32>,
    x: &mut FxHashSet<SymbolU32>,
) -> FxHashSet<SymbolU32> {
    if p.len() == 0 && x.len() == 0 {
        return r.clone();
    }

    let pivot = p.union(x).max_by_key(|v| graph[v].len()).unwrap();

    let mut max = FxHashSet::default();
    for v in p.clone().difference(&graph[pivot]) {
        r.insert(*v);

        let mut pv = p.intersection(&graph[v]).cloned().collect();
        let mut xv = x.intersection(&graph[v]).cloned().collect();

        let clique = maximal_clique_pivot(graph, r, &mut pv, &mut xv);
        if clique.len() > max.len() {
            max = clique;
        }

        r.remove(v);
        p.remove(v);
        x.insert(*v);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day23.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1194, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!("bd,bu,dv,gl,qc,rn,so,tm,wf,yl,ys,ze,zr", ans);
    }
}
