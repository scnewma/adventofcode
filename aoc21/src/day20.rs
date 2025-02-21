use fxhash::FxHashMap;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(solve(input, 2))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(solve(input, 50))
}

fn solve(input: &str, steps: usize) -> usize {
    // convert # to 1 and . to 0 when parsing input to make code later not need comparisons
    let (enhancement_algo, imgstr) = input.split_once("\n\n").unwrap();
    let enhancement_algo: Vec<usize> = enhancement_algo
        .chars()
        .map(|ch| if ch == '#' { 1 } else { 0 })
        .collect();
    let mut img = FxHashMap::default();
    for (r, line) in imgstr.lines().enumerate() {
        for (c, ch) in line.char_indices() {
            let n: usize = if ch == '#' { 1 } else { 0 };
            img.insert((r as isize, c as isize), n);
        }
    }

    // the input maps 0 => 1 and 511 => 0 so the infinite space changes state every turn
    // the example doesn't do this, but the input does...
    let should_alternate = enhancement_algo[0] == 1 && enhancement_algo[511] == 0;

    let min_r = img.keys().map(|(r, _)| *r).min().unwrap();
    let min_c = img.keys().map(|(_, c)| *c).min().unwrap();
    let max_r = img.keys().map(|(r, _)| *r).max().unwrap();
    let max_c = img.keys().map(|(_, c)| *c).max().unwrap();

    let mut newimg = img.clone();

    for step in 0..steps {
        let inf_state = if !should_alternate || step % 2 == 0 {
            0
        } else {
            1
        };

        // the amount per side that the grid has increased since the original image
        let size_inc = (step + 1) as isize;

        for r in min_r - size_inc..=max_r + size_inc {
            for c in min_c - size_inc..=max_c + size_inc {
                let index = (img.get(&(r - 1, c - 1)).unwrap_or(&inf_state) << 8) | (img.get(&(r - 1, c)).unwrap_or(&inf_state) << 7) | (img.get(&(r - 1, c + 1)).unwrap_or(&inf_state) << 6) | (img.get(&(r, c - 1)).unwrap_or(&inf_state) << 5) | (img.get(&(r, c)).unwrap_or(&inf_state) << 4) | (img.get(&(r, c + 1)).unwrap_or(&inf_state) << 3) | (img.get(&(r + 1, c - 1)).unwrap_or(&inf_state) << 2) | (img.get(&(r + 1, c)).unwrap_or(&inf_state) << 1)
                    | img.get(&(r + 1, c + 1)).unwrap_or(&inf_state);

                newimg.insert((r, c), enhancement_algo[index]);
            }
        }

        // reuse the same 2 hashmaps by alternating uses to prevent need to allocate a new one
        (img, newimg) = (newimg, img);
    }

    img.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day20.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(5437, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(19340, ans);
    }
}
