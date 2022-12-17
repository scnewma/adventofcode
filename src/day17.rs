use crate::SolveInfo;
use arrayvec::ArrayVec;
use bittle::Bits;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        // part01: part01(input)?.to_string(),
        part01: "".to_string(),
        part02: part02(input)?.to_string(),
    })
}

const BOT_GAP: usize = 3;
const DEBUG: bool = false;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let rocks = vec![
        // (height, mask)
        // room is only 7 wide so the leftmost bit MUST always be 1
        // * will need to handle this in the algorithm with an XOR or something
        (1, [0u8, 0u8, 0u8, 0b0011110u8]), // Horizonal Line
        (3, [0u8, 0b0001000u8, 0b0011100u8, 0b0001000u8]), // Plus
        (3, [0u8, 0b0000100u8, 0b0000100u8, 0b0011100u8]), // J
        (4, [0b0010000u8, 0b0010000u8, 0b0010000u8, 0b0010000u8]), // Vertical Line
        (2, [0u8, 0u8, 0b0011000u8, 0b0011000u8]), // Box
    ];
    let mut jets = input.chars().cycle();
    let mut grid = vec![0u8; 5000];
    let mut highest = 0;
    for rock in rocks.into_iter().cycle().take(2022) {
        let mut sprite = rock.1;
        let mut y = grid.len() - 1 - highest - BOT_GAP;
        if DEBUG {
            println!("new rock");
            draw(&grid, sprite, y);
        }

        loop {
            // move left / right, if necessary
            let shfn = match jets.next().unwrap() {
                '<' => {
                    if DEBUG {
                        println!("move left");
                    }
                    shl
                }
                '>' => {
                    if DEBUG {
                        println!("move right");
                    }
                    shr
                }
                _ch => continue,
            };
            let mut new_sprite = sprite.clone();
            let mut hit_wall = false;
            for i in 0..4 {
                match shfn(sprite[i]) {
                    Some(line) => {
                        new_sprite[i] = line;
                        let prev = grid[y - (4 - i - 1)] | sprite[i];
                        let shifted = grid[y - (4 - i - 1)] | line;
                        if shifted.count_ones() != prev.count_ones() {
                            hit_wall = true;
                            break;
                        }
                    }
                    None => {
                        hit_wall = true;
                        break;
                    }
                }
            }

            if !hit_wall {
                if DEBUG {
                    println!("success");
                }
                sprite = new_sprite;
            }
            if DEBUG {
                draw(&grid, sprite, y);

                println!("move down");
            }
            if y == grid.len() - 1 || (sprite[3] & grid[y + 1] != 0 || sprite[2] & grid[y] != 0) {
                // put sprite in grid
                for i in 0..4 {
                    grid[y - (4 - i - 1)] |= sprite[i];
                }
                highest = highest.max(grid.len() - y - 1 + rock.0);
                if DEBUG {
                    println!("came to a rest; highest={highest}");
                    draw(&grid, [0; 4], y);
                }
                break;
            }
            y += 1;
            if DEBUG {
                draw(&grid, sprite, y);
            }
        }
    }
    Ok(highest)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let rocks = [
        // (height, mask)
        // room is only 7 wide so the leftmost bit MUST always be 1
        // * will need to handle this in the algorithm with an XOR or something
        (1, [0u8, 0u8, 0u8, 0b0011110u8]), // Horizonal Line
        (3, [0u8, 0b0001000u8, 0b0011100u8, 0b0001000u8]), // Plus
        (3, [0u8, 0b0000100u8, 0b0000100u8, 0b0011100u8]), // J
        (4, [0b0010000u8, 0b0010000u8, 0b0010000u8, 0b0010000u8]), // Vertical Line
        (2, [0u8, 0u8, 0b0011000u8, 0b0011000u8]), // Box
    ];
    let mut jets = input.chars().cycle();
    let mut grid = ArrayVec::<u8, 600>::new();
    (0..grid.capacity()).for_each(|_| grid.push(0));
    let mut num_rocks = 0;
    let mut grid_max = 0;
    let mut discarded = 0;
    let mut highest = 0;
    // const ROCKS: usize = 1000000000000;
    const ROCKS: usize = 2022;
    for rock in rocks.into_iter().cycle().take(ROCKS) {
        num_rocks += 1;
        grid_max = grid_max.max(grid.len());
        if num_rocks % 10_000_000 == 0 {
            println!(
                "rocks={num_rocks} grid_max={grid_max} ({:.5}%)",
                num_rocks as f64 / ROCKS as f64
            );
        }
        let mut sprite = rock.1;
        let mut y = match grid.len().checked_sub(1 + highest + BOT_GAP) {
            Some(v) if v > 3 => v,
            _ => {
                // compress grid
                for idx in 0..grid.len() {
                    if grid[idx] == 0b01111111u8 {
                        let discard = grid.len() - idx;
                        discarded += discard;
                        highest -= discard;

                        for i in (0..grid.len()).rev() {
                            if discard > i {
                                grid[i] = 0;
                            } else {
                                grid[i] = grid[i - discard];
                            }
                        }

                        // we only need to compress from the highest row
                        break;
                    }
                }
                grid.len().checked_sub(1 + highest + BOT_GAP).unwrap()
            }
        };

        // enable to check wierd edge cases
        // if y < 3 {
        //     draw(&grid, sprite, y);
        //     panic!()
        // }

        if DEBUG {
            println!("new rock");
            draw(&grid, sprite, y);
        }

        loop {
            // move left / right, if necessary
            let jet = unsafe { jets.next().unwrap_unchecked() };
            match jet {
                '<' => {
                    if DEBUG {
                        println!("move left");
                    }
                    let new_sprite = [
                        shl_unchecked(sprite[0]),
                        shl_unchecked(sprite[1]),
                        shl_unchecked(sprite[2]),
                        shl_unchecked(sprite[3]),
                    ];

                    // hits wall if leftmost (7th) bit is 1
                    if sprite[0] & 0b01000000u8 == 0b01000000u8
                        || sprite[1] & 0b01000000u8 == 0b01000000u8
                        || sprite[2] & 0b01000000u8 == 0b01000000u8
                        || sprite[3] & 0b01000000u8 == 0b01000000u8
                        // check if hit rock
                        || grid[y - 3] & new_sprite[0] != 0
                        || grid[y - 2] & new_sprite[1] != 0
                        || grid[y - 1] & new_sprite[2] != 0
                        || grid[y] & new_sprite[3] != 0
                    {
                        // hit wall
                    } else {
                        sprite = new_sprite;
                    }
                }
                '>' => {
                    if DEBUG {
                        println!("move right");
                    }
                    let new_sprite = [
                        shr_unchecked(sprite[0]),
                        shr_unchecked(sprite[1]),
                        shr_unchecked(sprite[2]),
                        shr_unchecked(sprite[3]),
                    ];

                    // hits wall if rightmost bit is 1
                    if sprite[0] & 1 == 1
                        || sprite[1] & 1 == 1
                        || sprite[2] & 1 == 1
                        || sprite[3] & 1 == 1
                        // check if hit rock
                        || grid[y - 3] & new_sprite[0] != 0
                        || grid[y - 2] & new_sprite[1] != 0
                        || grid[y - 1] & new_sprite[2] != 0
                        || grid[y] & new_sprite[3] != 0
                    {
                        // hit wall
                    } else {
                        sprite = new_sprite;
                    }
                }
                _ch => continue,
            }

            if DEBUG {
                draw(&grid, sprite, y);

                println!("move down");
            }
            if y == grid.len() - 1 || (sprite[3] & grid[y + 1] != 0 || sprite[2] & grid[y] != 0) {
                grid[y - 3] |= sprite[0];
                grid[y - 2] |= sprite[1];
                grid[y - 1] |= sprite[2];
                grid[y] |= sprite[3];
                highest = highest.max(grid.len() - y - 1 + rock.0);
                if DEBUG {
                    println!("came to a rest; highest={highest}");
                    draw(&grid, [0; 4], y);
                }
                break;
            }
            y += 1;
            if DEBUG {
                draw(&grid, sprite, y);
            }
        }
    }
    Ok(highest + discarded)
}

