use std::str::FromStr;

use arrayvec::ArrayVec;
use itertools::Itertools;

// 1 weapon, 1 armor, 2 rings
const MAX_ITEMS: usize = 4;

pub fn run(input: &str, _: bool) -> anyhow::Result<crate::SolveInfo> {
    Ok(crate::SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> u32 {
    let enemy: Player = input.parse().unwrap();

    kits()
        .into_iter()
        .map(Player::new)
        .filter(|p| simulate_battle(p, &enemy))
        .map(|p| p.cost())
        .min()
        .unwrap()
}

pub fn part02(input: &str) -> u32 {
    let enemy: Player = input.parse().unwrap();

    kits()
        .into_iter()
        .map(Player::new)
        .filter(|p| !simulate_battle(p, &enemy))
        .map(|p| p.cost())
        .max()
        .unwrap()
}

fn kits() -> Vec<ArrayVec<Item, MAX_ITEMS>> {
    let (weapons, armors, rings) = build_items();
    // base kits have 1 weapon, 0 armor, 0 rings
    let mut kits = vec![];
    for weapon in weapons {
        let mut kit = ArrayVec::new();
        kit.push(weapon);
        kits.push(kit);
    }

    // add 1 weapon, 1 armor, 0 rings kits
    let kits2 = kits.clone();
    for armor in armors {
        for kit in &kits2 {
            let mut k = kit.clone();
            k.push(armor.clone());
            kits.push(k);
        }
    }

    // add 1 ring to weapon+armor kits
    let kits2 = kits.clone();
    for ring in &rings {
        for kit in &kits2 {
            let mut k = kit.clone();
            k.push(ring.clone());
            kits.push(k);
        }
    }

    // add 2 rings to weapon+armor kits (kits2 stays same)
    for (r1, r2) in rings.iter().tuple_combinations() {
        for kit in &kits2 {
            let mut k = kit.clone();
            k.push(r1.clone());
            k.push(r2.clone());
            kits.push(k);
        }
    }

    kits
}

// true if player wins
fn simulate_battle(player: &Player, enemy: &Player) -> bool {
    let mut player = player.clone();
    let mut enemy = enemy.clone();
    while player.hp > 0 && enemy.hp > 0 {
        enemy.hp = enemy.hp.saturating_sub(player.attack(&enemy));
        if enemy.hp == 0 {
            return true;
        }
        player.hp = player.hp.saturating_sub(enemy.attack(&player));
    }
    player.hp > 0
}

fn build_items() -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    (
        vec![
            Item::new("Dagger        8     4       0"),
            Item::new("Shortsword   10     5       0"),
            Item::new("Warhammer    25     6       0"),
            Item::new("Longsword    40     7       0"),
            Item::new("Greataxe     74     8       0"),
        ],
        vec![
            Item::new("Leather      13     0       1"),
            Item::new("Chainmail    31     0       2"),
            Item::new("Splintmail   53     0       3"),
            Item::new("Bandedmail   75     0       4"),
            Item::new("Platemail   102     0       5"),
        ],
        vec![
            Item::new("Damage +1    25     1       0"),
            Item::new("Damage +2    50     2       0"),
            Item::new("Damage +3   100     3       0"),
            Item::new("Defense +1   20     0       1"),
            Item::new("Defense +2   40     0       2"),
            Item::new("Defense +3   80     0       3"),
        ],
    )
}

#[derive(Debug, Clone)]
struct Player {
    hp: u32,
    damage: u32,
    armor: u32,
    items: ArrayVec<Item, MAX_ITEMS>,
}

impl Player {
    fn new(items: ArrayVec<Item, MAX_ITEMS>) -> Player {
        Player {
            hp: 100,
            damage: 0,
            armor: 0,
            items,
        }
    }

    fn attack(&self, other: &Player) -> u32 {
        let dmg = self.damage + self.items.iter().map(|i| i.damage).sum::<u32>();
        let armor = other.armor + other.items.iter().map(|i| i.armor).sum::<u32>();
        dmg.saturating_sub(armor).max(1)
    }

    fn cost(&self) -> u32 {
        self.items.iter().map(|i| i.cost).sum()
    }
}

impl FromStr for Player {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Player {
            hp: 0,
            damage: 0,
            armor: 0,
            items: ArrayVec::new(),
        };
        for line in s.lines() {
            let (attr, val) = line.split_once(": ").unwrap();
            let val: u32 = val.parse().unwrap();
            match attr {
                "Hit Points" => p.hp = val,
                "Damage" => p.damage = val,
                "Armor" => p.armor = val,
                _ => unreachable!(),
            }
        }
        Ok(p)
    }
}

#[derive(Debug, Clone)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn new(s: &str) -> Item {
        let mut fields = s.split_whitespace().rev();
        let armor: u32 = fields.next().unwrap().parse().unwrap();
        let damage: u32 = fields.next().unwrap().parse().unwrap();
        let cost: u32 = fields.next().unwrap().parse().unwrap();
        Item {
            cost,
            damage,
            armor,
        }
    }
}
