use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let ans: usize =
        BufReader::new(File::open("inputfiles/day2/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let (surface, extra) = l
                    .split('x')
                    .map(|n| n.parse::<usize>().unwrap())
                    .tuple_combinations()
                    .map(|(a, b)| a * b)
                    .fold((0, 0), |(s, m), x| {
                        (s + x, if m == 0 || m > x { x } else { m })
                    });
                2 * surface + extra
            })
            .sum();
    //
    println!("They must order {:?} ft²", ans);
}

fn part2() {
    let ans: usize =
        BufReader::new(File::open("inputfiles/day2/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let dims: Vec<usize> =
                    l.split('x').map(|n| n.parse().unwrap()).collect();
                //
                let volume: usize = dims.iter().product();
                let len = dims
                    .into_iter()
                    .combinations(2)
                    .map(|c| c.into_iter().sum::<usize>())
                    .min()
                    .unwrap()
                    * 2;
                volume + len
            })
            .sum();
    println!("They must order {:?} ft", ans);
}
