use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG: Regex = Regex::new(r"[A-Z][a-z]?").unwrap();
}

fn load_input(filename: &str) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let lines: Vec<_> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();
    //
    let mut map: HashMap<_, Vec<_>> = HashMap::new();
    //
    for (a, b) in lines[..lines.len() - 2].iter().map(|l| {
        if let [a, b] = l.split(" => ").collect::<Vec<_>>()[..] {
            (a.to_owned(), b.to_owned())
        } else {
            panic!("Ill formatted inputfile!")
        }
    }) {
        if let Some(v) = map.get_mut(&a) {
            v.push(b);
        } else {
            map.insert(a, vec![b]);
        }
    }
    //
    let init = REG
        .find_iter(&lines[lines.len() - 1])
        .map(|x| x.as_str().to_owned())
        .collect::<Vec<_>>();
    //
    (map, init)
}

fn part1() {
    let (map, init) = load_input("inputfiles/day19/input.txt");
    //
    let ans = init
        .iter()
        .enumerate()
        .filter_map(|(i, x)| map.get(x).and_then(|x| Some((i, x))))
        .map(|(i, x)| {
            x.iter().map(move |x| (i, x)).map(|(i, x)| {
                let mut new: Vec<_> = init.iter().collect();
                new[i] = x;
                new
            })
        })
        .flatten()
        .map(|v| v.iter().map(|s| s.chars()).flatten().collect())
        .collect::<HashSet<Vec<_>>>()
        .len();
    //
    println!("{:?}", ans);
}

fn part2() {
    let (_, init) = load_input("inputfiles/day19/input.txt");
    //
    let a = init
        .iter()
        .filter(|x| matches!(x.as_str(), "Rn" | "Ar"))
        .count();
    //
    let b = init.iter().filter(|x| matches!(x.as_str(), "Y")).count();
    //
    println!("{}", init.len() - a - 2 * b - 1);
}
