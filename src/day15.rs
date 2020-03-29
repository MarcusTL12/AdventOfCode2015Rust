use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use regex::Regex;

use lazy_static::*;

use num::clamp;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG: Regex = Regex::new(concat!(
        r"\w+: capacity (-?\d+), ",
        r"durability (-?\d+), ",
        r"flavor (-?\d+), ",
        r"texture (-?\d+), ",
        r"calories (-?\d+)"
    ))
    .unwrap();
}

fn load_input(filename: &str) -> Vec<[i64; 5]> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let Some(c) = REG.captures(&l) {
                let mut buf = [0; 5];
                for i in 0..5 {
                    buf[i] = c[i + 1].parse().unwrap();
                }
                buf
            } else {
                panic!("Shit!")
            }
        })
        .collect::<Vec<_>>()
}

fn score(ingredients: &[[i64; 5]], recipie: &[i64]) -> i64 {
    (0..4)
        .map(|i| -> i64 {
            clamp(
                recipie
                    .iter()
                    .zip(ingredients.iter().map(|ing| ing[i]))
                    .map(|(a, b)| a * b)
                    .sum(),
                0,
                i64::max_value(),
            )
        })
        .fold(1, |a, b| a * b)
}

fn cals(ingredients: &[[i64; 5]], recipie: &[i64]) -> i64 {
    recipie
        .iter()
        .zip(ingredients.iter().map(|ing| ing[4]))
        .map(|(a, b)| a * b)
        .sum()
}

fn all_combs(
    n: usize,
    tot: i64,
) -> Box<dyn Iterator<Item = Box<dyn Iterator<Item = i64>>>> {
    if n == 2 {
        Box::from((0..tot + 1).map(move |i| -> Box<dyn Iterator<Item = i64>> {
            Box::from(iter::once(i).chain(iter::once(tot - i)))
        }))
    } else {
        Box::from(
            (0..tot + 1)
                .map(move |i| {
                    all_combs(n - 1, tot - i).map(
                        move |seq| -> Box<dyn Iterator<Item = i64>> {
                            Box::from(iter::once(i).chain(seq))
                        },
                    )
                })
                .flatten(),
        )
    }
}

fn part1() {
    let inp = load_input("inputfiles/day15/input.txt");
    //
    let ans = all_combs(inp.len(), 100)
        .map(|seq| score(&inp, &seq.collect::<Vec<_>>()))
        .max()
        .unwrap();
    //
    println!("{:?}", ans);
}

fn part2() {
    let inp = load_input("inputfiles/day15/input.txt");
    //
    let ans = all_combs(inp.len(), 100)
        .map(|seq| seq.collect::<Vec<_>>())
        .filter(|seq| cals(&inp, &seq) == 500)
        .map(|seq| score(&inp, &seq))
        .max()
        .unwrap();
    //
    println!("{:?}", ans);
}
