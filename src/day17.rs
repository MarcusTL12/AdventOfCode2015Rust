use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> Vec<u32> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn all_combs(
    n: usize,
) -> Box<dyn Iterator<Item = Box<dyn Iterator<Item = bool>>>> {
    if n == 1 {
        let f: Box<dyn Iterator<Item = bool>> = Box::from(iter::once(false));
        let t: Box<dyn Iterator<Item = bool>> = Box::from(iter::once(true));
        Box::from(iter::once(f).chain(iter::once(t)))
    } else {
        Box::from(
            [false, true]
                .iter()
                .map(move |&x| {
                    all_combs(n - 1).map(
                        move |y| -> Box<dyn Iterator<Item = bool>> {
                            Box::from(iter::once(x).chain(y))
                        },
                    )
                })
                .flatten(),
        )
    }
}

fn part1() {
    let inp = load_input("inputfiles/day17/input.txt");
    //
    let ans = all_combs(inp.len())
        .map(|comb| {
            comb.zip(inp.iter())
                .filter_map(
                    |(include, &size)| if include { Some(size) } else { None },
                )
                .sum::<u32>()
                == 150
        })
        .filter(|&x| x)
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let inp = load_input("inputfiles/day17/input.txt");
    //
    let amts = all_combs(inp.len())
        .filter_map(|comb| {
            let mut amt = 0;
            if comb
                .zip(inp.iter())
                .filter_map(|(include, &size)| {
                    if include {
                        amt += 1;
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u32>()
                == 150
            {
                Some(amt)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    //
    let min = amts.iter().min().unwrap();
    //
    let ans = amts.iter().filter(|&x| x == min).count();
    //
    println!("{}", ans);
}
