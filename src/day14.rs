use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use num::clamp;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> Vec<(u32, u32, u32)> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            if let [_, _, _, speed, _, _, time, _, _, _, _, _, _, rest, _] =
                l.split(' ').collect::<Vec<_>>()[..]
            {
                (
                    speed.parse().unwrap(),
                    time.parse().unwrap(),
                    rest.parse().unwrap(),
                )
            } else {
                panic!("Oh damn")
            }
        })
        .collect()
}

fn raindist(t: u32, (speed, time, rest): (u32, u32, u32)) -> u32 {
    (t / (time + rest)) * speed * time
        + clamp(t % (time + rest), 0, time) * speed
}

fn part1() {
    let inp = load_input("inputfiles/day14/input.txt");
    //
    let ans = inp.iter().map(|&r| raindist(2503, r)).max().unwrap();
    //
    println!("{}", ans);
}

fn part2() {
    let inp = load_input("inputfiles/day14/input.txt");
    //
    let ans = (0..2503)
        .fold(vec![0u32; inp.len()], |mut scores, t| {
            let m = inp.iter().map(|&r| raindist(t + 1, r)).max().unwrap();
            //
            for (i, _) in inp
                .iter()
                .enumerate()
                .filter(|(_, &r)| raindist(t + 1, r) == m)
            {
                scores[i] += 1;
            }
            //
            scores
        })
        .into_iter()
        .max()
        .unwrap();
    //
    println!("{:?}", ans);
}
