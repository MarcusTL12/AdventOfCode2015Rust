use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use lazy_static::*;

use regex::Regex;

use itertools::Itertools;

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

fn do_turn(
    (p_hp, p_dps, p_armor): &mut (i32, i32, i32),
    (b_hp, b_dps, b_armor): &mut (i32, i32, i32),
) -> Option<bool> {
    *b_hp -= max(*p_dps - *b_armor, 1);
    //
    if *b_hp <= 0 {
        return Some(true);
    }
    //
    *p_hp -= max(*b_dps - *p_armor, 1);
    //
    if *p_hp <= 0 {
        return Some(false);
    }
    //
    None
}

fn playgame(mut player: (i32, i32, i32), mut boss: (i32, i32, i32)) -> bool {
    let mut outcome = None;
    //
    while outcome.is_none() {
        outcome = do_turn(&mut player, &mut boss);
    }
    //
    outcome.unwrap()
}

fn part1() {
    let (weapons, armor, rings) = load_store();
    let boss = load_boss();
    //
    let ans = weapons
        .iter()
        .map(|(_, c, d, a)| (*c, *d, *a))
        .cartesian_product(
            iter::once((0, 0, 0))
                .chain(armor.iter().map(|(_, c, d, a)| (*c, *d, *a))),
        )
        .cartesian_product(iter::once((0, 0, 0)).chain(
            rings.iter().map(|(_, c, d, a)| (*c, *d, *a)).chain(
                rings.iter().enumerate().flat_map(|(i, (_, c1, d1, a1))| {
                    rings.iter().skip(i + 1).map(move |(_, c2, d2, a2)| {
                        (*c1 + *c2, *d1 + *d2, *a1 + *a2)
                    })
                }),
            ),
        ))
        .map(|(((c1, d1, a1), (c2, d2, a2)), (c3, d3, a3))| {
            (c1 + c2 + c3, d1 + d2 + d3, a1 + a2 + a3)
        })
        .filter(|&(_, d, a)| playgame((100, d, a), boss))
        .map(|(c, _, _)| c)
        .min()
        .unwrap();
    //
    println!("{}", ans);
}

fn part2() {
    let (weapons, armor, rings) = load_store();
    let boss = load_boss();
    //
    let ans = weapons
        .iter()
        .map(|(_, c, d, a)| (*c, *d, *a))
        .cartesian_product(
            iter::once((0, 0, 0))
                .chain(armor.iter().map(|(_, c, d, a)| (*c, *d, *a))),
        )
        .cartesian_product(iter::once((0, 0, 0)).chain(
            rings.iter().map(|(_, c, d, a)| (*c, *d, *a)).chain(
                rings.iter().enumerate().flat_map(|(i, (_, c1, d1, a1))| {
                    rings.iter().skip(i + 1).map(move |(_, c2, d2, a2)| {
                        (*c1 + *c2, *d1 + *d2, *a1 + *a2)
                    })
                }),
            ),
        ))
        .map(|(((c1, d1, a1), (c2, d2, a2)), (c3, d3, a3))| {
            (c1 + c2 + c3, d1 + d2 + d3, a1 + a2 + a3)
        })
        .filter(|&(_, d, a)| !playgame((100, d, a), boss))
        .map(|(c, _, _)| c)
        .max()
        .unwrap();
    //
    println!("{}", ans);
}
