use std::{
    collections::{HashMap, HashSet},
    fs::{File, self},
    io::{BufRead, BufReader}, time::Instant,
};

use itertools::Itertools;

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

fn isnice_naive(s: &str) -> bool {
    lazy_static! {
        static ref VOWELS: HashSet<char> = "aeiou".chars().collect();
        static ref BLACKLIST: HashSet<(char, char)> =
            [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]
                .iter()
                .cloned()
                .collect();
    }
    let r1 = s.chars().filter(|c| VOWELS.contains(&c)).count() >= 3;
    let r2 = s.chars().tuple_windows().any(|(a, b)| a == b);
    let r3 = s.chars().tuple_windows().all(|s| !BLACKLIST.contains(&s));
    r1 && r2 && r3
}

fn isnice_opt(s: &str) -> bool {
    let r1 = s
        .chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        >= 3;
    let r2 = s.chars().tuple_windows().any(|(a, b)| a == b);
    let r3 = s.chars().tuple_windows().all(|s| {
        !matches!(s, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y'))
    });
    r1 && r2 && r3
}

fn part1() {
    let inp = fs::read_to_string("inputfiles/day5/input.txt").unwrap();

    let t = Instant::now();
    let ans = inp.split('\n').filter(|l| isnice_naive(&l)).count();
    let t = t.elapsed();
    println!("Naive: {ans} took {t:?}");

    let t = Instant::now();
    let ans = inp.split('\n').filter(|l| isnice_opt(&l)).count();
    let t = t.elapsed();
    println!("  Opt: {ans} took {t:?}");
}

fn isreallynice(s: &str) -> bool {
    let r1 = s
        .chars()
        .tuple_windows()
        .enumerate()
        .scan(HashMap::new(), |buf, (i, (a, b))| {
            if let Some(v) = buf.get_mut(&(a, b)) {
                Some((i as isize - *v as isize).abs() >= 2)
            } else {
                buf.insert((a, b), i);
                Some(false)
            }
        })
        .any(|b| b);
    let r2 = s.chars().tuple_windows().any(|(a, _, b)| a == b);
    r1 && r2
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day5/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| isreallynice(&l))
        .count();
    println!("{}", ans);
}
