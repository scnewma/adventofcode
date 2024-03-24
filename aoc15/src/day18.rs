use itertools::iproduct;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

const STEPS: usize = 100;

const DELTAS: [(isize, isize); 8] = [
    (-1, -1), // NW
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, 1),   // E
    (1, 1),   // SE
    (1, 0),   // S
    (1, -1),  // SW
    (0, -1),  // W
];

pub fn part01(input: &str) -> usize {
    simulate(&mut parse_input(input), STEPS, |_, _| true)
}

pub fn part02(input: &str) -> usize {
    let mut grid = parse_input(input);
    let size = grid.len();
    let corners = [(0, 0), (0, size - 1), (size - 1, 0), (size - 1, size - 1)];
    corners.iter().for_each(|(r, c)| grid[*r][*c] = true);
    simulate(&mut grid, STEPS, |r, c| !corners.contains(&(r, c)))
}

fn simulate<F>(grid: &mut Vec<Vec<bool>>, steps: usize, should_update: F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    let size = grid.len();

    macro_rules! bounded_checked_add {
        ($lhs:expr, $rhs:expr) => {
            $lhs.checked_add_signed($rhs)
                .and_then(|i| if i >= size { None } else { Some(i) })
        };
    }

    for _ in 1..=steps {
        let mut updated = grid.clone();

        for (r, c) in iproduct!(0..size, 0..size) {
            if !should_update(r, c) {
                continue;
            }

            let mut neighbors_on = 0;
            for (dr, dc) in DELTAS {
                match (bounded_checked_add!(r, dr), bounded_checked_add!(c, dc)) {
                    (Some(r), Some(c)) => {
                        if grid[r][c] {
                            neighbors_on += 1;
                        }
                    }
                    _ => continue,
                }
            }

            updated[r][c] = neighbors_on == 3 || (grid[r][c] && neighbors_on == 2)
        }

        *grid = updated;
    }

    iproduct!(0..size, 0..size).fold(0, |acc, (r, c)| acc + if grid[r][c] { 1 } else { 0 })
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}
