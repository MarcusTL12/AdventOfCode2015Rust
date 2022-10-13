use std::{io::Write, iter, time::Instant};

use rayon::{current_num_threads, prelude::*};

use md5;

pub const PARTS: [fn(); 2] = [part1, part2];

const INPUT: &[u8] = b"ckczppom";

fn checknum<const AMT_ZEROS: usize>(num: usize, buffer: &mut Vec<u8>) -> bool {
    buffer.clear();
    buffer.extend(INPUT.iter());
    write!(buffer, "{}", num).unwrap();
    format!("{:x}", md5::compute(buffer))
        .chars()
        .take(AMT_ZEROS)
        .eq(iter::repeat('0').take(AMT_ZEROS))
}

fn look_for_zeros<const AMT_ZEROS: usize>(
    start: usize,
    step: usize,
    amt: usize,
    buffer: &mut Vec<u8>,
) -> Option<usize> {
    (start..)
        .step_by(step)
        .take(amt)
        .filter(|&i| checknum::<AMT_ZEROS>(i, buffer))
        .next()
}

fn solve_naive<const AMT_ZEROS: usize>() -> usize {
    (0..)
        .scan(Vec::new(), |buf, i| {
            if checknum::<AMT_ZEROS>(i, buf) {
                None
            } else {
                Some(())
            }
        })
        .count()
}

fn solve_par<const AMT_ZEROS: usize>() -> usize {
    const CHUNKSIZE: usize = 1024;

    let nt = current_num_threads();

    let mut results = Vec::new();

    for start in (0..).step_by(CHUNKSIZE * nt) {
        (0..nt).into_par_iter().map_init(
            || Vec::new(),
            |buffer, i| {
                look_for_zeros::<AMT_ZEROS>(start + i, nt, CHUNKSIZE, buffer)
            },
        ).collect_into_vec(&mut results);

        if let Some(x) = results.iter().filter_map(|&x| x).next() {
            return x;
        }
    }

    unreachable!()
}

fn part1() {
    const AMT_ZEROS: usize = 5;

    let t = Instant::now();
    let ans = solve_naive::<AMT_ZEROS>();
    let t = t.elapsed();
    println!("Naive: {ans} took {t:?}");

    let t = Instant::now();
    let ans = solve_par::<AMT_ZEROS>();
    let t = t.elapsed();
    println!("  Par: {ans} took {t:?}");
}

fn part2() {
    const AMT_ZEROS: usize = 6;

    let t = Instant::now();
    let ans = solve_naive::<AMT_ZEROS>();
    let t = t.elapsed();
    println!("Naive: {ans} took {t:?}");

    let t = Instant::now();
    let ans = solve_par::<AMT_ZEROS>();
    let t = t.elapsed();
    println!("  Par: {ans} took {t:?}");
}
