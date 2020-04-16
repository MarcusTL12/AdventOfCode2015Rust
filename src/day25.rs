use num::traits::{Num, One};

use mod_exp::mod_exp;

pub const PARTS: [fn(); 2] = [part1, part2];

fn find_index<T: Num + One + Copy>(i: T, j: T) -> T {
    (i + j) * (i + j + T::one()) / (T::one() + T::one()) + j
}

fn part1() {
    const X1: u64 = 20151125;
    const A: u64 = 252533;
    const P: u64 = 33554393;
    //
    let ans = (mod_exp(A, find_index(3009, 3018), P) * X1) % P;
    //
    println!("{}", ans);
}

fn part2() {}
