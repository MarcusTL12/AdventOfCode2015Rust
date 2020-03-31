use std::iter;

use divisors::get_divisors;

pub const PARTS: [fn(); 2] = [part1, part2];

const INPUT: u64 = 29000000;

fn packages(housenr: u64) -> u64 {
    (get_divisors(housenr).into_iter().sum::<u64>() + 1 + housenr) * 10
}

fn part1() {
    let ans = (1..)
        .map(|i| (i, packages(i)))
        .filter(|(_, x)| *x >= INPUT)
        .next()
        .unwrap()
        .0;
    //
    println!("{}", ans);
}

fn packages2(housenr: u64) -> u64 {
    let div = match housenr {
        1 | 2 => vec![],
        _ => get_divisors(housenr),
    };
    //
    iter::once(1)
        .chain(div.into_iter())
        .chain(iter::once(housenr))
        .filter(|&x| x >= housenr / 50)
        .sum::<u64>()
        * 11
}

fn part2() {
    let ans = (1..)
        .map(|i| (i, packages2(i)))
        .filter(|(_, x)| *x >= INPUT)
        .next()
        .unwrap()
        .0;
    //
    println!("{}", ans);
}
