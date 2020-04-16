use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use num::Complex;

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day3/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '>' => Complex { re: 1, im: 0 },
            '<' => Complex { re: -1, im: 0 },
            '^' => Complex { re: 0, im: 1 },
            'v' => Complex { re: 0, im: -1 },
            _ => panic!("Unexpected character in input!"),
        })
        .scan(Complex { re: 0, im: 0 }, |pos, dir| {
            *pos += dir;
            Some(*pos)
        })
        .chain(iter::once(Complex { re: 0, im: 0 }))
        .collect::<HashSet<_>>()
        .len();
    //
    println!("{} houses got packages", ans);
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day3/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '>' => Complex { re: 1, im: 0 },
            '<' => Complex { re: -1, im: 0 },
            '^' => Complex { re: 0, im: 1 },
            'v' => Complex { re: 0, im: -1 },
            _ => panic!("Unexpected character in input!"),
        })
        .tuples()
        .scan(
            (Complex { re: 0, im: 0 }, Complex { re: 0, im: 0 }),
            |(santa, robo), (d1, d2)| {
                *santa += d1;
                *robo += d2;
                Some(iter::once(*santa).chain(iter::once(*robo)))
            },
        )
        .flatten()
        .chain(iter::once(Complex { re: 0, im: 0 }))
        .collect::<HashSet<_>>()
        .len();
    //
    println!("{:?} houses got packages", ans);
}
