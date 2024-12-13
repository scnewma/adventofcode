use num::ToPrimitive;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

// equation solving help: https://www.mathpapa.com/equation-solver/
//
// py = a*ay + b*by
//  b = (py-a*ay)/by
//
// px = a*ax + b*bx
//  px = a*ax + ((py-a*ay)/by)*bx
//  multiply by by:
//    px*by = a*by*ax - a*bx*ay + bx*py
//  "- bx*py":
//    px*by-bx*py = a*by*ax - a*bx*ay
//  factor out a:
//    px*by-bx*py = a(by*ax - bx*ay)
//  solve:
//    (px*by-bx*py) / (by*ax - bx*ay)  = a

pub fn part01(input: &str) -> anyhow::Result<usize> {
    Ok(solve(input, 0))
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    Ok(solve(input, 10000000000000))
}

fn solve(input: &str, scale: usize) -> usize {
    let games = parse_input(input).into_iter().map(|game| Game {
        px: game.px + scale as f64,
        py: game.py + scale as f64,
        ..game
    });

    let mut sum = 0;
    for Game {
        ax,
        ay,
        bx,
        by,
        px,
        py,
    } in games
    {
        let a = ((px * by - bx * py) / (by * ax - bx * ay)).round();
        let b = ((py - a * ay) / by).round();

        if a * ax + b * bx == px && a * ay + b * by == py {
            sum += a.to_usize().unwrap() * 3 + b.to_usize().unwrap();
        }
    }
    sum
}

#[derive(Debug)]
struct Game {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    px: f64,
    py: f64,
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    let extract_vector = |s: &str| -> (f64, f64) {
        let (_, xy) = s.split_once(": ").unwrap();
        let (x, y) = xy.split_once(", ").unwrap();
        (x[2..].parse().unwrap(), y[2..].parse().unwrap())
    };

    let extract_prize = |s: &str| -> (f64, f64) {
        let (_, pos) = s.split_once(": ").unwrap();
        let (x, y) = pos.split_once(", ").unwrap();
        (x[2..].parse().unwrap(), y[2..].parse().unwrap())
    };

    for section in input.split("\n\n") {
        let mut lines = section.lines();
        let button_a = extract_vector(lines.next().unwrap());
        let button_b = extract_vector(lines.next().unwrap());
        let prize = extract_prize(lines.next().unwrap());
        games.push(Game {
            ax: button_a.0,
            ay: button_a.1,
            bx: button_b.0,
            by: button_b.1,
            px: prize.0,
            py: prize.1,
        });
    }
    games
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day13.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(30973, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(95688837203288, ans);
    }
}
