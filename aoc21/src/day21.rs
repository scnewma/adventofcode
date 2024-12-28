use fxhash::FxHashMap;
use itertools::iproduct;

pub fn run(input: &str) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input)?.to_string(),
        part02: part02(input)?.to_string(),
    })
}

pub fn part01(input: &str) -> anyhow::Result<usize> {
    let mut positions = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap());

    let mut players = [
        Player::new(positions.next().unwrap()),
        Player::new(positions.next().unwrap()),
    ];

    let mut rolls = 0;

    for pid in (0..2).cycle() {
        let triple_roll = 3 * rolls + 6;
        players[pid] = players[pid].advance(triple_roll);
        rolls += 3;
        if players[pid].score >= 1000 {
            break;
        }
    }

    let min_score = players.iter().map(|p| p.score).min().unwrap();
    Ok(min_score * rolls)
}

pub fn part02(input: &str) -> anyhow::Result<usize> {
    let mut positions = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap());

    // there are 27 different universes generated after 3 dice rolls. this precomputes those
    // possible rolls and how many universes generated the same sum of 3 dice rolls
    let mut dice_roll_universes = FxHashMap::<usize, usize>::default();
    for (i, j, k) in iproduct!(1..=3, 1..=3, 1..=3) {
        *dice_roll_universes.entry(i + j + k).or_default() += 1;
    }

    fn inner(
        dice_roll_universes: &FxHashMap<usize, usize>,
        cache: &mut FxHashMap<(Player, Player), (usize, usize)>,
        player1: Player,
        player2: Player,
    ) -> (usize, usize) {
        if let Some(ans) = cache.get(&(player1, player2)) {
            return *ans;
        }

        const WINNING_SCORE: usize = 21;
        if player2.score >= WINNING_SCORE {
            return (0, 1);
        }

        let (mut p1wins, mut p2wins) = (0, 0);
        for (roll, universes_with_roll) in dice_roll_universes {
            let (wins2, wins1) = inner(dice_roll_universes, cache, player2, player1.advance(*roll));
            p1wins += wins1 * universes_with_roll;
            p2wins += wins2 * universes_with_roll;
        }

        cache.insert((player1, player2), (p1wins, p2wins));
        (p1wins, p2wins)
    }

    let (p1wins, p2wins) = inner(
        &dice_roll_universes,
        &mut FxHashMap::default(),
        Player::new(positions.next().unwrap()),
        Player::new(positions.next().unwrap()),
    );
    Ok(p1wins.max(p2wins))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn advance(&self, roll: usize) -> Self {
        let next_position = (self.position + roll - 1) % 10 + 1;
        Self {
            position: next_position,
            score: self.score + next_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day21.input.txt");

    #[test]
    fn test_part_one() {
        let ans = part01(INPUT).unwrap();
        assert_eq!(556206, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part02(INPUT).unwrap();
        assert_eq!(630797200227453, ans);
    }
}
