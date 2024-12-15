use fxhash::FxHashMap;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (mut grid, mut pos, moves) = parse_input(input);

    for m in moves.into_iter() {
        match m {
            '<' => pos = do_move(&mut grid, pos, 0, -1),
            '>' => pos = do_move(&mut grid, pos, 0, 1),
            '^' => pos = do_move(&mut grid, pos, -1, 0),
            'v' => pos = do_move(&mut grid, pos, 1, 0),
            _ => unreachable!(),
        }
    }
    let mut sum = 0;
    for (pos, v) in grid {
        if v != 'O' {
            continue;
        }
        sum += pos.0 * 100 + pos.1;
    }
    Ok(sum as usize)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let (grid, mut pos, moves) = parse_input(input);

    let mut expanded = FxHashMap::default();
    for ((row, col), v) in grid {
        let (l, r) = match v {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '.' => ('.', '.'),
            _ => unreachable!(),
        };
        expanded.insert((row, col * 2), l);
        expanded.insert((row, col * 2 + 1), r);
    }
    pos = (pos.0, pos.1 * 2);

    for m in moves.into_iter() {
        match m {
            '<' => pos = do_move_lr(&mut expanded, pos, -1),
            '>' => pos = do_move_lr(&mut expanded, pos, 1),
            '^' => pos = do_move_ud(&mut expanded, pos, -1),
            'v' => pos = do_move_ud(&mut expanded, pos, 1),
            _ => unreachable!(),
        }
    }
    let mut sum = 0;
    for (pos, v) in expanded {
        if v != '[' {
            continue;
        }
        sum += pos.0 * 100 + pos.1;
    }
    Ok(sum as usize)
}

fn do_move(grid: &mut FxHashMap<Pos, char>, mut pos: Pos, dr: isize, dc: isize) -> Pos {
    let (mut r, mut c) = pos;

    let (mut nr, mut nc) = pos;
    loop {
        nr += dr;
        nc += dc;
        match grid[&(nr, nc)] {
            '.' => break,
            '#' => return pos,
            _ => (),
        }
    }

    let mut carry = false;
    loop {
        r += dr;
        c += dc;
        match grid[&(r, c)] {
            'O' => {
                if carry {
                    grid.insert((r, c), 'O');
                } else {
                    grid.insert((r, c), '.');
                    pos = (r, c);
                }
                carry = true;
            }
            '.' => {
                if carry {
                    grid.insert((r, c), 'O');
                    carry = false;
                } else {
                    pos = (r, c);
                }
            }
            '#' => break,
            _ => unreachable!(),
        }

        if !carry {
            break;
        }
    }
    pos
}

fn do_move_lr(grid: &mut FxHashMap<Pos, char>, mut pos: Pos, dc: isize) -> Pos {
    let orig_pos = pos;
    let (r, mut c) = pos;

    let mut ngrid = grid.clone();

    let mut carry = None;

    loop {
        c += dc;
        let ch = ngrid[&(r, c)];
        match ch {
            '[' | ']' => {
                if let Some(ch) = carry {
                    ngrid.insert((r, c), ch);
                } else {
                    ngrid.insert((r, c), '.');
                    pos = (r, c);
                }
                carry = Some(ch)
            }
            '.' => {
                if carry.is_some() {
                    ngrid.insert((r, c), carry.unwrap());
                    carry = None;
                } else {
                    pos = (r, c);
                }
            }
            '#' => {
                return orig_pos;
            }
            _ => unreachable!(),
        }

        if carry.is_none() {
            break;
        }
    }
    *grid = ngrid;
    pos
}

fn do_move_ud(grid: &mut FxHashMap<Pos, char>, pos: Pos, dr: isize) -> Pos {
    let (mut r, c) = pos;

    let mut ngrid = grid.clone();

    let mut carry = (c, c);

    loop {
        r += dr;

        if (carry.0..=carry.1).any(|c| grid[&(r, c)] == '#') {
            return pos;
        }

        (carry.0..=carry.1).for_each(|c| {
            ngrid.insert((r, c), grid[&(r - dr, c)]);
        });

        let open = (carry.0..=carry.1).all(|c| grid[&(r, c)] == '.');
        if open {
            break;
        }

        // adjust range to fit blocks we're currently pushing
        if grid[&(r, carry.0)] == ']' {
            ngrid.insert((r, carry.0 - 1), '.');
            carry.0 -= 1;
        }
        if grid[&(r, carry.1)] == '[' {
            ngrid.insert((r, carry.1 + 1), '.');
            carry.1 += 1;
        }
        while grid[&(r, carry.0)] == '.' {
            carry.0 += 1;
        }
        while grid[&(r, carry.1)] == '.' {
            carry.1 -= 1;
        }
    }
    *grid = ngrid;
    (pos.0 + dr, pos.1)
}

type Pos = (isize, isize);

fn parse_input(input: &str) -> (FxHashMap<Pos, char>, Pos, Vec<char>) {
    let (g, m) = input.split_once("\n\n").unwrap();

    let mut grid = FxHashMap::default();
    let mut pos = None;
    for (r, line) in g.lines().enumerate() {
        for (c, mut ch) in line.char_indices() {
            if ch == '@' {
                ch = '.';
                pos = Some((r as isize, c as isize))
            }
            grid.insert((r as isize, c as isize), ch);
        }
    }

    let mut moves = Vec::new();
    for line in m.lines() {
        for ch in line.chars() {
            moves.push(ch);
        }
    }

    (grid, pos.unwrap(), moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day15.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(1526673, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1535509, ans);
    }
}
