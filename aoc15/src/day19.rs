use std::{collections::VecDeque, fmt::Write};

use fxhash::{FxHashMap, FxHashSet};

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    if USE_SAMPLE {
        Ok(crate::SolveInfo {
            part01: part01(SAMPLE.trim()).to_string(),
            part02: part02(SAMPLE2.trim()).to_string(),
        })
    } else {
        Ok(crate::SolveInfo {
            part01: part01(input).to_string(),
            part02: part02(input).to_string(),
        })
    }
}

const USE_SAMPLE: bool = false;

const SAMPLE: &str = r#"
H => HO
H => OH
O => HH

HOHOHO
"#;

const SAMPLE2: &str = r#"
e => H
e => O
H => HO
H => OH
O => HH

HOHOHO
"#;

pub fn part01(input: &str) -> usize {
    let (replacements, molecule) = parse_input(input);
    calculate_molecules(&molecule, &replacements, 0).len()
}

// :idea: Is it possible to compute a graph of all ways to get a certain sequence (C, Rn, Ca, etc.)?
// If so, I think you could trim generated modules if they do not have any of the given sequences?
// Meaning, you can get Rn from any of [Al, Ca, H, N, O, P], so if the next pair you need is Rn and
// you don't have one of those you can ignore the molecule?
// You would also need to somehow take into account where in the replacement sequence Rn shows up
// since the replacement sequences have different lengths.
// Some sequences can produce themselves, both in the 1st and subsequent positions. Meaning they
// can "move" themselves forward in the chain... I'm unsure if that needs to be handled directly or
// not.
//
// :idea: Can you compress all of the 2 letter sequences down to a single character? That might
// make reasoning about some things easier (and would be computationally less expensive).

pub fn part02(input: &str) -> usize {
    let (replacements, med_molecule) = parse_input(input);

    let mut anchors = FxHashSet::default();
    anchors.insert("Rn");
    anchors.insert("Ar");

    let mut lowest = FxHashMap::default();
    let mut q = VecDeque::new();
    q.push_back((0, "e".to_string(), 0));
    while let Some((n, molecule, n_solved)) = q.pop_front() {
        println!("{}", molecule);
        if molecule.len() > med_molecule.len() {
            continue;
        }
        let e = *lowest.entry(molecule.clone()).or_insert(n);
        if n > e {
            continue;
        }
        let n = n + 1;

        let molecules = calculate_molecules(&molecule, &replacements, n_solved);
        if molecules.contains(&med_molecule) {
            return n;
        }
        for molecule in molecules {
            if !molecule.starts_with("CRn")
                && !molecule.starts_with("H")
                && !molecule.starts_with("N")
                && !molecule.starts_with("O")
            {
                continue;
            }
            // let do_print = molecule.starts_with("CRn");
            // if do_print {
            //     println!("= {}", molecule);
            // }
            // i don't think this approach works because it assumes that you cannot grow to the
            // left of the anchor, but _technically_ there is nothing stopping that.
            let rindex = anchors
                .iter()
                .flat_map(|anchor| {
                    molecule
                        .match_indices(anchor)
                        .filter(|&(i, _)| med_molecule.starts_with(&molecule[..=i]))
                        .last()
                        .map(|(i, _)| i)
                })
                .max();
            // if do_print {
            //     println!(" rindex {rindex:?}");
            // }
            match rindex {
                Some(i) => {
                    // println!("{}", molecule);
                    // +2 bc all anchors are 2 chars
                    q.push_back((n, molecule, i + 2));
                }
                None => q.push_back((n, molecule, 0)),
            }
            // if anchors.contains(&molecule[molecule.len() - 2..]) {
            //     if med_molecule.starts_with(&molecule) {
            //         // if molecule.len() > max {
            //         //     max = molecule.len();
            //         // }
            //         println!("{}", molecule);
            //         q.push_back((n, molecule));
            //     }
            // } else {
            //     q.push_back((n, molecule));
            // }
        }
    }
    panic!("no solution found")
}

fn calculate_molecules(
    current: &str,
    replacements: &[(String, String)],
    start: usize,
) -> FxHashSet<String> {
    let mut molecules = FxHashSet::default();
    for (src, rep) in replacements {
        for (i, _) in current[start..].match_indices(src) {
            let mut new = current[..start + i].to_string();
            new.write_str(rep).unwrap();
            new.write_str(&current[start + i + src.len()..]).unwrap();
            molecules.insert(new);
        }
    }
    molecules
}

fn parse_input(input: &str) -> (Vec<(String, String)>, String) {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let replacements = replacements
        .lines()
        .map(|l| l.split_once(" => ").unwrap())
        .map(|(l, r)| (l.to_string(), r.to_string()))
        .collect();
    (replacements, molecule.to_string())
}
