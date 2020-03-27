use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: i32 =
        BufReader::new(File::open("inputfiles/day1/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("Not a parenthesis!"),
            })
            .sum();
    //
    println!("He ended up on floor {}", ans);
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day1/input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Not a parenthesis!"),
        })
        .scan(0, |s, i| {
            *s += i;
            Some(*s)
        })
        .enumerate()
        .find(|(_, x)| *x == -1)
        .expect("Did not go to the basement")
        .0
        + 1;
    //
    println!("Went to the basement on step {}", ans);
}
