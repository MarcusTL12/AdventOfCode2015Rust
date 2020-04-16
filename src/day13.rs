use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> HashMap<(String, String), i32> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let [a, _, s, n, _, _, _, _, _, _, b] =
                l.split(' ').collect::<Vec<_>>()[..]
            {
                let mut b = b.to_owned();
                b.pop();
                (
                    (a.to_owned(), b),
                    n.parse::<i32>().unwrap()
                        * match s {
                            "gain" => 1,
                            "lose" => -1,
                            _ => panic!("Other stuff went wrong!"),
                        },
                )
            } else {
                panic!("Stuff went wrong!")
            }
        })
        .collect()
}

fn happiness(pref: &HashMap<(String, String), i32>, order: &[&str]) -> i32 {
    let n1: i32 = order
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| pref[&(a.to_owned(), b.to_owned())])
        .sum();
    //
    let n2: i32 = order
        .iter()
        .rev()
        .tuple_windows()
        .map(|(&a, &b)| pref[&(a.to_owned(), b.to_owned())])
        .sum();
    //
    n1 + n2
        + pref[&(order[0].to_owned(), order[order.len() - 1].to_owned())]
        + pref[&(order[order.len() - 1].to_owned(), order[0].to_owned())]
}

fn part1() {
    let pref = load_input("inputfiles/day13/input.txt");
    //
    let people: HashSet<_> = pref
        .keys()
        .map(|(a, b)| iter::once(a).chain(iter::once(b)))
        .flatten()
        .collect();
    //
    let ans = people
        .iter()
        .permutations(people.len())
        .map(|p| {
            happiness(
                &pref,
                &p.into_iter().map(|&x| x.as_str()).collect::<Vec<_>>(),
            )
        })
        .max()
        .unwrap();
    //
    println!("{:?}", ans);
}

fn part2() {
    let mut pref = load_input("inputfiles/day13/input.txt");
    //
    let you = "You".to_owned();
    //
    let people: HashSet<_> = pref
        .keys()
        .cloned()
        .map(|(a, b)| iter::once(a).chain(iter::once(b)))
        .flatten()
        .chain(iter::once(you.clone()))
        .collect();
    //
    for p in people.iter() {
        pref.insert((you.clone(), p.clone()), 0);
        pref.insert((p.clone(), you.clone()), 0);
    }
    //
    let ans = people
        .iter()
        .permutations(people.len())
        .map(|p| {
            happiness(
                &pref,
                &p.into_iter().map(|x| x.as_str()).collect::<Vec<_>>(),
            )
        })
        .max()
        .unwrap();
    //
    println!("{:?}", ans);
}
