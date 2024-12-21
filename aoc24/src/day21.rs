use arrayvec::ArrayVec;
use fxhash::FxHashMap;
use itertools::Itertools;

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
    Ok(solve(input, 25))
}

// the general idea is to start from the human operator and calculate the costs for the next keypad
// to push a specific button. we can then repeat that for every keypad until the numerical keypad.
// this approach prevents us from calculating any recursive paths as we only need to know the
// projected cost to make a move between two points at a given depth.
fn solve(input: &str, num_robots: usize) -> usize {
    let codes: Vec<&str> = input.lines().collect();

    let dir_paths = build_paths(
        FxHashMap::from_iter([
            ('^', (0isize, 1isize)),
            ('A', (0, 2)),
            ('<', (1, 0)),
            ('v', (1, 1)),
            ('>', (1, 2)),
        ]),
        (0, 0),
    );

    let numerical_paths = build_paths(
        FxHashMap::from_iter([
            ('7', (0isize, 0isize)),
            ('8', (0, 1)),
            ('9', (0, 2)),
            ('4', (1, 0)),
            ('5', (1, 1)),
            ('6', (1, 2)),
            ('1', (2, 0)),
            ('2', (2, 1)),
            ('3', (2, 2)),
            ('0', (3, 1)),
            ('A', (3, 2)),
        ]),
        (0, 3),
    );

    // calculate cost for human to direct robot1
    let mut costs: FxHashMap<&(char, char), usize> = dir_paths
        .iter()
        .map(|(pair, paths)| {
            // +1 b/c we need to push the button
            (pair, paths.iter().map(|path| path.len()).min().unwrap() + 1)
        })
        .collect();

    // if we are the last robot, we control the numerical pad instead of the directional pad
    let sequence = std::iter::repeat(&dir_paths)
        .take(num_robots - 1)
        .chain([&numerical_paths]);

    // calulate cost for robotN to direct robotN+1
    for grid_paths in sequence {
        costs = grid_paths
            .iter()
            .map(|(pair, paths)| {
                // traverse every way to get between pair of points and take the cheapest path
                let min_cost = paths
                    .iter()
                    .map(|path| {
                        // we know the robot always starts (problem description) and ends (needs to push
                        // button) on A, so we calculate the cost of each of it's moves
                        let positions = ['A'].into_iter().chain(path.chars()).chain(['A']);

                        // the cheapest path from any two points is the sum of the cheapest paths
                        // between all pairs of points on the path
                        // e.g. cost(A,v) == cost(A,>) + cost(>,v)
                        positions
                            .into_iter()
                            .tuple_windows()
                            .map(|(start, end)| costs[&(start, end)])
                            .sum()
                    })
                    .min()
                    .unwrap();
                (pair, min_cost)
            })
            .collect();
    }

    let mut ans = 0;
    for code in codes {
        let positions = ['A'].into_iter().chain(code.chars());

        let min_cost: usize = positions
            .into_iter()
            .tuple_windows()
            .map(|(start, end)| costs[&(start, end)])
            .sum();

        let complexity = code[..3].parse::<usize>().unwrap() * min_cost;
        ans += complexity;
    }
    ans
}

// calculates all possible paths from A -> B within the grid
fn build_paths(
    grid: FxHashMap<char, (isize, isize)>,
    invalid: (isize, isize),
) -> FxHashMap<(char, char), ArrayVec<String, 2>> {
    let mut final_paths = FxHashMap::default();
    for pair in grid.keys().permutations(2) {
        let (ay, ax) = grid[pair[0]];
        let (by, bx) = grid[pair[1]];
        let horiz = if ax - bx < 0 {
            ">".repeat(ax.abs_diff(bx))
        } else {
            "<".repeat(ax.abs_diff(bx))
        };
        let vert = if ay - by < 0 {
            "v".repeat(ay.abs_diff(by))
        } else {
            "^".repeat(ay.abs_diff(by))
        };

        // if keys are vertically or horizontally aligned then only one path is possible
        let mut possible_paths = ArrayVec::<String, 2>::new();
        let apath = format!("{horiz}{vert}");
        let bpath = format!("{vert}{horiz}");
        if bpath != apath {
            possible_paths.push(bpath);
        }
        possible_paths.push(apath);

        let paths = possible_paths
            .into_iter()
            .filter(|path| {
                let (mut x, mut y) = (ax, ay);
                for symbol in path.chars() {
                    match symbol {
                        '^' => y -= 1,
                        'v' => y += 1,
                        '<' => x -= 1,
                        '>' => x += 1,
                        _ => unreachable!(),
                    }
                    if (x, y) == invalid {
                        return false;
                    }
                }
                true
            })
            .collect();

        final_paths.insert((*pair[0], *pair[1]), paths);
    }
    for sym in grid.keys() {
        final_paths.insert((*sym, *sym), ArrayVec::from_iter(["".to_string()]));
    }
    final_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day21.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(237342, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(294585598101704, ans);
    }
}
