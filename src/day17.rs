use crate::SolveInfo;
use arrayvec::ArrayVec;
use bittle::Bits;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        // part01: part01(input.trim_end())?.to_string(),
        part01: "".to_string(),
        part02: part02(input)?.to_string(),
    })
}

const BOT_GAP: usize = 3;
const DEBUG: bool = false;

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let input = input.trim_end();
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
                _ => panic!(),
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
    let input = input.trim_end();
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
    // let rle = compress_str(input);
    // println!("{:?} = {}", rle, rle.len());
    let mut jets = input.chars().cycle();
    let mut grid = ArrayVec::<u8, 550>::new();
    (0..grid.capacity()).for_each(|_| grid.push(0));
    let mut num_rocks = 0;
    let mut discarded = 0;
    let mut highest = 0;
    // const ROCKS: usize = 1_000_000_000_000;
    const ROCKS: usize = 2022;
    loop {
        // for rock in rocks.into_iter().cycle().take(ROCKS) {
        if num_rocks == ROCKS {
            break;
        }

        let rock = rocks[num_rocks % 5];
        num_rocks += 1;
        if num_rocks % 10_000_000 == 0 {
            println!(
                "rocks={num_rocks} ({:.5}%)",
                num_rocks as f64 / ROCKS as f64
            );
        }
        let mut sprite = rock.1;
        // 5 = 1 (idx offset) + 4 (height of sprite)
        let mut y = if 5 + highest + BOT_GAP > grid.len() {
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
            grid.len() - 1 - highest - BOT_GAP
        } else {
            grid.len() - 1 - highest - BOT_GAP
        };

        // enable to check wierd edge cases
        // if y < 3 {
        //     draw(&grid, sprite, y);
        //     panic!()
        // }

        // if DEBUG {
        //     println!("new rock");
        //     draw(&grid, sprite, y);
        // }

        // OPTIMIZATION: you do not need to check for collisions with rocks for the first 3 moves
        // since we always spawn at least 3 units above the highest rock.
        for _ in 0..3 {
            let jet = unsafe { jets.next().unwrap_unchecked() };
            match jet {
                '<' => {
                    // hits wall if leftmost (7th) bit is 1
                    // * only need to check bottom 2 rows as that is where the max width is
                    if (sprite[2] >> 6) & 1 == 0 && (sprite[3] >> 6) & 1 == 0 {
                        sprite[0] = shl_unchecked(sprite[0]);
                        sprite[1] = shl_unchecked(sprite[1]);
                        sprite[2] = shl_unchecked(sprite[2]);
                        sprite[3] = shl_unchecked(sprite[3]);
                    }
                }
                '>' => {
                    // hits wall if rightmost bit is 1
                    // * only need to check bottom 2 rows as that is where the max width is
                    if sprite[2] & 1 == 0 && sprite[3] & 1 == 0 {
                        sprite[0] >>= 1;
                        sprite[1] >>= 1;
                        sprite[2] >>= 1;
                        sprite[3] >>= 1;
                    }
                }
                _ => panic!(),
            }
            y += 1;
        }

        loop {
            // move left / right, if necessary
            let jet = unsafe { jets.next().unwrap_unchecked() };
            match jet {
                '<' => {
                    // if DEBUG {
                    //     println!("move left");
                    // }

                    // hits wall if leftmost (7th) bit is 1
                    // * only need to check bottom 2 rows as that is where the max width is
                    if (sprite[2] >> 6) & 1 == 0
                        && (sprite[3] >> 6) & 1 == 0
                        && grid[y] & shl_unchecked(sprite[3]) == 0
                        && grid[y - 1] & shl_unchecked(sprite[2]) == 0
                        && grid[y - 2] & shl_unchecked(sprite[1]) == 0
                        && grid[y - 3] & shl_unchecked(sprite[0]) == 0
                    {
                        sprite[0] = shl_unchecked(sprite[0]);
                        sprite[1] = shl_unchecked(sprite[1]);
                        sprite[2] = shl_unchecked(sprite[2]);
                        sprite[3] = shl_unchecked(sprite[3]);
                    }
                }
                '>' => {
                    // if DEBUG {
                    //     println!("move right");
                    // }

                    // hits wall if rightmost bit is 1
                    // * only need to check bottom 2 rows as that is where the max width is
                    if sprite[2] & 1 == 0
                        && sprite[3] & 1 == 0
                        // check if hit rock
                        && grid[y] & sprite[3] >> 1 == 0
                        && grid[y - 1] & sprite[2] >> 1 == 0
                        && grid[y - 2] & sprite[1] >> 1 == 0
                        && grid[y - 3] & sprite[0] >> 1 == 0
                    {
                        sprite[0] >>= 1;
                        sprite[1] >>= 1;
                        sprite[2] >>= 1;
                        sprite[3] >>= 1;
                    }
                }
                _ => panic!(),
            }

            // if DEBUG {
            //     draw(&grid, sprite, y);

            //     println!("move down");
            // }
            if y == grid.len() - 1 || (sprite[3] & grid[y + 1] != 0 || sprite[2] & grid[y] != 0) {
                grid[y - 3] |= sprite[0];
                grid[y - 2] |= sprite[1];
                grid[y - 1] |= sprite[2];
                grid[y] |= sprite[3];
                highest = highest.max(grid.len() - y - 1 + rock.0);
                // if DEBUG {
                //     println!("came to a rest; highest={highest}");
                //     draw(&grid, [0; 4], y);
                // }
                break;
            }
            y += 1;
            // if DEBUG {
            //     draw(&grid, sprite, y);
            // }
        }
    }
    Ok(highest + discarded)
}

#[inline]
fn shl_unchecked(line: u8) -> u8 {
    line << 1 & 0b01111111u8
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

fn compress_str(input: &str) -> Vec<(char, u32)> {
    let mut chars = input.chars().peekable();
    let mut count = 0;
    let mut rle = Vec::new();
    while let Some(ch) = chars.next() {
        count += 1;
        if chars.peek() != Some(&ch) {
            rle.push((ch, count));
            count = 0;
        }
    }
    rle
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

    #[test]
    fn test_compress_str() {
        assert_eq!(vec![('<', 1)], compress_str("<"));
        assert_eq!(vec![('<', 1), ('>', 1)], compress_str("<>"));
        assert_eq!(vec![('<', 2), ('>', 1)], compress_str("<<>"));
        assert_eq!(vec![('<', 2), ('>', 1), ('<', 2)], compress_str("<<><<"));
    }
}
