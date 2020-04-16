use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use lazy_static::*;

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG: Regex =
        Regex::new(r"(\w+ ?\w*) (\d+),(\d+) through (\d+),(\d+)").unwrap();
}

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day6/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let Some(c) = REG.captures(&l) {
                (
                    match &c[1] {
                        "turn on" => 0,
                        "toggle" => 1,
                        "turn off" => 2,
                        _ => panic!("Unexpected action!"),
                    },
                    c[2].parse::<usize>().unwrap(),
                    c[3].parse::<usize>().unwrap(),
                    c[4].parse::<usize>().unwrap(),
                    c[5].parse::<usize>().unwrap(),
                )
            } else {
                panic!("Line did not match regex!\n{}", l)
            }
        })
        .fold(
            vec![vec![false; 1000]; 1000],
            |grid, (action, x1, y1, x2, y2)| {
                let mut grid = grid;
                for y in y1..y2 + 1 {
                    for x in x1..x2 + 1 {
                        match action {
                            0 => grid[y][x] = true,
                            1 => grid[y][x] = !grid[y][x],
                            2 => grid[y][x] = false,
                            _ => panic!("Unexpected action v2!"),
                        }
                    }
                }
                grid
            },
        )
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x)
        .count();
    println!("{:?}", ans);
}

fn part2() {
    let ans: i32 =
        BufReader::new(File::open("inputfiles/day6/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Some(c) = REG.captures(&l) {
                    (
                        match &c[1] {
                            "turn on" => 1,
                            "toggle" => 2,
                            "turn off" => -1,
                            _ => panic!("Unexpected action!"),
                        },
                        c[2].parse::<usize>().unwrap(),
                        c[3].parse::<usize>().unwrap(),
                        c[4].parse::<usize>().unwrap(),
                        c[5].parse::<usize>().unwrap(),
                    )
                } else {
                    panic!("Line did not match regex!\n{}", l)
                }
            })
            .fold(
                vec![vec![0; 1000]; 1000],
                |grid, (action, x1, y1, x2, y2)| {
                    let mut grid = grid;
                    for y in y1..y2 + 1 {
                        for x in x1..x2 + 1 {
                            grid[y][x] += action;
                            if grid[y][x] < 0 {
                                grid[y][x] = 0;
                            }
                        }
                    }
                    grid
                },
            )
            .iter()
            .flat_map(|row| row.iter())
            .sum();
    println!("{:?}", ans);
}
