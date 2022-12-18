use std::collections::HashMap;

use crate::SolveInfo;
use arrayvec::ArrayVec;
use bittle::Bits;

pub fn run(input: &str, _: bool) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input.trim_end())?.to_string(),
        // part01: "".to_string(),
        part02: part02(input)?.to_string(),
    })
}

const BOT_GAP: usize = 3;
const DEBUG: bool = false;

pub fn part01(input: &str) -> anyhow::Result<usize> {
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
    let mut jets = input.chars().cycle();
    let mut grid = ArrayVec::<u8, 50000>::new();
    (0..grid.capacity()).for_each(|_| grid.push(0));
    let mut num_rocks = 0;
    let mut highest = 0;
    const ROCKS: usize = 2022;
    loop {
        if num_rocks == ROCKS {
            break Ok(highest);
        }

        let rock = rocks[num_rocks % 5];
        num_rocks += 1;

        let mut sprite = rock.1;
        let mut y = grid.len() - 1 - highest - BOT_GAP;

        loop {
            // move left / right, if necessary
            let jet = unsafe { jets.next().unwrap_unchecked() };
            match jet {
                '<' => {
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

            // check for if rock settles here
            if y == grid.len() - 1 || (sprite[3] & grid[y + 1] != 0 || sprite[2] & grid[y] != 0) {
                grid[y - 3] |= sprite[0];
                grid[y - 2] |= sprite[1];
                grid[y - 1] |= sprite[2];
                grid[y] |= sprite[3];
                highest = highest.max(grid.len() - y - 1 + rock.0);
                break;
            }

            y += 1;
        }
    }
}

pub fn part02(input: &str) -> anyhow::Result<u64> {
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
    let mut jets = input.chars().cycle();
    let mut grid = ArrayVec::<u8, 50000>::new();
    (0..grid.capacity()).for_each(|_| grid.push(0));
    let mut num_rocks = 0;
    let mut highest = 0;
    // const ROCKS: usize = 1_000_000_000_000;
    // const ROCKS: usize = 2022;
    const ROCKS: usize = 20000;
    loop {
        if num_rocks == ROCKS {
            break;
        }

        let rock = rocks[num_rocks % 5];
        num_rocks += 1;

        let mut sprite = rock.1;
        let mut y = grid.len() - 1 - highest - BOT_GAP;

        loop {
            // move left / right, if necessary
            let jet = unsafe { jets.next().unwrap_unchecked() };
            match jet {
                '<' => {
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

            // check for if rock settles here
            if y == grid.len() - 1 || (sprite[3] & grid[y + 1] != 0 || sprite[2] & grid[y] != 0) {
                grid[y - 3] |= sprite[0];
                grid[y - 2] |= sprite[1];
                grid[y - 1] |= sprite[2];
                grid[y] |= sprite[3];
                highest = highest.max(grid.len() - y - 1 + rock.0);
                break;
            }

            y += 1;
        }
    }

    let (pattern_length, offset) = find_pattern(&grid);
    // println!("pattern_length={pattern_length} offset={offset}");
    let pattern = &grid[grid.len() - pattern_length - offset..grid.len() - offset];
    let offset_pattern = &grid[grid.len() - offset..];
    // draw_grid(offset_pattern);
    // draw_grid(&grid);

    let mut jets = input.chars().cycle();
    let mut grid = ArrayVec::<u8, 50000>::new();
    (0..grid.capacity()).for_each(|_| grid.push(0));
    let mut num_rocks = 0;
    let mut highest = 0;
    let mut offset_rocks = None;
    let mut grid_heights = HashMap::new();
    loop {
        if num_rocks == ROCKS {
            // println!("{}", grid_heights.get(&2022).unwrap());
            panic!("did not find answer");
        }
        grid_heights.insert(num_rocks, highest);

        if offset_rocks == None && &grid[grid.len() - offset..] == offset_pattern {
            offset_rocks = Some(num_rocks);
        }
        if &grid[grid.len() - pattern_length - offset..grid.len() - offset] == pattern {
            if offset_rocks.is_none() {
                panic!("did not find offset");
            }
            let pattern_rocks = num_rocks - offset_rocks.unwrap();

            // 583090379
            // 583090378
            // 583090377
            // 583090378
            // 447 + ((1000000000000-285)/1715 * 2574) + 2202
            // offset -> pattern = 2000
            // pattern1 -> pattern2 = 1715
            // pattern2 -> pattern3 = 1715

            // 1287000000894 - too low
            // 1500874636011 - too high
            // 1500874635594

            println!("pattern_height={pattern_length}");
            println!("pattern_length_rocks={pattern_rocks}");
            println!("offset_height={offset}");
            println!("offset_length_rocks={}", offset_rocks.unwrap());

            // sample
            // got:  1060000000050
            // want: 1514285714288
            //       1500874635621
            //       1500874636034

            const TOTAL_ROCKS: u64 = 1000000000000;
            let rocks = TOTAL_ROCKS - offset_rocks.unwrap() as u64;
            let pattern_repeat_count = rocks / pattern_rocks as u64;
            let remaining = rocks % pattern_rocks as u64 + offset_rocks.unwrap() as u64;
            let leftover = *grid_heights.get(&(remaining as usize)).unwrap() as u64 - offset as u64;
            println!("remaining = {} leftover = {}", remaining, leftover);
            break Ok(offset as u64 + pattern_length as u64 * pattern_repeat_count + leftover);
        }

        let rock = rocks[num_rocks % 5];
        num_rocks += 1;

        let mut sprite = rock.1;
        let mut y = grid.len() - 1 - highest - BOT_GAP;

        loop {
            // move left / right, if necessary
            let jet = unsafe { jets.next().unwrap_unchecked() };
            match jet {
                '<' => {
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

            // check for if rock settles here
            if y == grid.len() - 1 || (sprite[3] & grid[y + 1] != 0 || sprite[2] & grid[y] != 0) {
                grid[y - 3] |= sprite[0];
                grid[y - 2] |= sprite[1];
                grid[y - 1] |= sprite[2];
                grid[y] |= sprite[3];
                highest = highest.max(grid.len() - y - 1 + rock.0);
                break;
            }

            y += 1;
        }
    }
}

// even though i found this, it doesn't tell me how many rocks were dropped to start this
// pattern...
fn find_pattern(grid: &[u8]) -> (usize, usize) {
    for pattern_length in 1..10000 {
        for offset in 0..pattern_length {
            // println!("pattern_length={pattern_length} offset={offset}");
            let bot = &grid[grid.len() - pattern_length - offset..grid.len() - offset];
            let top = &grid
                [(grid.len() - pattern_length * 2) - offset..grid.len() - pattern_length - offset];
            let toptop = &grid[(grid.len() - pattern_length * 3) - offset
                ..grid.len() - pattern_length * 2 - offset];
            // println!("bottom:");
            // println!("top:");
            // draw_grid(&top);
            if bot == top && bot == toptop {
                println!("found pattern of length {pattern_length} @ {offset}");
                // draw_grid(&bot);
                return (pattern_length, offset);
            }
        }
    }
    panic!("no pattern found")
}

// // OPTIMIZATION: you do not need to check for collisions with rocks for the first 3 moves
// // since we always spawn at least 3 units above the highest rock.
// for _ in 0..3 {
//     let jet = unsafe { jets.next().unwrap_unchecked() };
//     match jet {
//         '<' => {
//             // hits wall if leftmost (7th) bit is 1
//             // * only need to check bottom 2 rows as that is where the max width is
//             if (sprite[2] >> 6) & 1 == 0 && (sprite[3] >> 6) & 1 == 0 {
//                 sprite[0] = shl_unchecked(sprite[0]);
//                 sprite[1] = shl_unchecked(sprite[1]);
//                 sprite[2] = shl_unchecked(sprite[2]);
//                 sprite[3] = shl_unchecked(sprite[3]);
//             }
//         }
//         '>' => {
//             // hits wall if rightmost bit is 1
//             // * only need to check bottom 2 rows as that is where the max width is
//             if sprite[2] & 1 == 0 && sprite[3] & 1 == 0 {
//                 sprite[0] >>= 1;
//                 sprite[1] >>= 1;
//                 sprite[2] >>= 1;
//                 sprite[3] >>= 1;
//             }
//         }
//         _ => panic!(),
//     }
//     y += 1;
// }

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

fn draw_grid(grid: &[u8]) {
    for y in 0..grid.len() {
        for x in (0..7).rev() {
            let cell = match grid[y].test_bit(x) {
                true => "#",
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
        assert_eq!(1514285714288, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(1500874635587, ans);
    }

    #[test]
    fn test_compress_str() {
        assert_eq!(vec![('<', 1)], compress_str("<"));
        assert_eq!(vec![('<', 1), ('>', 1)], compress_str("<>"));
        assert_eq!(vec![('<', 2), ('>', 1)], compress_str("<<>"));
        assert_eq!(vec![('<', 2), ('>', 1), ('<', 2)], compress_str("<<><<"));
    }
}
