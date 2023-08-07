use std::str::FromStr;

use anyhow::anyhow;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

// HACK: didn't solve this generically :shrug:
pub fn part01(input: &str) -> u32 {
    let ingredients: Vec<_> = input.lines().flat_map(Ingredient::from_str).collect();
    let (i1, i2, i3, i4) = (
        &ingredients[0],
        &ingredients[1],
        &ingredients[2],
        &ingredients[3],
    );

    let mut max = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                for l in 0..=100 {
                    let total_tsp = i + j + k + l;
                    if total_tsp != 100 {
                        continue;
                    }

                    macro_rules! calc_prop {
                        ($prop:ident) => {{
                            let v = i1.$prop * i + i2.$prop * j + i3.$prop * k + i4.$prop * l;
                            v.max(0) as u32
                        }};
                    }

                    let capacity = calc_prop!(capacity);
                    let durability = calc_prop!(durability);
                    let flavor = calc_prop!(flavor);
                    let texture = calc_prop!(texture);

                    let score = capacity * durability * flavor * texture;
                    max = max.max(score);
                }
            }
        }
    }
    max
}

pub fn part02(input: &str) -> u32 {
    let ingredients: Vec<_> = input.lines().flat_map(Ingredient::from_str).collect();
    let (i1, i2, i3, i4) = (
        &ingredients[0],
        &ingredients[1],
        &ingredients[2],
        &ingredients[3],
    );

    let mut max = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                for l in 0..=100 {
                    let total_tsp = i + j + k + l;
                    if total_tsp != 100 {
                        continue;
                    }

                    macro_rules! calc_prop {
                        ($prop:ident) => {{
                            let v = i1.$prop * i + i2.$prop * j + i3.$prop * k + i4.$prop * l;
                            v.max(0) as u32
                        }};
                    }

                    let calories = calc_prop!(calories);
                    if calories != 500 {
                        continue;
                    }

                    let capacity = calc_prop!(capacity);
                    let durability = calc_prop!(durability);
                    let flavor = calc_prop!(flavor);
                    let texture = calc_prop!(texture);

                    let score = capacity * durability * flavor * texture;
                    max = max.max(score);
                }
            }
        }
    }
    max
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, properties) = s.split_once(": ").ok_or(anyhow!("malformed ingredient"))?;
        let mut properties = properties.split(", ");

        macro_rules! next_property {
            () => {{
                let (_, n) = properties.next().unwrap().split_once(' ').unwrap();
                let n: i32 = n.parse().unwrap();
                n
            }};
        }

        Ok(Ingredient {
            capacity: next_property!(),
            durability: next_property!(),
            flavor: next_property!(),
            texture: next_property!(),
            calories: next_property!(),
        })
    }
}
