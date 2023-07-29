use anyhow::Context;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i32 {
    solve(input, false)
}

pub fn part02(input: &str) -> i32 {
    solve(input, true)
}

pub fn solve(input: &str, insert_self: bool) -> i32 {
    let mut graph = parse_input(input);
    let mut guests: FxHashSet<_> = graph.iter().map(|((from, _), _)| *from).collect();

    if insert_self {
        for guest in guests.iter() {
            graph.insert((guest, "self"), 0);
            graph.insert(("self", guest), 0);
        }
        guests.insert("self");
    }

    let k = guests.len();
    guests
        .into_iter()
        .permutations(k)
        .map(|guests| calculate_happiness(&graph, &guests))
        .max()
        .unwrap()
}

fn calculate_happiness(graph: &FxHashMap<(&str, &str), i32>, guests: &[&str]) -> i32 {
    let mut happiness = 0;
    for (i, guest) in guests.iter().enumerate() {
        let left = if i == 0 {
            guests.last().unwrap()
        } else {
            guests[i - 1]
        };
        let right = if i == guests.len() - 1 {
            guests.first().unwrap()
        } else {
            guests[i + 1]
        };

        happiness += graph
            .get(&(guest, left))
            .with_context(|| {
                format!(
                    "no happiness delta found for '{}' sitting next to '{}'",
                    guest, left
                )
            })
            .unwrap();
        happiness += graph
            .get(&(guest, right))
            .with_context(|| {
                format!(
                    "no happiness delta found for '{}' sitting next to '{}'",
                    guest, right
                )
            })
            .unwrap();
    }
    happiness
}

fn parse_input(input: &str) -> FxHashMap<(&str, &str), i32> {
    let mut happiness = FxHashMap::default();
    for line in input.lines() {
        let (from, to, amt) = parse_line(line).unwrap();
        happiness.insert((from, to), amt);
    }
    happiness
}

fn parse_line(line: &str) -> anyhow::Result<(&str, &str, i32)> {
    let mut words = line.split_whitespace();
    let from = words.next().context("no beginning name")?;
    let mut words = words.skip(1);
    let gainlose = words.next().context("no gain/lose")?;
    let amt: u32 = words.next().context("no amount")?.parse()?;
    let mut words = words.skip(6); // happiness units by sitting next to
    let to = words
        .next()
        .context("no ending name")?
        .trim_end_matches('.');

    let amt: i32 = match gainlose {
        "gain" => amt as i32,
        "lose" => -(amt as i32),
        _ => unreachable!("word #3 should be either 'gain' or 'lose'"),
    };
    Ok((from, to, amt))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
"#;

    #[test]
    fn test_example() {
        let ans = part01(EXAMPLE.trim());
        assert_eq!(330, ans);
        let ans = part02(EXAMPLE.trim());
        assert_eq!(286, ans);
    }
}
