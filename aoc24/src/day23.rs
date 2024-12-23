use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

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

    let mut ans = 0;
    for (a, b, c) in graph.keys().tuple_combinations() {
        let yes = (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
            && graph[a].contains(b)
            && graph[a].contains(c)
            && graph[b].contains(c);
        if yes {
            ans += 1;
        }
    }

    Ok(ans)
}

pub fn part02(input: &str) -> anyhow::Result<String> {
    let mut graph = FxHashMap::<&str, FxHashSet<&str>>::default();
    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        graph.entry(l).or_default().insert(r);
        graph.entry(r).or_default().insert(l);
    }

    let mut largest_network = FxHashSet::default();

    let mut visited = FxHashSet::default();
    for node in graph.keys() {
        if visited.contains(node) {
            continue;
        }

        let mut component_visited = visited.clone();
        dfs(&graph, node, &mut component_visited);

        let mut connected: FxHashSet<&str> =
            component_visited.difference(&visited).cloned().collect();
        let clique = maximal_clique_pivot(
            &graph,
            &mut FxHashSet::default(),
            &mut connected,
            &mut FxHashSet::default(),
        );
        if clique.len() > largest_network.len() {
            largest_network = clique;
        }

        visited = component_visited;
    }

    Ok(largest_network.into_iter().sorted().join(","))
}

fn dfs<'a>(
    graph: &'a FxHashMap<&'a str, FxHashSet<&'a str>>,
    node: &'a str,
    visited: &mut FxHashSet<&'a str>,
) {
    visited.insert(node);
    for v in &graph[node] {
        if !visited.contains(v) {
            dfs(graph, v, visited);
        }
    }
}

// bron kerbasch
fn maximal_clique_pivot<'a>(
    graph: &'a FxHashMap<&'a str, FxHashSet<&'a str>>,
    r: &mut FxHashSet<&'a str>,
    p: &mut FxHashSet<&'a str>,
    x: &mut FxHashSet<&'a str>,
) -> FxHashSet<&'a str> {
    if p.len() == 0 && x.len() == 0 {
        return r.clone();
    }

    let mut pivot = "";
    let mut pivot_len = 0;
    for v in p.union(x) {
        if graph[v].len() > pivot_len {
            pivot = v;
            pivot_len = graph[v].len();
        }
    }

    let mut max = FxHashSet::default();
    for v in p.difference(&graph[pivot]) {
        let mut r = r.clone();
        r.insert(v);

        let mut p = p.intersection(&graph[v]).cloned().collect();
        let mut x = x.intersection(&graph[v]).cloned().collect();

        let clique = maximal_clique_pivot(graph, &mut r, &mut p, &mut x);
        if clique.len() > max.len() {
            max = clique;
        }

        p.remove(v);
        x.insert(v);
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
