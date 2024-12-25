use std::collections::HashMap;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let edges = parse_input(input);
    let mut visits = HashMap::new();
    visits.insert("start", 1);
    enumerate(&edges, &Vec::new(), "start", &visits, 1)
}

pub fn part02(input: &str) -> i64 {
    let edges = parse_input(input);
    let mut visits = HashMap::new();
    visits.insert("start", 2);
    enumerate(&edges, &Vec::new(), "start", &visits, 2)
}

fn enumerate<'a>(
    edges: &HashMap<&'a str, Vec<&'a str>>,
    path: &[&'a str],
    current_cave: &'a str,
    visits: &HashMap<&'a str, i64>,
    max_visits: i64,
) -> i64 {
    let mut path = path.to_owned();
    path.push(current_cave);
    println!("{:?} - {:?}", path, visits);

    if current_cave == "end" {
        return 1;
    }

    let mut count = 0;
    for neighbor in edges.get(current_cave).unwrap() {
        // we shadow these variables here so that they are unique from this part of the path onward
        let mut visits = visits.clone();
        let mut max_visits = max_visits;

        if is_small_cave(neighbor) {
            // check small cave to determine if we have visited it more times than allowed
            // if so then we end this path here and pop this neighbor off the stack
            let num_visits = visits.entry(neighbor).or_default();
            if *num_visits >= max_visits {
                continue;
            }

            // we are going to visit this cave
            *num_visits += 1;

            // if we have visited this small cave more than once, lower the max visits
            // so that no more caves can be visited more than once
            if *num_visits > 1 {
                max_visits = 1;
            }
        }

        // count the total number of paths that start with the currently active path, that make it
        // to "end"
        count += enumerate(edges, &path, neighbor, &visits, max_visits);
    }

    count
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        edges.entry(left).or_default().push(right);
        edges.entry(right).or_default().push(left);
    }
    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day12.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT);
        assert_eq!(3738, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT);
        assert_eq!(120506, ans);
    }
}
