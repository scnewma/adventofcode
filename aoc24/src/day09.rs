use std::collections::VecDeque;

use anyhow::Context;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut blocks = parse_input(input);

    let mut defrag = Vec::new();
    while let Some(block) = blocks.pop_front() {
        match block {
            Block::File(..) => defrag.push(block),
            Block::Free(mut free_space) => {
                while free_space > 0 && !blocks.is_empty() {
                    if let Some(last_block) = blocks.pop_back() {
                        if let Block::File(id, size) = last_block {
                            let rem = size.saturating_sub(free_space);
                            let space_taken = free_space.min(size);
                            if rem > 0 {
                                blocks.push_back(Block::File(id, rem));
                            }
                            defrag.push(Block::File(id, space_taken));
                            free_space -= space_taken;
                        }
                    }
                }
            }
        }
    }

    Ok(checksum(&defrag))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let blocks = parse_input(input);

    let mut defrag: Vec<Block> = blocks.into_iter().collect();
    let mut right = defrag.len() - 1;
    while right > 0 {
        let (id, size) = match defrag[right] {
            Block::File(id, size) => (id, size),
            Block::Free(..) => {
                right -= 1;
                continue;
            }
        };

        for left in 0..right {
            if let Block::Free(free_space) = defrag[left] {
                if free_space >= size {
                    defrag[left] = Block::File(id, size);
                    defrag[right] = Block::Free(size);
                    if free_space - size > 0 {
                        defrag.insert(left + 1, Block::Free(free_space - size));
                    }
                    break;
                }
            }
        }
        right -= 1;
    }

    Ok(checksum(&defrag))
}

fn checksum(blocks: &[Block]) -> usize {
    let mut checksum = 0;
    let mut pos = 0;
    for block in blocks {
        match block {
            Block::File(id, size) => {
                (0..*size).for_each(|_| {
                    checksum += pos * id;
                    pos += 1;
                });
            }
            Block::Free(free_space) => pos += *free_space as usize,
        }
    }
    checksum
}

fn parse_input(input: &str) -> VecDeque<Block> {
    let mut is_file = true;
    let mut id = 0;
    let mut blocks = VecDeque::new();
    for ch in input.trim().chars() {
        let n = ch
            .to_digit(10)
            .with_context(|| format!("'{ch}' is not a digit"))
            .unwrap();
        if is_file {
            blocks.push_back(Block::File(id, n));
            id += 1;
        } else {
            blocks.push_back(Block::Free(n));
        }
        is_file = !is_file;
    }
    blocks
}

#[derive(Debug)]
enum Block {
    File(usize, u32),
    Free(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day09.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(6353658451014, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(6382582136592, ans);
    }
}
