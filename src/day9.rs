use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use itertools::Itertools;

use regex::Regex;

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG: Regex = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
}

fn parse_input(
    filename: &str,
) -> (HashMap<(String, String), usize>, HashSet<String>) {
    let dists: HashMap<_, _> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|l| {
            if let Some(c) = REG.captures(&l) {
                let from = c[1].to_owned();
                let to = c[2].to_owned();
                let dist = c[3].parse::<usize>().unwrap();
                iter::once(((from.clone(), to.clone()), dist))
                    .chain(iter::once(((to, from), dist)))
            } else {
                panic!("Ill formatted input: {}", l)
            }
        })
        .collect();
    //
    let places = dists
        .keys()
        .flat_map(|(d1, d2)| iter::once(d1).chain(iter::once(d2)))
        .cloned()
        .collect();
    //
    (dists, places)
}

fn part1() {
    let (dists, places) = parse_input("inputfiles/day9/input.txt");
    //
    let ans = places
        .iter()
        .permutations(places.len())
        .map(|p| {
            p.iter()
                .tuple_windows()
                .map(|(&d1, &d2)| dists[&(d1.clone(), d2.clone())])
                .sum::<usize>()
        })
        .min()
        .unwrap();
    //
    println!("{:?}", ans);
}

fn part2() {
    let (dists, places) = parse_input("inputfiles/day9/input.txt");
    //
    let ans = places
        .iter()
        .permutations(places.len())
        .map(|p| {
            p.iter()
                .tuple_windows()
                .map(|(&d1, &d2)| dists[&(d1.clone(), d2.clone())])
                .sum::<usize>()
        })
        .max()
        .unwrap();
    //
    println!("{:?}", ans);
}
