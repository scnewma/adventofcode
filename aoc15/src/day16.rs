use std::{collections::HashMap, str::FromStr};
use Compound::*;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Compound {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl FromStr for Compound {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "children" => Ok(Children),
            "cats" => Ok(Cats),
            "samoyeds" => Ok(Samoyeds),
            "pomeranians" => Ok(Pomeranians),
            "akitas" => Ok(Akitas),
            "vizslas" => Ok(Vizslas),
            "goldfish" => Ok(Goldfish),
            "trees" => Ok(Trees),
            "cars" => Ok(Cars),
            "perfumes" => Ok(Perfumes),
            _ => Err(anyhow::anyhow!("unknown compound {s}")),
        }
    }
}

const NEED: [(Compound, usize); 10] = [
    (Children, 3),
    (Cats, 7),
    (Samoyeds, 2),
    (Pomeranians, 3),
    (Akitas, 0),
    (Vizslas, 0),
    (Goldfish, 5),
    (Trees, 3),
    (Cars, 2),
    (Perfumes, 1),
];

pub fn part01(input: &str) -> usize {
    let mut aunts = parse_input(input);

    for (comp, amt) in NEED {
        aunts.retain(|(_, aunt)| match aunt.get(&comp) {
            Some(&v) => amt == v,
            None => true,
        });
    }
    assert!(aunts.len() == 1);
    aunts[0].0
}

pub fn part02(input: &str) -> usize {
    let mut aunts = parse_input(input);

    for (comp, amt) in NEED {
        aunts.retain(|(_, aunt)| match aunt.get(&comp) {
            Some(&v) => match comp {
                Cats | Trees => v > amt,
                Pomeranians | Goldfish => v < amt,
                _ => amt == v,
            },
            None => true,
        });
    }
    assert!(aunts.len() == 1);
    aunts[0].0
}

fn parse_input(input: &str) -> Vec<(usize, HashMap<Compound, usize>)> {
    let mut aunts = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let idx = line.find(':').unwrap();
        let line = &line[idx + 2..];
        let mut comps = HashMap::new();
        for comp in line.split(", ") {
            let (name, amt) = comp.split_once(": ").unwrap();
            let amt: usize = amt.parse().unwrap();
            comps.insert(name.parse().unwrap(), amt);
        }
        aunts.push((i + 1, comps))
    }
    aunts
}
