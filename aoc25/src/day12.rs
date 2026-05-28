use bittle::BitsMut;

const N_SHAPES: usize = 6;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let (shapes, regions) = parse_input(input);
    let shape_areas = shapes.map(|shape| shape.area());

    Ok(regions
        .iter()
        .filter(|region| region.required_area(&shape_areas) <= region.area())
        .count())
}

pub fn part02(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

fn parse_input(input: &str) -> ([Shape; N_SHAPES], Vec<Region>) {
    let mut sections = input.split("\n\n");

    let shapes = std::array::from_fn(|_| {
        let section = sections.next().unwrap();

        let mut cells = 0;
        for (r, row) in section.lines().skip(1).enumerate() {
            for (c, cell) in row.chars().enumerate() {
                if cell == '#' {
                    cells.set_bit((r * 3 + c) as u32);
                }
            }
        }

        Shape(cells)
    });

    let regions = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            let (width, height) = lhs.split_once('x').unwrap();
            let shapes_wanted = rhs
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            Region {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                shapes_wanted,
            }
        })
        .collect();

    (shapes, regions)
}

#[derive(Debug, Copy, Clone)]
struct Shape(u16);

impl Shape {
    fn area(self) -> usize {
        self.0.count_ones() as usize
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shapes_wanted: [usize; N_SHAPES],
}

impl Region {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn required_area(&self, shape_areas: &[usize; N_SHAPES]) -> usize {
        self.shapes_wanted
            .iter()
            .zip(shape_areas)
            .map(|(count, area)| count * area)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day12.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(485, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(0, ans);
    }
}
