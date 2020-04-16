use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use lazy_static::*;

use regex::Regex;

use itertools::Itertools;

use num::Integer;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_store() -> (
    Vec<(String, i32, i32, i32)>,
    Vec<(String, i32, i32, i32)>,
    Vec<(String, i32, i32, i32)>,
) {
    fn parse_item(s: &str) -> (String, i32, i32, i32) {
        lazy_static! {
            static ref REG: Regex =
                Regex::new(r"(\w+(?: \+\d+)?)\s+(\d+)\s+(\d+)\s+(\d+)")
                    .unwrap();
        }
        if let Some(c) = REG.captures(s) {
            let name = c[1].to_owned();
            let cost = c[2].parse::<i32>().unwrap();
            let damage = c[3].parse::<i32>().unwrap();
            let armor = c[4].parse::<i32>().unwrap();
            //
            (name, cost, damage, armor)
        } else {
            panic!()
        }
    }
    //
    let lines: Vec<_> =
        BufReader::new(File::open("inputfiles/day21/store.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect();
    //
    fn find_items(
        lines: &[String],
        itemtype: &str,
    ) -> Vec<(String, i32, i32, i32)> {
        let offset = lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.starts_with(itemtype))
            .next()
            .unwrap()
            .0;
        lines
            .iter()
            .skip(offset + 1)
            .take_while(|l| l.len() > 0)
            .map(|l| parse_item(l))
            .collect::<Vec<_>>()
    }
    //
    let weapons = find_items(&lines, "Weapons");
    let armor = find_items(&lines, "Armor");
    let rings = find_items(&lines, "Rings");
    //
    (weapons, armor, rings)
}

fn load_boss() -> (i32, i32, i32) {
    let mut boss = (0, 0, 0);
    for l in BufReader::new(File::open("inputfiles/day21/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        match &l.split(": ").collect::<Vec<_>>()[..] {
            ["Hit Points", a] => boss.0 = a.parse().unwrap(),
            ["Damage", a] => boss.1 = a.parse().unwrap(),
            ["Armor", a] => boss.2 = a.parse().unwrap(),
            _ => panic!(),
        }
    }
    boss
}

fn _maketurn(attacker: &(i32, i32, i32), defender: &mut (i32, i32, i32)) {
    let damage = if defender.2 > attacker.1 {
        1
    } else {
        attacker.1 - defender.2
    };
    //
    defender.0 -= damage;
}

// fn playgame(mut player: (i32, i32, i32), mut boss: (i32, i32, i32)) -> bool {
//     let mut attacker = &mut player;
//     let mut defender = &mut boss;
//     //
//     let mut turn = false;
//     while attacker.0 > 0 && defender.0 > 0 {
//         maketurn(attacker, defender);
//         //
//         let temp = attacker;
//         attacker = defender;
//         defender = temp;
//         //
//         turn = !turn;
//     }
//     turn
// }

fn playgame(player: (i32, i32, i32), boss: (i32, i32, i32)) -> bool {
    let pd = Integer::div_ceil(&player.0, &cmp::max(boss.1 - player.2, 1));
    let bd = Integer::div_ceil(&boss.0, &cmp::max(player.1 - boss.2, 1));
    pd > bd
}

fn part1() {
    let (weapons, armor, rings) = load_store();
    let boss = load_boss();
    //
    let weap_combs = weapons.iter().map(|(_, c, d, a)| (*c, *d, *a));
    //
    let arm_combs = iter::once((0, 0, 0))
        .chain(armor.iter().map(|(_, c, d, a)| (*c, *d, *a)));
    //
    let ring_combs = iter::once((0, 0, 0))
        .chain(rings.iter().map(|(_, c, d, a)| (*c, *d, *a)))
        .chain(
            rings
                .iter()
                .combinations(2)
                .map(|v| (v[0].1 + v[1].1, v[0].2 + v[1].2, v[0].3 + v[1].3)),
        );
    //
    let temp = weap_combs
        .cartesian_product(arm_combs)
        .cartesian_product(ring_combs)
        .map(|(((c1, d1, a1), (c2, d2, a2)), (c3, d3, a3))| {
            (c1 + c2 + c3, d1 + d2 + d3, a1 + a2 + a3)
        })
        .filter(|&player| playgame(player, boss))
        // .min_by(|(a, _, _), (b, _, _)| a.cmp(b))
        .map(|(c, _, _)| c)
        .min()
        .unwrap();
    // .collect::<Vec<_>>();
    //
    println!("{:?}", temp);
}

fn part2() {}
