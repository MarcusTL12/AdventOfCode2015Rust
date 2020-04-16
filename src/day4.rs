use std::{io::Write, iter};

use md5;

pub const PARTS: [fn(); 2] = [part1, part2];

const INPUT: &[u8] = b"ckczppom";

fn checknum(num: usize, buffer: &mut Vec<u8>, amt_zeros: usize) -> bool {
    buffer.clear();
    buffer.extend(INPUT.iter());
    write!(buffer, "{}", num).unwrap();
    format!("{:x}", md5::compute(buffer))
        .chars()
        .take(amt_zeros)
        .eq(iter::repeat('0').take(amt_zeros))
}

fn part1() {
    let ans = (0..)
        .scan(Vec::new(), |buf, i| {
            if checknum(i, buf, 5) {
                None
            } else {
                Some(())
            }
        })
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let ans = (0..)
        .scan(Vec::new(), |buf, i| {
            if checknum(i, buf, 6) {
                None
            } else {
                Some(())
            }
        })
        .count();
    //
    println!("{}", ans);
}
