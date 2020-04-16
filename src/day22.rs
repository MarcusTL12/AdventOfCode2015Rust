use std::cmp::max;

use priority_queue::PriorityQueue;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

fn mana_cost(spell: Spell) -> i32 {
    match spell {
        Spell::MagicMissile => 53,
        Spell::Drain => 73,
        Spell::Shield => 113,
        Spell::Poison => 173,
        Spell::Recharge => 229,
    }
}

fn do_turn(
    (player_hp, player_armor, player_mana, boss_hp, boss_dps, effects): &mut (
        i32,
        i32,
        i32,
        i32,
        i32,
        Vec<(Spell, u32)>,
    ),
    spell: Spell,
    diff: i32,
) -> Option<bool> {
    *player_hp -= diff;
    //
    if *player_hp <= 0 {
        return Some(false);
    }
    //
    let mut removestack = Vec::new();
    for (i, (e, d)) in effects.iter_mut().enumerate() {
        *d -= 1;
        match e {
            Spell::Shield => {
                if *d == 0 {
                    *player_armor -= 7;
                }
            }
            Spell::Poison => *boss_hp -= 3,
            Spell::Recharge => *player_mana += 101,
            _ => unreachable!(),
        }
        if *d == 0 {
            removestack.push(i);
        }
    }
    //
    while let Some(i) = removestack.pop() {
        effects.remove(i);
    }
    //
    if effects.iter().find(|(x, _)| *x == spell).is_some() {
        return Some(false);
    }
    //
    match spell {
        Spell::MagicMissile => *boss_hp -= 4,
        Spell::Drain => {
            *boss_hp -= 2;
            *player_hp += 2;
        }
        Spell::Shield => {
            *player_armor += 7;
            effects.push((Spell::Shield, 6));
        }
        Spell::Poison => effects.push((Spell::Poison, 6)),
        Spell::Recharge => effects.push((Spell::Recharge, 5)),
    }
    //
    *player_mana -= mana_cost(spell);
    //
    if *player_mana < 0 {
        return Some(false);
    }
    //
    for (i, (e, d)) in effects.iter_mut().enumerate() {
        *d -= 1;
        match e {
            Spell::Shield => {
                if *d == 0 {
                    *player_armor -= 7;
                }
            }
            Spell::Poison => *boss_hp -= 3,
            Spell::Recharge => *player_mana += 101,
            _ => unreachable!(),
        }
        if *d == 0 {
            removestack.push(i);
        }
    }
    //
    while let Some(i) = removestack.pop() {
        effects.remove(i);
    }
    //
    if *player_hp <= 0 {
        return Some(false);
    } else if *boss_hp <= 0 {
        return Some(true);
    }
    //
    *player_hp -= max(*boss_dps - *player_armor, 1);
    //
    if *player_hp <= 0 {
        return Some(false);
    } else if *boss_hp <= 0 {
        return Some(true);
    }
    //
    None
}

fn dijkstra(
    state: (i32, i32, i32, i32, i32, Vec<(Spell, u32)>),
    diff: i32,
) -> i32 {
    const SPELLS: [Spell; 5] = [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ];
    //
    let mut queue = PriorityQueue::new();
    queue.push((state, Vec::new()), 0);
    //
    while let Some(((state, spells), mana_usage)) = queue.pop() {
        for &spell in SPELLS.iter() {
            let mut nstate = state.clone();
            let mut nspells = spells.clone();
            let outcome = do_turn(&mut nstate, spell, diff);
            match outcome {
                Some(true) => {
                    return mana_cost(spell) - mana_usage;
                }
                Some(false) => (),
                None => {
                    nspells.push(spell);
                    queue
                        .push((nstate, nspells), mana_usage - mana_cost(spell));
                }
            }
        }
    }
    panic!("Impossible to win")
}

fn part1() {
    let player_hp = 50;
    let player_armor = 0;
    let player_mana = 500;
    //
    let boss_hp = 58;
    let boss_dps = 9;
    //
    let effects = Vec::new();
    //
    let state = (
        player_hp,
        player_armor,
        player_mana,
        boss_hp,
        boss_dps,
        effects,
    );
    //
    let ans = dijkstra(state, 0);
    //
    println!("{:?}", ans);
}

fn part2() {
    let player_hp = 50;
    let player_armor = 0;
    let player_mana = 500;
    //
    let boss_hp = 58;
    let boss_dps = 9;
    //
    let effects = Vec::new();
    //
    let state = (
        player_hp,
        player_armor,
        player_mana,
        boss_hp,
        boss_dps,
        effects,
    );
    //
    let ans = dijkstra(state, 1);
    //
    println!("{:?}", ans);
}
