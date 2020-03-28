use std::iter;

use lazy_static::*;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref INPUT: Vec<u8> =
        "1321131112".chars().map(|x| x as u8 - b'0').collect();
}

fn lookandsay(
    it: Box<dyn Iterator<Item = u8>>,
) -> Box<dyn Iterator<Item = u8>> {
    Box::from(
        it.chain(iter::once(0))
            .scan((0, 0), |(a, b), c| {
                if *a != c {
                    let ret = iter::once(*b as u8).chain(iter::once(*a));
                    let first = *a == 0;
                    *a = c;
                    *b = 1;
                    if !first {
                        Some(Some(ret))
                    } else {
                        Some(None)
                    }
                } else {
                    *b += 1;
                    Some(None)
                }
            })
            .filter_map(|x| x)
            .flatten(),
    )
}

fn part1() {
    let init: Box<dyn Iterator<Item = u8>> = Box::from(INPUT.iter().cloned());
    let ans = (0..40).fold(init, |it, _| lookandsay(it)).count();
    //
    println!("{}", ans);
}

fn part2() {
    let init: Box<dyn Iterator<Item = u8>> = Box::from(INPUT.iter().cloned());
    let ans = (0..50).fold(init, |it, _| lookandsay(it)).count();
    //
    println!("{}", ans);
}
