use std::{collections::HashSet, convert::TryInto};

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref BLACKLIST: HashSet<u8> =
        "iol".chars().map(|x| x as u8 - b'a').collect();
    static ref INPUT: [u8; 8] = "hepxcrrq"
        .chars()
        .rev()
        .map(|x| x as u8 - b'a')
        .collect::<Vec<_>>()[..]
        .try_into()
        .unwrap();
}

fn increment(pass: &mut [u8]) {
    pass.iter_mut().fold(true, |mut s, x| {
        if s {
            *x += 1;
            *x %= 26;
            if *x != 0 {
                s = false;
            }
        }
        s
    });
}

fn isvalid(pass: &[u8]) -> bool {
    let (_, _, r1) =
        pass.iter().fold((0, 0, false), |(last, len, done), &x| {
            if x + 1 == last {
                (x, len + 1, (len) >= 2 || done)
            } else {
                (x, 1, done)
            }
        });
    //
    let r2 = pass.iter().any(|x| BLACKLIST.contains(x));
    //
    let r3 = pass
        .iter()
        .fold((0, false, 0), |(last, b, amt), &x| {
            if b && last == x {
                (0, false, amt + 1)
            } else {
                (x, true, amt)
            }
        })
        .2
        >= 2;
    //
    r1 && !r2 && r3
}

fn part1() {
    let ans = (0..)
        .scan(INPUT.clone(), |buf, _| {
            increment(buf);
            Some(*buf)
        })
        .filter(|x| isvalid(x))
        .next()
        .unwrap()
        .iter()
        .rev()
        .map(|c| (c + b'a') as char)
        .collect::<String>();
    //
    println!("{}", ans);
}

fn part2() {
    let ans = (0..)
        .scan(INPUT.clone(), |buf, _| {
            increment(buf);
            Some(*buf)
        })
        .filter(|x| isvalid(x))
        .skip(10000)
        .next()
        .unwrap()
        .iter()
        .rev()
        .map(|c| (c + b'a') as char)
        .collect::<String>();
    //
    println!("{}", ans);
}
