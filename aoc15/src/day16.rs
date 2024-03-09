use std::collections::HashMap;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

const NEED: [(&str, usize); 10] = [
    ("children", 3),
    ("cats", 7),
    ("samoyeds", 2),
    ("pomeranians", 3),
    ("akitas", 0),
    ("vizslas", 0),
    ("goldfish", 5),
    ("trees", 3),
    ("cars", 2),
    ("perfumes", 1),
];

pub fn part01(input: &str) -> usize {
    let mut aunts = parse_input(input);

    for (comp, amt) in NEED {
        aunts.retain(|(_, aunt)| match aunt.get(comp) {
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
        aunts.retain(|(_, aunt)| match aunt.get(comp) {
            Some(&v) => match comp {
                "cats" | "trees" => v > amt,
                "pomeranians" | "goldfish" => v < amt,
                _ => amt == v,
            },
            None => true,
        });
    }
    assert!(aunts.len() == 1);
    aunts[0].0
}

fn parse_input(input: &str) -> Vec<(usize, HashMap<&str, usize>)> {
    let mut aunts = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let idx = line.find(':').unwrap();
        let line = &line[idx + 2..];
        let mut comps = HashMap::new();
        for comp in line.split(", ") {
            let (name, amt) = comp.split_once(": ").unwrap();
            let amt: usize = amt.parse().unwrap();
            comps.insert(name, amt);
        }
        aunts.push((i + 1, comps))
    }
    aunts
}