#[inline]
fn shl_unchecked(line: u8) -> u8 {
    line << 1 & 0b01111111u8
}

#[inline]
fn shr_unchecked(line: u8) -> u8 {
    line >> 1
}

fn shl(line: u8) -> Option<u8> {
    let ones = line.count_ones();
    // let next = (line << 1) ^ 0b10000000u8;
    let next = (line << 1) & 0b01111111u8;
    if ones == next.count_ones() {
        Some(next)
    } else {
        None
    }
}

fn shr(line: u8) -> Option<u8> {
    let ones = line.count_ones();
    let next = line >> 1;
    if ones == next.count_ones() {
        Some(next)
    } else {
        None
    }
}

fn draw(grid: &[u8], sprite: [u8; 4], sprite_y: usize) {
    for y in 0..grid.len() {
        for x in (0..7).rev() {
            let mut sprite_bit = false;
            if (sprite_y - 3..=sprite_y).contains(&y) {
                sprite_bit = sprite[3 - (sprite_y - y)].test_bit(x);
            }
            let cell = match (grid[y].test_bit(x), sprite_bit) {
                (true, true) => panic!("both grid bit and sprite are set at ({x},{y})"),
                (true, false) => "#",
                (false, true) => "@",
                _ => ".",
            };
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = include_str!("../inputs/day17.sample.txt");
    const INPUT: &'static str = include_str!("../inputs/day17.input.txt");

    #[test]
    fn test_part_one_sample() {
        let ans = part01(SAMPLE).unwrap();
        assert_eq!(3068, ans);
    }

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(3059, ans);
    }

    #[test]
    fn test_part_two_sample() {
        let ans = part02(SAMPLE).unwrap();
        assert_eq!(0, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
