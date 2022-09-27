use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1_naive(s: &str) -> i32 {
    s.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn part1_opt(s: &[u8]) -> i32 {
    s.iter()
        .map(|c| match c {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum()
}

fn part1() {
    let inp: String =
        BufReader::new(File::open("inputfiles/day1/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap();
    //
    let t = Instant::now();
    let ans_naive = part1_naive(&inp);
    println!("Naive: {ans_naive} took {:?}", t.elapsed());

    let t = Instant::now();
    let ans_opt = part1_opt(inp.as_bytes());
    println!("Opt:   {ans_opt} took {:?}", t.elapsed());
}

fn part2_naive(s: &str) -> usize {
    unsafe {
        s.chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .scan(0, |s, i| {
                *s += i;
                Some(*s)
            })
            .enumerate()
            .find(|(_, x)| *x == -1)
            .unwrap_unchecked()
    }
    .0 + 1
}

fn part2_opt(s: &[u8]) -> usize {
    unsafe {
        s.iter()
            .map(|c| match c {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            })
            .scan(0, |s, i| {
                *s += i;
                Some(*s)
            })
            .enumerate()
            .find(|(_, x)| *x == -1)
            .unwrap_unchecked()
    }
    .0 + 1
}

fn part2() {
    let inp: String =
        BufReader::new(File::open("inputfiles/day1/input.txt").unwrap())
            .lines()
            .next()
            .unwrap()
            .unwrap();
    //
    let t = Instant::now();
    let ans_naive = part2_naive(&inp);
    println!("Naive: {ans_naive} took {:?}", t.elapsed());

    let t = Instant::now();
    let ans_opt = part2_opt(inp.as_bytes());
    println!("Opt:   {ans_opt} took {:?}", t.elapsed());
}
