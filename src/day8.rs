use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

// use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn actual_len(s: &str) -> usize {
    s.chars()
        .fold((0, 0), |(s, n), c| {
            if s == 0 {
                (if c == '\\' { -1 } else { 0 }, n + 1)
            } else if s == -1 {
                (if c == 'x' { 2 } else { 0 }, n)
            } else {
                (s - 1, n)
            }
        })
        .1
}

fn part1() {
    let ans: usize =
        BufReader::new(File::open("inputfiles/day8/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.len() - actual_len(&l) + 2)
            .sum();
    //
    println!("ans = {}", ans);
}

fn part2() {
    let ans: usize =
        BufReader::new(File::open("inputfiles/day8/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                l.chars()
                    .flat_map(|c| -> Box<dyn Iterator<Item = char>> {
                        match c {
                            '\\' => Box::from("\\\\".chars()),
                            '"' => Box::from("\\\"".chars()),
                            _ => Box::from(iter::once(c)),
                        }
                    })
                    .count()
                    + 2
                    - l.len()
            })
            .sum();
    //
    println!("ans = {}", ans)
}
