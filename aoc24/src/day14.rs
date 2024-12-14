use fxhash::{FxHashMap, FxHashSet};

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut robots = parse_input(input);
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.px = (robot.px + robot.vx) % WIDTH;
            if robot.px < 0 {
                robot.px = WIDTH - robot.px.abs();
            }

            robot.py = (robot.py + robot.vy) % HEIGHT;
            if robot.py < 0 {
                robot.py = HEIGHT - robot.py.abs();
            }
        }
    }

    let midx = WIDTH / 2;
    let midy = HEIGHT / 2;

    let mut quadrants = [0; 4];
    for Robot { px, py, .. } in robots {
        if px == midx || py == midy {
            continue;
        }
        let quadrant = if px < midx && py < midy {
            0
        } else if px > midx && py < midy {
            1
        } else if px < midx && py > midy {
            2
        } else if px > midx && py > midy {
            3
        } else {
            unreachable!();
        };

        quadrants[quadrant] += 1;
    }

    Ok(quadrants.into_iter().product())
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut robots = parse_input(input);

    // we determine if a xmas tree exists by counting the "grouped" robots. if a majority of
    // the robots are in a group, we assume it's a tree
    //
    // needed to relax the meaning of "majority" a bit, turns out the picture of the christmas
    // tree includes the border of the picture, which is not connected to the tree. example
    // (not to scale):
    //
    // #########
    // #       #
    // #   #   #
    // #  ###  #
    // # ##### #
    // #   #   #
    // #       #
    // #########
    //
    // Note: turns out the solution has all of the robots non-overlapping so there is a much more
    // efficient solution, but I'm not implementing that here as it seems like a hack.
    let min_robots = robots.len() as f64 * 0.45;

    let mut grid = FxHashMap::<(isize, isize), usize>::default();
    let mut visited = FxHashSet::default();
    for step in 1.. {
        grid.clear();

        for robot in robots.iter_mut() {
            robot.px = (robot.px + robot.vx) % WIDTH;
            if robot.px < 0 {
                robot.px = WIDTH - robot.px.abs();
            }

            robot.py = (robot.py + robot.vy) % HEIGHT;
            if robot.py < 0 {
                robot.py = HEIGHT - robot.py.abs();
            }

            *grid.entry((robot.px, robot.py)).or_default() += 1;
        }

        visited.clear();
        for &(x, y) in grid.keys() {
            if connected_robots(&grid, x, y, &mut visited) >= min_robots as usize {
                return Ok(step);
            }
        }
    }

    unreachable!("no solution")
}

fn connected_robots(
    grid: &FxHashMap<(isize, isize), usize>,
    x: isize,
    y: isize,
    visited: &mut FxHashSet<(isize, isize)>,
) -> usize {
    if x < 0
        || x >= WIDTH
        || y < 0
        || y >= HEIGHT
        || grid.get(&(x, y)).is_none()
        || !visited.insert((x, y))
    {
        return 0;
    }
    grid[&(x, y)]
        + connected_robots(grid, x + 1, y, visited)
        + connected_robots(grid, x - 1, y, visited)
        + connected_robots(grid, x, y + 1, visited)
        + connected_robots(grid, x, y - 1, visited)
}

#[derive(Debug)]
struct Robot {
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let (p, v) = line.split_once(' ').unwrap();
        let (px, py) = p[2..].split_once(',').unwrap();
        let (vx, vy) = v[2..].split_once(',').unwrap();
        let vx = vx.parse().unwrap();
        assert!(vx < WIDTH);
        let vy = vy.parse().unwrap();
        assert!(vy < HEIGHT);
        robots.push(Robot {
            px: px.parse().unwrap(),
            py: py.parse().unwrap(),
            vx,
            vy,
        })
    }
    robots
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day14.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(232253028, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(8179, ans);
    }
}
