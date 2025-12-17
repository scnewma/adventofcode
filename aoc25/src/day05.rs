pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ingredients: Vec<usize> = ingredients.lines().map(|l| l.parse().unwrap()).collect();
    let ranges: Vec<_> = ranges
        .lines()
        .map(|ln| {
            let (l, r) = ln.split_once('-').unwrap();
            let l: usize = l.parse().unwrap();
            let r: usize = r.parse().unwrap();
            assert!(l <= r, "line: {ln}");
            (l, r)
        })
        .collect();

    let mut n_fresh_ingredients = 0;
    for i in ingredients {
        for (lo, hi) in &ranges {
            if (*lo..=*hi).contains(&i) {
                n_fresh_ingredients += 1;
                break;
            }
        }
    }
    Ok(n_fresh_ingredients)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day05.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(664, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
