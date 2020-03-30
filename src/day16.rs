use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref TAPE: HashMap<&'static str, u32> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .cloned()
    .collect();
}

fn load_input(filename: &str) -> Vec<HashMap<String, u32>> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let s = l.split(' ').collect::<Vec<_>>();
            let temp = s[2..]
                .into_iter()
                .tuples()
                .map(|(&a, &b)| {
                    let mut a = a.to_owned();
                    a.pop();
                    let mut b = b.to_owned();
                    if b.chars().last().unwrap() == ',' {
                        b.pop();
                    }
                    (a, b.parse().unwrap())
                })
                .collect();
            temp
        })
        .collect()
}

fn part1() {
    let inp = load_input("inputfiles/day16/input.txt");
    //
    let ans = inp
        .iter()
        .enumerate()
        .filter_map(|(i, sue)| {
            if sue.iter().all(|(thing, n)| TAPE[thing.as_str()] == *n) {
                Some(i + 1)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    //
    println!("Aunt Sue nr: {}", ans);
}

fn part2() {
    let inp = load_input("inputfiles/day16/input.txt");
    //
    let ans = inp
        .iter()
        .enumerate()
        .filter_map(|(i, sue)| {
            if sue.iter().all(|(thing, n)| match thing.as_str() {
                "cats" | "trees" => TAPE[thing.as_str()] < *n,
                "pomeranians" | "goldfish" => TAPE[thing.as_str()] > *n,
                _ => TAPE[thing.as_str()] == *n,
            }) {
                Some(i + 1)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    //
    println!("Aunt Sue nr: {}", ans);
}
